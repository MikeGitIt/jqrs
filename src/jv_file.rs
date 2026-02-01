//! Module: jv_file
//!
//! Contains 1 transpiled functions:
//! - jv_load_file:6498013073032868138:./src/jv_file.c
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use crate::jv_parse::{jv_parser_new, jv_parser_set_buf, jv_parser_next, jv_parser_free, ExtendedJvParser};
use crate::types::*;
/// Check if a jv value is valid
#[inline]
pub fn jv_is_valid(x: &Jv) -> bool {
    (x.kind_flags & 0x0F) != JvKind::Invalid as u8
}
/// Create an invalid jv with an error message
fn jv_invalid_with_msg(msg: Jv) -> Jv {
    Jv {
        kind_flags: JvKind::Invalid as u8,
        pad_: 0,
        offset: 0,
        size: msg.size,
        u: msg.u,
    }
}
/// Create a formatted string jv
fn jv_string_fmt(fmt: &str, args: &[&str]) -> Jv {
    let mut result = fmt.to_string();
    for arg in args {
        if let Some(pos) = result.find("%s") {
            result.replace_range(pos..pos + 2, arg);
        }
    }
    jv_string(&result)
}
/// Create a string jv
fn jv_string(s: &str) -> Jv {
    Jv {
        kind_flags: JvKind::String as u8,
        pad_: 0,
        offset: 0,
        size: s.len() as i32,
        u: 0,
    }
}
/// Create an empty array jv
fn jv_array() -> Jv {
    Jv {
        kind_flags: JvKind::Array as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
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
    (v.kind_flags & 0x0F) == JvKind::Invalid as u8 && v.size > 0
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
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return jv_invalid_with_msg(
                jv_string_fmt("Could not open %s: %s", &[filename, &e.to_string()]),
            );
        }
    };
    let mut reader = BufReader::new(file);
    let mut parser: Option<Box<ExtendedJvParser>> = None;
    let mut data = if raw {
        jv_string("")
    } else {
        parser = Some(jv_parser_new(0));
        jv_array()
    };
    const MAX_UTF8_LEN: usize = 4;
    let mut buf = vec![0u8; 4096 + MAX_UTF8_LEN];
    let mut read_error = false;
    loop {
        let n = match reader.read(&mut buf[..4096]) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => {
                read_error = true;
                break;
            }
        };
        let mut total_read = n;
        if n > 0 {
            let mut additional_len = 0i32;
            if jvp_utf8_backtrack(&buf[n - 1], &buf[..n], &mut additional_len)
                && additional_len > 0
            {
                if let Ok(extra) = reader.read(&mut buf[n..n + additional_len as usize])
                {
                    total_read += extra;
                }
            }
        }
        if raw {
            data = jv_string_append_buf(data, &buf[..total_read], total_read);
        } else {
            if let Some(ref mut p) = parser {
                jv_parser_set_buf(p, &buf[..total_read], total_read as i32, true);
                loop {
                    let value = jv_parser_next(p);
                    if jv_is_valid(&value) {
                        data = jv_array_append(data, value);
                    } else {
                        if jv_invalid_has_msg(jv_copy(&value)) {
                            jv_free(data);
                            data = value;
                            read_error = true;
                        }
                        break;
                    }
                }
                if read_error {
                    break;
                }
            }
        }
    }
    if let Some(p) = parser {
        jv_parser_free(p);
    }
    if read_error {
        jv_free(data);
        return jv_invalid_with_msg(jv_string_fmt("Error reading from %s", &[filename]));
    }
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
