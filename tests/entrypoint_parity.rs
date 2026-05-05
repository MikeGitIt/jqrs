use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::Write;

struct RunOutput {
    code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn rust_jq() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_jq_with_autobuild")
        .map(PathBuf::from)
        .or_else(|| option_env!("CARGO_BIN_EXE_jq_with_autobuild").map(PathBuf::from))
        .expect("Cargo did not provide the Rust jq binary path")
}

fn c_jq() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("JQ_C_BIN").map(PathBuf::from) {
        return path.is_file().then_some(path);
    }
    let default = PathBuf::from("/Users/mickillah/jq/jq");
    default.is_file().then_some(default)
}

fn run(bin: &PathBuf, args: &[String], stdin: Option<&[u8]>) -> RunOutput {
    let mut child = Command::new(bin)
        .args(args)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("failed to run {}: {err}", bin.display()));
    if let Some(input) = stdin {
        child
            .stdin
            .take()
            .expect("stdin pipe")
            .write_all(input)
            .expect("write stdin");
    }
    let output = child
        .wait_with_output()
        .unwrap_or_else(|err| panic!("failed to wait for {}: {err}", bin.display()));
    RunOutput {
        code: output.status.code().unwrap_or(-1),
        stdout: output.stdout,
        stderr: output.stderr,
    }
}

fn normalize_output(bytes: &[u8], rust_bin: &PathBuf, c_bin: &PathBuf) -> String {
    let mut text = String::from_utf8_lossy(bytes).into_owned();
    text = text.replace(&rust_bin.display().to_string(), "<jq>");
    text.replace(&c_bin.display().to_string(), "<jq>")
}

fn assert_parity(
    name: &str,
    rust_bin: &PathBuf,
    c_bin: &PathBuf,
    args: &[String],
    stdin: Option<&[u8]>,
) {
    let rust = run(rust_bin, args, stdin);
    let c = run(c_bin, args, stdin);
    assert_eq!(rust.code, c.code, "{name}: exit code mismatch");
    assert_eq!(
        normalize_output(&rust.stdout, rust_bin, c_bin),
        normalize_output(&c.stdout, rust_bin, c_bin),
        "{name}: stdout mismatch"
    );
    assert_eq!(
        normalize_output(&rust.stderr, rust_bin, c_bin),
        normalize_output(&c.stderr, rust_bin, c_bin),
        "{name}: stderr mismatch"
    );
}

#[test]
fn rust_reports_broken_stdout() {
    let rust_bin = rust_jq();
    let mut child = Command::new(&rust_bin)
        .args(["-n", "[range(0;1000)]"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("failed to run {}: {err}", rust_bin.display()));
    drop(child.stdout.take());
    let output = child.wait_with_output().expect("wait for rust jq");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("jq: error: writing output failed:"),
        "stderr did not report broken stdout: {stderr}"
    );
}

#[test]
fn parity_entrypoint_smoke() {
    let Some(c_bin) = c_jq() else {
        eprintln!("skipping parity smoke: set JQ_C_BIN or build /Users/mickillah/jq/jq");
        return;
    };
    let rust_bin = rust_jq();
    let tempdir = tempfile::tempdir().expect("create tempdir");
    let json_file = tempdir.path().join("inputs.json");
    std::fs::write(&json_file, "{\"a\":1}\n{\"b\":2}\n").expect("write json fixture");
    let json_path = json_file.to_string_lossy().into_owned();

    let second_json_file = tempdir.path().join("more-inputs.json");
    std::fs::write(&second_json_file, "{\"c\":3}\n").expect("write second json fixture");
    let second_json_path = second_json_file.to_string_lossy().into_owned();
    let invalid_json_file = tempdir.path().join("invalid.json");
    std::fs::write(&invalid_json_file, "{\"a\":").expect("write invalid json fixture");
    let invalid_json_path = invalid_json_file.to_string_lossy().into_owned();
    let seq_file = tempdir.path().join("seq.json");
    std::fs::write(&seq_file, "\u{1e}{\"a\":1}\nnot-json\n\u{1e}{\"b\":2}\n")
        .expect("write seq fixture");
    let seq_path = seq_file.to_string_lossy().into_owned();
    let long_line_file = tempdir.path().join("long-line.json");
    let long_segment =
        r#"printchar ('\\\"', printcharfun); tem = ((struct Lisp_Vector *) ((obj).u.val))->contents[i];\n"#;
    let long_json = format!(
        "{{\"source_code\":\"{}\"}}\n",
        long_segment.repeat(80)
    );
    std::fs::write(&long_line_file, long_json).expect("write long line json fixture");
    let long_line_path = long_line_file.to_string_lossy().into_owned();
    let same_line_values_file = tempdir.path().join("same-line-values.json");
    std::fs::write(&same_line_values_file, "1 2\n").expect("write same-line values fixture");
    let same_line_values_path = same_line_values_file.to_string_lossy().into_owned();
    let number_no_newline_file = tempdir.path().join("number-no-newline.json");
    std::fs::write(&number_no_newline_file, "1").expect("write number without newline fixture");
    let number_no_newline_path = number_no_newline_file.to_string_lossy().into_owned();
    let object_no_newline_file = tempdir.path().join("object-no-newline.json");
    std::fs::write(&object_no_newline_file, "{\n\"a\":1\n}")
        .expect("write object without newline fixture");
    let object_no_newline_path = object_no_newline_file.to_string_lossy().into_owned();

    let cases: Vec<(&str, Vec<String>, Option<Vec<u8>>)> = vec![
        ("help", vec!["--help".into()], None),
        ("version", vec!["--version".into()], None),
        ("build configuration", vec!["--build-configuration".into()], None),
        ("unknown option", vec!["--definitely-not-jq".into()], None),
        ("missing arg parameter", vec!["--arg".into(), "name".into()], None),
        (
            "arg",
            vec![
                "--arg".into(),
                "name".into(),
                "value".into(),
                "-n".into(),
                "$name".into(),
            ],
            None,
        ),
        (
            "duplicate arg first wins",
            vec![
                "--arg".into(),
                "name".into(),
                "first".into(),
                "--arg".into(),
                "name".into(),
                "second".into(),
                "-n".into(),
                "$name".into(),
            ],
            None,
        ),
        (
            "argjson",
            vec![
                "--argjson".into(),
                "obj".into(),
                "{\"a\":1}".into(),
                "-n".into(),
                "$obj".into(),
            ],
            None,
        ),
        (
            "args",
            vec![
                "-n".into(),
                "$ARGS.positional".into(),
                "--args".into(),
                "a".into(),
                "b".into(),
            ],
            None,
        ),
        (
            "jsonargs",
            vec![
                "-n".into(),
                "$ARGS.positional".into(),
                "--jsonargs".into(),
                "{\"a\":1}".into(),
                "2".into(),
            ],
            None,
        ),
        (
            "exit status empty",
            vec!["-e".into(), "-n".into(), "empty".into()],
            None,
        ),
        (
            "exit status truthy",
            vec!["-e".into(), "-n".into(), "true".into()],
            None,
        ),
        (
            "exit status false",
            vec!["-e".into(), "-n".into(), "false".into()],
            None,
        ),
        (
            "ascii output",
            vec!["-a".into(), "-n".into(), "\"\u{00b5}\"".into()],
            None,
        ),
        (
            "raw ascii output",
            vec![
                "-r".into(),
                "-a".into(),
                "-n".into(),
                "\"\u{00b5}\"".into(),
            ],
            None,
        ),
        (
            "tab indentation",
            vec!["--tab".into(), ".".into()],
            Some(b"{\"a\":[1]}\n".to_vec()),
        ),
        (
            "rawfile argument",
            vec![
                "--rawfile".into(),
                "data".into(),
                "Cargo.toml".into(),
                "-n".into(),
                "$data | length".into(),
            ],
            None,
        ),
        (
            "slurpfile argument",
            vec![
                "--slurpfile".into(),
                "data".into(),
                json_path.clone(),
                "-n".into(),
                "$data".into(),
            ],
            None,
        ),
        (
            "input callback under null input",
            vec!["-n".into(), "input".into(), json_path.clone()],
            None,
        ),
        (
            "input filename builtin",
            vec!["input_filename".into(), json_path.clone()],
            None,
        ),
        (
            "input line number builtin",
            vec!["input_line_number".into(), json_path.clone()],
            None,
        ),
        (
            "input line number same-line values",
            vec!["input_line_number".into(), same_line_values_path],
            None,
        ),
        (
            "input line number no trailing newline number",
            vec!["input_line_number".into(), number_no_newline_path],
            None,
        ),
        (
            "stdin input",
            vec![".a".into()],
            Some(b"{\"a\":1}\n".to_vec()),
        ),
        (
            "object keys",
            vec!["keys".into()],
            Some(b"{\"b\":2,\"a\":1}\n".to_vec()),
        ),
        (
            "array keys",
            vec!["keys".into()],
            Some(b"[\"a\",\"b\"]\n".to_vec()),
        ),
        (
            "object to_entries",
            vec!["to_entries".into()],
            Some(b"{\"b\":2,\"a\":1}\n".to_vec()),
        ),
        (
            "array to_entries",
            vec!["to_entries".into()],
            Some(b"[\"a\",\"b\"]\n".to_vec()),
        ),
        (
            "object indexed by number error",
            vec![".[0]".into()],
            Some(b"{\"a\":1}\n".to_vec()),
        ),
        (
            "object indexed by number without trailing newline",
            vec![".[0]".into(), object_no_newline_path],
            None,
        ),
        (
            "array indexed by string error",
            vec![".call_graph".into()],
            Some(b"[]\n".to_vec()),
        ),
        (
            "decimal input",
            vec![".".into()],
            Some(b"{\"n\":0.0}\n".to_vec()),
        ),
        (
            "exponent input",
            vec![".m == 0.001".into()],
            Some(b"{\"m\":1e-3}\n".to_vec()),
        ),
        (
            "single file input",
            vec![".a".into(), json_path.clone()],
            None,
        ),
        (
            "multiple file input",
            vec![".".into(), json_path, second_json_path],
            None,
        ),
        (
            "parse error",
            vec![".".into(), invalid_json_path],
            None,
        ),
        (
            "seq parse error recovery",
            vec!["--seq".into(), ".".into(), seq_path],
            None,
        ),
        (
            "long string across input buffer",
            vec![".source_code | length".into(), long_line_path],
            None,
        ),
        (
            "runtime error",
            vec!["-n".into(), "\"boom\"|error".into()],
            None,
        ),
        (
            "halt error",
            vec!["-n".into(), "halt_error(7)".into()],
            None,
        ),
    ];

    for (name, args, stdin) in cases {
        assert_parity(name, &rust_bin, &c_bin, &args, stdin.as_deref());
    }
}
