//! Module: jv_print
//!
//! Contains 13 transpiled functions:
//! - put_char:2716371292585625033:./src/jv_print.c
//! - jv_dump_string:9504807833710346057:./src/jv_print.c
//! - jv_dump:1219146961517150128:./src/jv_print.c
//! - put_refcnt:14494530491080811139:./src/jv_print.c
//! - jv_show:13923024414539252758:./src/jv_print.c
//! - jv_dump_term:15415922214526667467:./src/jv_print.c
//! - put_indent:11285333772322634116:./src/jv_print.c
//! - put_str:3529553261230838825:./src/jv_print.c
//! - jv_dumpf:15226245779486998786:./src/jv_print.c
//! - put_buf:9833420416662901885:./src/jv_print.c
//! - jv_dump_string_trunc:9470236168420923533:./src/jv_print.c
//! - jvp_dump_string:11409141874396919673:./src/jv_print.c
//! - jq_set_colors:11763536512137222280:./src/jv_print.c
use std::io::{self, Write};
// Note: jv_dumpf, put_buf are defined locally in this file
use crate::types::{Jv, JvKind};
use std::fmt::Write as FmtWrite;
use crate::jv::{
    jv_get_kind, jv_copy, jv_free, jv_string_value,
    jv_string_length_bytes, jv_string_append_buf, jv_string, jv_array_length,
    jv_array_get, jv_object_length, jv_object_iter, jv_object_iter_next,
    jv_object_iter_valid, jv_object_iter_key, jv_object_iter_value,
    jv_object_get, jv_null, jv_invalid_get_msg, jv_number_value, jv_number_get_literal,
    jv_get_refcnt,
};
use crate::jv_aux::jv_keys;
use crate::jv_dtoa::{DtoaContext, jvp_dtoa_fmt};
use crate::jv_dtoa_tsd::tsd_dtoa_context_get;
use crate::jv_unicode::jvp_utf8_next;
/// Dump a jv value to stdout
pub fn jv_dump(x: Jv, flags: i32) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    jv_dumpf(x, &mut handle, flags);
}
/// Put a single character to output
fn put_char<W: Write>(
    c: char,
    fout: &mut Option<&mut W>,
    strout: &mut Option<&mut Jv>,
    _is_tty: i32,
) {
    if let Some(s) = strout {
        let mut buf = [0u8; 4];
        let encoded = c.encode_utf8(&mut buf);
        **s = jv_string_append_buf((*s).clone(), encoded.as_bytes(), encoded.len() as i32);
    } else if let Some(f) = fout {
        let mut buf = [0u8; 4];
        let encoded = c.encode_utf8(&mut buf);
        let _ = f.write_all(encoded.as_bytes());
    }
}
/// Write a string to the output
///
/// Parameters:
/// - s: The string to write
/// - fout: Optional file output writer
/// - strout: Optional string output buffer
/// - t: TTY flag for formatting
pub fn put_str<W: Write>(
    s: &str,
    fout: &mut Option<&mut W>,
    strout: &mut Option<&mut Jv>,
    t: i32,
) {
    put_buf(s, s.len() as i32, fout, strout, t);
}
/// JV print flags
pub const JV_PRINT_PRETTY: i32 = 1;
pub const JV_PRINT_ASCII: i32 = 2;
pub const JV_PRINT_COLOR: i32 = 4;
pub const JV_PRINT_SORTED: i32 = 8;
pub const JV_PRINT_INVALID: i32 = 16;
pub const JV_PRINT_REFCOUNT: i32 = 32;
pub const JV_PRINT_TAB: i32 = 64;
pub const JV_PRINT_ISATTY: i32 = 128;
pub const JV_PRINT_SPACE0: i32 = 0 << 8;
pub const JV_PRINT_SPACE1: i32 = 1 << 8;
pub const JV_PRINT_SPACE2: i32 = 2 << 8;
const MAX_PRINT_DEPTH: i32 = 256;
/// Default color definitions for syntax highlighting
const DEF_COLORS: [&str; 8] = [
    "\x1b[1;30m",
    "\x1b[0;39m",
    "\x1b[0;39m",
    "\x1b[0;39m",
    "\x1b[0;32m",
    "\x1b[1;39m",
    "\x1b[1;39m",
    "\x1b[34;1m",
];
const RESET_COLOR: &str = "\x1b[0m";
/// Thread-local storage for custom colors
thread_local! {
    static COLORS : std::cell::RefCell < [String; 8] > =
    std::cell::RefCell::new([String::new(), String::new(), String::new(), String::new(),
    String::new(), String::new(), String::new(), String::new(),]); static
    USE_CUSTOM_COLORS : std::cell::Cell < bool > = std::cell::Cell::new(false);
}
fn get_color(kind_idx: usize) -> String {
    USE_CUSTOM_COLORS
        .with(|use_custom| {
            if use_custom.get() {
                COLORS
                    .with(|colors| {
                        let colors = colors.borrow();
                        if colors[kind_idx].is_empty() {
                            DEF_COLORS[kind_idx].to_string()
                        } else {
                            colors[kind_idx].clone()
                        }
                    })
            } else {
                DEF_COLORS[kind_idx].to_string()
            }
        })
}
/// Put a buffer to output
pub fn put_buf<W: Write>(
    s: &str,
    len: i32,
    fout: &mut Option<&mut W>,
    strout: &mut Option<&mut Jv>,
    _is_tty: i32,
) {
    let slice = if len as usize <= s.len() { &s[..len as usize] } else { s };
    if let Some(out) = strout {
        **out = jv_string_append_buf((*out).clone(), slice.as_bytes(), len);
    } else if let Some(f) = fout {
        let _ = f.write_all(slice.as_bytes());
    }
}
/// Put indentation to output
pub fn put_indent<W: Write>(
    n: i32,
    flags: i32,
    fout: &mut Option<&mut W>,
    strout: &mut Option<&mut Jv>,
    t: i32,
) {
    if flags & (JV_PRINT_TAB as i32) != 0 {
        for _ in 0..n {
            put_char('\t', fout, strout, t);
        }
    } else {
        let spaces = n
            * ((flags
                & ((JV_PRINT_SPACE0 as i32) | (JV_PRINT_SPACE1 as i32)
                    | (JV_PRINT_SPACE2 as i32))) >> 8);
        for _ in 0..spaces {
            put_char(' ', fout, strout, t);
        }
    }
}
/// Put reference count to output
pub fn put_refcnt<W: Write>(
    c: &mut DtoaContext,
    refcnt: i32,
    f: &mut Option<&mut W>,
    s: &mut Option<&mut Jv>,
    t: i32,
) {
    let mut buf = String::new();
    put_char(' ', f, s, t);
    put_char('(', f, s, t);
    let formatted = jvp_dtoa_fmt(c, refcnt as f64);
    put_str(&formatted, f, s, t);
    put_char(')', f, s, t);
}
/// Dump a jv string with proper escaping
pub fn jvp_dump_string<W: Write>(
    str_val: Jv,
    ascii_only: i32,
    f: &mut Option<&mut W>,
    s: &mut Option<&mut Jv>,
    t: i32,
) {
    assert!(jv_get_kind(& str_val) == JvKind::String);
    let str_bytes = jv_string_value(&str_val);
    let _len = jv_string_length_bytes(&str_val);
    put_char('"', f, s, t);
    let mut remaining = str_bytes.as_bytes();
    while !remaining.is_empty() {
        let cstart_len = remaining.len();
        let mut codepoint: i32 = 0;
        let next_slice = match jvp_utf8_next(remaining, &mut codepoint) {
            Some(rest) => rest,
            None => break,
        };
        let consumed = cstart_len - next_slice.len();
        let cstart = str_bytes.len() - cstart_len;
        let c = codepoint;
        assert!(c != -1);
        let mut unicode_escape = false;
        if c >= 0x20 && c <= 0x7E {
            if c == '"' as i32 || c == '\\' as i32 {
                put_char('\\', f, s, t);
            }
            put_char(char::from_u32(c as u32).unwrap_or('?'), f, s, t);
        } else if c < 0x20 || c == 0x7F {
            match c as u8 as char {
                '\x08' => {
                    put_char('\\', f, s, t);
                    put_char('b', f, s, t);
                }
                '\t' => {
                    put_char('\\', f, s, t);
                    put_char('t', f, s, t);
                }
                '\r' => {
                    put_char('\\', f, s, t);
                    put_char('r', f, s, t);
                }
                '\n' => {
                    put_char('\\', f, s, t);
                    put_char('n', f, s, t);
                }
                '\x0c' => {
                    put_char('\\', f, s, t);
                    put_char('f', f, s, t);
                }
                _ => {
                    unicode_escape = true;
                }
            }
        } else {
            if ascii_only != 0 {
                unicode_escape = true;
            } else {
                let slice = &str_bytes.as_bytes()[cstart..cstart + consumed];
                if let Ok(utf8_str) = std::str::from_utf8(slice) {
                    put_str(utf8_str, f, s, t);
                }
            }
        }
        if unicode_escape {
            let mut buf = String::with_capacity(32);
            if c <= 0xffff {
                write!(buf, "\\u{:04x}", c).unwrap();
            } else {
                let adjusted = c - 0x10000;
                let high = 0xD800 | ((adjusted & 0xffc00) >> 10);
                let low = 0xDC00 | (adjusted & 0x003ff);
                write!(buf, "\\u{:04x}\\u{:04x}", high, low).unwrap();
            }
            put_str(&buf, f, s, t);
        }
        remaining = next_slice;
    }
    put_char('"', f, s, t);
}
/// Check if a number is NaN
fn jvp_number_is_nan(x: &Jv) -> bool {
    if jv_get_kind(x) == JvKind::Number {
        let val = jv_number_value(x);
        val.is_nan()
    } else {
        false
    }
}
/// Main term dumping function
pub fn jv_dump_term<W: Write>(
    c: &mut DtoaContext,
    x: Jv,
    flags: i32,
    indent: i32,
    f: &mut Option<&mut W>,
    s: &mut Option<&mut Jv>,
) {
    let mut buf = String::new();
    let refcnt = if flags & (JV_PRINT_REFCOUNT as i32) != 0 {
        jv_get_refcnt(&x) - 1
    } else {
        -1
    };
    let kind = jv_get_kind(&x);
    let color = if flags & (JV_PRINT_COLOR as i32) != 0 && kind != JvKind::Invalid {
        Some(get_color(kind as usize - 1))
    } else {
        None
    };
    if let Some(ref col) = color {
        put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
    }
    if indent > MAX_PRINT_DEPTH {
        put_str("<skipped: too deep>", f, s, flags & (JV_PRINT_ISATTY as i32));
    } else {
        match kind {
            JvKind::Invalid => {
                if flags & (JV_PRINT_INVALID as i32) != 0 {
                    let msg = jv_invalid_get_msg(jv_copy(&x));
                    if jv_get_kind(&msg) == JvKind::String {
                        put_str("<invalid:", f, s, flags & (JV_PRINT_ISATTY as i32));
                        jvp_dump_string(
                            msg,
                            flags | (JV_PRINT_ASCII as i32),
                            f,
                            s,
                            flags & (JV_PRINT_ISATTY as i32),
                        );
                        put_char('>', f, s, flags & (JV_PRINT_ISATTY as i32));
                    } else {
                        put_str("<invalid>", f, s, flags & (JV_PRINT_ISATTY as i32));
                    }
                } else {
                    panic!("Invalid value");
                }
            }
            JvKind::Null => {
                put_str("null", f, s, flags & (JV_PRINT_ISATTY as i32));
            }
            JvKind::False => {
                put_str("false", f, s, flags & (JV_PRINT_ISATTY as i32));
            }
            JvKind::True => {
                put_str("true", f, s, flags & (JV_PRINT_ISATTY as i32));
            }
            JvKind::Number => {
                if jvp_number_is_nan(&x) {
                    jv_dump_term(c, jv_null(), flags, indent, f, s);
                } else {
                    if let Some(literal) = jv_number_get_literal(&x) {
                        put_str(&literal, f, s, flags & (JV_PRINT_ISATTY as i32));
                    } else {
                        let d = jv_number_value(&x);
                        if d.is_nan() {
                            put_str("null", f, s, flags & (JV_PRINT_ISATTY as i32));
                        } else {
                            let d = d.clamp(-f64::MAX, f64::MAX);
                            let formatted = jvp_dtoa_fmt(c, d);
                            put_str(&formatted, f, s, flags & (JV_PRINT_ISATTY as i32));
                        }
                    }
                }
            }
            JvKind::String => {
                jvp_dump_string(
                    x.clone(),
                    flags & (JV_PRINT_ASCII as i32),
                    f,
                    s,
                    flags & (JV_PRINT_ISATTY as i32),
                );
                if flags & (JV_PRINT_REFCOUNT as i32) != 0 {
                    put_refcnt(c, refcnt, f, s, flags & (JV_PRINT_ISATTY as i32));
                }
            }
            JvKind::Array => {
                let len = jv_array_length(&x);
                if len == 0 {
                    put_str("[]", f, s, flags & (JV_PRINT_ISATTY as i32));
                } else {
                    put_char('[', f, s, flags & (JV_PRINT_ISATTY as i32));
                    for i in 0..len {
                        if i != 0 {
                            if let Some(ref col) = color {
                                put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            put_char(',', f, s, flags & (JV_PRINT_ISATTY as i32));
                        }
                        if color.is_some() {
                            put_str(RESET_COLOR, f, s, flags & (JV_PRINT_ISATTY as i32));
                        }
                        if flags & (JV_PRINT_PRETTY as i32) != 0 {
                            put_char('\n', f, s, flags & (JV_PRINT_ISATTY as i32));
                            put_indent(
                                indent + 1,
                                flags,
                                f,
                                s,
                                flags & (JV_PRINT_ISATTY as i32),
                            );
                        }
                        let elem = jv_array_get(jv_copy(&x), i);
                        jv_dump_term(c, elem, flags, indent + 1, f, s);
                    }
                    if flags & (JV_PRINT_PRETTY as i32) != 0 {
                        put_char('\n', f, s, flags & (JV_PRINT_ISATTY as i32));
                        put_indent(
                            indent,
                            flags,
                            f,
                            s,
                            flags & (JV_PRINT_ISATTY as i32),
                        );
                    }
                    if let Some(ref col) = color {
                        put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                    }
                    put_char(']', f, s, flags & (JV_PRINT_ISATTY as i32));
                    if flags & (JV_PRINT_REFCOUNT as i32) != 0 {
                        put_refcnt(c, refcnt, f, s, flags & (JV_PRINT_ISATTY as i32));
                    }
                }
            }
            JvKind::Object => {
                let len = jv_object_length(&x);
                if len == 0 {
                    put_str("{}", f, s, flags & (JV_PRINT_ISATTY as i32));
                } else {
                    put_char('{', f, s, flags & (JV_PRINT_ISATTY as i32));
                    let mut first = true;
                    if flags & (JV_PRINT_SORTED as i32) != 0 {
                        let keyset = jv_keys(jv_copy(&x));
                        let key_len = jv_array_length(&keyset);
                        for i in 0..key_len {
                            let key = jv_array_get(jv_copy(&keyset), i);
                            let value = jv_object_get(&x, jv_copy(&key));
                            if !first {
                                if let Some(ref col) = color {
                                    put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                                }
                                put_char(',', f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            if color.is_some() {
                                put_str(
                                    RESET_COLOR,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            if flags & (JV_PRINT_PRETTY as i32) != 0 {
                                put_char('\n', f, s, flags & (JV_PRINT_ISATTY as i32));
                                put_indent(
                                    indent + 1,
                                    flags,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            first = false;
                            if color.is_some() {
                                put_str(
                                    &get_color(7),
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            jvp_dump_string(
                                key.clone(),
                                flags & (JV_PRINT_ASCII as i32),
                                f,
                                s,
                                flags & (JV_PRINT_ISATTY as i32),
                            );
                            jv_free(key);
                            if color.is_some() {
                                put_str(
                                    RESET_COLOR,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            if let Some(ref col) = color {
                                put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            put_char(':', f, s, flags & (JV_PRINT_ISATTY as i32));
                            if color.is_some() {
                                put_str(
                                    RESET_COLOR,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            if flags & (JV_PRINT_PRETTY as i32) != 0 {
                                put_char(' ', f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            jv_dump_term(c, value, flags, indent + 1, f, s);
                        }
                        jv_free(keyset);
                    } else {
                        let mut iter = jv_object_iter(&x);
                        while jv_object_iter_valid(&x, iter) {
                            let key = jv_object_iter_key(&x, iter);
                            let value = jv_object_iter_value(&x, iter);
                            if !first {
                                if let Some(ref col) = color {
                                    put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                                }
                                put_char(',', f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            if color.is_some() {
                                put_str(
                                    RESET_COLOR,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            if flags & (JV_PRINT_PRETTY as i32) != 0 {
                                put_char('\n', f, s, flags & (JV_PRINT_ISATTY as i32));
                                put_indent(
                                    indent + 1,
                                    flags,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            first = false;
                            if color.is_some() {
                                put_str(
                                    &get_color(7),
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            jvp_dump_string(
                                key.clone(),
                                flags & (JV_PRINT_ASCII as i32),
                                f,
                                s,
                                flags & (JV_PRINT_ISATTY as i32),
                            );
                            jv_free(key);
                            if color.is_some() {
                                put_str(
                                    RESET_COLOR,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            if let Some(ref col) = color {
                                put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            put_char(':', f, s, flags & (JV_PRINT_ISATTY as i32));
                            if color.is_some() {
                                put_str(
                                    RESET_COLOR,
                                    f,
                                    s,
                                    flags & (JV_PRINT_ISATTY as i32),
                                );
                            }
                            if flags & (JV_PRINT_PRETTY as i32) != 0 {
                                put_char(' ', f, s, flags & (JV_PRINT_ISATTY as i32));
                            }
                            jv_dump_term(c, value, flags, indent + 1, f, s);
                            iter = jv_object_iter_next(&x, iter);
                        }
                    }
                    if flags & (JV_PRINT_PRETTY as i32) != 0 {
                        put_char('\n', f, s, flags & (JV_PRINT_ISATTY as i32));
                        put_indent(
                            indent,
                            flags,
                            f,
                            s,
                            flags & (JV_PRINT_ISATTY as i32),
                        );
                    }
                    if let Some(ref col) = color {
                        put_str(col, f, s, flags & (JV_PRINT_ISATTY as i32));
                    }
                    put_char('}', f, s, flags & (JV_PRINT_ISATTY as i32));
                    if flags & (JV_PRINT_REFCOUNT as i32) != 0 {
                        put_refcnt(c, refcnt, f, s, flags & (JV_PRINT_ISATTY as i32));
                    }
                }
            }
        }
    }
    jv_free(x);
    if color.is_some() {
        put_str(RESET_COLOR, f, s, flags & (JV_PRINT_ISATTY as i32));
    }
}
/// Dump a jv value to a file
pub fn jv_dumpf<W: Write>(x: Jv, f: &mut W, flags: i32) {
    let mut ctx = tsd_dtoa_context_get();
    let mut fout: Option<&mut W> = Some(f);
    let mut sout: Option<&mut Jv> = None;
    jv_dump_term(&mut ctx, x, flags, 0, &mut fout, &mut sout);
}
/// Convert a jv value to a string representation
pub fn jv_dump_string(x: Jv, flags: i32) -> Jv {
    let mut s = jv_string("");
    let mut ctx = tsd_dtoa_context_get();
    let mut fout: Option<&mut io::Stdout> = None;
    let mut sout: Option<&mut Jv> = Some(&mut s);
    jv_dump_term(&mut ctx, x, flags, 0, &mut fout, &mut sout);
    s
}
/// Dump a jv value to stderr for debugging
pub fn jv_show(x: Jv, flags: i32) {
    let mut actual_flags = flags;
    if flags == -1 {
        actual_flags = (JV_PRINT_PRETTY as i32) | (JV_PRINT_COLOR as i32)
            | (2 << 8 | (JV_PRINT_PRETTY as i32));
    }
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    jv_dumpf(jv_copy(&x), &mut handle, actual_flags | (JV_PRINT_INVALID as i32));
    let _ = handle.flush();
}
/// Dump a jv value to a truncated string buffer
pub fn jv_dump_string_trunc(x: Jv, bufsize: usize) -> String {
    let x = jv_dump_string(x, 0);
    let p = jv_string_value(&x);
    let len = p.len();
    let mut result = if len <= bufsize - 1 {
        p.to_string()
    } else {
        p[..bufsize - 1].to_string()
    };
    if len > bufsize - 1 && bufsize >= 4 {
        let result_len = result.len();
        if result_len >= 3 {
            unsafe {
                let bytes = result.as_bytes_mut();
                bytes[result_len - 1] = b'.';
                bytes[result_len - 2] = b'.';
                bytes[result_len - 3] = b'.';
            }
        }
    }
    jv_free(x);
    result
}
/// Set custom colors for syntax highlighting
/// Format: colon-separated ANSI codes (e.g., "1;30:0;39:0;39:0;39:0;32:1;39:1;39:34;1")
pub fn jq_set_colors(c: Option<&str>) -> i32 {
    let c = match c {
        Some(s) => s,
        None => return 1,
    };
    if c.is_empty() {
        USE_CUSTOM_COLORS.with(|use_custom| use_custom.set(false));
        return 1;
    }
    let parts: Vec<&str> = c.split(':').collect();
    COLORS
        .with(|colors| {
            let mut colors = colors.borrow_mut();
            for (i, part) in parts.iter().enumerate() {
                if i >= 8 {
                    break;
                }
                if !part.chars().all(|ch| ch.is_ascii_digit() || ch == ';') {
                    return 0;
                }
                if part.len() > 60 {
                    return 0;
                }
                colors[i] = format!("\x1b[{}m", part);
            }
            1
        });
    USE_CUSTOM_COLORS.with(|use_custom| use_custom.set(true));
    1
}
/// Check if a jv value is valid
pub fn jv_is_valid(x: &Jv) -> bool {
    jv_get_kind(x) != JvKind::Invalid
}
