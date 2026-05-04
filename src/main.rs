//! Module: main
//!
//! Contains 8 transpiled functions:
//! - main:14488106908486361945:./src/main.c
//! - stderr_cb:18180879711188430346:./src/main.c
//! - isoptish:8245702026156913532:./src/main.c
//! - usage:1807477498479083561:./src/main.c
//! - isoption:9631227784079607476:./src/main.c
//! - debug_cb:16109236487809366000:./src/main.c
//! - die:4605565059772087902:./src/main.c
//! - process:10692635711031878123:./src/main.c
use std::env;
use std::cell::RefCell;
use std::io::{self, Write, IsTerminal};
use std::path::Path;
use std::process as std_process;
use std::rc::Rc;
use jq_with_autobuild::jv::{
    Jv, JvKind, jv_string_value, jv_number_value, jv_invalid_get_msg,
    jv_object_iter, jv_object_iter_key, jv_object_iter_next, jv_object_iter_valid,
};
use jq_with_autobuild::jv_file::jv_load_file;
use jq_with_autobuild::jv_print::{jv_dumpf, jv_dump_string, jq_set_colors};
use jq_with_autobuild::jv_parse::jv_parse;
use jq_with_autobuild::execute::{
    jq_init, jq_start, jq_next, jq_halted, jq_get_exit_code, jq_get_error_message,
    jq_compile_args, jq_set_attr, jq_set_debug_cb, jq_set_input_cb, jq_set_stderr_cb,
    jq_dump_disassembly, jq_teardown,
};
use jq_with_autobuild::jq_test::jq_testsuite;
use jq_with_autobuild::types::{JqState, JqUtilInputState};
use jq_with_autobuild::util::{
    jq_util_input_init, jq_util_input_add_input, jq_util_input_errors,
    jq_util_input_next_input, jq_util_input_set_parser, jq_util_input_free,
    jq_realpath, jv_is_valid,
};
use jq_with_autobuild::jv_parse::jv_parser_new;
pub const JQ_OK: i32 = 0;
pub const JQ_OK_NULL_KIND: i32 = -1;
pub const JQ_ERROR_SYSTEM: i32 = 2;
pub const JQ_ERROR_COMPILE: i32 = 3;
pub const JQ_OK_NO_OUTPUT: i32 = -4;
pub const JQ_ERROR_UNKNOWN: i32 = 5;
pub const SLURP: i32 = 1;
pub const RAW_INPUT: i32 = 2;
pub const PROVIDE_NULL: i32 = 4;
pub const RAW_OUTPUT: i32 = 8;
pub const RAW_OUTPUT0: i32 = 16;
pub const ASCII_OUTPUT: i32 = 32;
pub const COLOR_OUTPUT: i32 = 64;
pub const NO_COLOR_OUTPUT: i32 = 128;
pub const SORTED_OUTPUT: i32 = 256;
pub const FROM_FILE: i32 = 512;
pub const RAW_NO_LF: i32 = 1024;
pub const UNBUFFERED_OUTPUT: i32 = 2048;
pub const EXIT_STATUS: i32 = 4096;
pub const SEQ: i32 = 16384;
pub const RUN_TESTS: i32 = 32768;
pub const DUMP_DISASM: i32 = 65536;
pub const JQ_DEBUG_TRACE: i32 = 1;
pub const JQ_DEBUG_TRACE_ALL: i32 = 2;
pub const JV_PRINT_PRETTY: i32 = 1;
pub const JV_PRINT_TAB: i32 = 2;
pub const JV_PRINT_COLOR: i32 = 4;
pub const JV_PRINT_SORTED: i32 = 8;
pub const JV_PRINT_ASCII: i32 = 16;
pub const JV_PRINT_ISATTY: i32 = 32;
pub const JV_PARSE_STREAMING: i32 = 1;
pub const JV_PARSE_STREAM_ERRORS: i32 = 2;
pub const JV_PARSE_SEQ: i32 = 4;
const JQ_VERSION: &str = "1.7.1-57-gba741e5-dirty";
const JQ_CONFIG: &str = "";
type SharedInputState = Rc<RefCell<Box<JqUtilInputState>>>;
#[derive(Clone, Default)]
pub struct CliCallbackData {
    dumpopts: i32,
    input_state: Option<SharedInputState>,
}
static mut PROGNAME: Option<String> = None;
fn get_progname() -> &'static str {
    unsafe { PROGNAME.as_deref().unwrap_or("jq") }
}
fn set_progname(name: String) {
    unsafe {
        PROGNAME = Some(name);
    }
}
fn jq_exit_with_status(ret: i32) -> ! {
    std_process::exit(ret.abs());
}
fn jq_exit(ret: i32) -> ! {
    std_process::exit(if ret > 0 { ret } else { 0 });
}
fn jq_exit_for_options(ret: i32, options: i32) -> ! {
    if (options & EXIT_STATUS) != 0 {
        jq_exit_with_status(ret);
    }
    jq_exit(ret);
}
fn dirname_jv(path: &str) -> Jv {
    let dirname = Path::new(path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string());
    Jv::string(&dirname)
}
fn input_position(input_state: Option<&SharedInputState>) -> String {
    let Some(input_state) = input_state else {
        return "<unknown>".to_string();
    };
    let state = input_state.borrow();
    if state.current_filename.get_kind() != JvKind::String {
        return "<unknown>".to_string();
    }
    format!("{}:{}", jv_string_value(&state.current_filename), state.current_line)
}
fn invalid_has_msg(value: &Jv) -> bool {
    !jv_is_valid(value)
        && value.get_kind() == JvKind::Invalid
        && (value.kind_flags & jq_with_autobuild::jv::JVP_PAYLOAD_ALLOCATED) != 0
}
fn object_has_key(object: &Jv, key: &str) -> bool {
    let mut iter = jv_object_iter(object);
    while jv_object_iter_valid(object, iter) {
        let object_key = jv_object_iter_key(object, iter);
        if object_key.get_kind() == JvKind::String && jv_string_value(&object_key) == key {
            return true;
        }
        iter = jv_object_iter_next(object, iter);
    }
    false
}
struct OutputTracker<W: Write> {
    inner: W,
    first_error: Option<io::Error>,
}
impl<W: Write> OutputTracker<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            first_error: None,
        }
    }
    fn record_error(&mut self, err: &io::Error) {
        if self.first_error.is_none() {
            self.first_error = Some(io::Error::new(err.kind(), err.to_string()));
        }
    }
    fn take_error(&mut self) -> Option<io::Error> {
        self.first_error.take()
    }
}
impl<W: Write> Write for OutputTracker<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.write(buf) {
            Ok(n) => Ok(n),
            Err(err) => {
                self.record_error(&err);
                Err(err)
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self.inner.flush() {
            Ok(()) => Ok(()),
            Err(err) => {
                self.record_error(&err);
                Err(err)
            }
        }
    }
}
fn sync_jq_input_position<T>(jq: &mut JqState<T>, input_state: &SharedInputState) {
    let state = input_state.borrow();
    jq.input_filename = state.current_filename.clone();
    jq.input_line = state.current_line;
}
/// Compute dump options from indent value
fn compute_dumpopts(indent: i32) -> i32 {
    if indent < 0 || indent > 7 {
        JV_PRINT_TAB | JV_PRINT_PRETTY
    } else if indent == 0 {
        0
    } else {
        (indent << 8) | JV_PRINT_PRETTY
    }
}
/// Write to output stream
pub fn priv_fwrite(s: &str, out: &mut dyn Write, is_tty: bool) -> io::Result<()> {
    out.write_all(s.as_bytes())?;
    if is_tty {
        out.flush()?;
    }
    Ok(())
}
/// Process a single input value through jq
pub fn process<T, W: Write>(
    jq: &mut JqState<T>,
    value: Jv,
    flags: i32,
    dumpopts: i32,
    options: i32,
    stdout: &mut W,
    input_state: Option<&SharedInputState>,
) -> i32 {
    let mut ret = JQ_OK_NO_OUTPUT;
    let mut stderr = io::stderr();
    let is_tty = (dumpopts & JV_PRINT_ISATTY) != 0;
    jq_start(jq, value, flags);
    loop {
        let result = jq_next(jq);
        if !jv_is_valid(&result) {
            if invalid_has_msg(&result) {
                let msg = jv_invalid_get_msg(result);
                let pos = input_position(input_state);
                if msg.get_kind() == JvKind::String {
                    eprintln!("jq: error (at {}): {}", pos, jv_string_value(&msg));
                } else {
                    let msg_str = jv_dump_string(msg.clone(), 0);
                    eprintln!(
                        "jq: error (at {}) (not a string): {}",
                        pos,
                        jv_string_value(&msg_str)
                    );
                }
                ret = JQ_ERROR_UNKNOWN;
            }
            break;
        }
        if (options & RAW_OUTPUT) != 0 && result.get_kind() == JvKind::String {
            if (options & ASCII_OUTPUT) != 0 {
                jv_dumpf(result.clone(), stdout, JV_PRINT_ASCII);
            } else if (options & RAW_OUTPUT0) != 0 {
                let s = jv_string_value(&result);
                if s.contains('\0') {
                    eprintln!(
                        "jq: error (at {}): Cannot dump a string containing NUL with --raw-output0 option",
                        input_position(input_state)
                    );
                    ret = JQ_ERROR_UNKNOWN;
                    break;
                }
                let _ = priv_fwrite(s, stdout, is_tty);
            } else {
                let s = jv_string_value(&result);
                let _ = priv_fwrite(s, stdout, is_tty);
            }
            ret = JQ_OK;
        } else {
            match result.get_kind() {
                JvKind::False | JvKind::Null => ret = JQ_OK_NULL_KIND,
                _ => ret = JQ_OK,
            }
            if (options & SEQ) != 0 {
                let _ = priv_fwrite("\x1e", stdout, is_tty);
            }
            jv_dumpf(result.clone(), stdout, dumpopts);
        }
        if (options & RAW_NO_LF) == 0 {
            let _ = priv_fwrite("\n", stdout, is_tty);
        }
        if (options & RAW_OUTPUT0) != 0 {
            let _ = priv_fwrite("\0", stdout, is_tty);
        }
        if (options & UNBUFFERED_OUTPUT) != 0 {
            let _ = stdout.flush();
        }
    }
    if jq_halted(jq) {
        let exit_code = jq_get_exit_code(jq);
        if !jv_is_valid(&exit_code) {
            ret = JQ_OK;
        } else if exit_code.get_kind() == JvKind::Number {
            ret = jv_number_value(&exit_code) as i32;
        } else {
            ret = JQ_ERROR_UNKNOWN;
        }
        let error_message = jq_get_error_message(jq);
        match error_message.get_kind() {
            JvKind::String => {
                let s = jv_string_value(&error_message);
                let _ = priv_fwrite(s, &mut stderr, is_tty);
            }
            JvKind::Null => {}
            _ => {
                if jv_is_valid(&error_message) {
                    let msg_str = jv_dump_string(error_message.clone(), 0);
                    let s = jv_string_value(&msg_str);
                    eprintln!("{}", s);
                }
            }
        }
        let _ = stderr.flush();
    }
    ret
}
/// Check if text looks like a command-line option
pub fn isoptish(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes[0] != b'-' {
        return false;
    }
    if bytes.len() < 2 {
        return false;
    }
    bytes[1] == b'-' || bytes[1].is_ascii_alphabetic()
}
/// Check if text matches a specific option
pub fn isoption(
    text: &str,
    shortopt: Option<char>,
    longopt: &str,
    short_opts: &mut usize,
) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes[0] != b'-' {
        *short_opts = 0;
        return false;
    }
    if bytes.len() > 1 && bytes[1] == b'-' {
        *short_opts = 0;
        if text.len() > 2 && &text[2..] == longopt {
            return true;
        }
        return false;
    }
    if let Some(short) = shortopt {
        if text.contains(short) {
            *short_opts += 1;
            return true;
        }
    }
    false
}
/// Print usage information
pub fn usage(code: i32, keep_it_short: bool) {
    let progname = get_progname();
    let output = if code == 0 {
        format!(
            "jq - commandline JSON processor [version {}]\n\n\
            Usage:\t{} [options] <jq filter> [file...]\n\
            \t{} [options] --args <jq filter> [strings...]\n\
            \t{} [options] --jsonargs <jq filter> [JSON_TEXTS...]\n\n\
            jq is a tool for processing JSON inputs, applying the given filter to\n\
            its JSON text inputs and producing the filter's results as JSON on\n\
            standard output.\n\n\
            The simplest filter is ., which copies jq's input to its output\n\
            unmodified except for formatting. For more advanced filters see\n\
            the jq(1) manpage (\"man jq\") and/or https://jqlang.github.io/jq/.\n\n\
            Example:\n\n\t$ echo '{{\"foo\": 0}}' | jq .\n\
            \t{{\n\t  \"foo\": 0\n\t}}\n\n",
            JQ_VERSION, progname, progname, progname
        )
    } else {
        format!(
            "jq - commandline JSON processor [version {}]\n\n\
            Usage:\t{} [options] <jq filter> [file...]\n\n",
            JQ_VERSION, progname
        )
    };
    if code == 0 {
        print!("{}", output);
    } else {
        eprint!("{}", output);
    }
    if keep_it_short {
        eprintln!("For listing the command options, use {} --help.", progname);
    } else if code == 0 {
        println!(
            "{}",
            concat!(
                "Command options:\n",
                "  -n, --null-input          use `null` as the single input value;\n",
                "  -R, --raw-input           read each line as string instead of JSON;\n",
                "  -s, --slurp               read all inputs into an array and use it as\n",
                "                            the single input value;\n",
                "  -c, --compact-output      compact instead of pretty-printed output;\n",
                "  -r, --raw-output          output strings without escapes and quotes;\n",
                "      --raw-output0         implies -r and output NUL after each output;\n",
                "  -j, --join-output         implies -r and output without newline after\n",
                "                            each output;\n",
                "  -a, --ascii-output        output strings by only ASCII characters\n",
                "                            using escape sequences;\n",
                "  -S, --sort-keys           sort keys of each object on output;\n",
                "  -C, --color-output        colorize JSON output;\n",
                "  -M, --monochrome-output   disable colored output;\n",
                "      --tab                 use tabs for indentation;\n",
                "      --indent n            use n spaces for indentation (max 7 spaces);\n",
                "      --unbuffered          flush output stream after each output;\n",
                "      --stream              parse the input value in streaming fashion;\n",
                "      --stream-errors       implies --stream and report parse error as\n",
                "                            an array;\n",
                "      --seq                 parse input/output as application/json-seq;\n",
                "  -f, --from-file file      load filter from the file;\n",
                "  -L directory              search modules from the directory;\n",
                "      --arg name value      set $name to the string value;\n",
                "      --argjson name value  set $name to the JSON value;\n",
                "      --slurpfile name file set $name to an array of JSON values read\n",
                "                            from the file;\n",
                "      --rawfile name file   set $name to string contents of file;\n",
                "      --args                consume remaining arguments as positional\n",
                "                            string values;\n",
                "      --jsonargs            consume remaining arguments as positional\n",
                "                            JSON values;\n",
                "  -e, --exit-status         set exit status code based on the output;\n",
                "  -V, --version             show the version;\n",
                "  --build-configuration     show jq's build configuration;\n",
                "  -h, --help                show the help;\n",
                "  --                        terminates argument processing;\n\n",
                "Named arguments are also available as $ARGS.named[], while\n",
                "positional arguments are available as $ARGS.positional[]."
            )
        );
    }
    std_process::exit(code);
}
/// Exit with error message
pub fn die() -> ! {
    let progname = get_progname();
    eprintln!("Use {} --help for help with command-line options,", progname);
    eprintln!("or see the jq manpage, or online docs  at https://jqlang.github.io/jq");
    std_process::exit(2);
}
/// Debug callback
pub fn debug_cb(data: &mut CliCallbackData, input: Jv) {
    let dumpopts = data.dumpopts;
    let mut stderr = io::stderr();
    let arr = Jv::array().array_append(Jv::string("DEBUG:")).array_append(input);
    jv_dumpf(arr, &mut stderr, dumpopts & !JV_PRINT_PRETTY);
    eprintln!();
}
/// Stderr callback
pub fn stderr_cb(data: &mut CliCallbackData, input: Jv) {
    let dumpopts = data.dumpopts;
    let mut stderr = io::stderr();
    let is_tty = (dumpopts & JV_PRINT_ISATTY) != 0;
    if input.get_kind() == JvKind::String {
        let s = jv_string_value(&input);
        let _ = priv_fwrite(s, &mut stderr, is_tty);
    } else {
        let msg_str = jv_dump_string(input.clone(), 0);
        let s = jv_string_value(&msg_str);
        eprint!("{}", s);
    }
}
/// Input callback for jq `input`/`inputs` builtins.
pub fn input_cb(jq: &mut JqState<CliCallbackData>, data: &mut CliCallbackData) -> Jv {
    let Some(input_state) = &data.input_state else {
        return Jv::invalid();
    };
    let (value, filename, line) = {
        let mut state = input_state.borrow_mut();
        let value = jq_util_input_next_input(state.as_mut());
        (value, state.current_filename.clone(), state.current_line)
    };
    jq.input_filename = filename;
    jq.input_line = line;
    value
}
/// Main entry point
pub fn main() {
    let debug_compile = std::env::var("DEBUG_COMPILE").is_ok();
    let args: Vec<String> = env::args().collect();
    let argc = args.len() as i32;
    if !args.is_empty() {
        set_progname(args[0].clone());
    }
    let mut ret = JQ_OK_NO_OUTPUT;
    let mut compiled = false;
    let mut parser_flags = 0;
    let mut nfiles = 0;
    let mut last_result = -1;
    let mut options = 0;
    let mut jq_flags = 0;
    let mut jq_args = Jv::array();
    let mut program_arguments = Jv::object();
    let mut program: Option<String> = None;
    let mut dumpopts = compute_dumpopts(2);
    let mut jq_opt: Option<Box<JqState<CliCallbackData>>> = match jq_init() {
        Some(jq) => Some(jq),
        None => {
            eprintln!("jq_init failed");
            ret = JQ_ERROR_SYSTEM;
            jq_exit(ret);
        }
    };
    let jq = jq_opt.as_mut().unwrap().as_mut();
    // Initialize input state for handling files - C: jq_util_input_init(NULL, NULL)
    let input_state = Rc::new(RefCell::new(jq_util_input_init(None, None)));
    let mut further_args_are_strings = false;
    let mut further_args_are_json = false;
    let mut args_done = false;
    let mut short_opts: usize = 0;
    let mut lib_search_paths: Option<Jv> = None;
    let mut i = 1;
    while i < argc as usize {
        let arg = &args[i];
        short_opts = 0;
        if args_done || !isoptish(arg) {
            if program.is_none() {
                program = Some(arg.clone());
            } else if further_args_are_strings {
                jq_args = jq_args.array_append(Jv::string(arg));
            } else if further_args_are_json {
                let v = jv_parse(arg);
                if !jv_is_valid(&v) {
                    eprintln!(
                        "{}: invalid JSON text passed to --jsonargs", get_progname()
                    );
                    die();
                }
                jq_args = jq_args.array_append(v);
            } else {
                // C: jq_util_input_add_input(input_state, argv[i])
                let mut state = input_state.borrow_mut();
                jq_util_input_add_input(state.as_mut(), arg);
                nfiles += 1;
            }
        } else if arg == "--" {
            args_done = true;
        } else {
            if arg.starts_with("-L") {
                if lib_search_paths.is_none() {
                    lib_search_paths = Some(Jv::array());
                }
                if arg.len() > 2 {
                    let path = jq_realpath(Jv::string(&arg[2..]));
                    let paths = lib_search_paths.take().unwrap_or_else(Jv::array);
                    lib_search_paths = Some(paths.array_append(path));
                } else if i + 1 < argc as usize {
                    i += 1;
                    let path = jq_realpath(Jv::string(&args[i]));
                    let paths = lib_search_paths.take().unwrap_or_else(Jv::array);
                    lib_search_paths = Some(paths.array_append(path));
                } else {
                    eprintln!(
                        "-L takes a parameter: (e.g. -L /search/path or -L/search/path)"
                    );
                    die();
                }
                i += 1;
                continue;
            }
            if isoption(arg, Some('s'), "slurp", &mut short_opts) {
                options |= SLURP;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('r'), "raw-output", &mut short_opts) {
                options |= RAW_OUTPUT;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, None, "raw-output0", &mut short_opts) {
                options |= RAW_OUTPUT | RAW_NO_LF | RAW_OUTPUT0;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('j'), "join-output", &mut short_opts) {
                options |= RAW_OUTPUT | RAW_NO_LF;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('c'), "compact-output", &mut short_opts) {
                dumpopts &= !(JV_PRINT_TAB | compute_dumpopts(7));
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('C'), "color-output", &mut short_opts) {
                options |= COLOR_OUTPUT;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('M'), "monochrome-output", &mut short_opts) {
                options |= NO_COLOR_OUTPUT;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('a'), "ascii-output", &mut short_opts) {
                options |= ASCII_OUTPUT;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, None, "unbuffered", &mut short_opts) {
                options |= UNBUFFERED_OUTPUT;
                i += 1;
                continue;
            }
            if isoption(arg, Some('S'), "sort-keys", &mut short_opts) {
                options |= SORTED_OUTPUT;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('R'), "raw-input", &mut short_opts) {
                options |= RAW_INPUT;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('n'), "null-input", &mut short_opts) {
                options |= PROVIDE_NULL;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('f'), "from-file", &mut short_opts) {
                options |= FROM_FILE;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, Some('b'), "binary", &mut short_opts) {
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, None, "tab", &mut short_opts) {
                dumpopts &= !compute_dumpopts(7);
                dumpopts |= JV_PRINT_TAB | JV_PRINT_PRETTY;
                i += 1;
                continue;
            }
            if isoption(arg, None, "indent", &mut short_opts) {
                if i + 1 >= argc as usize {
                    eprintln!("{}: --indent takes one parameter", get_progname());
                    die();
                }
                dumpopts &= !(JV_PRINT_TAB | compute_dumpopts(7));
                let indent: i32 = args[i + 1].parse().unwrap_or(-2);
                if indent < -1 || indent > 7 {
                    eprintln!(
                        "{}: --indent takes a number between -1 and 7", get_progname()
                    );
                    die();
                }
                dumpopts |= compute_dumpopts(indent);
                i += 2;
                continue;
            }
            if isoption(arg, None, "seq", &mut short_opts) {
                options |= SEQ;
                i += 1;
                continue;
            }
            if isoption(arg, None, "stream", &mut short_opts) {
                parser_flags |= JV_PARSE_STREAMING;
                i += 1;
                continue;
            }
            if isoption(arg, None, "stream-errors", &mut short_opts) {
                parser_flags |= JV_PARSE_STREAMING | JV_PARSE_STREAM_ERRORS;
                i += 1;
                continue;
            }
            if isoption(arg, Some('e'), "exit-status", &mut short_opts) {
                options |= EXIT_STATUS;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, None, "args", &mut short_opts) {
                further_args_are_strings = true;
                further_args_are_json = false;
                i += 1;
                continue;
            }
            if isoption(arg, None, "jsonargs", &mut short_opts) {
                further_args_are_strings = false;
                further_args_are_json = true;
                i += 1;
                continue;
            }
            if isoption(arg, None, "arg", &mut short_opts) {
                if i + 2 >= argc as usize {
                    eprintln!(
                        "{}: --arg takes two parameters (e.g. --arg varname value)",
                        get_progname()
                    );
                    die();
                }
                let name = &args[i + 1];
                let value = &args[i + 2];
                if !object_has_key(&program_arguments, name) {
                    program_arguments = program_arguments
                        .object_set(Jv::string(name), Jv::string(value));
                }
                i += 3;
                continue;
            }
            if isoption(arg, None, "argjson", &mut short_opts) {
                if i + 2 >= argc as usize {
                    eprintln!(
                        "{}: --argjson takes two parameters (e.g. --argjson varname text)",
                        get_progname()
                    );
                    die();
                }
                let name = &args[i + 1];
                let json_text = &args[i + 2];
                if !object_has_key(&program_arguments, name) {
                    let v = jv_parse(json_text);
                    if !jv_is_valid(&v) {
                        eprintln!(
                            "{}: invalid JSON text passed to --argjson", get_progname()
                        );
                        die();
                    }
                    program_arguments = program_arguments
                        .object_set(Jv::string(name), v);
                }
                i += 3;
                continue;
            }
            if isoption(arg, None, "rawfile", &mut short_opts)
                || isoption(arg, None, "slurpfile", &mut short_opts)
            {
                let raw = isoption(arg, None, "rawfile", &mut short_opts);
                let which = if raw { "rawfile" } else { "slurpfile" };
                if i + 2 >= argc as usize {
                    eprintln!(
                        "{}: --{} takes two parameters (e.g. --{} varname filename)",
                        get_progname(),
                        which,
                        which
                    );
                    die();
                }
                let name = &args[i + 1];
                let filename = &args[i + 2];
                if !object_has_key(&program_arguments, name) {
                    let data = jv_load_file(filename, raw);
                    if !jv_is_valid(&data) {
                        let msg = jv_invalid_get_msg(data);
                        eprintln!(
                            "{}: Bad JSON in --{} {} {}: {}",
                            get_progname(),
                            which,
                            name,
                            filename,
                            jv_string_value(&msg)
                        );
                        ret = JQ_ERROR_SYSTEM;
                        jq_exit_for_options(ret, options);
                    }
                    program_arguments = program_arguments.object_set(Jv::string(name), data);
                }
                i += 3;
                continue;
            }
            if isoption(arg, None, "debug-dump-disasm", &mut short_opts) {
                options |= DUMP_DISASM;
                i += 1;
                continue;
            }
            if isoption(arg, None, "debug-trace=all", &mut short_opts) {
                jq_flags |= JQ_DEBUG_TRACE_ALL;
                if short_opts == 0 {
                    i += 1;
                    continue;
                }
            }
            if isoption(arg, None, "debug-trace", &mut short_opts) {
                jq_flags |= JQ_DEBUG_TRACE;
                i += 1;
                continue;
            }
            if isoption(arg, Some('h'), "help", &mut short_opts) {
                usage(0, false);
            }
            if isoption(arg, Some('V'), "version", &mut short_opts) {
                println!("jq-{}", JQ_VERSION);
                jq_exit(JQ_OK);
            }
            if isoption(arg, None, "build-configuration", &mut short_opts) {
                println!("{}", JQ_CONFIG);
                jq_exit(JQ_OK);
            }
            if isoption(arg, None, "run-tests", &mut short_opts) {
                options |= RUN_TESTS;
                i += 1;
                let libdirs = lib_search_paths.clone().unwrap_or_else(Jv::null);
                ret = jq_testsuite(
                    libdirs,
                    (options & DUMP_DISASM) != 0 || (jq_flags & JQ_DEBUG_TRACE) != 0,
                    (argc as usize - i) as i32,
                    args[i..].to_vec(),
                );
                jq_exit_for_options(ret, options);
            }
            if arg.len() != short_opts + 1 {
                eprintln!("{}: Unknown option {}", get_progname(), arg);
                die();
            }
        }
        i += 1;
    }
    if io::stdout().is_terminal() {
        dumpopts |= JV_PRINT_ISATTY | JV_PRINT_COLOR;
        if let Ok(no_color) = env::var("NO_COLOR") {
            if !no_color.is_empty() {
                dumpopts &= !JV_PRINT_COLOR;
            }
        }
    }
    if (options & SORTED_OUTPUT) != 0 {
        dumpopts |= JV_PRINT_SORTED;
    }
    if (options & ASCII_OUTPUT) != 0 {
        dumpopts |= JV_PRINT_ASCII;
    }
    if (options & COLOR_OUTPUT) != 0 {
        dumpopts |= JV_PRINT_COLOR;
    }
    if (options & NO_COLOR_OUTPUT) != 0 {
        dumpopts &= !JV_PRINT_COLOR;
    }
    if let Ok(colors) = env::var("JQ_COLORS") {
        if jq_set_colors(Some(&colors)) == 0 {
            eprintln!("Failed to set $JQ_COLORS");
        }
    }
    let lib_paths = lib_search_paths
        .unwrap_or_else(|| {
            Jv::array()
                .array_append(Jv::string("~/.jq"))
                .array_append(Jv::string("$ORIGIN/../lib/jq"))
                .array_append(Jv::string("$ORIGIN/../lib"))
    });
    jq_set_attr(jq, Jv::string("JQ_LIBRARY_PATH"), lib_paths);
    jq_set_attr(jq, Jv::string("JQ_ORIGIN"), dirname_jv(&args[0]));
    if !JQ_VERSION.contains('-') {
        jq_set_attr(jq, Jv::string("VERSION_DIR"), Jv::string(JQ_VERSION));
    } else {
        let idx = JQ_VERSION.find('-').unwrap();
        let version_dir = format!("{}-master", & JQ_VERSION[..idx]);
        jq_set_attr(jq, Jv::string("VERSION_DIR"), Jv::string(&version_dir));
    }
    if program.is_none()
        && (!io::stdout().is_terminal() || !io::stdin().is_terminal())
    {
        program = Some(".".to_string());
    }
    let program = match program {
        Some(p) => p,
        None => {
            usage(2, true);
            std_process::exit(2);
        }
    };
    if debug_compile { eprintln!("DEBUG main: program={:?}", program); }
    let args_obj = Jv::object()
        .object_set(Jv::string("positional"), jq_args)
        .object_set(Jv::string("named"), program_arguments.clone());
    program_arguments = program_arguments
        .object_set(Jv::string("ARGS"), args_obj.clone());
    if !object_has_key(&program_arguments, "JQ_BUILD_CONFIGURATION") {
        program_arguments = program_arguments
            .object_set(Jv::string("JQ_BUILD_CONFIGURATION"), Jv::string(JQ_CONFIG));
    }
    if (options & FROM_FILE) != 0 {
        let data = jv_load_file(&program, true);
        if !jv_is_valid(&data) {
            let msg = jv_invalid_get_msg(data);
            eprintln!("{}: {}", get_progname(), jv_string_value(&msg));
            ret = JQ_ERROR_SYSTEM;
            jq_exit_for_options(ret, options);
        }
        jq_set_attr(
            jq,
            Jv::string("PROGRAM_ORIGIN"),
            jq_realpath(dirname_jv(&program)),
        );
        compiled = jq_compile_args(jq, jv_string_value(&data), program_arguments.clone());
    } else {
        jq_set_attr(jq, Jv::string("PROGRAM_ORIGIN"), jq_realpath(Jv::string(".")));
        if debug_compile { eprintln!("DEBUG main: calling jq_compile_args with program={:?}", program); }
        compiled = jq_compile_args(jq, &program, program_arguments.clone());
        if debug_compile { eprintln!("DEBUG main: jq_compile_args returned {}", compiled); }
    }
    if !compiled {
        if debug_compile { eprintln!("DEBUG main: compilation failed, exiting with code 3"); }
        ret = JQ_ERROR_COMPILE;
        jq_exit_for_options(ret, options);
    }
    if (options & DUMP_DISASM) != 0 {
        jq_dump_disassembly(jq, 0);
        println!();
    }
    if (options & SEQ) != 0 {
        parser_flags |= JV_PARSE_SEQ;
    }
    // C: if ((options & RAW_INPUT))
    //     jq_util_input_set_parser(input_state, NULL, (options & SLURP) ? 1 : 0);
    // else
    //     jq_util_input_set_parser(input_state, jv_parser_new(parser_flags), (options & SLURP) ? 1 : 0);
    {
        let mut state = input_state.borrow_mut();
        if (options & RAW_INPUT) != 0 {
            jq_util_input_set_parser(state.as_mut(), None, (options & SLURP) != 0);
        } else {
            jq_util_input_set_parser(
                state.as_mut(),
                Some(jv_parser_new(parser_flags)),
                (options & SLURP) != 0,
            );
        }
    }
    let callback_data = CliCallbackData {
        dumpopts,
        input_state: Some(Rc::clone(&input_state)),
    };
    jq_set_input_cb(jq, Some(input_cb), Some(Box::new(callback_data.clone())));
    jq_set_debug_cb(jq, Some(debug_cb), Some(Box::new(callback_data.clone())));
    jq_set_stderr_cb(jq, stderr_cb, Box::new(callback_data));
    // C: if (nfiles == 0) jq_util_input_add_input(input_state, "-");
    if nfiles == 0 {
        let mut state = input_state.borrow_mut();
        jq_util_input_add_input(state.as_mut(), "-");
    }

    let mut stdout = OutputTracker::new(io::stdout());
    if (options & PROVIDE_NULL) != 0 {
        sync_jq_input_position(jq, &input_state);
        ret = process(
            jq,
            Jv::null(),
            jq_flags,
            dumpopts,
            options,
            &mut stdout,
            Some(&input_state),
        );
    } else {
        // C: while (jq_util_input_errors(input_state) == 0 &&
        //          (jv_is_valid((value = jq_util_input_next_input(input_state))) || jv_invalid_has_msg(jv_copy(value))))
        loop {
            if {
                let state = input_state.borrow();
                jq_util_input_errors(state.as_ref()) != 0
            } {
                break;
            }
            let value = {
                let mut state = input_state.borrow_mut();
                jq_util_input_next_input(state.as_mut())
            };
            sync_jq_input_position(jq, &input_state);
            let has_msg = invalid_has_msg(&value);
            if !jv_is_valid(&value) && !has_msg {
                break;
            }

            if jv_is_valid(&value) {
                ret = process(
                    jq,
                    value,
                    jq_flags,
                    dumpopts,
                    options,
                    &mut stdout,
                    Some(&input_state),
                );
                if ret <= 0 && ret != JQ_OK_NO_OUTPUT {
                    last_result = if ret != JQ_OK_NULL_KIND { 1 } else { 0 };
                }
                if jq_halted(jq) {
                    break;
                }
            } else if (options & SEQ) == 0 {
                // Parse error
                ret = JQ_ERROR_UNKNOWN;
                let msg = jv_invalid_get_msg(value);
                if jv_is_valid(&msg) {
                    if msg.get_kind() == JvKind::String {
                        let s = jv_string_value(&msg);
                        eprintln!("jq: parse error: {}", s);
                    } else {
                        let msg_str = jv_dump_string(msg.clone(), 0);
                        let s = jv_string_value(&msg_str);
                        eprintln!("jq: parse error: {}", s);
                    }
                }
                break;
            } else {
                let msg = jv_invalid_get_msg(value);
                if jv_is_valid(&msg) {
                    if msg.get_kind() == JvKind::String {
                        eprintln!("jq: ignoring parse error: {}", jv_string_value(&msg));
                    } else {
                        let msg_str = jv_dump_string(msg.clone(), 0);
                        eprintln!("jq: ignoring parse error: {}", jv_string_value(&msg_str));
                    }
                }
            }
        }
    }
    if {
        let state = input_state.borrow();
        jq_util_input_errors(state.as_ref()) != 0
    } {
        ret = JQ_ERROR_SYSTEM;
    }
    let _ = stdout.flush();
    if let Some(e) = stdout.take_error() {
        eprintln!("jq: error: writing output failed: {}", e);
        ret = JQ_ERROR_SYSTEM;
    }
    // The jq reference goes out of scope naturally, then we can teardown
    let _ = jq;
    jq_teardown(&mut jq_opt);
    if let Ok(cell) = Rc::try_unwrap(input_state) {
        let mut state = Some(cell.into_inner());
        jq_util_input_free(&mut state);
    }
    if (options & EXIT_STATUS) != 0 {
        if ret != JQ_OK_NO_OUTPUT {
            jq_exit_with_status(ret);
        } else {
            match last_result {
                -1 => jq_exit_with_status(JQ_OK_NO_OUTPUT),
                0 => jq_exit_with_status(JQ_OK_NULL_KIND),
                _ => jq_exit_with_status(JQ_OK),
            }
        }
    } else {
        jq_exit(ret);
    }
}
