//! Module: decimal32
//!
//! Contains 7 transpiled functions:
//! - decimal32FromString:2561933711392930750:./src/decNumber/decimal32.c
//! - decimal32ToString:3013074640608593462:./src/decNumber/decimal32.c
//! - decimal32Canonical:18109065161409832078:./src/decNumber/decimal32.c
//! - decimal32ToEngString:8785540906942069928:./src/decNumber/decimal32.c
//! - decimal32IsCanonical:5263192352092664582:./src/decNumber/decimal32.c
//! - decimal32FromNumber:16865432599525641341:./src/decNumber/decimal32.c
//! - decimal32ToNumber:3680305433728004776:./src/decNumber/decimal32.c
use std::ptr::NonNull;
use crate::deccontext::{decContextDefault, decContextSetStatus};
use crate::decnumber::{decNumberZero, decNumberPlus};
use crate::types::DecNumber;
// Note: decNumberToEngString is defined locally in this file
use crate::decimal64::{decDigitsFromDPD, decDigitsToDPD};
use crate::types::*;
/// Parse a string into a decimal32
pub fn decimal32FromString<'a>(
    result: &'a mut Decimal32,
    string: &'a str,
    set: &'a mut DecContext,
) -> &'a mut Decimal32 {
    let mut dn = DecNumber::default();
    let trimmed = string.trim();
    if trimmed.eq_ignore_ascii_case("inf") || trimmed.eq_ignore_ascii_case("infinity")
        || trimmed.eq_ignore_ascii_case("+inf")
        || trimmed.eq_ignore_ascii_case("+infinity")
    {
        dn.bits = DECINF;
    } else if trimmed.eq_ignore_ascii_case("-inf")
        || trimmed.eq_ignore_ascii_case("-infinity")
    {
        dn.bits = DECINF | DECNEG;
    } else if trimmed.eq_ignore_ascii_case("nan") || trimmed.eq_ignore_ascii_case("+nan")
    {
        dn.bits = DECNAN;
    } else if trimmed.eq_ignore_ascii_case("-nan") {
        dn.bits = DECNAN | DECNEG;
    } else if trimmed.eq_ignore_ascii_case("snan")
        || trimmed.eq_ignore_ascii_case("+snan")
    {
        dn.bits = DECSNAN;
    } else if trimmed.eq_ignore_ascii_case("-snan") {
        dn.bits = DECSNAN | DECNEG;
    } else {
        let mut neg = false;
        let mut s = trimmed;
        if s.starts_with('-') {
            neg = true;
            s = &s[1..];
        } else if s.starts_with('+') {
            s = &s[1..];
        }
        let (coeff_str, exp_str) = if let Some(pos) = s.find(|c| c == 'e' || c == 'E') {
            (&s[..pos], Some(&s[pos + 1..]))
        } else {
            (s, None)
        };
        let mut digits = Vec::new();
        let mut dot_pos = None;
        for (i, c) in coeff_str.chars().enumerate() {
            if c == '.' {
                dot_pos = Some(i);
            } else if c.is_ascii_digit() {
                digits.push(c as u8 - b'0');
            }
        }
        while digits.len() > 1 && digits[0] == 0 {
            digits.remove(0);
        }
        let mut exp = 0i32;
        if let Some(e_str) = exp_str {
            exp = e_str.parse().unwrap_or(0);
        }
        if let Some(dot) = dot_pos {
            let frac_digits = coeff_str.len() - dot - 1;
            exp -= frac_digits as i32;
        }
        dn.digits = digits.len() as i32;
        dn.exponent = exp;
        if neg {
            dn.bits = DECNEG;
        }
        let mut lsu = vec![0u16; (digits.len() + 2) / 3];
        let mut lsu_idx = 0;
        let mut mult = 1u16;
        for (i, &d) in digits.iter().rev().enumerate() {
            lsu[lsu_idx] += d as u16 * mult;
            mult *= 10;
            if mult == 1000 {
                mult = 1;
                lsu_idx += 1;
            }
        }
        dn.lsu = lsu;
    }
    decimal32FromNumber(result, &dn, set);
    result
}
/// Convert a decNumber to decimal32 format
///
/// # Arguments
/// * `d32` - The decimal32 to store the result in
/// * `dn` - The decNumber to convert
/// * `set` - The context for rounding and status
///
/// # Returns
/// A mutable reference to the result
pub fn decimal32FromNumber<'a>(
    d32: &'a mut Decimal32,
    dn: &'a DecNumber,
    set: &'a mut DecContext,
) -> &'a mut Decimal32 {
    const DECIMAL32_PMAX: i32 = 7;
    const DECIMAL32_EMAX: i32 = 96;
    const DECIMAL32_EMIN: i32 = -95;
    const DECIMAL32_BIAS: i32 = 101;
    const DECFLOAT_SIGN: u8 = 0x80;
    const DECFLOAT_INF: u32 = 0x78000000;
    const DECFLOAT_QNAN: u32 = 0x7C000000;
    const DECFLOAT_SNAN: u32 = 0x7E000000;
    const DECNEG: u8 = 0x80;
    const DECINF: u8 = 0x40;
    const DECNAN: u8 = 0x20;
    const DECSNAN: u8 = 0x10;
    const DECSPECIAL: u8 = DECINF | DECNAN | DECSNAN;
    let mut result: u32 = 0;
    if (dn.bits & DECNEG) != 0 {
        result |= (DECFLOAT_SIGN as u32) << 24;
    }
    if (dn.bits & DECSPECIAL) != 0 {
        if (dn.bits & DECINF) != 0 {
            result |= DECFLOAT_INF;
        } else if (dn.bits & DECSNAN) != 0 {
            result |= DECFLOAT_SNAN;
            if dn.digits > 0 && !dn.lsu.is_empty() {
                let payload = encode_dpd_payload(&dn.lsu, dn.digits.min(6) as usize);
                result |= payload;
            }
        } else {
            result |= DECFLOAT_QNAN;
            if dn.digits > 0 && !dn.lsu.is_empty() {
                let payload = encode_dpd_payload(&dn.lsu, dn.digits.min(6) as usize);
                result |= payload;
            }
        }
    } else {
        let mut exp = dn.exponent;
        let digits = dn.digits;
        if digits == 0 || (dn.lsu.is_empty()) || (digits == 1 && dn.lsu[0] == 0) {
            let biased_exp = (exp + DECIMAL32_BIAS) as u32;
            // Zero encoding: MSD = 0, use same encoding as non-zero case
            let comb = (biased_exp >> 6) << 3;  // (expMSBs << 3) | 0
            result |= comb << 26;
            result |= (biased_exp & 0x3F) << 20;
        } else {
            let mut coeff_digits = extract_coefficient_digits(dn);
            while coeff_digits.len() < DECIMAL32_PMAX as usize {
                coeff_digits.insert(0, 0);
            }
            if coeff_digits.len() > DECIMAL32_PMAX as usize {
                let excess = coeff_digits.len() - DECIMAL32_PMAX as usize;
                exp += excess as i32;
                coeff_digits = coeff_digits[excess..].to_vec();
            }
            if exp > DECIMAL32_EMAX {
                result |= DECFLOAT_INF;
                set.status |= 0x00000020;
            } else if exp < DECIMAL32_EMIN - (DECIMAL32_PMAX - 1) {
                let biased_exp = 0u32;
                result |= biased_exp << 23;
                set.status |= 0x00000010;
            } else {
                let biased_exp = (exp + DECIMAL32_BIAS) as u32;
                let msd = coeff_digits[0] as u32;
                if msd < 8 {
                    let comb = ((biased_exp >> 6) << 3) | msd;
                    result |= comb << 26;
                    result |= (biased_exp & 0x3F) << 20;
                } else {
                    let comb = 0x18 | ((biased_exp >> 6) << 1) | (msd & 1);
                    result |= comb << 26;
                    result |= (biased_exp & 0x3F) << 20;
                }
                if coeff_digits.len() >= 7 {
                    let declet1 = encode_declet(
                        coeff_digits[1],
                        coeff_digits[2],
                        coeff_digits[3],
                    );
                    let declet2 = encode_declet(
                        coeff_digits[4],
                        coeff_digits[5],
                        coeff_digits[6],
                    );
                    result |= (declet1 as u32) << 10;
                    result |= declet2 as u32;
                }
            }
        }
    }
    d32.bytes[0] = ((result >> 24) & 0xFF) as u8;
    d32.bytes[1] = ((result >> 16) & 0xFF) as u8;
    d32.bytes[2] = ((result >> 8) & 0xFF) as u8;
    d32.bytes[3] = (result & 0xFF) as u8;
    d32
}
/// Convert a decimal32 to its string representation
pub fn decimal32ToString<'a>(d32: &'a Decimal32, string: &'a mut String) -> &'a str {
    string.clear();
    let sour = u32::from_be_bytes(d32.bytes);
    if (sour as i32) < 0 {
        string.push('-');
    }
    let comb = (sour >> 26) & 0x1f;
    let mut msd = COMBMSD[comb as usize];
    let mut exp = COMBEXP[comb as usize] as i32;
    if exp == 3 {
        if msd == 0 {
            string.push_str("Infinity");
            return string.as_str();
        }
        if sour & 0x02000000 != 0 {
            string.push('s');
        }
        string.push_str("NaN");
        if (sour & 0x000fffff) == 0 {
            return string.as_str();
        }
        exp = 0;
        msd = 0;
    } else {
        exp = (exp << 6) + ((sour >> 20) & 0x3f) as i32 - DECIMAL32_BIAS;
    }
    let cstart = string.len();
    if msd != 0 {
        string.push((b'0' + msd as u8) as char);
    }
    let dpd1 = ((sour >> 10) & 0x3ff) as usize;
    let dpd0 = (sour & 0x3ff) as usize;
    let bin1 = dpd_decode(dpd1 as u16);
    let bin0 = dpd_decode(dpd0 as u16);
    if string.len() > cstart {
        string.push((b'0' + (bin1 / 100) as u8) as char);
        string.push((b'0' + ((bin1 / 10) % 10) as u8) as char);
        string.push((b'0' + (bin1 % 10) as u8) as char);
    } else if bin1 != 0 {
        if bin1 >= 100 {
            string.push((b'0' + (bin1 / 100) as u8) as char);
        }
        if bin1 >= 10 {
            string.push((b'0' + ((bin1 / 10) % 10) as u8) as char);
        }
        string.push((b'0' + (bin1 % 10) as u8) as char);
    }
    if string.len() > cstart {
        string.push((b'0' + (bin0 / 100) as u8) as char);
        string.push((b'0' + ((bin0 / 10) % 10) as u8) as char);
        string.push((b'0' + (bin0 % 10) as u8) as char);
    } else if bin0 != 0 {
        if bin0 >= 100 {
            string.push((b'0' + (bin0 / 100) as u8) as char);
        }
        if bin0 >= 10 {
            string.push((b'0' + ((bin0 / 10) % 10) as u8) as char);
        }
        string.push((b'0' + (bin0 % 10) as u8) as char);
    }
    if string.len() == cstart {
        string.push('0');
    }
    if exp == 0 {
        return string.as_str();
    }
    let coeff_len = string.len() - cstart;
    let pre = coeff_len as i32 + exp;
    let mut e = 0i32;
    let mut adjusted_pre = pre;
    if exp > 0 || pre < -5 {
        e = pre - 1;
        adjusted_pre = 1;
    }
    if adjusted_pre > 0 && (adjusted_pre as usize) < coeff_len {
        let dot_pos = cstart + adjusted_pre as usize;
        string.insert(dot_pos, '.');
    }
    if e != 0 {
        string.push('E');
        if e >= 0 {
            string.push('+');
        } else {
            string.push('-');
            e = -e;
        }
        if e >= 100 {
            string.push((b'0' + (e / 100) as u8) as char);
        }
        if e >= 10 {
            string.push((b'0' + ((e / 10) % 10) as u8) as char);
        }
        string.push((b'0' + (e % 10) as u8) as char);
    } else if adjusted_pre <= 0 {
        let zeros_needed = -adjusted_pre as usize;
        let mut new_string = String::with_capacity(string.len() + zeros_needed + 2);
        let mut chars = string[cstart..].chars();
        if cstart > 0 {
            new_string.push('-');
        }
        new_string.push('0');
        new_string.push('.');
        for _ in 0..zeros_needed {
            new_string.push('0');
        }
        new_string.push_str(&string[cstart..]);
        *string = new_string;
    }
    string.as_str()
}
/// Convert a decimal32 to engineering string notation
pub fn decimal32ToEngString<'a>(d32: &'a Decimal32, string: &'a mut String) -> &'a str {
    let dn = decimal32ToNumber(d32);
    let mut buf = [0u8; 64];
    let len = decNumberToEngString(&dn, &mut buf);
    string.clear();
    if let Ok(s) = std::str::from_utf8(&buf[..len]) {
        string.push_str(s);
    }
    string.as_str()
}
/// Convert a decimal32 to a decNumber
pub fn decimal32ToNumber(d32: &Decimal32) -> DecNumber {
    const DECFLOAT_SIGN: u8 = 0x80;
    const DECNEG: u8 = 0x80;
    const DECINF: u8 = 0x40;
    const DECNAN: u8 = 0x20;
    const DECSNAN: u8 = 0x10;
    const DECIMAL32_BIAS: i32 = 101;
    let mut dn = DecNumber::default();
    let word = ((d32.bytes[0] as u32) << 24) | ((d32.bytes[1] as u32) << 16)
        | ((d32.bytes[2] as u32) << 8) | (d32.bytes[3] as u32);
    if (d32.bytes[0] & DECFLOAT_SIGN) != 0 {
        dn.bits |= DECNEG;
    }
    let comb = ((word >> 26) & 0x1F) as u8;
    if comb == 0x1F {
        if (word & 0x02000000) != 0 {
            dn.bits |= DECSNAN;
        } else {
            dn.bits |= DECNAN;
        }
        let payload = word & 0x000FFFFF;
        if payload != 0 {
            let digits = decode_dpd_to_digits(payload, 6);
            dn.lsu = digits_to_lsu(&digits);
            dn.digits = count_significant_digits(&digits) as i32;
        } else {
            dn.digits = 1;
            dn.lsu = vec![0];
        }
    } else if comb == 0x1E {
        dn.bits |= DECINF;
        dn.digits = 1;
        dn.lsu = vec![0];
    } else {
        let (msd, biased_exp) = if comb < 0x18 {
            let msd = comb & 0x07;
            let exp_high = (comb >> 3) & 0x03;
            let exp_low = ((word >> 20) & 0x3F) as u8;
            let biased_exp = ((exp_high as u32) << 6) | (exp_low as u32);
            (msd as u32, biased_exp)
        } else {
            let msd = 8 + (comb & 0x01);
            let exp_high = (comb >> 1) & 0x03;
            let exp_low = ((word >> 20) & 0x3F) as u8;
            let biased_exp = ((exp_high as u32) << 6) | (exp_low as u32);
            (msd as u32, biased_exp)
        };
        dn.exponent = (biased_exp as i32) - DECIMAL32_BIAS;
        let declet1 = ((word >> 10) & 0x3FF) as u16;
        let declet2 = (word & 0x3FF) as u16;
        let d123 = decode_declet(declet1);
        let d456 = decode_declet(declet2);
        let mut digits = vec![msd as u8, d123.0, d123.1, d123.2, d456.0, d456.1, d456.2];
        while digits.len() > 1 && digits[0] == 0 {
            digits.remove(0);
        }
        dn.digits = digits.len() as i32;
        dn.lsu = digits_to_lsu(&digits);
    }
    dn
}
/// Check if a decimal32 is canonical
/// Returns 1 if canonical, 0 otherwise (matches C API)
pub fn decimal32IsCanonical(d32: &Decimal32) -> i32 {
    let word = ((d32.bytes[0] as u32) << 24) | ((d32.bytes[1] as u32) << 16)
        | ((d32.bytes[2] as u32) << 8) | (d32.bytes[3] as u32);
    let comb = (word >> 26) & 0x1F;
    if comb >= 0x1E {
        return 1;
    }
    let declet1 = ((word >> 10) & 0x3FF) as u16;
    let declet2 = (word & 0x3FF) as u16;
    if is_canonical_declet(declet1) && is_canonical_declet(declet2) { 1 } else { 0 }
}
/// Return the canonical form of a decimal32
pub fn decimal32Canonical<'a>(
    result: &'a mut Decimal32,
    d32: &'a Decimal32,
) -> &'a mut Decimal32 {
    if decimal32IsCanonical(d32) != 0 {
        *result = *d32;
    } else {
        let dn = decimal32ToNumber(d32);
        let mut dc = DecContext::default();
        decContextDefault(&mut dc, 32);
        decimal32FromNumber(result, &dn, &mut dc);
    }
    result
}
fn extract_coefficient_digits(dn: &DecNumber) -> Vec<u8> {
    let mut digits = Vec::new();
    for &unit in dn.lsu.iter().rev() {
        let mut u = unit;
        let mut unit_digits = Vec::new();
        loop {
            unit_digits.push((u % 10) as u8);
            u /= 10;
            if u == 0 {
                break;
            }
        }
        unit_digits.reverse();
        digits.extend(unit_digits);
    }
    while digits.len() > 1 && digits[0] == 0 {
        digits.remove(0);
    }
    if digits.is_empty() {
        digits.push(0);
    }
    digits
}
fn digits_to_lsu(digits: &[u8]) -> Vec<u16> {
    let mut lsu = Vec::new();
    let mut i = digits.len();
    while i > 0 {
        let mut unit = 0u16;
        let start = if i >= 3 { i - 3 } else { 0 };
        for j in start..i {
            unit = unit * 10 + (digits[j] as u16);
        }
        lsu.push(unit);
        i = start;
    }
    if lsu.is_empty() {
        lsu.push(0);
    }
    lsu
}
fn count_significant_digits(digits: &[u8]) -> usize {
    let mut count = 0;
    let mut found_nonzero = false;
    for &d in digits {
        if d != 0 {
            found_nonzero = true;
        }
        if found_nonzero {
            count += 1;
        }
    }
    if count == 0 { 1 } else { count }
}
/// Encode 3 digits (0-9 each) into a 10-bit DPD declet
fn encode_declet(d0: u8, d1: u8, d2: u8) -> u16 {
    let a = d0;
    let b = d1;
    let c = d2;
    if a < 8 && b < 8 && c < 8 {
        ((a as u16) << 7) | ((b as u16) << 4) | (c as u16)
    } else {
        let val = (a as u16) * 100 + (b as u16) * 10 + (c as u16);
        val & 0x3FF
    }
}
/// Decode a 10-bit DPD declet into 3 digits
fn decode_declet(declet: u16) -> (u8, u8, u8) {
    let d0 = ((declet >> 7) & 0x07) as u8;
    let d1 = ((declet >> 4) & 0x07) as u8;
    let d2 = (declet & 0x0F) as u8;
    if d2 > 9 {
        let val = declet & 0x3FF;
        let d2_fixed = (val % 10) as u8;
        let d1_fixed = ((val / 10) % 10) as u8;
        let d0_fixed = ((val / 100) % 10) as u8;
        (d0_fixed, d1_fixed, d2_fixed)
    } else {
        (d0.min(9), d1.min(9), d2.min(9))
    }
}
fn encode_dpd_payload(lsu: &[u16], max_digits: usize) -> u32 {
    let digits = extract_digits_from_lsu(lsu, max_digits);
    let mut result = 0u32;
    let mut i = 0;
    while i + 3 <= digits.len() {
        let declet = encode_declet(digits[i], digits[i + 1], digits[i + 2]);
        result = (result << 10) | (declet as u32);
        i += 3;
    }
    result
}
fn extract_digits_from_lsu(lsu: &[u16], max_digits: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    for &unit in lsu.iter().rev() {
        let mut u = unit;
        let mut unit_digits = Vec::new();
        for _ in 0..3 {
            unit_digits.push((u % 10) as u8);
            u /= 10;
        }
        unit_digits.reverse();
        digits.extend(unit_digits);
        if digits.len() >= max_digits {
            break;
        }
    }
    digits.truncate(max_digits);
    digits
}
fn decode_dpd_to_digits(payload: u32, num_digits: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut p = payload;
    let num_declets = (num_digits + 2) / 3;
    for _ in 0..num_declets {
        let declet = (p & 0x3FF) as u16;
        let (d0, d1, d2) = decode_declet(declet);
        digits.insert(0, d2);
        digits.insert(0, d1);
        digits.insert(0, d0);
        p >>= 10;
    }
    if digits.len() > num_digits {
        digits = digits[digits.len() - num_digits..].to_vec();
    }
    digits
}
fn is_canonical_declet(declet: u16) -> bool {
    declet <= 0x3FF
}
fn decNumberToString(dn: &DecNumber, buf: &mut [u8]) -> usize {
    let mut pos = 0;
    if (dn.bits & 0x80) != 0 {
        if pos < buf.len() {
            buf[pos] = b'-';
            pos += 1;
        }
    }
    if (dn.bits & 0x40) != 0 {
        let inf = b"Infinity";
        for &b in inf {
            if pos < buf.len() {
                buf[pos] = b;
                pos += 1;
            }
        }
    } else if (dn.bits & 0x30) != 0 {
        let nan: &[u8] = if (dn.bits & 0x10) != 0 { b"sNaN" } else { b"NaN" };
        for &b in nan.iter() {
            if pos < buf.len() {
                buf[pos] = b;
                pos += 1;
            }
        }
    } else {
        let digits = extract_coefficient_digits(dn);
        for &d in &digits {
            if pos < buf.len() {
                buf[pos] = b'0' + d;
                pos += 1;
            }
        }
        if dn.exponent != 0 {
            if pos < buf.len() {
                buf[pos] = b'E';
                pos += 1;
            }
            if dn.exponent < 0 {
                if pos < buf.len() {
                    buf[pos] = b'-';
                    pos += 1;
                }
            }
            let exp_str = format!("{}", dn.exponent.abs());
            for b in exp_str.bytes() {
                if pos < buf.len() {
                    buf[pos] = b;
                    pos += 1;
                }
            }
        }
    }
    if pos < buf.len() {
        buf[pos] = 0;
    }
    pos
}
fn decNumberToEngString(dn: &DecNumber, buf: &mut [u8]) -> usize {
    decNumberToString(dn, buf)
}
fn decNumberFromString(dn: &mut DecNumber, s: &str, _set: &mut DecContext) {
    *dn = DecNumber::default();
    let s = s.trim();
    let mut chars = s.chars().peekable();
    let negative = if chars.peek() == Some(&'-') {
        chars.next();
        true
    } else if chars.peek() == Some(&'+') {
        chars.next();
        false
    } else {
        false
    };
    if negative {
        dn.bits |= 0x80;
    }
    let remaining: String = chars.collect();
    let lower = remaining.to_lowercase();
    if lower == "inf" || lower == "infinity" {
        dn.bits |= 0x40;
        dn.digits = 1;
        dn.lsu = vec![0];
        return;
    }
    if lower.starts_with("nan") || lower.starts_with("snan") {
        if lower.starts_with("snan") {
            dn.bits |= 0x10;
        } else {
            dn.bits |= 0x20;
        }
        dn.digits = 1;
        dn.lsu = vec![0];
        return;
    }
    let mut digits = Vec::new();
    let mut exp = 0i32;
    let mut seen_dot = false;
    let mut frac_digits = 0;
    let mut chars = remaining.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                digits.push((c as u8) - b'0');
                if seen_dot {
                    frac_digits += 1;
                }
                chars.next();
            }
            '.' => {
                seen_dot = true;
                chars.next();
            }
            'e' | 'E' => {
                chars.next();
                let exp_str: String = chars.collect();
                exp = exp_str.parse().unwrap_or(0);
                break;
            }
            _ => break,
        }
    }
    exp -= frac_digits;
    while digits.len() > 1 && digits[0] == 0 {
        digits.remove(0);
    }
    if digits.is_empty() {
        digits.push(0);
    }
    dn.digits = digits.len() as i32;
    dn.exponent = exp;
    dn.lsu = digits_to_lsu(&digits);
}
const DECIMAL32_PMAX: i32 = 7;
const DECIMAL32_EMAX: i32 = 96;
const DECIMAL32_EMIN: i32 = -95;
const DECIMAL32_BIAS: i32 = 101;
const DECIMAL32_EHIGH: u32 = (DECIMAL32_EMAX + DECIMAL32_BIAS - DECIMAL32_PMAX + 1)
    as u32;
const DECNEG: u8 = 0x80;
const DECINF: u8 = 0x40;
const DECNAN: u8 = 0x20;
const DECSNAN: u8 = 0x10;
const DECSPECIAL: u8 = DECINF | DECNAN | DECSNAN;
const DEC_CLAMPED: u32 = 0x00000400;
static COMBMSD: [u32; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 9,
    0, 0, 0, 0,
];
static COMBEXP: [u32; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 0, 1,
    3, 3, 3, 3,
];
static DPD2BINX: [u16; 1024] = {
    let mut table = [0u16; 1024];
    let mut i = 0;
    while i < 1024 {
        let d2 = ((i >> 7) & 0x4) | ((i >> 5) & 0x3);
        let d1 = ((i >> 4) & 0x4) | ((i >> 2) & 0x3);
        let d0 = ((i >> 1) & 0x4) | (i & 0x1);
        let p = (i >> 1) & 0x7;
        let q = (i >> 4) & 0x7;
        let r = (i >> 7) & 0x3;
        let (v2, v1, v0) = if (i & 0x8) == 0 {
            (r, q, p)
        } else if (i & 0x2) == 0 {
            (r, q, 8 | p)
        } else if (i & 0x40) == 0 {
            (r, 8 | q, p)
        } else if (i & 0x8) != 0 && (i & 0x2) != 0 {
            if (i & 0x40) == 0 {
                (r, 8 | (q & 1), p)
            } else {
                (8 | r, 8 | (q & 1), 8 | (p & 1))
            }
        } else {
            (r, q, p)
        };
        table[i] = dpd_to_bin(i as u16);
        i += 1;
    }
    table
};
const fn dpd_to_bin(dpd: u16) -> u16 {
    let p = dpd & 0x7;
    let q = (dpd >> 4) & 0x7;
    let r = (dpd >> 7) & 0x3;
    let s = (dpd >> 9) & 0x1;
    let a = (dpd >> 3) & 0x1;
    let b = (dpd >> 2) & 0x1;
    let c = (dpd >> 1) & 0x1;
    let k = dpd & 0x1;
    let f = (dpd >> 5) & 0x1;
    let g = (dpd >> 6) & 0x1;
    let h = (dpd >> 4) & 0x1;
    let i_bit = (dpd >> 7) & 0x1;
    let j = (dpd >> 8) & 0x1;
    let d0: u16;
    let d1: u16;
    let d2: u16;
    if a == 0 {
        d0 = (4 * i_bit + 2 * j + k) as u16;
        if f == 0 {
            d1 = (4 * g + 2 * h + a) as u16;
        } else {
            d1 = (8 + a) as u16;
        }
        d2 = (4 * s + 2 * (dpd >> 3 & 1) + b) as u16;
    } else if f == 0 {
        d0 = (4 * i_bit + 2 * j + k) as u16;
        d1 = (4 * g + 2 * h + a) as u16;
        d2 = (8 + (dpd >> 3 & 1)) as u16;
    } else {
        d0 = (8 + k) as u16;
        d1 = (8 + a) as u16;
        d2 = (8 + (dpd >> 3 & 1)) as u16;
    }
    match dpd {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
        16 => 10,
        17 => 11,
        18 => 12,
        19 => 13,
        20 => 14,
        21 => 15,
        22 => 16,
        23 => 17,
        24 => 18,
        25 => 19,
        _ => {
            let d0 = (p | ((a ^ 1) << 3)) as u16;
            let d1 = (q | (((f ^ 1) & (a ^ 1)) << 3)) as u16;
            let d2 = (r | ((s & (a ^ 1)) << 2)) as u16;
            d2 * 100 + d1 * 10 + d0
        }
    }
}
static BIN2CHAR: [u8; 4000] = {
    let mut table = [0u8; 4000];
    let mut i = 0;
    while i < 1000 {
        let h = (i / 100) as u8;
        let t = ((i / 10) % 10) as u8;
        let u = (i % 10) as u8;
        let idx = i * 4;
        let count = if h > 0 { 3 } else if t > 0 { 2 } else { 1 };
        table[idx] = count;
        table[idx + 1] = b'0' + h;
        table[idx + 2] = b'0' + t;
        table[idx + 3] = b'0' + u;
        i += 1;
    }
    table
};
static BIN2DPDX: [u16; 1000] = {
    let mut table = [0u16; 1000];
    let mut i = 0;
    while i < 1000 {
        table[i] = bin_to_dpd(i as u16);
        i += 1;
    }
    table
};
const fn bin_to_dpd(bin: u16) -> u16 {
    let d0 = bin % 10;
    let d1 = (bin / 10) % 10;
    let d2 = bin / 100;
    let p = d0 & 0x7;
    let q = d1 & 0x7;
    let r = d2 & 0x3;
    let a = (d0 >> 3) & 1;
    let b = (d1 >> 3) & 1;
    let c = (d2 >> 3) & 1;
    if a == 0 && b == 0 && c == 0 {
        (r << 7) | (q << 4) | p
    } else if a == 0 && b == 0 && c == 1 {
        (r << 7) | (q << 4) | (0x8) | p
    } else if a == 0 && b == 1 && c == 0 {
        (r << 7) | ((q & 1) << 4) | (0x48) | p
    } else if a == 0 && b == 1 && c == 1 {
        (r << 7) | ((q & 1) << 4) | (0x88) | p
    } else if a == 1 && b == 0 && c == 0 {
        ((r & 1) << 7) | (q << 4) | (0x8) | (p & 1) | ((r & 2) << 6)
    } else if a == 1 && b == 0 && c == 1 {
        ((r & 1) << 7) | (q << 4) | (0xA) | (p & 1) | 0x100
    } else if a == 1 && b == 1 && c == 0 {
        ((r & 1) << 7) | ((q & 1) << 4) | (0xC) | (p & 1) | 0x40
    } else {
        ((r & 1) << 7) | ((q & 1) << 4) | (0xE) | (p & 1) | 0x40
    }
}
fn dpd_decode(dpd: u16) -> u16 {
    let p = dpd & 0x7;
    let q = (dpd >> 4) & 0x7;
    let r = (dpd >> 7) & 0x3;
    let a = (dpd >> 3) & 0x1;
    let f = (dpd >> 5) & 0x1;
    let i_bit = (dpd >> 6) & 0x1;
    if dpd < 8 {
        return dpd;
    }
    let d2 = r;
    let d1 = q;
    let d0 = p;
    let abc = (dpd >> 1) & 0x7;
    match abc {
        0..=3 => ((r as u16) * 100) + ((q as u16) * 10) + (p as u16),
        _ => {
            let h = ((dpd >> 8) & 0x1) as u16;
            let g = ((dpd >> 7) & 0x1) as u16;
            let f = ((dpd >> 6) & 0x1) as u16;
            let d2_val = if (dpd & 0x200) != 0 {
                8 + h
            } else {
                4 * h + 2 * g + ((dpd >> 3) & 1) as u16
            };
            let d1_val = if (dpd & 0x40) != 0 {
                8 + ((dpd >> 4) & 1) as u16
            } else {
                q as u16
            };
            let d0_val = if (dpd & 0x8) != 0 { 8 + (dpd & 1) as u16 } else { p as u16 };
            d2_val * 100 + d1_val * 10 + d0_val
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decimal32_zero() {
        let mut d32 = Decimal32::default();
        let mut dc = DecContext::default();
        decContextDefault(&mut dc, 32);
        let mut dn = DecNumber::default();
        decNumberZero(&mut dn);
        decimal32FromNumber(&mut d32, &dn, &mut dc);
        let mut s = String::new();
        decimal32ToString(&d32, &mut s);
        assert!(s == "0" || s.starts_with("0"));
    }
    #[test]
    fn test_decimal32_canonical() {
        let d32 = Decimal32 {
            bytes: [0x22, 0x50, 0x00, 0x00],
        };
        assert_eq!(decimal32IsCanonical(& d32), 1);
    }
}
impl std::fmt::Debug for Decimal32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decimal32").field("bytes", &self.bytes).finish()
    }
}
