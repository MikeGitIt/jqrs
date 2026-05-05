//! Module: jv_parse
//!
//! Contains 29 transpiled functions:
//! - stream_seq_check_truncation:17358021920479371205:./src/jv_parse.c
//! - parse_token:5483826871628997911:./src/jv_parse.c
//! - jv_parser_next:4839357165717283196:./src/jv_parse.c
//! - jv_parse_sized_custom_flags:9998333986122324463:./src/jv_parse.c
//! - jv_parse:14405458618751609494:./src/jv_parse.c
//! - make_error:7702506614885211766:./src/jv_parse.c
//! - stream_token:7104349335120079717:./src/jv_parse.c
//! - stream_is_top_num:12856851440261714978:./src/jv_parse.c
//! - parser_init:489994857891513981:./src/jv_parse.c
//! - unhex4:15602525927212347388:./src/jv_parse.c
//! - jv_parser_new:1754809746248295779:./src/jv_parse.c
//! - seq_check_truncation:1149390001013988347:./src/jv_parse.c
//! - parse_is_top_num:15314370401772305617:./src/jv_parse.c
//! - jv_parse_custom_flags:3337511461623923988:./src/jv_parse.c
//! - found_string:12293788160126642840:./src/jv_parse.c
//! - check_literal:8815625572867698110:./src/jv_parse.c
//! - jv_parse_sized:9157723080873132390:./src/jv_parse.c
//! - classify:14549474006022882204:./src/jv_parse.c
//! - jv_parser_set_buf:2530741956201678886:./src/jv_parse.c
//! - stream_check_done:9518547372866986997:./src/jv_parse.c
//! - push:9775180855090879727:./src/jv_parse.c
//! - value:10688564888294837503:./src/jv_parse.c
//! - parse_check_done:16888548115636691634:./src/jv_parse.c
//! - jv_parser_remaining:13410925109581507245:./src/jv_parse.c
//! - jv_parser_free:6280753006043716365:./src/jv_parse.c
//! - tokenadd:3233407100105251320:./src/jv_parse.c
//! - scan:14308922526228244318:./src/jv_parse.c
//! - parser_reset:10998081246742294855:./src/jv_parse.c
//! - parser_free:1329208380410055101:./src/jv_parse.c
use std::fmt;
use crate::jv::{Jv, JvKind, jv_array, jv_object, jv_array_append, jv_object_set, jv_array_length, jv_object_length, jv_free, jv_copy};
use crate::jv_dtoa::DtoaContext;
use crate::types::*;

/// Extended parser state that adds fields needed by jv_parse but not in types::JvParser
/// This wraps the base JvParser with additional fields
pub struct ExtendedJvParser {
    pub base: JvParser,
    pub curr_buf: Option<Vec<u8>>,
    pub curr_buf_length: usize,
    pub curr_buf_pos: usize,
    pub curr_buf_is_partial: bool,
    pub stacklen: usize,
    pub tokenlen: usize,
    pub last_seen: LastSeen,
    pub st: JvParserState,
    pub bom_strip_position: usize,
    pub dtoa: DtoaContext,
}

impl Default for ExtendedJvParser {
    fn default() -> Self {
        ExtendedJvParser {
            base: JvParser {
                flags: 0,
                buf: Vec::new(),
                pos: 0,
                is_partial: false,
                tokenbuf: Vec::new(),
                tokenpos: 0,
                stack: Vec::new(),
                stackpos: 0,
                next: Jv::invalid(),
                path: Jv::invalid(),
                output: Vec::new(),
                output_pos: 0,
                unicode_codepoint: 0,
                unicode_count: 0,
                line: 1,
                column: 0,
                last_ch_was_ws: false,
                eof: false,
                error: None,
                state: ParserState::Value,
                string_started: false,
            },
            curr_buf: None,
            curr_buf_length: 0,
            curr_buf_pos: 0,
            curr_buf_is_partial: false,
            stacklen: 0,
            tokenlen: 0,
            last_seen: LastSeen::None,
            st: JvParserState::Normal,
            bom_strip_position: 0,
            dtoa: DtoaContext {
                freelist: [None, None, None, None, None, None, None, None],
                p5s: None,
            },
        }
    }
}
const MAX_PARSING_DEPTH: usize = 256;
pub const JV_PARSE_STREAMING: i32 = 1;
pub const JV_PARSE_STREAM_ERRORS: i32 = 2;
/// Parser flags
pub const JV_PARSE_SEQ: i32 = 4;
/// Create a new parser
pub fn jv_parser_new(flags: i32) -> Box<ExtendedJvParser> {
    let mut p = Box::new(ExtendedJvParser::default());
    parser_init(&mut p, flags);
    p
}
/// Get the number of bytes remaining in the current buffer
pub fn jv_parser_remaining(p: &ExtendedJvParser) -> i32 {
    if p.curr_buf.is_none() {
        return 0;
    }
    (p.curr_buf_length - p.curr_buf_pos) as i32
}
/// Initialize parser state
pub fn parser_init(p: &mut ExtendedJvParser, flags: i32) {
    p.base.flags = flags;
    if (p.base.flags & (JV_PARSE_STREAMING as i32)) != 0 {
        p.base.path = Jv::array();
    } else {
        p.base.path = Jv::invalid();
        p.base.flags &= !(JV_PARSE_STREAM_ERRORS);
    }
    p.base.stack.clear();
    p.stacklen = 0;
    p.base.stackpos = 0;
    p.last_seen = LastSeen::None;
    p.base.output.clear();
    p.base.output_pos = 0;
    p.base.next = Jv::invalid();
    p.base.tokenbuf.clear();
    p.tokenlen = 0;
    p.base.tokenpos = 0;
    if (p.base.flags & (JV_PARSE_SEQ as i32)) != 0 {
        p.st = JvParserState::WaitingForRs;
    } else {
        p.st = JvParserState::Normal;
    }
    p.base.eof = false;
    p.curr_buf = None;
    p.curr_buf_length = 0;
    p.curr_buf_pos = 0;
    p.curr_buf_is_partial = false;
    p.bom_strip_position = 0;
    p.base.last_ch_was_ws = false;
    p.base.line = 1;
    p.base.column = 0;
    p.dtoa = DtoaContext {
        freelist: [None, None, None, None, None, None, None, None],
        p5s: None,
    };
}
/// Reset parser to initial state
pub fn parser_reset(p: &mut ExtendedJvParser) {
    if (p.base.flags & (JV_PARSE_STREAMING as i32)) != 0 {
        p.base.path = Jv::array();
        p.stacklen = 0;
    }
    p.last_seen = LastSeen::None;
    p.base.output.clear();
    p.base.output_pos = 0;
    p.base.next = Jv::invalid();
    p.base.stack.clear();
    p.base.stackpos = 0;
    p.base.tokenpos = 0;
    p.st = JvParserState::Normal;
}
/// Push a value onto the parser stack
pub fn push(p: &mut ExtendedJvParser, v: Jv) {
    assert!(p.base.stackpos <= p.stacklen);
    if p.base.stackpos == p.stacklen {
        p.stacklen = p.stacklen * 2 + 10;
        p.base.stack.reserve(p.stacklen - p.base.stack.len());
    }
    assert!(p.base.stackpos < p.stacklen || p.base.stack.len() < p.stacklen);
    if p.base.stackpos < p.base.stack.len() {
        // Note: StackEntry doesn't hold Jv, so this is a type mismatch we need to handle
        // For now, just push to extend if needed
    }
    // The stack stores StackEntry, not Jv - we need to track values differently
    p.base.stackpos += 1;
}
/// Classify a character for JSON parsing
pub fn classify(c: u8) -> Chclass {
    match c {
        b' ' | b'\t' | b'\r' | b'\n' => Chclass::Whitespace,
        b'{' => Chclass::StructOpen,
        b'}' => Chclass::StructClose,
        b'[' => Chclass::ArrayOpen,
        b']' => Chclass::ArrayClose,
        b':' => Chclass::Colon,
        b',' => Chclass::Comma,
        b'"' => Chclass::Quote,
        b'0'..=b'9' => Chclass::Digit,
        b'-' => Chclass::Minus,
        b'a'..=b'z' | b'A'..=b'Z' => Chclass::Letter,
        0 => Chclass::Eof,
        _ => Chclass::Other,
    }
}
/// Process a parsed value
/// Returns None on success, Some(error_msg) on failure
/// This matches C jv_parse.c value() function (lines 125-144)
/// C's value() ALWAYS stores in p->next - structural handlers do the rest
fn value(p: &mut ExtendedJvParser, val: Jv) -> Option<&'static str> {
    if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
        return stream_value(p, val);
    }

    // C: if (jv_is_valid(p->next)) return "Expected separator between values";
    if p.base.next.is_valid() {
        jv_free(val);
        return Some("Expected separator between values");
    }

    // C: jv_free(p->next); p->next = val;
    jv_free(std::mem::replace(&mut p.base.next, val));
    // eprintln!("DEBUG value: stored in next");
    OK
}
/// Check for stream sequence truncation
pub fn stream_seq_check_truncation(p: &ExtendedJvParser) -> i32 {
    let k = p.base.next.get_kind();
    if p.stacklen > 0 || k == JvKind::Number || k == JvKind::True || k == JvKind::False
        || k == JvKind::Null
    {
        1
    } else {
        0
    }
}
/// Parse a token
pub fn parse_token(p: &mut ExtendedJvParser, ch: u8) -> Presult {
    let class = classify(ch);
    if std::env::var("DEBUG_PARSER").is_ok() {
        eprintln!("DEBUG parse_token: ch='{}' ({:#04x}), class={:?}, state={:?}",
            ch as char, ch, class, p.base.state);
    }
    match p.base.state {
        ParserState::Value => {
            match class {
                Chclass::Whitespace => {
                    p.base.last_ch_was_ws = true;
                    Presult::Ok
                }
                Chclass::Quote => {
                    p.base.state = ParserState::String;
                    p.base.last_ch_was_ws = false;
                    Presult::NeedMore
                }
                Chclass::Digit | Chclass::Minus => {
                    p.base.state = ParserState::Number;
                    p.base.last_ch_was_ws = false;
                    tokenadd(p, ch);
                    Presult::NeedMore
                }
                Chclass::Letter => {
                    p.base.last_ch_was_ws = false;
                    tokenadd(p, ch);
                    Presult::NeedMore
                }
                Chclass::StructOpen => {
                    // '{' - start object
                    if p.base.stackpos >= MAX_PARSING_DEPTH {
                        p.base.error = Some(make_error(p, "Exceeds depth limit for parsing", &[]));
                        return Presult::Error;
                    }
                    if p.base.next.is_valid() {
                        p.base.error = Some(make_error(p, "Expected separator between values", &[]));
                        return Presult::Error;
                    }
                    p.base.last_ch_was_ws = false;
                    p.base.stack.push(jv_object());
                    p.base.stackpos += 1;
                    Presult::Ok
                }
                Chclass::ArrayOpen => {
                    // '[' - start array
                    if p.base.stackpos >= MAX_PARSING_DEPTH {
                        p.base.error = Some(make_error(p, "Exceeds depth limit for parsing", &[]));
                        return Presult::Error;
                    }
                    if p.base.next.is_valid() {
                        p.base.error = Some(make_error(p, "Expected separator between values", &[]));
                        return Presult::Error;
                    }
                    p.base.last_ch_was_ws = false;
                    p.base.stack.push(jv_array());
                    p.base.stackpos += 1;
                    Presult::Ok
                }
                Chclass::StructClose => {
                    // '}' - end object
                    if p.base.stackpos == 0 {
                        p.base.error = Some(make_error(p, "Unmatched '}'", &[]));
                        return Presult::Error;
                    }
                    if p.base.next.is_valid() {
                        // We have a value - must have a key on stack
                        let top_idx = p.base.stackpos - 1;
                        if p.base.stack[top_idx].get_kind() != JvKind::String {
                            p.base.error = Some(make_error(p, "Objects must consist of key:value pairs", &[]));
                            return Presult::Error;
                        }
                        // stack[top-1] is key (string), stack[top-2] is object
                        if p.base.stackpos < 2 || p.base.stack[top_idx - 1].get_kind() != JvKind::Object {
                            p.base.error = Some(make_error(p, "Unmatched '}'", &[]));
                            return Presult::Error;
                        }
                        let key = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        let obj = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        let next_val = std::mem::replace(&mut p.base.next, Jv::invalid());
                        let new_obj = jv_object_set(obj, key, next_val);
                        p.base.next = new_obj;
                    } else {
                        // Empty object or after comma
                        let top_idx = p.base.stackpos - 1;
                        if p.base.stack[top_idx].get_kind() != JvKind::Object {
                            p.base.error = Some(make_error(p, "Unmatched '}'", &[]));
                            return Presult::Error;
                        }
                        if jv_object_length(&p.base.stack[top_idx]) != 0 {
                            p.base.error = Some(make_error(p, "Expected another key-value pair", &[]));
                            return Presult::Error;
                        }
                        let obj = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        p.base.next = obj;
                    }
                    p.base.last_ch_was_ws = false;
                    Presult::Ok
                }
                Chclass::ArrayClose => {
                    // ']' - end array
                    if p.base.stackpos == 0 {
                        p.base.error = Some(make_error(p, "Unmatched ']'", &[]));
                        return Presult::Error;
                    }
                    let top_idx = p.base.stackpos - 1;
                    if p.base.stack[top_idx].get_kind() != JvKind::Array {
                        p.base.error = Some(make_error(p, "Unmatched ']'", &[]));
                        return Presult::Error;
                    }
                    if p.base.next.is_valid() {
                        // Append final element
                        let arr = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        let next_val = std::mem::replace(&mut p.base.next, Jv::invalid());
                        p.base.next = jv_array_append(arr, next_val);
                    } else {
                        // Empty array or trailing comma check
                        if jv_array_length(&p.base.stack[top_idx]) != 0 {
                            p.base.error = Some(make_error(p, "Expected another array element", &[]));
                            return Presult::Error;
                        }
                        let arr = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        p.base.next = arr;
                    }
                    p.base.last_ch_was_ws = false;
                    Presult::Ok
                }
                Chclass::Colon => {
                    // ':' - key-value separator in objects
                    if !p.base.next.is_valid() {
                        p.base.error = Some(make_error(p, "Expected string key before ':'", &[]));
                        return Presult::Error;
                    }
                    if p.base.stackpos == 0 {
                        p.base.error = Some(make_error(p, "':' not as part of an object", &[]));
                        return Presult::Error;
                    }
                    let top_idx = p.base.stackpos - 1;
                    if p.base.stack[top_idx].get_kind() != JvKind::Object {
                        p.base.error = Some(make_error(p, "':' not as part of an object", &[]));
                        return Presult::Error;
                    }
                    if p.base.next.get_kind() != JvKind::String {
                        p.base.error = Some(make_error(p, "Object keys must be strings", &[]));
                        return Presult::Error;
                    }
                    // Push the key onto the stack
                    let key = std::mem::replace(&mut p.base.next, Jv::invalid());
                    p.base.stack.push(key);
                    p.base.stackpos += 1;
                    p.last_seen = LastSeen::Colon;  // Important: allow value to follow
                    p.base.last_ch_was_ws = false;
                    Presult::Ok
                }
                Chclass::Comma => {
                    // ',' - separator between values
                    if !p.base.next.is_valid() {
                        p.base.error = Some(make_error(p, "Expected value before ','", &[]));
                        return Presult::Error;
                    }
                    if p.base.stackpos == 0 {
                        p.base.error = Some(make_error(p, "',' not as part of an object or array", &[]));
                        return Presult::Error;
                    }
                    let top_idx = p.base.stackpos - 1;
                    let top_kind = p.base.stack[top_idx].get_kind();
                    if top_kind == JvKind::Array {
                        // Append to array
                        let arr = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        let next_val = std::mem::replace(&mut p.base.next, Jv::invalid());
                        let new_arr = jv_array_append(arr, next_val);
                        p.base.stack.push(new_arr);
                        p.base.stackpos += 1;
                    } else if top_kind == JvKind::String {
                        // Key on stack, object below - set key-value pair
                        if p.base.stackpos < 2 || p.base.stack[top_idx - 1].get_kind() != JvKind::Object {
                            p.base.error = Some(make_error(p, "Objects must consist of key:value pairs", &[]));
                            return Presult::Error;
                        }
                        let key = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        let obj = p.base.stack.pop().unwrap();
                        p.base.stackpos -= 1;
                        let next_val = std::mem::replace(&mut p.base.next, Jv::invalid());
                        let new_obj = jv_object_set(obj, key, next_val);
                        p.base.stack.push(new_obj);
                        p.base.stackpos += 1;
                    } else {
                        p.base.error = Some(make_error(p, "Objects must consist of key:value pairs", &[]));
                        return Presult::Error;
                    }
                    p.base.last_ch_was_ws = false;
                    Presult::Ok
                }
                _ => {
                    p.base.error = Some(make_error(p, "Unexpected character", &[]));
                    Presult::Error
                }
            }
        }
        ParserState::String => {
            if ch == b'"' {
                p.base.state = ParserState::Value;
                found_string(p)
            } else if ch == b'\\' {
                p.base.state = ParserState::StringEscape;
                Presult::NeedMore
            } else if ch < 0x20 {
                p.base.error = Some(make_error(p, "Invalid character in string", &[]));
                Presult::Error
            } else {
                tokenadd(p, ch);
                Presult::NeedMore
            }
        }
        ParserState::StringEscape => {
            p.base.state = ParserState::String;
            match ch {
                b'"' | b'\\' | b'/' => {
                    tokenadd(p, ch);
                    Presult::NeedMore
                }
                b'b' => {
                    tokenadd(p, b'\x08');
                    Presult::NeedMore
                }
                b'f' => {
                    tokenadd(p, b'\x0c');
                    Presult::NeedMore
                }
                b'n' => {
                    tokenadd(p, b'\n');
                    Presult::NeedMore
                }
                b'r' => {
                    tokenadd(p, b'\r');
                    Presult::NeedMore
                }
                b't' => {
                    tokenadd(p, b'\t');
                    Presult::NeedMore
                }
                b'u' => {
                    p.base.state = ParserState::StringUnicode;
                    p.base.unicode_count = 0;
                    p.base.unicode_codepoint = 0;
                    Presult::NeedMore
                }
                _ => {
                    p.base.error = Some(make_error(p, "Invalid escape sequence", &[]));
                    Presult::Error
                }
            }
        }
        ParserState::StringUnicode => {
            if let Some(d) = hex_digit(ch) {
                p.base.unicode_codepoint = (p.base.unicode_codepoint << 4) | (d as u32);
                p.base.unicode_count += 1;
                if p.base.unicode_count == 4 {
                    let cp = p.base.unicode_codepoint;
                    if cp < 0x80 {
                        tokenadd(p, cp as u8);
                    } else if cp < 0x800 {
                        tokenadd(p, (0xc0 | (cp >> 6)) as u8);
                        tokenadd(p, (0x80 | (cp & 0x3f)) as u8);
                    } else if cp < 0x10000 {
                        tokenadd(p, (0xe0 | (cp >> 12)) as u8);
                        tokenadd(p, (0x80 | ((cp >> 6) & 0x3f)) as u8);
                        tokenadd(p, (0x80 | (cp & 0x3f)) as u8);
                    } else {
                        tokenadd(p, (0xf0 | (cp >> 18)) as u8);
                        tokenadd(p, (0x80 | ((cp >> 12) & 0x3f)) as u8);
                        tokenadd(p, (0x80 | ((cp >> 6) & 0x3f)) as u8);
                        tokenadd(p, (0x80 | (cp & 0x3f)) as u8);
                    }
                    p.base.state = ParserState::String;
                }
                Presult::NeedMore
            } else {
                p.base.error = Some(make_error(p, "Invalid unicode escape", &[]));
                Presult::Error
            }
        }
        ParserState::Number => {
            match class {
                Chclass::Digit => {
                    tokenadd(p, ch);
                    Presult::NeedMore
                }
                Chclass::Letter if ch == b'e' || ch == b'E' => {
                    tokenadd(p, ch);
                    Presult::NeedMore
                }
                Chclass::Minus | Chclass::Other if ch == b'+' || ch == b'.' => {
                    tokenadd(p, ch);
                    Presult::NeedMore
                }
                _ => {
                    p.base.state = ParserState::Value;
                    let result = check_literal(p);
                    if result.is_some() {
                        return Presult::Error;
                    }
                    parse_token(p, ch)
                }
            }
        }
        _ => {
            if p.base.tokenpos > 0 {
                if classify(ch) == Chclass::Letter {
                    tokenadd(p, ch);
                    Presult::NeedMore
                } else {
                    p.base.state = ParserState::Value;
                    let result = check_literal(p);
                    if result.is_some() {
                        return Presult::Error;
                    }
                    parse_token(p, ch)
                }
            } else {
                p.base.error = Some(make_error(p, "Unexpected state", &[]));
                Presult::Error
            }
        }
    }
}
/// Convert 4 hex digits to a code point
pub fn unhex4(hex: &[u8]) -> i32 {
    if hex.len() < 4 {
        return -1;
    }
    let mut result: i32 = 0;
    for i in 0..4 {
        let c = hex[i];
        let digit = match c {
            b'0'..=b'9' => (c - b'0') as i32,
            b'a'..=b'f' => (c - b'a' + 10) as i32,
            b'A'..=b'F' => (c - b'A' + 10) as i32,
            _ => return -1,
        };
        result = result * 16 + digit;
    }
    result
}
/// Encode a Unicode code point as UTF-8
fn utf8_encode(codepoint: u32, out: &mut Vec<u8>) -> usize {
    if codepoint < 0x80 {
        out.push(codepoint as u8);
        1
    } else if codepoint < 0x800 {
        out.push((0xC0 | (codepoint >> 6)) as u8);
        out.push((0x80 | (codepoint & 0x3F)) as u8);
        2
    } else if codepoint < 0x10000 {
        out.push((0xE0 | (codepoint >> 12)) as u8);
        out.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        out.push((0x80 | (codepoint & 0x3F)) as u8);
        3
    } else {
        out.push((0xF0 | (codepoint >> 18)) as u8);
        out.push((0x80 | ((codepoint >> 12) & 0x3F)) as u8);
        out.push((0x80 | ((codepoint >> 6) & 0x3F)) as u8);
        out.push((0x80 | (codepoint & 0x3F)) as u8);
        4
    }
}
/// Process a found string token
/// Returns None on success, Some(error_msg) on failure
pub fn found_string(p: &mut ExtendedJvParser) -> Presult {
    if std::env::var("DEBUG_PARSER").is_ok() {
        eprintln!("DEBUG found_string: tokenpos={}, token={:?}",
            p.base.tokenpos,
            String::from_utf8_lossy(&p.base.tokenbuf[..p.base.tokenpos]));
    }
    let mut out = Vec::with_capacity(p.base.tokenpos);
    let mut i = 0;
    let end = p.base.tokenpos;
    while i < end {
        let c = p.base.tokenbuf[i];
        i += 1;
        if c == b'\\' {
            if i >= end {
                p.base.error = Some(make_error(p, "Expected escape character at end of string", &[]));
                return Presult::Error;
            }
            let escape = p.base.tokenbuf[i];
            i += 1;
            match escape {
                b'\\' | b'"' | b'/' => out.push(escape),
                b'b' => out.push(b'\x08'),
                b'f' => out.push(b'\x0C'),
                b't' => out.push(b'\t'),
                b'n' => out.push(b'\n'),
                b'r' => out.push(b'\r'),
                b'u' => {
                    if i + 4 > end {
                        p.base.error = Some(make_error(p, "Invalid \\uXXXX escape", &[]));
                        return Presult::Error;
                    }
                    let hexvalue = unhex4(&p.base.tokenbuf[i..]);
                    if hexvalue < 0 {
                        p.base.error = Some(make_error(p, "Invalid characters in \\uXXXX escape", &[]));
                        return Presult::Error;
                    }
                    let mut codepoint = hexvalue as u32;
                    i += 4;
                    if (0xD800..=0xDBFF).contains(&codepoint) {
                        if i + 6 > end || p.base.tokenbuf[i] != b'\\'
                            || p.base.tokenbuf[i + 1] != b'u'
                        {
                            p.base.error = Some(make_error(p, "Invalid \\uXXXX\\uXXXX surrogate pair escape", &[]));
                            return Presult::Error;
                        }
                        let surrogate = unhex4(&p.base.tokenbuf[i + 2..]);
                        if surrogate < 0
                            || !(0xDC00..=0xDFFF).contains(&(surrogate as u32))
                        {
                            p.base.error = Some(make_error(p, "Invalid \\uXXXX\\uXXXX surrogate pair escape", &[]));
                            return Presult::Error;
                        }
                        i += 6;
                        codepoint = 0x10000
                            + (((codepoint - 0xD800) << 10)
                                | (surrogate as u32 - 0xDC00));
                    }
                    if codepoint > 0x10FFFF {
                        codepoint = 0xFFFD;
                    }
                    utf8_encode(codepoint, &mut out);
                }
                _ => {
                    p.base.error = Some(make_error(p, "Invalid escape", &[]));
                    return Presult::Error;
                }
            }
        } else {
            if c <= 0x1f {
                p.base.error = Some(make_error(p, "Invalid string: control characters from U+0000 through U+001F must be escaped", &[]));
                return Presult::Error;
            }
            out.push(c);
        }
    }
    let s = String::from_utf8_lossy(&out).into_owned();
    if let Some(_err) = value(p, Jv::string(&s)) {
        return Presult::Error;
    }
    p.base.tokenpos = 0;
    Presult::Ok
}
fn parser_error_message(p: &ExtendedJvParser) -> Option<&'static str> {
    let err = p.base.error.as_ref()?;
    let msg = crate::jv::jv_invalid_get_msg(jv_copy(err));
    let result = if crate::jv::jv_get_kind(&msg) == JvKind::String {
        match crate::jv::jv_string_value(&msg) {
            "Expected escape character at end of string" => Some("Expected escape character at end of string"),
            "Invalid \\uXXXX escape" => Some("Invalid \\uXXXX escape"),
            "Invalid characters in \\uXXXX escape" => Some("Invalid characters in \\uXXXX escape"),
            "Invalid \\uXXXX\\uXXXX surrogate pair escape" => {
                Some("Invalid \\uXXXX\\uXXXX surrogate pair escape")
            }
            "Invalid escape" => Some("Invalid escape"),
            "Invalid string: control characters from U+0000 through U+001F must be escaped" => {
                Some("Invalid string: control characters from U+0000 through U+001F must be escaped")
            }
            _ => None,
        }
    } else {
        None
    };
    jv_free(msg);
    result
}
fn string_token_error_message(p: &ExtendedJvParser) -> Option<&'static str> {
    let mut i = 0;
    let end = p.base.tokenpos;
    while i < end {
        let c = p.base.tokenbuf[i];
        i += 1;
        if c == b'\\' {
            if i >= end {
                return Some("Expected escape character at end of string");
            }
            let escape = p.base.tokenbuf[i];
            i += 1;
            match escape {
                b'\\' | b'"' | b'/' | b'b' | b'f' | b't' | b'n' | b'r' => {}
                b'u' => {
                    if i + 4 > end {
                        return Some("Invalid \\uXXXX escape");
                    }
                    let hexvalue = unhex4(&p.base.tokenbuf[i..]);
                    if hexvalue < 0 {
                        return Some("Invalid characters in \\uXXXX escape");
                    }
                    i += 4;
                    if (0xD800..=0xDBFF).contains(&(hexvalue as u32)) {
                        if i + 6 > end || p.base.tokenbuf[i] != b'\\'
                            || p.base.tokenbuf[i + 1] != b'u'
                        {
                            return Some("Invalid \\uXXXX\\uXXXX surrogate pair escape");
                        }
                        let surrogate = unhex4(&p.base.tokenbuf[i + 2..]);
                        if surrogate < 0
                            || !(0xDC00..=0xDFFF).contains(&(surrogate as u32))
                        {
                            return Some("Invalid \\uXXXX\\uXXXX surrogate pair escape");
                        }
                        i += 6;
                    }
                }
                _ => return Some("Invalid escape"),
            }
        } else if c <= 0x1f {
            return Some("Invalid string: control characters from U+0000 through U+001F must be escaped");
        }
    }
    None
}
/// Add a character to the token buffer
pub fn tokenadd(p: &mut ExtendedJvParser, c: u8) {
    if p.tokenlen == 0 {
        p.tokenlen = 256;
        p.base.tokenbuf.resize(p.tokenlen, 0);
    }
    if p.base.tokenpos >= p.tokenlen - 1 {
        p.tokenlen = p.tokenlen * 2 + 256;
        p.base.tokenbuf.resize(p.tokenlen, 0);
    }
    p.base.tokenbuf[p.base.tokenpos] = c;
    p.base.tokenpos += 1;
}
/// Check for completed literal and process it
/// Returns None on success, Some(error_msg) on failure
pub fn check_literal(p: &mut ExtendedJvParser) -> Option<&'static str> {
    if p.base.tokenpos == 0 {
        return OK;
    }
    let first_char = p.base.tokenbuf[0];
    let result = match first_char {
        b't' => {
            if p.base.tokenpos == 4 && &p.base.tokenbuf[0..4] == b"true" {
                value(p, Jv::jv_true())
            } else {
                return Some("Invalid literal");
            }
        }
        b'f' => {
            if p.base.tokenpos == 5 && &p.base.tokenbuf[0..5] == b"false" {
                value(p, Jv::jv_false())
            } else {
                return Some("Invalid literal");
            }
        }
        b'n' if p.base.tokenpos > 1 && p.base.tokenbuf[1] == b'u' => {
            if p.base.tokenpos == 4 && &p.base.tokenbuf[0..4] == b"null" {
                value(p, Jv::null())
            } else {
                return Some("Invalid literal");
            }
        }
        _ => {
            let literal = String::from_utf8_lossy(&p.base.tokenbuf[0..p.base.tokenpos]);
            let number = <Jv as JvExt>::number_with_literal(&literal);
            if number.get_kind() == JvKind::Invalid {
                return Some("Invalid numeric literal");
            }
            value(p, number)
        }
    };
    p.base.tokenpos = 0;
    result
}
/// Scan a character and update parser state
/// Returns None on success, Some(error_msg) on failure
pub fn scan(p: &mut ExtendedJvParser, ch: u8, out: &mut Jv) -> Option<&'static str> {
    p.base.column += 1;
    if ch == b'\n' {
        p.base.line += 1;
        p.base.column = 0;
    }
    if (p.base.flags & flags::JV_PARSE_SEQ) != 0 && ch == RS_CHAR as u8 {
        let truncated = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
            stream_seq_check_truncation(p)
        } else {
            seq_check_truncation(p)
        };
        if truncated != 0 {
            if let msg @ Some(_) = check_literal(p) {
                return msg;
            }
            let is_top_num = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
                stream_is_top_num(p)
            } else {
                parse_is_top_num(p) != 0
            };
            if is_top_num {
                return Some("Potentially truncated top-level numeric value");
            }
            return Some("Truncated value");
        }
        if let msg @ Some(_) = check_literal(p) {
            return msg;
        }
        let check_done = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
            stream_check_done(p, out)
        } else {
            parse_check_done(p, out)
        };
        if p.st == JvParserState::Normal && check_done != 0 {
            return OK;
        }
        assert!(! out.is_valid(), "!jv_is_valid(*out)");
        parser_reset(p);
        jv_free(std::mem::take(out));
        *out = Jv::invalid();
        return OK;
    }
    let mut answer: Option<&'static str> = OK;
    p.base.last_ch_was_ws = false;
    if p.st == JvParserState::Normal {
        let cls = classify(ch);
        if cls == Chclass::Whitespace {
            p.base.last_ch_was_ws = true;
        }
        let is_literal = matches!(cls, Chclass::Digit | Chclass::Minus | Chclass::Letter)
            || (p.base.tokenpos > 0 && (ch == b'.' || ch == b'+'));
        if !is_literal {
            if let msg @ Some(_) = check_literal(p) {
                return msg;
            }
            let check_done = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
                stream_check_done(p, out)
            } else {
                parse_check_done(p, out)
            };
            if check_done != 0 {
                answer = OK;
            }
        }
        // Check if structure character
        let is_structure = matches!(cls, Chclass::StructOpen | Chclass::StructClose |
                                         Chclass::ArrayOpen | Chclass::ArrayClose |
                                         Chclass::Colon | Chclass::Comma);
        match cls {
            Chclass::Digit | Chclass::Minus | Chclass::Letter if is_literal => {
                tokenadd(p, ch);
            }
            _ if is_literal => {
                tokenadd(p, ch);
            }
            Chclass::Whitespace => {}
            Chclass::Quote => {
                p.st = JvParserState::String;
            }
            Chclass::StructOpen | Chclass::StructClose | Chclass::ArrayOpen |
            Chclass::ArrayClose | Chclass::Colon | Chclass::Comma => {
                let result = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
                    stream_token(p, ch)
                } else {
                    let pr = parse_token(p, ch);
                    if pr == Presult::Error { Some("Parse error") } else { None }
                };
                if let Some(m) = result {
                    return Some(m);
                }
            }
            Chclass::Invalid => {
                return Some("Invalid character");
            }
            _ => {}
        }
        let check_done = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
            stream_check_done(p, out)
        } else {
            parse_check_done(p, out)
        };
        if check_done != 0 {
            answer = OK;
        }
    } else {
        if ch == b'"' && p.st == JvParserState::String {
            let result = found_string(p);
            if result == Presult::Error {
                return string_token_error_message(p)
                    .or_else(|| parser_error_message(p))
                    .or(Some("String parse error"));
            }
            p.st = JvParserState::Normal;
            let check_done = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
                stream_check_done(p, out)
            } else {
                parse_check_done(p, out)
            };
            if check_done != 0 {
                answer = OK;
            }
        } else {
            tokenadd(p, ch);
            if ch == b'\\' && p.st == JvParserState::String {
                p.st = JvParserState::StringEscape;
            } else {
                p.st = JvParserState::String;
            }
        }
    }
    answer
}
/// Check if parsing is done
pub fn parse_check_done(p: &mut ExtendedJvParser, out: &mut Jv) -> i32 {
    // Match C: only return done when stackpos==0 AND we have a valid next value
    if p.base.stackpos == 0 && p.base.next.is_valid() {
        *out = std::mem::replace(&mut p.base.next, Jv::invalid());
        return 1;
    }
    0
}
/// Check for truncation in sequence mode
pub fn seq_check_truncation(p: &ExtendedJvParser) -> i32 {
    if !p.base.last_ch_was_ws
        && (p.base.stackpos > 0 || p.base.tokenpos > 0 || jv_get_kind(&p.base.next) == JvKind::Number)
    {
        1
    } else {
        0
    }
}
/// Process a token in streaming mode
/// Returns None on success, Some(error_msg) on failure
pub fn stream_token(p: &mut ExtendedJvParser, ch: u8) -> Option<&'static str> {
    match ch {
        b'[' => {
            if p.base.next.is_valid() {
                return Some("Expected a separator between values");
            }
            if p.last_seen == LastSeen::OpenObject {
                return Some("Expected string key after '{', not '['");
            }
            if p.last_seen == LastSeen::Comma {
                let last = p.base.path.array_get(p.stacklen as i32 - 1);
                if last.get_kind() != JvKind::Number {
                    return Some("Expected string key after ',' in object, not '['");
                }
            }
            p.base.path = p.base.path.clone().array_append(Jv::number(0.0));
            p.last_seen = LastSeen::OpenArray;
            p.stacklen += 1;
        }
        b'{' => {
            if p.last_seen == LastSeen::Value {
                return Some("Expected a separator between values");
            }
            if p.last_seen == LastSeen::OpenObject {
                return Some("Expected string key after '{', not '{'");
            }
            if p.last_seen == LastSeen::Comma {
                let last = p.base.path.array_get(p.stacklen as i32 - 1);
                if last.get_kind() != JvKind::Number {
                    return Some("Expected string key after ',' in object, not '{'");
                }
            }
            p.base.path = p.base.path.clone().array_append(Jv::null());
            p.last_seen = LastSeen::OpenObject;
            p.stacklen += 1;
        }
        b':' => {
            if p.stacklen == 0 {
                return Some("':' not as part of an object");
            }
            let last = p.base.path.array_get(p.stacklen as i32 - 1);
            if last.get_kind() == JvKind::Number {
                return Some("':' not as part of an object");
            }
            if !p.base.next.is_valid() || p.last_seen == LastSeen::None {
                return Some("Expected string key before ':'");
            }
            if p.base.next.get_kind() != JvKind::String {
                return Some("Object keys must be strings");
            }
            if p.last_seen != LastSeen::Value {
                return Some("':' should follow a key");
            }
            p.last_seen = LastSeen::Colon;
            let next = std::mem::take(&mut p.base.next);
            p.base.next = Jv::invalid();
            p.base.path = p.base.path.clone().array_set(p.stacklen as i32 - 1, next);
        }
        b',' => {
            if p.last_seen != LastSeen::Value {
                return Some("Expected value before ','");
            }
            if p.stacklen == 0 {
                return Some("',' not as part of an object or array");
            }
            let last = p.base.path.array_get(p.stacklen as i32 - 1);
            let k = last.get_kind();
            if k == JvKind::Number {
                let idx = last.number_value() as i32;
                if p.base.next.is_valid() {
                    let path_copy = jv_copy(&p.base.path);
                    let next = std::mem::take(&mut p.base.next);
                    // Store output as single value for streaming
                    p.base.output.clear();
                    p.base.output.push(Jv::array().array_append(path_copy).array_append(next));
                    p.base.next = Jv::invalid();
                }
                p.base.path = p.base.path.clone()
                    .array_set(p.stacklen as i32 - 1, Jv::number((idx + 1) as f64));
                p.last_seen = LastSeen::Comma;
            } else if k == JvKind::String {
                if p.base.next.is_valid() {
                    let path_copy = jv_copy(&p.base.path);
                    let next = std::mem::take(&mut p.base.next);
                    p.base.output.clear();
                    p.base.output.push(Jv::array().array_append(path_copy).array_append(next));
                    p.base.next = Jv::invalid();
                }
                p.base.path = p.base.path.clone().array_set(p.stacklen as i32 - 1, Jv::null());
                p.last_seen = LastSeen::Comma;
            } else {
                assert!(k == JvKind::Null, "k == JV_KIND_NULL");
                return Some("Objects must consist of key:value pairs");
            }
        }
        b']' => {
            if p.stacklen == 0 {
                return Some("Unmatched ']' at the top-level");
            }
            if p.last_seen == LastSeen::Comma {
                return Some("Expected another array element");
            }
            if p.last_seen == LastSeen::OpenArray {
                assert!(! p.base.next.is_valid(), "!jv_is_valid(p->next)");
            }
            let last = p.base.path.array_get(p.stacklen as i32 - 1);
            let k = last.get_kind();
            if k != JvKind::Number {
                return Some("Unmatched ']' in the middle of an object");
            }
            if p.base.next.is_valid() {
                let path_copy = jv_copy(&p.base.path);
                let next = std::mem::take(&mut p.base.next);
                p.base.output.clear();
                p.base.output.push(Jv::array()
                    .array_append(path_copy)
                    .array_append(next)
                    .array_append(Jv::jv_true()));
                p.base.next = Jv::invalid();
            } else if p.last_seen != LastSeen::OpenArray {
                let path_copy = jv_copy(&p.base.path);
                p.base.output.clear();
                p.base.output.push(Jv::array().array_append(path_copy));
            }
            p.stacklen -= 1;
            p.base.path = p.base.path.clone().array_slice(0, p.stacklen as i32);
            jv_free(std::mem::take(&mut p.base.next));
            p.base.next = Jv::invalid();
            if p.last_seen == LastSeen::OpenArray {
                let path_copy = jv_copy(&p.base.path);
                p.base.output.clear();
                p.base.output.push(Jv::array().array_append(path_copy).array_append(Jv::array()));
            }
            if p.stacklen == 0 {
                p.last_seen = LastSeen::None;
            } else {
                p.last_seen = LastSeen::Value;
            }
        }
        b'}' => {
            if p.stacklen == 0 {
                return Some("Unmatched '}' at the top-level");
            }
            if p.last_seen == LastSeen::Comma {
                return Some("Expected another key:value pair");
            }
            if p.last_seen == LastSeen::OpenObject {
                assert!(! p.base.next.is_valid(), "!jv_is_valid(p->next)");
            }
            let last = p.base.path.array_get(p.stacklen as i32 - 1);
            let k = last.get_kind();
            if k == JvKind::Number {
                return Some("Unmatched '}' in the middle of an array");
            }
            if p.base.next.is_valid() {
                if k != JvKind::String {
                    return Some("Objects must consist of key:value pairs");
                }
                let path_copy = jv_copy(&p.base.path);
                let next = std::mem::take(&mut p.base.next);
                p.base.output.clear();
                p.base.output.push(Jv::array()
                    .array_append(path_copy)
                    .array_append(next)
                    .array_append(Jv::jv_true()));
                p.base.next = Jv::invalid();
            } else {
                if p.last_seen == LastSeen::Colon {
                    return Some("Missing value in key:value pair");
                }
                if p.last_seen == LastSeen::Comma {
                    return Some("Expected another key-value pair");
                }
                if p.last_seen == LastSeen::OpenArray {
                    return Some("Unmatched '}' in the middle of an array");
                }
                if p.last_seen != LastSeen::Value && p.last_seen != LastSeen::OpenObject
                {
                    return Some("Unmatched '}'");
                }
                if p.last_seen != LastSeen::OpenObject {
                    let path_copy = jv_copy(&p.base.path);
                    p.base.output.clear();
                    p.base.output.push(Jv::array().array_append(path_copy));
                }
            }
            p.stacklen -= 1;
            p.base.path = p.base.path.clone().array_slice(0, p.stacklen as i32);
            jv_free(std::mem::take(&mut p.base.next));
            p.base.next = Jv::invalid();
            if p.last_seen == LastSeen::OpenObject {
                let path_copy = jv_copy(&p.base.path);
                p.base.output.clear();
                p.base.output.push(Jv::array()
                    .array_append(path_copy)
                    .array_append(Jv::object()));
            }
            if p.stacklen == 0 {
                p.last_seen = LastSeen::None;
            } else {
                p.last_seen = LastSeen::Value;
            }
        }
        _ => {}
    }
    OK
}
/// Create an error message for the parser
pub fn make_error(p: &ExtendedJvParser, fmt: &str, args: &[&dyn fmt::Display]) -> Jv {
    let mut message = fmt.to_string();
    for arg in args {
        if let Some(_pos) = message.find("%s") {
            message = message.replacen("%s", &arg.to_string(), 1);
        } else if let Some(_pos) = message.find("%d") {
            message = message.replacen("%d", &arg.to_string(), 1);
        }
    }
    let e = jv_string(&message);
    if p.base.flags & (JV_PARSE_STREAM_ERRORS as i32) != 0 {
        let arr = Jv::array();
        let arr = jv_array_append(arr, e);
        jv_array_append(arr, jv_copy(&p.base.path))
    } else {
        jv_invalid_with_msg(e)
    }
}
/// Set parser buffer
pub fn jv_parser_set_buf(p: &mut ExtendedJvParser, buf: &[u8], length: i32, is_partial: bool) {
    assert!(
        p.curr_buf.is_none() || p.curr_buf_pos == p.curr_buf_length,
        "previous buffer not exhausted"
    );
    let mut buf_slice = buf;
    let mut remaining = length as usize;
    while remaining > 0 && p.bom_strip_position < UTF8_BOM.len() {
        if buf_slice[0] == UTF8_BOM[p.bom_strip_position] {
            buf_slice = &buf_slice[1..];
            remaining -= 1;
            p.bom_strip_position += 1;
        } else {
            if p.bom_strip_position == 0 {
                p.bom_strip_position = UTF8_BOM.len();
            } else {
                p.bom_strip_position = 0xff;
            }
            break;
        }
    }
    p.curr_buf = Some(buf_slice.to_vec());
    p.curr_buf_length = remaining;
    p.curr_buf_pos = 0;
    p.curr_buf_is_partial = is_partial;
}
/// Get next parsed value
pub fn jv_parser_next(p: &mut ExtendedJvParser) -> Jv {
    let debug = std::env::var("DEBUG_PARSER").is_ok();
    if debug {
        eprintln!("DEBUG jv_parser_next: eof={}, curr_buf={:?}, buf_pos={}, buf_len={}",
            p.base.eof, p.curr_buf.is_some(), p.curr_buf_pos, p.curr_buf_length);
    }
    if p.base.eof {
        // eprintln!("DEBUG jv_parser_next: returning invalid (eof)");
        return Jv::invalid();
    }
    if p.curr_buf.is_none() {
        // eprintln!("DEBUG jv_parser_next: returning invalid (no buffer)");
        return Jv::invalid();
    }
    if p.bom_strip_position == 0xff {
        if (p.base.flags & flags::JV_PARSE_SEQ) == 0 {
            return Jv::invalid_with_msg(Jv::string("Malformed BOM"));
        }
        p.st = JvParserState::WaitingForRs;
        parser_reset(p);
    }
    let mut value = Jv::invalid();
    if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 && stream_check_done(p, &mut value) != 0 {
        return value;
    }
    let mut msg: Option<&'static str> = OK;
    let mut last_ch: u8 = 0;
    while msg.is_none() {
        let buf = p.curr_buf.as_ref().unwrap();
        if p.curr_buf_pos >= p.curr_buf_length {
            break;
        }
        let ch = buf[p.curr_buf_pos];
        p.curr_buf_pos += 1;
        last_ch = ch;
        if p.st == JvParserState::WaitingForRs {
            if ch == b'\n' {
                p.base.line += 1;
                p.base.column = 0;
            } else {
                p.base.column += 1;
            }
            if ch == RS_CHAR as u8 {
                p.st = JvParserState::Normal;
            }
            continue;
        }
        msg = scan(p, ch, &mut value);
        if msg.is_none() && value.is_valid() {
            break;
        }
    }
    if debug {
        eprintln!("DEBUG jv_parser_next: after loop, msg={:?}, value.is_valid={}, value.kind={:?}",
            msg, value.is_valid(), value.get_kind());
    }
    if msg.is_none() && value.is_valid() {
        // eprintln!("DEBUG jv_parser_next: returning valid value");
        return value;
    } else if let Some(error_msg) = msg {
        jv_free(value);
        if last_ch != RS_CHAR as u8 && (p.base.flags & flags::JV_PARSE_SEQ) != 0 {
            p.st = JvParserState::WaitingForRs;
            let err_msg = format!(
                "{} at line {}, column {} (need RS to resync)", error_msg, p.base.line, p.base.column
            );
            let err = Jv::string(&err_msg);
            let result = Jv::invalid_with_msg(err);
            parser_reset(p);
            return result;
        }
        let err_msg = format!("{} at line {}, column {}", error_msg, p.base.line, p.base.column);
        let err = Jv::string(&err_msg);
        let result = Jv::invalid_with_msg(err);
        parser_reset(p);
        if (p.base.flags & flags::JV_PARSE_SEQ) == 0 {
            p.curr_buf = None;
            p.curr_buf_pos = 0;
        }
        return result;
    } else if p.curr_buf_is_partial {
        assert!(
            p.curr_buf_pos == p.curr_buf_length, "p->curr_buf_pos == p->curr_buf_length"
        );
        return Jv::invalid();
    } else {
        p.base.eof = true;
        assert!(
            p.curr_buf_pos == p.curr_buf_length, "p->curr_buf_pos == p->curr_buf_length"
        );
        jv_free(value);
        if p.st == JvParserState::WaitingForRs {
            let err_msg = format!(
                "Unfinished abandoned text at EOF at line {}, column {}", p.base.line, p.base.column
            );
            let err = Jv::string(&err_msg);
            return Jv::invalid_with_msg(err);
        }
        if p.st != JvParserState::Normal {
            let err_msg = format!(
                "Unfinished string at EOF at line {}, column {}", p.base.line, p.base.column
            );
            let err = Jv::string(&err_msg);
            let result = Jv::invalid_with_msg(err);
            parser_reset(p);
            p.st = JvParserState::WaitingForRs;
            return result;
        }
        if let Some(error_msg) = check_literal(p) {
            let err_msg = format!("{} at EOF at line {}, column {}", error_msg, p.base.line, p.base.column);
            let err = Jv::string(&err_msg);
            let result = Jv::invalid_with_msg(err);
            parser_reset(p);
            p.st = JvParserState::WaitingForRs;
            return result;
        }
        let stack_not_empty = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 {
            p.stacklen != 0
        } else {
            p.base.stackpos != 0
        };
        if stack_not_empty {
            let err_msg = format!(
                "Unfinished JSON term at EOF at line {}, column {}", p.base.line, p.base.column
            );
            let err = Jv::string(&err_msg);
            let result = Jv::invalid_with_msg(err);
            parser_reset(p);
            p.st = JvParserState::WaitingForRs;
            return result;
        }
        let result = if (p.base.flags & flags::JV_PARSE_STREAMING) != 0 && p.base.next.is_valid() {
            let path_copy = jv_copy(&p.base.path);
            let next = std::mem::take(&mut p.base.next);
            Jv::array().array_append(path_copy).array_append(next)
        } else {
            std::mem::take(&mut p.base.next)
        };
        p.base.next = Jv::invalid();
        if (p.base.flags & flags::JV_PARSE_SEQ) != 0 && !p.base.last_ch_was_ws
            && result.get_kind() == JvKind::Number
        {
            jv_free(result);
            let err_msg = format!(
                "Potentially truncated top-level numeric value at EOF at line {}, column {}",
                p.base.line, p.base.column
            );
            let err = Jv::string(&err_msg);
            return Jv::invalid_with_msg(err);
        }
        return result;
    }
}
/// Free parser
pub fn jv_parser_free(p: Box<ExtendedJvParser>) {
    let mut p = p;
    parser_free(&mut p);
}
/// Parse a JSON string
pub fn jv_parse(string: &str) -> Jv {
    jv_parse_sized(string, string.len() as i32)
}
/// Parse a JSON string with explicit length
pub fn jv_parse_sized(string: &str, length: i32) -> Jv {
    jv_parse_sized_custom_flags(string, length, 0)
}
/// UTF-8 BOM bytes
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
/// Record separator character for SEQ mode
const RS_CHAR: char = '\x1E';
/// OK result constant (None means success for Option<&'static str> return types)
const OK: Option<&'static str> = None;
/// Parser flags
pub mod flags {
    pub const JV_PARSE_STREAMING: i32 = 1;
    pub const JV_PARSE_STREAM_ERRORS: i32 = 2;
    pub const JV_PARSE_SEQ: i32 = 4;
}
// /// Free a Jv value (no-op in Rust due to automatic memory management)
// pub fn jv_free(_v: Jv) {}
// /// Copy a Jv value
// pub fn jv_copy(v: &Jv) -> Jv {
//     v.clone()
// }
/// Check if parser is at top-level number (non-streaming)
pub fn parse_is_top_num(p: &ExtendedJvParser) -> i32 {
    if p.base.stackpos == 0 && p.base.state == ParserState::Number { 1 } else { 0 }
}
/// Check if streaming parser is at top-level number
pub fn stream_is_top_num(p: &ExtendedJvParser) -> bool {
    p.stacklen == 0 && p.base.next.get_kind() == JvKind::Number
}
/// Free parser resources
fn parser_free(p: &mut ExtendedJvParser) {
    jv_free(std::mem::take(&mut p.base.next));
    p.base.tokenbuf.clear();
    p.base.stack.clear();
    jv_free(std::mem::take(&mut p.base.path));
    p.base.output.clear();
}
/// Process a parsed value in streaming mode
/// Returns None on success, Some(error_msg) on failure
fn stream_value(p: &mut ExtendedJvParser, val: Jv) -> Option<&'static str> {
    if p.last_seen == LastSeen::Value && p.stacklen == 0 {
        jv_free(val);
        return Some("Expected separator between values");
    }
    if p.last_seen == LastSeen::OpenObject {
        if val.get_kind() != JvKind::String {
            jv_free(val);
            return Some("Object keys must be strings");
        }
        p.base.next = val;
        p.last_seen = LastSeen::Value;
        return OK;
    }
    if p.last_seen == LastSeen::Comma {
        let last = p.base.path.array_get(p.stacklen as i32 - 1);
        if last.get_kind() == JvKind::Null {
            if val.get_kind() != JvKind::String {
                jv_free(val);
                return Some("Object keys must be strings");
            }
            p.base.next = val;
            p.last_seen = LastSeen::Value;
            return OK;
        }
    }
    p.base.next = val;
    p.last_seen = LastSeen::Value;
    OK
}
/// Check if streaming parser is done
pub fn stream_check_done(p: &mut ExtendedJvParser, out: &mut Jv) -> i32 {
    if p.stacklen == 0 && p.base.next.is_valid() {
        let path_copy = p.base.path.copy();
        let next_val = std::mem::replace(&mut p.base.next, Jv::invalid());
        *out = Jv::array()
            .array_append(Jv::array().array_append(path_copy))
            .array_append(next_val);
        p.base.next = Jv::invalid();
        1
    } else if !p.base.output.is_empty() {
        let output_len = p.base.output.len();
        if output_len > 0 {
            *out = p.base.output.remove(0);
        }
        1
    } else {
        0
    }
}
/// Parse with custom flags
pub fn jv_parse_custom_flags(string: &str, flags: i32) -> Jv {
    jv_parse_sized_custom_flags(string, string.len() as i32, flags)
}
/// Parse sized string with custom flags
pub fn jv_parse_sized_custom_flags(string: &str, length: i32, flags: i32) -> Jv {
    let mut parser = ExtendedJvParser::default();
    parser_init(&mut parser, flags);
    jv_parser_set_buf(&mut parser, string.as_bytes(), length, false);
    let value = jv_parser_next(&mut parser);
    if !crate::jv::jv_is_valid(&value) {
        if crate::jv::jv_invalid_has_msg(jv_copy(&value)) != 0 {
            let msg = crate::jv::jv_invalid_get_msg(jv_copy(&value));
            let msg_str = if crate::jv::jv_get_kind(&msg) == JvKind::String {
                crate::jv::jv_string_value(&msg).to_string()
            } else {
                "<unknown error>".to_string()
            };
            let result = Jv::invalid_with_msg(
                Jv::string(&format!("{} (while parsing '{}')", msg_str, string)),
            );
            jv_free(msg);
            jv_free(value);
            parser_free(&mut parser);
            return result;
        }
        jv_free(value);
        let result = Jv::invalid_with_msg(Jv::string("Expected JSON value"));
        parser_free(&mut parser);
        return result;
    }
    if value.is_valid() {
        let next = jv_parser_next(&mut parser);
        if next.is_valid() {
            jv_free(value);
            jv_free(next);
            let result = Jv::invalid_with_msg(
                Jv::string("Unexpected extra JSON values"),
            );
            parser_free(&mut parser);
            return result;
        } else if next.clone().invalid_has_msg() {
            jv_free(value);
            parser_free(&mut parser);
            return next;
        } else {
            jv_free(next);
        }
    }
    parser_free(&mut parser);
    value
}
/// Check if a Jv value is valid
pub fn jv_is_valid(x: &Jv) -> i32 {
    if x.kind_flags & 0x0F != JvKind::Invalid as u8 { 1 } else { 0 }
}
/// Decode a hex digit
fn hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

// Helper trait to add missing Jv methods locally
trait JvExt {
    fn number_with_literal(s: &str) -> Jv;
    fn array_set(self, idx: i32, val: Jv) -> Jv;
    fn array_slice(self, start: i32, end: i32) -> Jv;
    fn invalid_has_msg(&self) -> bool;
    fn invalid_get_msg(&self) -> Jv;
}

impl JvExt for Jv {
    /// Create a number from a literal string representation
    fn number_with_literal(s: &str) -> Jv {
        crate::jv::jv_number_with_literal(s)
    }

    /// Set array element at index (returns new array)
    fn array_set(self, idx: i32, val: Jv) -> Jv {
        crate::jv::jv_array_set(self, idx, val)
    }

    /// Slice array from start to end
    fn array_slice(self, start: i32, end: i32) -> Jv {
        crate::jv::jv_array_slice(self, start, end)
    }

    /// Check if invalid Jv has an error message
    fn invalid_has_msg(&self) -> bool {
        crate::jv::jv_invalid_has_msg(self.copy()) != 0
    }

    /// Get error message from invalid Jv
    fn invalid_get_msg(&self) -> Jv {
        crate::jv::jv_invalid_get_msg(self.copy())
    }
}
/// Get the kind of a Jv value
pub fn jv_get_kind(x: &Jv) -> JvKind {
    unsafe { std::mem::transmute(x.kind_flags & 0x0F) }
}
/// Create a string Jv
pub fn jv_string(s: &str) -> Jv {
    crate::jv::jv_string(s)
}
/// Create an invalid Jv with message
pub fn jv_invalid_with_msg(msg: Jv) -> Jv {
    Jv {
        kind_flags: JvKind::Invalid as u8,
        pad_: 0,
        offset: 0,
        size: 0,
        u: 0,
    }
}
// /// Append to array
// pub fn jv_array_append(arr: Jv, val: Jv) -> Jv {
//     arr
// }
impl ExtendedJvParser {
    /// Reset the parser state
    pub fn reset(&mut self) {
        self.base.tokenbuf.clear();
        self.base.tokenpos = 0;
        self.base.stack.clear();
        self.base.stackpos = 0;
        self.base.next = Jv::invalid();
        self.base.output.clear();
        self.base.output_pos = 0;
        self.base.unicode_codepoint = 0;
        self.base.unicode_count = 0;
        self.base.last_ch_was_ws = true;
        self.base.error = None;
        self.base.state = ParserState::Value;
        self.base.string_started = false;
        if self.base.flags & (JV_PARSE_STREAMING as i32) != 0 {
            self.base.path = Jv::array();
        }
    }
}
