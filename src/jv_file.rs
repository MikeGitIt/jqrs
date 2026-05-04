//! Module: jv_file
//!
//! Contains 1 transpiled functions:
//! - jv_load_file:6498013073032868138:./src/jv_file.c
use std::path::Path;
use crate::jv_parse::jv_parser_new;
use crate::types::*;
use crate::util::{
    jq_util_input_add_input, jq_util_input_errors, jq_util_input_free, jq_util_input_init,
    jq_util_input_next_input, jq_util_input_set_parser,
};
/// Check if a jv value is valid
#[inline]
pub fn jv_is_valid(x: &Jv) -> bool {
    (x.kind_flags & 0x0F) != JvKind::Invalid as u8
}
/// Create an invalid jv with an error message
fn jv_invalid_with_msg(msg: Jv) -> Jv {
    crate::jv::jv_invalid_with_msg(msg)
}
/// Create a formatted string jv
fn jv_string_fmt(fmt: &str, args: &[&str]) -> Jv {
    let mut result = fmt.to_string();
    for arg in args {
        if let Some(pos) = result.find("%s") {
            result.replace_range(pos..pos + 2, arg);
        }
    }
    crate::jv::jv_string(&result)
}
/// Create a string jv
fn jv_string(s: &str) -> Jv {
    crate::jv::jv_string(s)
}
/// Create an empty array jv
fn jv_array() -> Jv {
    crate::jv::jv_array()
}
/// Append a buffer to a string jv
fn jv_string_append_buf(s: Jv, buf: &[u8], n: usize) -> Jv {
    crate::jv::jv_string_append_buf(s, buf, n as i32)
}
/// Append a value to an array jv
fn jv_array_append(arr: Jv, value: Jv) -> Jv {
    crate::jv::jv_array_append(arr, value)
}
/// Check if invalid jv has a message
fn jv_invalid_has_msg(v: Jv) -> bool {
    v.get_kind() == JvKind::Invalid
        && (v.kind_flags & crate::jv::JVP_PAYLOAD_ALLOCATED) != 0
}
/// Copy a jv value
fn jv_copy(v: &Jv) -> Jv {
    v.clone()
}
/// Free a jv value
fn jv_free(_v: Jv) {}
/// Backtrack UTF-8 to find complete character boundary
/// Returns true if we need to read more bytes, sets len to number of additional bytes needed
fn jvp_utf8_backtrack(end: &u8, _start: &[u8], len: &mut i32) -> bool {
    let byte = *end;
    if (byte & 0xC0) == 0x80 {
        *len = 1;
        return true;
    }
    if (byte & 0xE0) == 0xC0 {
        *len = 1;
        return true;
    } else if (byte & 0xF0) == 0xE0 {
        *len = 2;
        return true;
    } else if (byte & 0xF8) == 0xF0 {
        *len = 3;
        return true;
    }
    *len = 0;
    false
}
/// Load a file and return its contents as a jv value
///
/// If raw is true, returns the file contents as a string.
/// If raw is false, parses the file as JSON and returns an array of parsed values.
pub fn jv_load_file(filename: &str, raw: bool) -> Jv {
    let path = Path::new(filename);
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            return jv_invalid_with_msg(
                jv_string_fmt("Could not open %s: %s", &[filename, &e.to_string()]),
            );
        }
    };
    if metadata.is_dir() {
        return jv_invalid_with_msg(
            jv_string_fmt("Could not open %s: %s", &[filename, "It's a directory"]),
        );
    }
    if raw {
        let buf = match std::fs::read(path) {
            Ok(buf) => buf,
            Err(e) => {
                return jv_invalid_with_msg(
                    jv_string_fmt("Error reading from %s: %s", &[filename, &e.to_string()]),
                );
            }
        };
        return jv_string_append_buf(jv_string(""), &buf, buf.len());
    }

    let mut input_state = Some(jq_util_input_init(None, None));
    let state = input_state.as_mut().unwrap();
    jq_util_input_add_input(state.as_mut(), filename);
    jq_util_input_set_parser(state.as_mut(), Some(jv_parser_new(0)), false);
    let mut data = jv_array();
    loop {
        let value = jq_util_input_next_input(state.as_mut());
        if jv_is_valid(&value) {
            data = jv_array_append(data, value);
        } else if jv_invalid_has_msg(jv_copy(&value)) {
            jv_free(data);
            jq_util_input_free(&mut input_state);
            return value;
        } else {
            break;
        }
    }
    if jq_util_input_errors(state.as_ref()) != 0 {
        jv_free(data);
        jq_util_input_free(&mut input_state);
        return jv_invalid_with_msg(jv_string_fmt("Error reading from %s", &[filename]));
    }
    jq_util_input_free(&mut input_state);
    data
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jv_is_valid() {
        let valid = Jv {
            kind_flags: JvKind::String as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        };
        assert!(jv_is_valid(& valid));
        let invalid = Jv {
            kind_flags: JvKind::Invalid as u8,
            pad_: 0,
            offset: 0,
            size: 0,
            u: 0,
        };
        assert!(! jv_is_valid(& invalid));
    }
}
