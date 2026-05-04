# Rust Entry Point Parity PRD

## Goal

Bring the root Rust CLI entry point to parity with `/Users/mickillah/jq/src/main.c` for option parsing, runtime setup, input handling, output and error handling, cleanup, and exit codes.

The selected path is to implement parity in the active root crate only, centered on `src/main.rs` and the minimum supporting public interfaces needed by the translated modules. Directories whose names start with `jq`, including `jqvert-test/`, `jqvert-test_rwasm_rewrite/`, and `jq_port_*`, are out of scope and must not be modified.

## Source of Truth

- C parity oracle: `/Users/mickillah/jq/src/main.c`.
- Rust target: root crate in `/Users/mickillah/Code/rust_projects/jqrs`.
- Test oracle: compare Rust CLI behavior against the C jq binary for stdout, stderr, and process exit code.

## Current Gaps

- Status constants do not match C. Rust currently gives different numeric values for `JQ_OK_NO_OUTPUT`, `JQ_OK_NULL_KIND`, `JQ_ERROR_SYSTEM`, `JQ_ERROR_UNKNOWN`, and option bit flags.
- Exit helpers do not fully match C. C uses `jq_exit_with_status(r) = exit(abs(r))` for `-e` and `jq_exit(r) = exit(r > 0 ? r : 0)` otherwise.
- Missing options: `--rawfile`, `--slurpfile`, and `--run-tests` are not wired in `src/main.rs`.
- Input callback wiring is missing. C calls `jq_set_input_cb(jq, jq_util_input_next_input_cb, input_state)` so `input`, `inputs`, `input_filename`, and `input_line_number` can consume the same input stream as the main loop.
- `JQ_COLORS` is not applied. Rust should call `jq_set_colors(Some(value))` and print `Failed to set $JQ_COLORS` on rejection.
- Runtime attributes and environment handling are incomplete or weaker: `JQ_LIBRARY_PATH`, `JQ_ORIGIN`, `PROGRAM_ORIGIN`, `VERSION_DIR`, and `JQ_BUILD_CONFIGURATION` must match C behavior, including realpath handling where C uses `jq_realpath`.
- Build configuration output is incomplete. `--build-configuration` and `$JQ_BUILD_CONFIGURATION` should use the Rust equivalent of C `JQ_CONFIG`; if the C build's `JQ_CONFIG` is empty, Rust should preserve that empty value for byte-for-byte parity.
- Error reporting is weaker. Runtime errors should include input position text like `jq: error (at <position>): <message>`, and non-string errors should follow C formatting.
- Stdout write failure handling is missing. C checks `ferror(stdout)` and `fclose(stdout)` before exit and reports `jq: error: writing output failed: <errno text>`.
- `--seq` behavior is incomplete. C treats parse errors as non-fatal under `--seq` and logs `jq: ignoring parse error: <message>`.
- Cleanup is incomplete. C frees `ARGS`, `program_arguments`, input state, and jq state on all `goto out` paths after checking output failure.

## Implementation Phases

### 1. Add Parity Test Harness

Create an integration harness that runs the Rust CLI and the C jq reference on the same cases and compares:

- exit code
- stdout bytes
- stderr bytes

Recommended shape:

- Add `tests/entrypoint_parity.rs`.
- Build the Rust binary with Cargo's `CARGO_BIN_EXE_jq_with_autobuild` integration-test support.
- Resolve the C jq binary from an environment variable such as `JQ_C_BIN`, defaulting to `/Users/mickillah/jq/jq` if present.
- Skip parity tests with a clear message if the C binary is unavailable.
- Use `tempfile` for fixture files and shell-free `std::process::Command` invocation.
- Keep byte comparisons exact for stable cases. For OS-dependent path or errno strings, compare normalized stderr or assert required substrings.

Initial fixtures should cover stable behavior before implementation changes:

- `--help`, `--version`, `--build-configuration`
- unknown option and missing option parameter errors
- `--arg`, `--argjson`, `--args`, `--jsonargs`
- stdin input, file input, multiple file inputs, and missing files
- raw/slurp input combinations
- `-e` exit-status cases
- parse errors with and without `--seq`
- runtime errors and `halt_error`

This harness must be the first phase so every behavior change can be validated against the C entry point.

### 2. Align Status Constants and Exit Helpers

Update `src/main.rs` constants to match C exactly:

```text
JQ_OK = 0
JQ_OK_NULL_KIND = -1
JQ_ERROR_SYSTEM = 2
JQ_ERROR_COMPILE = 3
JQ_OK_NO_OUTPUT = -4
JQ_ERROR_UNKNOWN = 5
```

Align option bits with C:

```text
SLURP = 1
RAW_INPUT = 2
PROVIDE_NULL = 4
RAW_OUTPUT = 8
RAW_OUTPUT0 = 16
ASCII_OUTPUT = 32
COLOR_OUTPUT = 64
NO_COLOR_OUTPUT = 128
SORTED_OUTPUT = 256
FROM_FILE = 512
RAW_NO_LF = 1024
UNBUFFERED_OUTPUT = 2048
EXIT_STATUS = 4096
SEQ = 16384
RUN_TESTS = 32768
DUMP_DISASM = 65536
```

Add local helpers in `src/main.rs`:

- `jq_exit_with_status(ret: i32) -> !` exits with `ret.abs()`.
- `jq_exit(ret: i32) -> !` exits with `ret` only when `ret > 0`, otherwise exits `0`.

Use these helpers on every final exit path, including compile failures, system errors, `-e`, and early option exits.

### 3. Implement Missing Options

Implement `--rawfile name file` and `--slurpfile name file` in the option loop:

- Both require two following parameters.
- Preserve C's "first definition wins" behavior: if `program_arguments` already has `name`, skip loading.
- Use `jv_load_file(file, raw)` where `raw` is true for `--rawfile` and false for `--slurpfile`.
- On invalid data, print:
  - `<progname>: Bad JSON in --<which> <name> <file>: <message>`
- Set `ret = JQ_ERROR_SYSTEM` and go through common cleanup.

Implement `--run-tests` only if the translated test-suite binding is usable:

- Wire to `jq_testsuite(lib_search_paths, verbose, argc_remaining, argv_remaining)`.
- `verbose` is true when `DUMP_DISASM` or `JQ_DEBUG_TRACE` is active.
- If the binding is not yet semantically usable, add a focused parity test marked ignored and document the blocker in code comments near the option branch.

Keep option stacking semantics identical to C by preserving `isoption` and `short_opts` behavior.

### 4. Match Runtime Attributes and Environment Behavior

Apply terminal and color logic in C order:

- If stdout is a TTY, set `JV_PRINT_ISATTY | JV_PRINT_COLOR`.
- Honor non-empty `NO_COLOR` by clearing `JV_PRINT_COLOR`.
- Apply `-S`, `-a`, `-C`, and `-M` after TTY detection.
- If `JQ_COLORS` is present, call `jq_set_colors(Some(value))`; if it returns false, print `Failed to set $JQ_COLORS`.

Set runtime attributes to match C:

- `JQ_LIBRARY_PATH`: use user-provided `-L` paths if any; otherwise default to `["~/.jq", "$ORIGIN/../lib/jq", "$ORIGIN/../lib"]`.
- For `-L`, pass paths through `jq_realpath` like C.
- `JQ_ORIGIN`: dirname of `argv[0]`.
- `VERSION_DIR`: `JQ_VERSION` when there is no hyphen; otherwise `<prefix-before-hyphen>-master`.
- `PROGRAM_ORIGIN`: for `-f`, dirname of the filter file passed through `jq_realpath`; otherwise `jq_realpath(".")`.
- `JQ_BUILD_CONFIGURATION`: use the Rust equivalent of `JQ_CONFIG` and expose the same string via `--build-configuration`.

Add a single Rust build-configuration constant near the CLI constants, generated or maintained from the translated config source. Do not synthesize a Rust-specific value when the C oracle reports an empty `JQ_CONFIG`; in that case `--build-configuration` and `$JQ_BUILD_CONFIGURATION` should also be empty.

### 5. Wire Input Callbacks and Parser/Input Loop Behavior

Use one shared input state for:

- the main CLI input loop
- the jq `input` and `inputs` builtins
- `input_filename`
- `input_line_number`
- runtime error position reporting

Selected implementation path:

- Keep `jq_util_input_state` ownership in the Rust CLI through a shared handle such as `Rc<RefCell<JqUtilInputState>>`.
- Use a CLI callback-data struct for `JqState` callback data that can hold `dumpopts` and the shared input-state handle.
- Wire `jq_set_input_cb` from `src/main.rs` before execution begins.
- Make `jq_util_input_next_input_cb` or a small CLI wrapper call `jq_util_input_next_input` on the shared state.
- Update builtin `f_input` to call the registered input callback and return `invalid("break")` only when the callback returns no valid value and no invalid message, matching C `input`/`inputs` expectations.
- Update `jq_util_input_get_position`, `input_filename`, and `input_line_number` to read the shared state instead of returning placeholders.

Parser setup must match C:

- If `RAW_INPUT`, call `jq_util_input_set_parser(input_state, None, slurp)`.
- Otherwise call `jq_util_input_set_parser(input_state, Some(jv_parser_new(parser_flags)), slurp)`.
- Add `JV_PARSE_SEQ` to parser flags after compilation and before parser creation when `--seq` is set.
- Add `-` as input when no files were supplied.

Main input loop behavior must match C:

- Continue while `jq_util_input_errors(input_state) == 0` and the next value is valid or invalid with a message.
- For valid values, call `process`, update `last_result`, and stop if `jq_halted`.
- For parse errors without `--seq`, set `ret = JQ_ERROR_UNKNOWN`, print `jq: parse error: <message>`, and stop.
- For parse errors with `--seq`, print `jq: ignoring parse error: <message>`, free/drop the message, keep going, and do not set `ret` to an error.
- After the loop, if input errors are non-zero, set `ret = JQ_ERROR_SYSTEM`.

### 6. Match Runtime Errors, Halt Behavior, Cleanup, and Output Failure Handling

Update `process` to match C:

- Initialize `ret = JQ_OK_NO_OUTPUT`.
- For raw string output, use byte length semantics. Under `--raw-output0`, reject strings containing NUL with the exact C message.
- For non-raw output under `--seq`, write ASCII record separator `\x1e` before each JSON value.
- For false or null output, set `ret = JQ_OK_NULL_KIND`; for other valid output, set `ret = JQ_OK`.
- After `jq_halted`, use `jq_get_exit_code` and `jq_get_error_message` exactly like C:
  - invalid exit code means `JQ_OK`
  - numeric exit code becomes the return code
  - nonnumeric valid exit code means `JQ_ERROR_UNKNOWN`
  - string halt error is written to stderr without a prefix
  - null halt error writes nothing
  - other valid halt error is dumped as a string with a trailing newline
- For uncaught jq exceptions, print:
  - `jq: error (at <position>): <message>`
  - `jq: error (at <position>) (not a string): <dumped-message>`

Replace fire-and-forget stdout writes with an output path that records write and flush failures:

- Record failures from raw writes, JSON dumps, newlines, NUL delimiters, record separators, and unbuffered flushes.
- Before final exit, flush and close/drop stdout in a way that detects broken pipe or other write errors where Rust can observe them.
- If output failed, print `jq: error: writing output failed: <error>` and set `ret = JQ_ERROR_SYSTEM`.

Use a common cleanup path:

- Drop/free `ARGS`.
- Drop/free `program_arguments`.
- Free input state through `jq_util_input_free`.
- Teardown jq through `jq_teardown`.
- Perform final exit through the C-compatible helpers.

## Public Interfaces

Add or expose only the interfaces needed by the entry point parity work:

- `jq_set_input_cb`: must support a CLI callback that can read from the shared input state.
- `jq_set_colors`: expose from `jv_print` to `main.rs` and call it for `JQ_COLORS`.
- `jv_load_file`: expose from `jv_file` to `main.rs` for `--rawfile`, `--slurpfile`, and `-f` parity.
- `jq_realpath`: use for `-L`, `PROGRAM_ORIGIN`, and default program origin parity.
- Build config constant: expose a Rust `JQ_CONFIG` equivalent for `--build-configuration` and `$JQ_BUILD_CONFIGURATION`.
- Test suite invocation: expose `jq_testsuite` only if the translated implementation is usable enough for `--run-tests`.
- Input position helpers: expose current filename and line from `jq_util_input_state` so runtime errors and builtins can match C.

Do not introduce parallel value types or ad hoc JSON parsing. Use existing `Jv`, `jv_parse`, `jv_load_file`, `jq_util_input_*`, and `jq_set_attr` helpers.

## Test Plan

Run targeted parity tests first, then the full Rust test suite.

Required integration cases:

- `--help` writes full help to stdout and exits `0`.
- no program on TTY-like invocation prints short usage to stderr and exits `2`.
- `--version` prints `jq-<version>` and exits `0`.
- `--build-configuration` prints the build config and exits `0`.
- unknown long option and unknown stacked short option match C stderr and exit `2`.
- missing parameters for `-L`, `--indent`, `--arg`, `--argjson`, `--rawfile`, and `--slurpfile`.
- `--indent -1`, `--indent 0`, `--indent 7`, and invalid indent values.
- `-Ldir` and `-L dir` populate `JQ_LIBRARY_PATH`.
- `--arg`, duplicate `--arg`, `--argjson`, invalid `--argjson`, `--args`, and `--jsonargs`.
- `--rawfile` loads bytes as a string; `--slurpfile` loads parsed JSON values as an array; invalid slurp JSON reports a system error.
- filter from command line and filter from `-f`.
- stdin input, single file input, multiple files, missing file input, raw input, slurp input, and raw slurp input.
- `-e` exits `0` for truthy output, `1` for false/null, `4` for no output, and positive runtime errors as C does.
- JSON parse error without `--seq` is fatal and uses `jq: parse error:`.
- JSON parse error with `--seq` is non-fatal and uses `jq: ignoring parse error:`.
- `--seq` output prefixes each JSON output with record separator.
- runtime `error`, non-string error, `halt`, and `halt_error`.
- `input`, `inputs`, `input_filename`, and `input_line_number` consume and report the same input stream as C.
- `debug` and `stderr` builtins write with C-compatible formatting.
- broken-output behavior where practical, such as piping to a process that closes early.

Verification commands:

```sh
cargo test entrypoint_parity
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Assumptions

- The active implementation scope is the root Rust crate only.
- `/Users/mickillah/jq/src/main.c` is the source of truth for entry-point behavior.
- Directories whose names start with `jq` are reference, generated, or archived material and are out of scope.
- Existing translated module boundaries should be preserved unless an interface must change to match C entry-point behavior.
- Windows-only `-b` behavior can remain a no-op on Unix, but option parsing and exit behavior should still match C on this platform.
- The parity harness may skip C comparisons when the C jq binary is unavailable, but local Rust tests should still compile and run.
- For OS-dependent stderr fragments, such as errno text and absolute path rendering, tests may normalize or assert stable substrings while preserving exact checks for stable output.

## Acceptance Criteria

- `SPECS/rust-entrypoint-parity-prd.md` exists and is valid Markdown.
- The document is decision-complete enough for another engineer or agent to implement without rediscovering scope.
- The selected implementation path is named explicitly.
- The document states that `jq*` directories are out of scope.
