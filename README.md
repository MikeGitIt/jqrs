# jqrs

`jqrs` is a Rust 2021 port of jq's C implementation. The active crate lives at
the repository root and builds a jq-like command line binary named `jqrs`.

This is not an idiomatic rewrite of jq. The codebase was emitted from
LLM-assisted C analysis and then repaired toward behavioral parity with jq.
Treat the C implementation and jq's own test suite as the source of truth.

## Current Status

The root crate can build and run jq filters, and the Rust `--run-tests` path has
been brought far enough to run jq's upstream `tests/jq.test` file.

Recently verified:

```sh
JQ_SRC=/path/to/jq
jqrs --run-tests "$JQ_SRC/tests/jq.test"
```

Expected result:

```text
457 of 457 tests passed (0 malformed, 0 skipped)
Running jq_start state tests...
jq_start state tests passed
```

This does not mean every translated module is complete or clean. Some modules
still contain generated-code scaffolding, compatibility helpers, and comments
from the original translation pass. When behavior is uncertain, compare against
the C jq binary.

## Source of Truth

Use these references when changing behavior:

- C jq source: `$JQ_SRC`
- C jq entry point: `$JQ_SRC/src/main.c`
- C jq test file: `$JQ_SRC/tests/jq.test`
- Rust root crate: this repository root

Set `JQ_SRC` to the local checkout of the C jq repository when running parity
checks:

```sh
export JQ_SRC=/path/to/jq
```

Directories whose names start with `jq`, such as `jqvert-test/`,
`jqvert-test_rwasm_rewrite/`, and `jq_port_*`, are reference, generated, or
archived material. They are not part of normal root-crate work.

## Build

Build the Rust binary:

```sh
cargo build
```

Build an optimized binary:

```sh
cargo build --release
```

The binary paths are:

```text
target/debug/jqrs
target/release/jqrs
```

Use `cargo run -- ...` during development, run `jqrs ...` when the binary is on
`PATH`, or invoke the built artifact directly from `target/`.

## Run

Run a filter during development:

```sh
cargo run -- '.foo' input.json
```

Run the installed or locally available binary:

```sh
jqrs '.foo' input.json
```

Read JSON from standard input:

```sh
printf '{"foo":1}\n' | jqrs '.foo'
```

Examples:

```sh
jqrs -n '{ok: true}'
jqrs -r '.name' data.json
jqrs --arg name value -n '$name'
jqrs --slurpfile data input.json -n '$data'
```

## Test

Run root crate tests:

```sh
cargo test
```

Run a targeted test group:

```sh
cargo test entrypoint
```

Run jq's upstream test file through the Rust test-suite runner:

```sh
JQ_SRC=/path/to/jq
jqrs --run-tests "$JQ_SRC/tests/jq.test"
```

For faster local parity checks, use the release binary:

```sh
cargo build --release
JQ_SRC=/path/to/jq
target/release/jqrs --run-tests "$JQ_SRC/tests/jq.test"
```

The integration parity tests compare Rust CLI behavior against a C jq binary.
Set `JQ_C_BIN` to the jq binary built from the C checkout. Tests that need
jq's upstream `jq.test` file read it from `JQ_TEST_FILE` when set, or from
`$JQ_SRC/tests/jq.test`.

```sh
JQ_SRC=/path/to/jq JQ_C_BIN=/path/to/jq/jq cargo test entrypoint_parity
```

The parity checks compare exit code, stdout, and stderr for selected CLI cases.
The full jq test-suite run is still the stronger signal for jq language
behavior.

## Supported CLI Surface

The Rust entry point is intended to match jq's command line behavior for:

- filter execution from an argument or `-f`
- stdin and file input
- `--arg`, `--argjson`, `--args`, and `--jsonargs`
- `--rawfile` and `--slurpfile`
- raw, compact, sorted, ASCII, colored, and unbuffered output modes
- `--stream`, `--stream-errors`, and `--seq`
- `input`, `inputs`, `input_filename`, and `input_line_number`
- runtime errors, parse errors, `halt`, `halt_error`, and `-e`
- `--run-tests`

If a command differs from C jq, treat it as a bug unless the difference is
explicitly documented in code or tests.

## Repository Layout

```text
src/main.rs                 CLI entry point translated from jq src/main.c
src/lib.rs                  root module exports
src/parser.rs               translated/generated parser logic
src/lexer.rs                translated/generated lexer logic
src/execute.rs              jq execution engine and runtime callbacks
src/compile.rs              jq block and bytecode compilation support
src/builtin.rs              jq builtin functions
src/jv.rs                   jq value representation
src/jv_*.rs                 jq value helpers, parsing, printing, files, dtoa
src/dec*.rs, src/decimal*.rs decimal-number support
src/linker.rs               module loading and library path handling
src/jq_test.rs              jq test-suite runner
tests/entrypoint_parity.rs  Rust-vs-C CLI parity tests
SPECS/                      design notes and investigation docs
```

## Debugging Workflow

For a behavior mismatch, start with a small Rust-vs-C comparison:

```sh
JQ_C_BIN=/path/to/jq/jq
"$JQ_C_BIN" '<filter>' input.json
jqrs '<filter>' input.json
```

Then add or narrow a parity case:

```sh
JQ_SRC=/path/to/jq JQ_C_BIN=/path/to/jq/jq cargo test entrypoint_parity -- --nocapture
```

For jq test-suite slices:

```sh
JQ_SRC=/path/to/jq
jqrs --run-tests --skip 340 --take 20 "$JQ_SRC/tests/jq.test"
```

Use the C jq result as the oracle for stdout, stderr, exit code, parser errors,
runtime errors, and module-loading behavior.

## Known Caveats

- The code is translated and patched, not hand-designed Rust.
- Some comments still describe older placeholders even where behavior has been
  repaired.
- Some generated parser, lexer, decimal, and bytecode structures are difficult
  to audit manually.
- Passing `jq.test` is necessary but not sufficient for complete jq parity.
- If the original C analysis omitted preprocessed or generated data, some Rust
  code may still need to be reconciled with jq's real build outputs.

## Release Notes for Maintainers

Before handing off behavior changes, prefer this minimum verification set:

```sh
cargo build
cargo test
JQ_SRC=/path/to/jq
jqrs --run-tests "$JQ_SRC/tests/jq.test"
git diff --check
```

If test output is too noisy because of existing warnings, keep the verification
focused on behavior and do not hide new compiler diagnostics introduced by the
current change.
