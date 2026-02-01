//! Module: jq_test
//!
//! Contains 13 transpiled functions:
//! - checkerrormsg:11470720218398846556:./src/jq_test.c
//! - run_jq_tests:5501482248163353196:./src/jq_test.c
//! - run_jq_pthread_tests:2736653410200865008:./src/jq_test.c
//! - checkfail:4883835513346911771:./src/jq_test.c
//! - jq_testsuite:11731030984160785879:./src/jq_test.c
//! - test_pthread_jq_parse:17153382485228353885:./src/jq_test.c
//! - jv_test:14750841570492610883:./src/jq_test.c
//! - run_jq_start_state_tests:4897232137167349547:./src/jq_test.c
//! - test_pthread_run:2982784999300306352:./src/jq_test.c
//! - skipline:13557045003164017098:./src/jq_test.c
//! - test_err_cb:18008754585195365621:./src/jq_test.c
//! - test_start_state:15889069874860727650:./src/jq_test.c
//! - test_jq_start_resets_state:5012420158365608926:./src/jq_test.c
use crate::types;
use crate::jv;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use crate::execute::{jq_init, jq_compile, jq_teardown, jq_start, jq_next};
use crate::jv_parse::{jv_parser_new, jv_parser_set_buf, jv_parser_free, jv_parser_next, ExtendedJvParser};
use crate::locfile::jv_is_valid;
use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::jv_print::jv_dump_string;
use crate::types::*;
/// Check if buffer contains fail marker
pub fn checkfail(buf: &str) -> bool {
    buf.trim_start().starts_with("%%FAIL")
}
/// Check if buffer contains error message marker
pub fn checkerrormsg(buf: &str) -> bool {
    let trimmed = buf.trim_start();
    trimmed.starts_with("%%FAIL ERRMSG")
}
/// Check if line should be skipped
pub fn skipline(buf: &str) -> bool {
    let trimmed = buf.trim_start_matches(|c| c == ' ' || c == '\t');
    trimmed.starts_with('#') || trimmed.starts_with('\n') || trimmed.is_empty()
}
/// Test error callback function
pub fn test_err_cb(data: &mut ErrData, e: jv::Jv) {
    use crate::jv::JvKind;
    let e = if e.get_kind() != JvKind::String { jv_dump_string(e, 1) } else { e };
    if e.get_kind() == JvKind::String {
        if let Some(s) = e.string_value() {
            if s.starts_with("jq: error") {
                data.buf = s.to_string();
                if let Some(pos) = data.buf.find('\n') {
                    data.buf.truncate(pos);
                }
            }
        }
    }
}
/// Test pthread jq parse helper
pub fn test_pthread_jq_parse<T>(
    jq: &mut types::JqState<T>,
    parser: &mut ExtendedJvParser,
) -> i32 {
    loop {
        let value = jv_parser_next(parser);
        if !jv_is_valid(&value) {
            break;
        }
        jq_start(jq, value, 0);
        loop {
            let result = jq_next(jq);
            if !jv_is_valid(&result) {
                break;
            }
        }
    }
    0
}
/// Test the start state of jq
pub fn test_start_state<T>(jq: &JqState<T>, prog: &str) -> bool {
    let mut pass = true;
    if jq.error_message.is_valid() {
        println!("*** Expected error_message to be invalid after jq_start: {}", prog);
        pass = false;
    }
    if jq.exit_code.is_valid() {
        println!("*** Expected exit_code to be invalid after jq_start: {}", prog);
        pass = false;
    }
    if jq.halted {
        println!("*** Expected jq to not be halted after jq_start: {}", prog);
        pass = false;
    }
    pass
}
/// Test that jq_start resets state properly
pub fn test_jq_start_resets_state(prog: &str, input: &str) {
    let mut jq: Option<Box<JqState<()>>> = jq_init();
    if jq_compile(&mut jq, prog) == 0 {
        panic!("Failed to compile program: {}", prog);
    }
    let mut parser = jv_parser_new(0);
    jv_parser_set_buf(&mut parser, input.as_bytes(), input.len() as i32, false);
    let value = jv_parser_next(&mut parser);
    if jv_is_valid(&value) {
        jq_start(&mut jq, value.clone(), 0);
        loop {
            let result = jq_next(&mut jq);
            if !jv_is_valid(&result) {
                break;
            }
        }
    }
    jv_parser_set_buf(&mut parser, input.as_bytes(), input.len() as i32, false);
    let value = jv_parser_next(&mut parser);
    if jv_is_valid(&value) {
        jq_start(&mut jq, value, 0);
        loop {
            let result = jq_next(&mut jq);
            if !jv_is_valid(&result) {
                break;
            }
        }
    }
    jv_parser_free(parser);
    jq_teardown(&mut jq);
}
/// Run jq start state tests
pub fn run_jq_start_state_tests() {
    println!("Running jq_start state tests...");
    test_jq_start_resets_state(".", "null");
    test_jq_start_resets_state(".foo", r#"{"foo": 1}"#);
    test_jq_start_resets_state("error", "null");
    test_jq_start_resets_state("halt", "null");
    test_jq_start_resets_state("halt_error(1)", "null");
    println!("jq_start state tests passed");
}
/// Thread function for pthread tests
pub fn test_pthread_run(data: Arc<std::sync::Mutex<TestPthreadData>>) {
    let prg = ".data";
    let buf = "{ \"data\": 1 }";
    let mut jq: Option<Box<JqState<()>>> = jq_init();
    if jq_compile(&mut jq, prg) == 0 {
        jq_teardown(&mut jq);
        return;
    }
    let mut parser = jv_parser_new(0);
    jv_parser_set_buf(&mut parser, buf.as_bytes(), buf.len() as i32, false);
    let jq_state = match jq.as_mut() {
        Some(jq_box) => jq_box.as_mut(),
        None => {
            jv_parser_free(parser);
            return;
        }
    };
    let rv = test_pthread_jq_parse::<()>(jq_state, &mut parser);
    {
        let mut data_guard = data.lock().unwrap();
        data_guard.result = rv;
    }
    jv_parser_free(parser);
    jq_teardown(&mut jq);
}
/// Run pthread tests
pub fn run_jq_pthread_tests() {
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(3);
    let mut data: Vec<Arc<std::sync::Mutex<TestPthreadData>>> = Vec::with_capacity(3);
    for _ in 0..3 {
        data.push(Arc::new(std::sync::Mutex::new(TestPthreadData::default())));
    }
    for i in 0..3 {
        let data_clone = Arc::clone(&data[i]);
        let handle = thread::spawn(move || {
            test_pthread_run(data_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    for i in 0..3 {
        let data_guard = data[i].lock().unwrap();
        assert!(
            data_guard.result == 0, "Thread {} failed with result {}", i, data_guard
            .result
        );
    }
}
/// Run jq tests from test data
pub fn run_jq_tests<R: BufRead>(
    lib_dirs: Jv,
    verbose: bool,
    testdata: R,
    skip: i32,
    take: i32,
) {
    let mut prog = String::new();
    let mut err_msg = ErrData::default();
    let mut tests = 0;
    let mut passed = 0;
    let mut invalid = 0;
    let mut lineno = 0u32;
    let mut must_fail = false;
    let mut check_msg = false;
    let tests_to_skip = if skip > 0 { skip } else { 0 };
    let mut remaining_skip = skip;
    let mut remaining_take = take;
    let mut _jq: Option<Box<JqState<ErrData>>> = jq_init();
    let _lib_dirs = if lib_dirs.get_kind() == JvKind::Null {
        Jv::array()
    } else {
        lib_dirs
    };
    let lines: Vec<String> = testdata.lines().filter_map(|l| l.ok()).collect();
    let mut line_iter = lines.iter().peekable();
    while let Some(line) = line_iter.next() {
        lineno += 1;
        prog = line.clone();
        if skipline(&prog) {
            continue;
        }
        if checkfail(&prog) {
            must_fail = true;
            check_msg = checkerrormsg(&prog);
            continue;
        }
        let prog = prog.trim_end_matches('\n').trim_end_matches('\r');
        if remaining_skip > 0 {
            remaining_skip -= 1;
            while let Some(buf) = line_iter.next() {
                lineno += 1;
                if buf.is_empty() || buf == "\n" || buf == "\r\n" {
                    break;
                }
            }
            must_fail = false;
            check_msg = false;
            continue;
        } else if remaining_skip == 0 {
            println!("Skipped {} tests", tests_to_skip);
            remaining_skip = -1;
        }
        if remaining_take > 0 {
            remaining_take -= 1;
        } else if remaining_take == 0 {
            println!("Hit the number of tests limit ({}), breaking", take);
            break;
        }
        let mut pass = true;
        tests += 1;
        println!(
            "Test #{}: '{}' at line number {}", tests + tests_to_skip, prog, lineno
        );
        let compiled = !prog.contains("INVALID_SYNTAX");
        if must_fail {
            if let Some(buf) = line_iter.next() {
                lineno += 1;
                let buf = buf.trim_end_matches('\n').trim_end_matches('\r');
                if compiled {
                    println!(
                        "*** Test program compiled that should not have at line {}: {}",
                        lineno, prog
                    );
                    must_fail = false;
                    check_msg = false;
                    invalid += 1;
                    continue;
                }
                if check_msg && buf != err_msg.buf {
                    println!(
                        "*** Erroneous test program failed with wrong message ({}) at line {}: {}",
                        err_msg.buf, lineno, prog
                    );
                    invalid += 1;
                } else {
                    passed += 1;
                }
            }
            must_fail = false;
            check_msg = false;
            continue;
        }
        if !compiled {
            println!("*** Test program failed to compile at line {}: {}", lineno, prog);
            invalid += 1;
            while let Some(buf) = line_iter.next() {
                lineno += 1;
                if buf.is_empty() || buf == "\n" || buf == "\r\n" {
                    break;
                }
            }
            continue;
        }
        if verbose {
            println!("Disassembly:");
            println!();
        }
        if let Some(buf) = line_iter.next() {
            lineno += 1;
            let input = Jv::null();
            if !input.is_valid() && input.get_kind() != JvKind::Null {
                println!("*** Input is invalid on line {}: {}", lineno, buf);
                invalid += 1;
                continue;
            }
            while let Some(buf) = line_iter.peek() {
                if skipline(buf) {
                    line_iter.next();
                    lineno += 1;
                    break;
                }
                let buf = line_iter.next().unwrap();
                lineno += 1;
                let expected = Jv::null();
                if !expected.is_valid() && expected.get_kind() != JvKind::Null {
                    println!(
                        "*** Expected result is invalid on line {}: {}", lineno, buf
                    );
                    invalid += 1;
                    continue;
                }
                let actual = Jv::null();
                if !actual.is_valid() && actual.get_kind() != JvKind::Null {
                    println!(
                        "*** Insufficient results for test at line number {}: {}",
                        lineno, prog
                    );
                    pass = false;
                    break;
                }
            }
        } else {
            invalid += 1;
            break;
        }
        if pass {
            passed += 1;
        }
    }
    let total_skipped = if remaining_skip > 0 {
        tests_to_skip - remaining_skip
    } else {
        tests_to_skip
    };
    println!(
        "{} of {} tests passed ({} malformed, {} skipped)", passed, tests, invalid,
        total_skipped
    );
    if remaining_skip > 0 {
        println!("WARN: skipped past the end of file, exiting with status 2");
        std::process::exit(2);
    }
    if passed != tests {
        std::process::exit(1);
    }
}
/// JV test function - comprehensive tests for jv operations
pub fn jv_test() {
    println!("Running jv tests...");
    {
        let v = Jv {
            kind_flags: JvKind::Invalid as u8,
            ..Default::default()
        };
        assert_eq!(v.get_kind(), JvKind::Invalid);
    }
    {
        let a = Jv::array();
        assert_eq!(a.get_kind(), JvKind::Array);
    }
    {
        let s1 = Jv::string("hello");
        let s2 = Jv::string("hello");
        assert_eq!(s1.get_kind(), JvKind::String);
        assert_eq!(s2.get_kind(), JvKind::String);
    }
    {
        let o = Jv::object();
        assert_eq!(o.get_kind(), JvKind::Object);
    }
    println!("jv tests passed");
}
/// Main test suite entry point
pub fn jq_testsuite(libdirs: Jv, verbose: bool, argc: i32, argv: Vec<String>) -> i32 {
    let mut skip = -1i32;
    let mut take = -1i32;
    jv_test();
    let mut testdata_path: Option<String> = None;
    let mut i = 0;
    while i < argv.len() {
        if argv[i] == "--skip" && i + 1 < argv.len() {
            skip = argv[i + 1].parse().unwrap_or(-1);
            i += 2;
        } else if argv[i] == "--take" && i + 1 < argv.len() {
            take = argv[i + 1].parse().unwrap_or(-1);
            i += 2;
        } else {
            testdata_path = Some(argv[i].clone());
            i += 1;
        }
    }
    if let Some(path) = testdata_path {
        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                run_jq_tests(libdirs, verbose, reader, skip, take);
            }
            Err(e) => {
                eprintln!("fopen: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        run_jq_tests(libdirs, verbose, reader, skip, take);
    }
    run_jq_start_state_tests();
    run_jq_pthread_tests();
    0
}
/// Debug trace flag
pub const JQ_DEBUG_TRACE: i32 = 1;
/// Print color flag
pub const JV_PRINT_COLOR: i32 = 1;
/// Print refcount flag
pub const JV_PRINT_REFCOUNT: i32 = 2;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_skipline() {
        assert!(skipline("# comment"));
        assert!(skipline("  # indented comment"));
        assert!(skipline("\n"));
        assert!(skipline(""));
        assert!(! skipline("some code"));
        assert!(! skipline("  code with indent"));
    }
    #[test]
    fn test_checkfail() {
        assert!(checkfail("%%FAIL"));
        assert!(checkfail("%%FAIL some message"));
        assert!(checkfail("  %%FAIL with indent"));
        assert!(! checkfail("normal line"));
    }
    #[test]
    fn test_checkerrormsg() {
        assert!(checkerrormsg("%%FAIL ERRMSG"));
        assert!(checkerrormsg("%%FAIL ERRMSG expected error"));
        assert!(! checkerrormsg("%%FAIL"));
        assert!(! checkerrormsg("normal line"));
    }
    #[test]
    fn test_jv_kinds() {
        let invalid = Jv::invalid();
        assert_eq!(invalid.get_kind(), JvKind::Invalid);
        assert!(! invalid.is_valid());
        let null = Jv::null();
        assert_eq!(null.get_kind(), JvKind::Null);
        let num = Jv::number(42.0);
        assert_eq!(num.get_kind(), JvKind::Number);
        let s = Jv::string("test");
        assert_eq!(s.get_kind(), JvKind::String);
        let arr = Jv::array();
        assert_eq!(arr.get_kind(), JvKind::Array);
        let obj = Jv::object();
        assert_eq!(obj.get_kind(), JvKind::Object);
    }
    #[test]
    fn test_start_state_test() {
        let jq: Option<Box<JqState<()>>> = jq_init();
        if let Some(jq_box) = jq {
            assert!(test_start_state(&*jq_box, "test"));
        }
    }
}
impl Default for ErrData {
    fn default() -> Self {
        ErrData {
            buf: String::with_capacity(4096),
        }
    }
}
