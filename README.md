# jqrs

`jqrs` is a Rust 2021 port of jq's C implementation. The active crate lives at
the repository root and builds a jq-like command line binary named
`jq_with_autobuild`.

This is not an idiomatic rewrite of jq. The codebase was emitted from
LLM-assisted C analysis and then repaired toward behavioral parity with jq.
Treat the C implementation and jq's own test suite as the source of truth.

## Current Status

The root crate can build and run jq filters, and the Rust `--run-tests` path has
been brought far enough to run jq's upstream `tests/jq.test` file.

Recently verified:

```sh
./target/debug/jq_with_autobuild --run-tests /Users/mickillah/jq/tests/jq.test
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

- C jq source: `/Users/mickillah/jq`
- C jq entry point: `/Users/mickillah/jq/src/main.c`
- C jq test file: `/Users/mickillah/jq/tests/jq.test`
- Rust root crate: this repository root

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
target/debug/jq_with_autobuild
target/release/jq_with_autobuild
```

## Run

Run a filter through Cargo:

```sh
cargo run -- '.foo' input.json
```

Run the built binary directly:

```sh
./target/debug/jq_with_autobuild '.foo' input.json
```

Read JSON from standard input:

```sh
printf '{"foo":1}\n' | ./target/debug/jq_with_autobuild '.foo'
```

Examples:

```sh
./target/debug/jq_with_autobuild -n '{ok: true}'
./target/debug/jq_with_autobuild -r '.name' data.json
./target/debug/jq_with_autobuild --arg name value -n '$name'
./target/debug/jq_with_autobuild --slurpfile data input.json -n '$data'
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
./target/debug/jq_with_autobuild --run-tests /Users/mickillah/jq/tests/jq.test
```

For faster local parity checks, use the release binary:

```sh
cargo build --release
./target/release/jq_with_autobuild --run-tests /Users/mickillah/jq/tests/jq.test
```

The integration parity tests compare Rust CLI behavior against a C jq binary.
They use `JQ_C_BIN` when set and otherwise try `/Users/mickillah/jq/jq`.

```sh
JQ_C_BIN=/Users/mickillah/jq/jq cargo test entrypoint_parity
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

## Development Rules

- Work in the root crate unless a task explicitly says otherwise.
- Do not modify `jq*` directories during normal crate work.
- Prefer jq's C source and `jq.test` over assumptions from the translated Rust.
- Keep changes scoped. The generated code is already noisy.
- Do not add new stubs, placeholders, or compile-through workarounds.
- When replacing a stub, connect the implementation to C behavior or a parity
  test.
- Do not run broad mechanical formatters on generated files unless that is the
  requested task.

## Debugging Workflow

For a behavior mismatch, start with a small Rust-vs-C comparison:

```sh
/Users/mickillah/jq/jq '<filter>' input.json
./target/debug/jq_with_autobuild '<filter>' input.json
```

Then add or narrow a parity case:

```sh
JQ_C_BIN=/Users/mickillah/jq/jq cargo test entrypoint_parity -- --nocapture
```

For jq test-suite slices:

```sh
./target/debug/jq_with_autobuild --run-tests --skip 340 --take 20 /Users/mickillah/jq/tests/jq.test
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
./target/debug/jq_with_autobuild --run-tests /Users/mickillah/jq/tests/jq.test
git diff --check
```

If test output is too noisy because of existing warnings, keep the verification
focused on behavior and do not hide new compiler diagnostics introduced by the
current change.
