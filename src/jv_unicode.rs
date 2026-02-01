//! Module: jv_unicode
//!
//! Contains 7 transpiled functions:
//! - jvp_codepoint_is_whitespace:4542390142155267332:./src/jv_unicode.c
//! - jvp_utf8_is_valid:8865250971996633971:./src/jv_unicode.c
//! - jvp_utf8_decode_length:9137976069133184200:./src/jv_unicode.c
//! - jvp_utf8_next:15540939544950912656:./src/jv_unicode.c
//! - jvp_utf8_encode:5983830246526016634:./src/jv_unicode.c
//! - jvp_utf8_backtrack:1152862640668075382:./src/jv_unicode.c
//! - jvp_utf8_encode_length:8736138309469215709:./src/jv_unicode.c
use crate::types::*;
/// UTF-8 coding length lookup table for the first byte
/// Values: 0 = invalid, 255 = continuation byte, 1-4 = sequence length
const UTF8_CODING_LENGTH: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = if i < 0x80 {
            1
        } else if i < 0xC0 {
            255
        } else if i < 0xE0 {
            2
        } else if i < 0xF0 {
            3
        } else if i < 0xF8 {
            4
        } else {
            0
        };
        i += 1;
    }
    table
};
/// Bits to extract from the first byte based on sequence length
const UTF8_CODING_BITS: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = if i < 0x80 {
            0x7F
        } else if i < 0xC0 {
            0x3F
        } else if i < 0xE0 {
            0x1F
        } else if i < 0xF0 {
            0x0F
        } else if i < 0xF8 {
            0x07
        } else {
            0
        };
        i += 1;
    }
    table
};
/// Minimum codepoint for each sequence length (to detect overlong encodings)
const UTF8_FIRST_CODEPOINT: [i32; 5] = [0, 0, 0x80, 0x800, 0x10000];
/// Decodes the next UTF-8 codepoint from a byte slice.
///
/// Returns the remaining slice and the decoded codepoint.
/// If the input is invalid UTF-8, returns codepoint = -1.
/// Returns None if input is empty.
///
/// # Arguments
/// * `input` - The input byte slice
/// * `codepoint_ret` - Mutable reference to store the decoded codepoint
///
/// # Returns
/// The remaining slice after consuming one codepoint, or None if input is empty
pub fn jvp_utf8_next<'a>(input: &'a [u8], codepoint_ret: &mut i32) -> Option<&'a [u8]> {
    if input.is_empty() {
        return None;
    }
    let first = input[0];
    let mut length = UTF8_CODING_LENGTH[first as usize] as usize;
    let mut codepoint: i32 = -1;
    if (first & 0x80) == 0 {
        codepoint = first as i32;
        length = 1;
    } else if length == 0 || length == 255 {
        length = 1;
    } else if length > input.len() {
        length = input.len();
    } else {
        codepoint = (first & UTF8_CODING_BITS[first as usize]) as i32;
        let mut valid = true;
        for i in 1..length {
            let ch = input[i];
            if UTF8_CODING_LENGTH[ch as usize] != 255 {
                codepoint = -1;
                length = i;
                valid = false;
                break;
            }
            codepoint = (codepoint << 6) | ((ch & 0x3F) as i32);
        }
        if valid {
            if codepoint < UTF8_FIRST_CODEPOINT[length] {
                codepoint = -1;
            }
            if (0xD800..=0xDFFF).contains(&codepoint) {
                codepoint = -1;
            }
            if codepoint > 0x10FFFF {
                codepoint = -1;
            }
        }
    }
    assert!(length > 0);
    *codepoint_ret = codepoint;
    Some(&input[length..])
}
/// Encodes a Unicode codepoint as UTF-8.
///
/// # Arguments
/// * `codepoint` - The Unicode codepoint to encode (must be 0..=0x10FFFF)
/// * `out` - Mutable slice to write the encoded bytes (must be at least 4 bytes)
///
/// # Returns
/// The number of bytes written
///
/// # Panics
/// Panics if codepoint is outside the valid Unicode range
pub fn jvp_utf8_encode(codepoint: i32, out: &mut [u8]) -> i32 {
    assert!(codepoint >= 0 && codepoint <= 0x10FFFF);
    let bytes_written = if codepoint <= 0x7F {
        out[0] = codepoint as u8;
        1
    } else if codepoint <= 0x7FF {
        out[0] = (0xC0 + ((codepoint & 0x7C0) >> 6)) as u8;
        out[1] = (0x80 + (codepoint & 0x03F)) as u8;
        2
    } else if codepoint <= 0xFFFF {
        out[0] = (0xE0 + ((codepoint & 0xF000) >> 12)) as u8;
        out[1] = (0x80 + ((codepoint & 0x0FC0) >> 6)) as u8;
        out[2] = (0x80 + (codepoint & 0x003F)) as u8;
        3
    } else {
        out[0] = (0xF0 + ((codepoint & 0x1C0000) >> 18)) as u8;
        out[1] = (0x80 + ((codepoint & 0x03F000) >> 12)) as u8;
        out[2] = (0x80 + ((codepoint & 0x000FC0) >> 6)) as u8;
        out[3] = (0x80 + (codepoint & 0x00003F)) as u8;
        4
    };
    assert!(bytes_written == jvp_utf8_encode_length(codepoint));
    bytes_written
}
/// Checks if a Unicode codepoint is whitespace.
///
/// This includes various Unicode whitespace characters, not just ASCII whitespace.
///
/// # Arguments
/// * `c` - The codepoint to check
///
/// # Returns
/// 1 if whitespace, 0 otherwise
pub fn jvp_codepoint_is_whitespace(c: i32) -> i32 {
    let is_ws = (c >= 0x0009 && c <= 0x000D) || c == 0x0020 || c == 0x0085 || c == 0x00A0
        || c == 0x1680 || (c >= 0x2000 && c <= 0x200A) || c == 0x2028 || c == 0x2029
        || c == 0x202F || c == 0x205F || c == 0x3000;
    if is_ws { 1 } else { 0 }
}
/// Returns the expected length of a UTF-8 sequence given its first byte.
///
/// # Arguments
/// * `startchar` - The first byte of a UTF-8 sequence
///
/// # Returns
/// The expected length in bytes (1-4)
pub fn jvp_utf8_decode_length(startchar: u8) -> i32 {
    if (startchar & 0x80) == 0 {
        1
    } else if (startchar & 0xE0) == 0xC0 {
        2
    } else if (startchar & 0xF0) == 0xE0 {
        3
    } else {
        4
    }
}
/// Backtracks through a UTF-8 string to find the start of a truncated sequence.
///
/// This is useful when you have a partial UTF-8 sequence at the end of a buffer
/// and need to find where the incomplete sequence starts.
///
/// # Arguments
/// * `start` - Position to start backtracking from (end of buffer)
/// * `min` - Minimum position (start of buffer)
/// * `missing_bytes` - If provided, receives the number of missing bytes
///
/// # Returns
/// The position of the start of the incomplete sequence, or None if invalid
pub fn jvp_utf8_backtrack<'a>(
    start: &'a [u8],
    min: &'a [u8],
    missing_bytes: Option<&mut i32>,
) -> Option<&'a [u8]> {
    assert!(min.len() >= start.len());
    let start_offset = min.len() - start.len();
    if start_offset == 0 {
        return Some(min);
    }
    let mut pos = start_offset;
    let mut seen = 1;
    loop {
        if pos == 0 {
            break;
        }
        let length = UTF8_CODING_LENGTH[min[pos - 1] as usize];
        if length != 255 {
            pos -= 1;
            break;
        }
        pos -= 1;
        seen += 1;
    }
    let length = UTF8_CODING_LENGTH[min[pos] as usize] as i32;
    if length == 0 || length == 255 || length - seen < 0 {
        return None;
    }
    if let Some(mb) = missing_bytes {
        *mb = length - seen;
    }
    Some(&min[pos..])
}
/// Returns the number of bytes needed to encode a codepoint in UTF-8.
///
/// # Arguments
/// * `codepoint` - The Unicode codepoint
///
/// # Returns
/// The number of bytes needed (1-4)
pub fn jvp_utf8_encode_length(codepoint: i32) -> i32 {
    if codepoint <= 0x7F {
        1
    } else if codepoint <= 0x7FF {
        2
    } else if codepoint <= 0xFFFF {
        3
    } else {
        4
    }
}
/// Checks if a byte slice contains valid UTF-8.
///
/// # Arguments
/// * `input` - The byte slice to validate
///
/// # Returns
/// 1 if valid UTF-8, 0 otherwise
pub fn jvp_utf8_is_valid(input: &[u8]) -> i32 {
    let mut remaining = input;
    let mut codepoint: i32 = 0;
    while let Some(next) = jvp_utf8_next(remaining, &mut codepoint) {
        if codepoint == -1 {
            return 0;
        }
        remaining = next;
    }
    1
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ascii() {
        let input = b"Hello";
        let mut remaining: &[u8] = input;
        let mut codepoint = 0;
        remaining = jvp_utf8_next(remaining, &mut codepoint).unwrap();
        assert_eq!(codepoint, 'H' as i32);
        remaining = jvp_utf8_next(remaining, &mut codepoint).unwrap();
        assert_eq!(codepoint, 'e' as i32);
    }
    #[test]
    fn test_multibyte() {
        let input = [0xC3, 0xA9];
        let mut codepoint = 0;
        let remaining = jvp_utf8_next(&input, &mut codepoint).unwrap();
        assert_eq!(codepoint, 0xE9);
        assert!(remaining.is_empty());
    }
    #[test]
    fn test_encode() {
        let mut buf = [0u8; 4];
        let len = jvp_utf8_encode('A' as i32, &mut buf);
        assert_eq!(len, 1);
        assert_eq!(buf[0], b'A');
        let len = jvp_utf8_encode(0xE9, &mut buf);
        assert_eq!(len, 2);
        assert_eq!(buf[0], 0xC3);
        assert_eq!(buf[1], 0xA9);
    }
    #[test]
    fn test_is_valid() {
        assert_eq!(jvp_utf8_is_valid(b"Hello"), 1);
        assert_eq!(jvp_utf8_is_valid(& [0xC3, 0xA9]), 1);
        assert_eq!(jvp_utf8_is_valid(& [0xFF]), 0);
        assert_eq!(jvp_utf8_is_valid(& [0xC3]), 0);
    }
    #[test]
    fn test_whitespace() {
        assert_eq!(jvp_codepoint_is_whitespace(' ' as i32), 1);
        assert_eq!(jvp_codepoint_is_whitespace('\t' as i32), 1);
        assert_eq!(jvp_codepoint_is_whitespace('\n' as i32), 1);
        assert_eq!(jvp_codepoint_is_whitespace('A' as i32), 0);
    }
    #[test]
    fn test_encode_length() {
        assert_eq!(jvp_utf8_encode_length(0x41), 1);
        assert_eq!(jvp_utf8_encode_length(0xE9), 2);
        assert_eq!(jvp_utf8_encode_length(0x4E2D), 3);
        assert_eq!(jvp_utf8_encode_length(0x1F600), 4);
    }
    #[test]
    fn test_decode_length() {
        assert_eq!(jvp_utf8_decode_length(0x41), 1);
        assert_eq!(jvp_utf8_decode_length(0xC3), 2);
        assert_eq!(jvp_utf8_decode_length(0xE4), 3);
        assert_eq!(jvp_utf8_decode_length(0xF0), 4);
    }
}
