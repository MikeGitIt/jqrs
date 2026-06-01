//! Module: util
//!
//! Contains 18 transpiled functions:
//! - fprinter:11900793304520160309:./src/util.c
//! - jq_util_input_init:12212695250592004423:./src/util.c
//! - jq_util_input_add_input:15808454133259937558:./src/util.c
//! - jq_util_input_errors:3716476318019785351:./src/util.c
//! - jq_util_input_get_current_line:6551901662964698886:./src/util.c
//! - jq_util_input_read_more:7075307367602679184:./src/util.c
//! - _jq_memmem:6443846402051902619:./src/util.c
//! - jq_realpath:7398732454840351177:./src/util.c
//! - jq_util_input_get_position:17986843621558044628:./src/util.c
//! - get_home:5644323564201478273:./src/util.c
//! - jq_util_input_next_input:12979333437935824050:./src/util.c
//! - jv_is_valid:7339449971924854417:./src/util.c
//! - expand_path:3967281103540934171:./src/util.c
//! - jq_util_input_set_parser:11534077509349252365:./src/util.c
//! - jq_util_input_get_current_filename:8686038951047374605:./src/util.c
//! - jq_util_input_free:1860070736067808844:./src/util.c
//! - jq_util_input_next_input_cb:15429146817113954657:./src/util.c
//! - next_file:936044596481893571:./src/util.c

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
// JqUtilInputState.parser is Box<dyn Any> to break circular dependency.
// We downcast to ExtendedJvParser when using the parser.
use crate::jv_parse::{ExtendedJvParser, jv_parser_remaining, jv_parser_set_buf, jv_parser_next};
use crate::jv_alloc::{jv_mem_alloc, jv_mem_free, jv_mem_realloc, jv_mem_strdup, jv_mem_calloc};
use crate::execute::jq_get_input_cb;
use crate::jv::{Jv, JvKind};
use crate::types::*;
use crate::types::{JvParser, JqState, JqInputCb};
use crate::inject_errors::{fwrite, ferror, fclose, clearerr, fopen, fgets};
/// Free a jq_util_input_state
pub fn jq_util_input_free(state: &mut Option<Box<JqUtilInputState>>) {
    let old_state = state.take();
    if let Some(mut s) = old_state {
        jv_free(std::mem::replace(&mut s.slurped, Jv::invalid()));
        jv_free(std::mem::replace(&mut s.current_filename, Jv::invalid()));
    }
}
/// Get the home directory
pub fn get_home() -> Jv {
    match env::var("HOME") {
        Ok(home) => Jv::string(&home),
        Err(_) => Jv::invalid_with_msg(Jv::string("Could not find home directory.")),
    }
}
/// Get current position (filename and line).
pub fn jq_util_input_get_position<T>(jq: &mut JqState<T>) -> Jv {
    if jv_get_kind(&jq.input_filename) == JvKind::String {
        let filename = crate::jv::jv_string_value(&jq.input_filename);
        Jv::string(&format!("{}:{}", filename, jq.input_line))
    } else {
        Jv::invalid()
    }
}
/// Get the next file from the input state
fn next_file(state: &mut JqUtilInputState) -> Option<&str> {
    if state.curr_file < state.nfiles {
        let idx = state.curr_file as usize;
        state.curr_file += 1;
        state.files.get(idx).map(|s| s.as_str())
    } else {
        None
    }
}
/// Add an input file to the state
pub fn jq_util_input_add_input(state: &mut JqUtilInputState, fname: &str) {
    state.files.push(fname.to_string());
    state.nfiles += 1;
}
/// Default error printer
fn fprinter(data: &mut dyn std::any::Any, fname: &str) {
    eprintln!(
        "jq: error: Could not open file {}: {}", fname, "No such file or directory"
    );
}
/// Set the parser for the input state
/// C: void jq_util_input_set_parser(jq_util_input_state *state, jv_parser *parser, int slurp)
pub fn jq_util_input_set_parser(
    state: &mut JqUtilInputState,
    parser: Option<Box<ExtendedJvParser>>,
    slurp: bool,
) {
    assert!(!jv_is_valid(&state.slurped));
    // Store ExtendedJvParser as Box<dyn Any>
    state.parser = parser.map(|p| p as Box<dyn std::any::Any>);
    if state.parser.is_none() && slurp {
        state.slurped = Jv::string("");
    } else if slurp {
        state.slurped = Jv::array();
    } else {
        state.slurped = Jv::invalid();
    }
}
/// Read more input from the current file
fn jq_util_input_read_more(state: &mut JqUtilInputState) -> bool {
    let need_new_file = state.current_input.is_none() || state.current_input_finished;
    if need_new_file {
        if state.current_input.is_some() {
            state.current_input = None;
        }
        state.current_input_finished = false;
        jv_free(std::mem::replace(&mut state.current_filename, Jv::invalid()));
        state.current_line = 0;
        let next = if state.curr_file < state.nfiles {
            let idx = state.curr_file as usize;
            state.curr_file += 1;
            state.files.get(idx).cloned()
        } else {
            None
        };
        if let Some(f) = next {
            if f == "-" {
                state.current_input = Some(BufReader::new(Box::new(io::stdin())));
                state.current_filename = Jv::string("<stdin>");
            } else {
                match File::open(&f) {
                    Ok(file) => {
                        state.current_input = Some(BufReader::new(Box::new(file)));
                        state.current_filename = Jv::string(&f);
                        state.current_input_finished = false;
                    }
                    Err(_) => {
                        if let Some(ref cb) = state.err_cb {
                            if let Some(ref mut data) = state.err_cb_data {
                                cb(data.as_mut(), &f);
                            }
                        }
                        state.failures += 1;
                    }
                }
            }
            state.current_line = 0;
        }
    }
    state.buf_valid_len = 0;
    if let Some(ref mut input) = state.current_input {
        let limit = state.buf.len().saturating_sub(1);
        let mut total = 0usize;
        let mut saw_newline = false;
        while total < limit {
            let available = match input.fill_buf() {
                Ok(bytes) => bytes,
                Err(_) => {
                    state.failures += 1;
                    state.current_input_finished = true;
                    break;
                }
            };
            if available.is_empty() {
                state.current_input_finished = true;
                break;
            }
            let remaining = limit - total;
            let scan_len = available.len().min(remaining);
            let take = match available[..scan_len].iter().position(|&b| b'\n' == b) {
                Some(pos) => {
                    saw_newline = true;
                    pos + 1
                }
                None => scan_len,
            };
            state.buf[total..total + take].copy_from_slice(&available[..take]);
            input.consume(take);
            total += take;
            if saw_newline {
                state.current_line += 1;
                break;
            }
            if take == remaining {
                break;
            }
        }
        if !saw_newline && state.parser.is_some() {
            state.buf_valid_len = state.buf[..total]
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(total);
        } else {
            state.buf_valid_len = total;
        }
        if state.buf_valid_len == 0 {
            state.buf[0] = 0;
        }
    }
    // Return true if this is the last input (no more files and no current file)
    state.curr_file >= state.nfiles
        && (state.current_input.is_none() || state.current_input_finished)
}
/// Get the real path of a file
pub fn jq_realpath(path: Jv) -> Jv {
    let path_str = match jv_string_value(&path) {
        Some(s) => s.to_string(),
        None => return path,
    };
    match std::fs::canonicalize(&path_str) {
        Ok(real_path) => {
            jv_free(path);
            Jv::string(real_path.to_string_lossy().as_ref())
        }
        Err(_) => path,
    }
}
/// Get the number of input errors
pub fn jq_util_input_errors(state: &JqUtilInputState) -> i32 {
    state.failures
}
/// Checks if a jv value is valid (not an invalid/error value)
///
/// Returns true if the value is valid, false if invalid
#[inline]
pub fn jv_is_valid(x: &Jv) -> bool {
    jv_get_kind(x) != JvKind::Invalid
}
/// Free a Jv value
fn jv_free(x: Jv) {
    crate::jv::jv_free(x);
}

/// Helper to get parser as mutable ExtendedJvParser reference (downcasts from Box<dyn Any>)
fn get_parser_mut(state: &mut JqUtilInputState) -> Option<&mut ExtendedJvParser> {
    state.parser.as_mut()?.downcast_mut::<ExtendedJvParser>()
}
/// Get string value from a Jv
fn jv_string_value(x: &Jv) -> Option<&str> {
    if x.get_kind() == JvKind::String {
        Some(crate::jv::jv_string_value(x))
    } else {
        None
    }
}
/// Gets the kind/type of a jv value from its kind_flags field
///
/// This extracts the type information from the jv structure
#[inline]
pub fn jv_get_kind(x: &Jv) -> JvKind {
    let kind_value = x.kind_flags & 0x0F;
    match kind_value {
        0 => JvKind::Invalid,
        1 => JvKind::Null,
        2 => JvKind::False,
        3 => JvKind::True,
        4 => JvKind::Number,
        5 => JvKind::String,
        6 => JvKind::Array,
        7 => JvKind::Object,
        _ => JvKind::Invalid,
    }
}
/// Input callback for jq_util
pub fn jq_util_input_next_input_cb<T>(
    _jq: &mut JqState<T>,
    _data: &mut dyn std::any::Any,
) -> Jv {
    Jv::invalid()
}
/// Initialize input state
pub fn jq_util_input_init(
    err_cb: Option<JqUtilMsgCb>,
    err_cb_data: Option<Box<dyn std::any::Any>>,
) -> Box<JqUtilInputState> {
    let (cb, cb_data) = if err_cb.is_none() {
        (Some(fprinter as JqUtilMsgCb), None)
    } else {
        (err_cb, err_cb_data)
    };
    let mut new_state = Box::new(JqUtilInputState::default());
    new_state.err_cb = cb;
    new_state.err_cb_data = cb_data;
    new_state.slurped = Jv::invalid();
    new_state.current_filename = Jv::invalid();
    new_state
}
/// Get the current filename from the input state
pub fn jq_util_input_get_current_filename(state: &JqUtilInputState) -> Jv {
    if jv_get_kind(&state.current_filename) == JvKind::String {
        state.current_filename.clone()
    } else {
        Jv::invalid()
    }
}
/// Get the current line number
pub fn jq_util_input_get_current_line<T>(jq: &mut JqState<T>) -> Jv {
    if jv_get_kind(&jq.input_filename) == JvKind::String {
        Jv::number(jq.input_line as f64)
    } else {
        Jv::null()
    }
}
/// Check if the input state is in slurp mode
pub fn jq_util_input_is_slurping(state: &JqUtilInputState) -> bool {
    jv_is_valid(&state.slurped)
}
/// Get the slurped value from the input state
pub fn jq_util_input_get_slurped(state: &mut JqUtilInputState) -> Jv {
    std::mem::replace(&mut state.slurped, Jv::invalid())
}
fn jv_string(s: &str) -> Jv {
    crate::jv::jv_string(s)
}
fn jv_string_sized(_buf: &[u8], _len: usize) -> Jv {
    crate::jv::jv_string_append_buf(Jv::string(""), _buf, _len as i32)
}
fn jv_string_concat(a: Jv, b: Jv) -> Jv {
    crate::jv::jv_string_concat(a, b)
}
fn jv_string_length_bytes(jv: Jv) -> i32 {
    crate::jv::jv_string_length_bytes(&jv)
}
fn jv_string_fmt(_fmt: &str, _args: &[&str]) -> Jv {
    jv_string("")
}
fn jv_number(n: usize) -> Jv {
    Jv::number(n as f64)
}
fn jv_invalid_with_msg(msg: Jv) -> Jv {
    crate::jv::jv_invalid_with_msg(msg)
}
fn jv_invalid_has_msg(jv: Jv) -> bool {
    jv.get_kind() == JvKind::Invalid
        && (jv.kind_flags & crate::jv::JVP_PAYLOAD_ALLOCATED) != 0
}
fn jv_invalid_get_msg(jv: Jv) -> Jv {
    crate::jv::jv_invalid_get_msg(jv)
}
fn jv_copy(jv: &Jv) -> Jv {
    jv.clone()
}
fn jv_array_append(arr: Jv, val: Jv) -> Jv {
    crate::jv::jv_array_append(arr, val)
}
/// Expand a path that starts with ~/
pub fn expand_path(path: Jv) -> Jv {
    assert!(jv_get_kind(&path) == JvKind::String);
    let pstr = match jv_string_value(&path) {
        Some(s) => s,
        None => return path,
    };
    let path_len = jv_string_length_bytes(jv_copy(&path));
    if path_len > 1 && pstr.starts_with("~/") {
        let home = get_home();
        if jv_is_valid(&home) {
            let home_str = jv_string_value(&home).unwrap_or("");
            let expanded = format!("{}/{}", home_str, &pstr[2..]);
            jv_free(home);
            jv_free(path);
            return jv_string(&expanded);
        } else {
            let emsg = jv_invalid_get_msg(home);
            let emsg_str = jv_string_value(&emsg).unwrap_or("");
            let ret = jv_invalid_with_msg(
                jv_string(&format!("Could not expand {}. ({})", pstr, emsg_str)),
            );
            jv_free(emsg);
            jv_free(path);
            return ret;
        }
    }
    path
}
/// Get the next input value
pub fn jq_util_input_next_input(state: &mut JqUtilInputState) -> Jv {
    let mut is_last = false;
    let mut has_more = false;
    let mut value = Jv::invalid();
    loop {
        if state.parser.is_none() {
            is_last = jq_util_input_read_more(state);
            if state.buf_valid_len == 0 {
                if is_last && !has_more {
                    break;
                }
                continue;
            }
            if jv_is_valid(&state.slurped) {
                state.slurped = jv_string_concat(
                    std::mem::replace(&mut state.slurped, Jv::invalid()),
                    jv_string_sized(&state.buf, state.buf_valid_len),
                );
            } else {
                if !jv_is_valid(&value) {
                    value = jv_string("");
                }
                if state.buf_valid_len > 0 && state.buf[state.buf_valid_len - 1] == b'\n'
                {
                    state.buf[state.buf_valid_len - 1] = 0;
                    return jv_string_concat(
                        value,
                        jv_string_sized(&state.buf, state.buf_valid_len - 1),
                    );
                }
                value = jv_string_concat(
                    value,
                    jv_string_sized(&state.buf, state.buf_valid_len),
                );
                state.buf[0] = 0;
                state.buf_valid_len = 0;
            }
        } else {
            // Check if parser needs more data using real jv_parser_remaining
            let needs_more = {
                if let Some(parser) = get_parser_mut(state) {
                    jv_parser_remaining(parser) == 0
                } else {
                    true // No parser, need more data
                }
            };

            if needs_more {
                is_last = jq_util_input_read_more(state);
                // Set the buffer on the parser using real jv_parser_set_buf
                let buf_len = state.buf_valid_len;
                if let Some(mut parser_any) = state.parser.take() {
                    if let Some(parser) = parser_any.downcast_mut::<ExtendedJvParser>() {
                        jv_parser_set_buf(parser, &state.buf[..buf_len], buf_len as i32, !is_last);
                    }
                    state.parser = Some(parser_any);
                }
            }

            // Get next value from parser using real jv_parser_next
            if let Some((parsed_value, parser_has_more)) = {
                get_parser_mut(state).map(|parser| {
                    let parsed_value = jv_parser_next(parser);
                    let parser_has_more = jv_parser_remaining(parser) > 0;
                    (parsed_value, parser_has_more)
                })
            } {
                value = parsed_value;
                has_more = parser_has_more;
            } else {
                value = Jv::invalid();
                has_more = false;
            }

            // Check slurped state
            if jv_is_valid(&state.slurped) {
                if jv_is_valid(&value) {
                    state.slurped = jv_array_append(
                        std::mem::replace(&mut state.slurped, Jv::invalid()),
                        value,
                    );
                    value = Jv::invalid();
                } else if jv_invalid_has_msg(jv_copy(&value)) {
                    return value;
                }
            } else if jv_is_valid(&value) || jv_invalid_has_msg(jv_copy(&value)) {
                return value;
            }
        }
        if is_last && !has_more {
            break;
        }
    }
    if jv_is_valid(&state.slurped) {
        value = std::mem::replace(&mut state.slurped, Jv::invalid());
        state.slurped = Jv::invalid();
    }
    value
}
/// Find needle in haystack (memmem equivalent)
pub fn _jq_memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|window| window == needle)
}
impl JqUtilInputState {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for JqUtilInputState {
    fn default() -> Self {
        Self {
            err_cb: None,
            err_cb_data: None,
            parser: None,
            current_input: None,
            current_input_finished: false,
            files: Vec::new(),
            nfiles: 0,
            curr_file: 0,
            failures: 0,
            slurped: Jv::invalid(),
            buf: vec![0u8; 4096],
            buf_valid_len: 0,
            current_filename: Jv::invalid(),
            current_line: 0,
        }
    }
}
