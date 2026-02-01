//! Module: decimal64
//!
//! Contains 9 transpiled functions:
//! - decimal64IsCanonical:3409402538672671975:./src/decNumber/decimal64.c
//! - decimal64FromNumber:2167219971503340766:./src/decNumber/decimal64.c
//! - decimal64ToNumber:7130991854869062294:./src/decNumber/decimal64.c
//! - decimal64FromString:2289447915803157169:./src/decNumber/decimal64.c
//! - decDigitsFromDPD:16677431529018410047:./src/decNumber/decimal64.c
//! - decDigitsToDPD:6347309599260150379:./src/decNumber/decimal64.c
//! - decimal64Canonical:7424306962765173211:./src/decNumber/decimal64.c
//! - decimal64ToEngString:13023393376021909851:./src/decNumber/decimal64.c
//! - decimal64ToString:2664880580566934467:./src/decNumber/decimal64.c
use std::ptr::NonNull;
use crate::deccontext::{decContextDefault, decContextSetStatus};
use crate::decnumber::{
    decNumberFromString, decNumberPlus, decNumberToEngString, decNumberToString,
    decNumberZero,
};
use crate::types::DecNumber;
use crate::types::*;
const DECIMAL64_PMAX: i32 = 16;
const DECIMAL64_EMAX: i32 = 384;
const DECIMAL64_EMIN: i32 = -383;
/// Bias for decimal64 exponent
const DECIMAL64_BIAS: i32 = 398;
const DECIMAL64_DECLETS: i32 = 5;
const DEC_CLAMPED: u32 = 0x00000400;
const DECNAN: u8 = 0x20;
const DECSNAN: u8 = 0x10;
const DECINF: u8 = 0x40;
/// Negative sign bit
const DECNEG: u8 = 0x80;
const DECSPECIAL: u8 = DECINF | DECNAN | DECSNAN;
/// Combination field to MSD mapping table
const COMBMSD: [u32; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8,
    9, 9, 9, 9,
];
/// Combination field to exponent continuation mapping table
const COMBEXP: [u32; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 3,
    0, 1, 2, 3,
];
const DPD2BIN: [u16; 1024] = generate_dpd2bin();
const BIN2DPD: [u16; 1000] = generate_bin2dpd();
const BIN2CHAR: [[u8; 4]; 1000] = generate_bin2char();
const DECPOWERS: [u32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
const MULTIES: [u32; 4] = [131072, 13108, 1311, 131];
const D2UTABLE: [usize; 50] = [
    0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9,
    10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 16,
    17,
];
const fn generate_dpd2bin() -> [u16; 1024] {
    let mut table = [0u16; 1024];
    let mut i = 0u16;
    while i < 1024 {
        let d0 = dpd_to_digit(i, 0);  // units
        let d1 = dpd_to_digit(i, 1);  // tens
        let d2 = dpd_to_digit(i, 2);  // hundreds
        table[i as usize] = d2 as u16 * 100 + d1 as u16 * 10 + d0 as u16;
        i += 1;
    }
    table
}
const fn dpd_to_digit(dpd: u16, pos: u8) -> u8 {
    // DPD bit layout for simple case (bit 3 = 0):
    // Bits 9-7: d2 (hundreds)
    // Bits 6-4: d1 (tens)
    // Bit 3: 0 (marker for simple case)
    // Bits 2-0: d0 (units)
    let bits = dpd as u32;

    // Extract d2, d1, d0 for simple case first
    let d0_simple = (bits & 0x7) as u8;           // bits 2-0
    let d1_simple = ((bits >> 4) & 0x7) as u8;    // bits 6-4
    let d2_simple = ((bits >> 7) & 0x7) as u8;    // bits 9-7

    // Check if it's the simple case (bit 3 = 0)
    let is_simple = (bits >> 3) & 1 == 0;

    if is_simple {
        match pos {
            0 => d0_simple,
            1 => d1_simple,
            2 => d2_simple,
            _ => 0,
        }
    } else {
        // Complex case - use the DPD2BIN lookup for simplicity
        // For large digits (8 or 9), the encoding is different
        let bin = dpd_to_bin_complex(dpd);
        match pos {
            0 => (bin % 10) as u8,
            1 => ((bin / 10) % 10) as u8,
            2 => (bin / 100) as u8,
            _ => 0,
        }
    }
}

// Helper for complex DPD decoding (when bit 3 = 1)
const fn dpd_to_bin_complex(dpd: u16) -> u16 {
    let bits = dpd as u32;

    // The encoding uses v (bit 3), w (bit 2), x (bit 1) to encode the type:
    // v=1, w=0, x=0: Only d0 >= 8
    // v=1, w=0, x=1: Only d1 >= 8
    // v=1, w=1, x=0: Only d2 >= 8
    // v=1, w=1, x=1: Multiple digits >= 8 (need additional bits to disambiguate)
    let v = (bits >> 3) & 1;
    let w = (bits >> 2) & 1;
    let x = (bits >> 1) & 1;
    let y = bits & 1;  // Low bit of d0

    // Pattern from bits 3,2,1 (v,w,x)
    let pattern = (v << 2) | (w << 1) | x;  // Note: v=1 always here since this is complex case

    match pattern {
        0b100 => {
            // v=1, w=0, x=0: Only d0 >= 8
            let d2 = ((bits >> 7) & 0x7) as u16;
            let d1 = ((bits >> 4) & 0x7) as u16;
            let d0 = 8 + y as u16;
            d2 * 100 + d1 * 10 + d0
        }
        0b101 => {
            // v=1, w=0, x=1: Only d1 >= 8
            // Encoding: p=d2b2, q=d2b1, r=d2b0, s=d0b2, t=d0b1, u=d1b0, y=d0b0
            let d2 = ((bits >> 7) & 0x7) as u16;
            let d1 = 8 + ((bits >> 4) & 1) as u16;  // u = d1 low bit
            // d0: bit 2 from s(bit 6), bit 1 from t(bit 5), bit 0 from y(bit 0)
            let d0 = ((((bits >> 6) & 1) << 2) | (((bits >> 5) & 1) << 1) | (bits & 1)) as u16;
            d2 * 100 + d1 * 10 + d0
        }
        0b110 => {
            // v=1, w=1, x=0: Only d2 >= 8
            // Encoding: p=d0b2, q=d0b1, r=d2b0, s=d1b2, t=d1b1, u=d1b0, y=d0b0
            let d2 = 8 + ((bits >> 7) & 1) as u16;  // r = d2 low bit
            let d1 = ((bits >> 4) & 0x7) as u16;   // s,t,u = d1 bits 2,1,0
            // d0: bit 2 from p(bit 9), bit 1 from q(bit 8), bit 0 from y(bit 0)
            let d0 = ((((bits >> 9) & 1) << 2) | (((bits >> 8) & 1) << 1) | (bits & 1)) as u16;
            d2 * 100 + d1 * 10 + d0
        }
        0b111 => {
            // v=1, w=1, x=1: Multiple digits >= 8
            // Need to look at more bits to determine which combination
            // Look at bits 6 and 5 to disambiguate
            let b6 = (bits >> 6) & 1;
            let b5 = (bits >> 5) & 1;

            if b6 == 0 && b5 == 0 {
                // d0 and d1 >= 8, d2 < 8
                let d2 = ((bits >> 7) & 0x7) as u16;
                let d1 = 8 + ((bits >> 4) & 1) as u16;
                let d0 = 8 + (bits & 1) as u16;
                d2 * 100 + d1 * 10 + d0
            } else if b6 == 0 && b5 == 1 {
                // d0 and d2 >= 8, d1 < 8
                // Encoding: p=f(d1b2), q=g(d1b1), r=d(d2b0), s=0, t=1, u=h(d1b0)
                let d2 = 8 + ((bits >> 7) & 1) as u16;
                // d1 bits are scattered: bit2 at pos 9, bit1 at pos 8, bit0 at pos 4
                let d1 = ((((bits >> 9) & 1) << 2) | (((bits >> 8) & 1) << 1) | ((bits >> 4) & 1)) as u16;
                let d0 = 8 + (bits & 1) as u16;
                d2 * 100 + d1 * 10 + d0
            } else if b6 == 1 && b5 == 0 {
                // d1 and d2 >= 8, d0 < 8
                // Encoding: p=j(d0b2), q=k(d0b1), r=d(d2b0), s=1, t=0, u=h(d1b0), y=m(d0b0)
                let d2 = 8 + ((bits >> 7) & 1) as u16;
                let d1 = 8 + ((bits >> 4) & 1) as u16;
                // d0 bits are scattered: bit2 at pos 9, bit1 at pos 8, bit0 at pos 0
                let d0 = ((((bits >> 9) & 1) << 2) | (((bits >> 8) & 1) << 1) | (bits & 1)) as u16;
                d2 * 100 + d1 * 10 + d0
            } else {
                // All three >= 8
                let d2 = 8 + ((bits >> 7) & 1) as u16;
                let d1 = 8 + ((bits >> 4) & 1) as u16;
                let d0 = 8 + (bits & 1) as u16;
                d2 * 100 + d1 * 10 + d0
            }
        }
        _ => {
            // Shouldn't reach here for complex case (v should be 1)
            0
        }
    }
}
const fn generate_bin2dpd() -> [u16; 1000] {
    let mut table = [0u16; 1000];
    let mut i = 0u16;
    while i < 1000 {
        let d0 = (i % 10) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = (i / 100) as u8;
        table[i as usize] = bin_to_dpd(d2, d1, d0);
        i += 1;
    }
    table
}
const fn bin_to_dpd(d2: u8, d1: u8, d0: u8) -> u16 {
    let a = (d2 >> 3) & 1;
    let b = (d2 >> 2) & 1;
    let c = (d2 >> 1) & 1;
    let d = d2 & 1;
    let e = (d1 >> 3) & 1;
    let f = (d1 >> 2) & 1;
    let g = (d1 >> 1) & 1;
    let h = d1 & 1;
    let i = (d0 >> 3) & 1;
    let j = (d0 >> 2) & 1;
    let k = (d0 >> 1) & 1;
    let m = d0 & 1;
    let p: u16;
    let q: u16;
    let r: u16;
    let s: u16;
    let t: u16;
    let u: u16;
    let v: u16;
    let w: u16;
    let x: u16;
    let y: u16;
    if a == 0 && e == 0 && i == 0 {
        p = b as u16;
        q = c as u16;
        r = d as u16;
        s = f as u16;
        t = g as u16;
        u = h as u16;
        v = 0;
        w = j as u16;
        x = k as u16;
        y = m as u16;
    } else if a == 0 && e == 0 && i == 1 {
        p = b as u16;
        q = c as u16;
        r = d as u16;
        s = f as u16;
        t = g as u16;
        u = h as u16;
        v = 1;
        w = 0;
        x = 0;
        y = m as u16;
    } else if a == 0 && e == 1 && i == 0 {
        p = b as u16;
        q = c as u16;
        r = d as u16;
        s = j as u16;
        t = k as u16;
        u = h as u16;
        v = 1;
        w = 0;
        x = 1;
        y = m as u16;
    } else if a == 0 && e == 1 && i == 1 {
        p = b as u16;
        q = c as u16;
        r = d as u16;
        s = 0;
        t = 0;
        u = h as u16;
        v = 1;
        w = 1;
        x = 1;
        y = m as u16;
    } else if a == 1 && e == 0 && i == 0 {
        p = j as u16;
        q = k as u16;
        r = d as u16;
        s = f as u16;
        t = g as u16;
        u = h as u16;
        v = 1;
        w = 1;
        x = 0;
        y = m as u16;
    } else if a == 1 && e == 0 && i == 1 {
        p = f as u16;
        q = g as u16;
        r = d as u16;
        s = 0;
        t = 1;
        u = h as u16;
        v = 1;
        w = 1;
        x = 1;
        y = m as u16;
    } else if a == 1 && e == 1 && i == 0 {
        p = j as u16;
        q = k as u16;
        r = d as u16;
        s = 1;
        t = 0;
        u = h as u16;
        v = 1;
        w = 1;
        x = 1;
        y = m as u16;
    } else {
        p = 0;
        q = 0;
        r = d as u16;
        s = 1;
        t = 1;
        u = h as u16;
        v = 1;
        w = 1;
        x = 1;
        y = m as u16;
    }
    (p << 9) | (q << 8) | (r << 7) | (s << 6) | (t << 5) | (u << 4) | (v << 3) | (w << 2)
        | (x << 1) | y
}
const fn generate_bin2char() -> [[u8; 4]; 1000] {
    let mut table = [[0u8; 4]; 1000];
    let mut i = 0usize;
    while i < 1000 {
        let d0 = (i % 10) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = (i / 100) as u8;
        let sig = if d2 != 0 { 3 } else if d1 != 0 { 2 } else { 1 };
        table[i][0] = sig;
        table[i][1] = b'0' + d2;
        table[i][2] = b'0' + d1;
        table[i][3] = b'0' + d0;
        i += 1;
    }
    table
}
#[inline]
fn DPD2BINx(dpd: u16) -> u16 {
    DPD2BIN[dpd as usize]
}
#[inline]
fn BIN2DPDx(bin: u16) -> u16 {
    if bin < 1000 { BIN2DPD[bin as usize] } else { 0 }
}
/// Helper function to get digits-to-units mapping
#[inline]
fn d2u(digits: i32) -> usize {
    if digits <= 49 {
        D2UTABLE[digits as usize]
    } else {
        ((digits + 3 - 1) / 3) as usize
    }
}
/// Convert a decimal64 to an engineering string
///
/// # Arguments
/// * `d64` - The decimal64 to convert
/// * `string` - The buffer to write the string to
///
/// # Returns
/// A mutable reference to the string buffer
pub fn decimal64ToEngString<'a>(
    d64: &'a Decimal64,
    string: &'a mut Vec<u8>,
) -> &'a mut Vec<u8> {
    let mut dn = DecNumber::default();
    decimal64ToNumber(d64, &mut dn);
    let result = crate::decnumber::decNumberToEngString(&dn);
    string.clear();
    string.extend_from_slice(result.as_bytes());
    string
}
/// Check if decimal64 is in canonical form
pub fn decimal64IsCanonical(d64: &Decimal64) -> u32 {
    let mut dn = DecNumber::default();
    let mut canon = Decimal64::default();
    let mut dc = DecContext::default();
    decContextDefault(&mut dc, 64);
    decimal64ToNumber(d64, &mut dn);
    decimal64FromNumber(&mut canon, &dn, &mut dc);
    if d64.bytes == canon.bytes { 1 } else { 0 }
}
/// Convert decNumber digits to DPD encoding
pub fn decDigitsToDPD(dn: &DecNumber, targ: &mut [u32], shift: i32) {
    let mut digits = dn.digits;
    let mut uout_idx = 0usize;
    let mut uoff = 0u32;
    let mut uar = vec![0u16; ((DECIMAL64_PMAX + 3 - 1) / 3) as usize + 4];
    let source_units: &[u16];
    if shift != 0 {
        let source_len = d2u(digits);
        let shift_units = d2u(shift);
        let target_len = d2u(digits + shift);
        let cut = 3 - (shift - ((shift_units as i32 - 1) * 3));
        if cut == 0 || cut == 3 {
            for i in 0..source_len {
                if i + shift_units <= target_len {
                    uar[i + shift_units] = if i < dn.lsu.len() { dn.lsu[i] } else { 0 };
                }
            }
        } else {
            let mut next = 0u32;
            let cut = cut as usize;
            for i in (0..source_len).rev() {
                let source_val = if i < dn.lsu.len() { dn.lsu[i] as u32 } else { 0 };
                let quot = (source_val >> cut) * MULTIES[cut] >> 17;
                let rem = source_val - quot * DECPOWERS[cut];
                next += quot;
                let target_idx = i + shift_units;
                if target_idx < target_len {
                    uar[target_idx] = next as u16;
                }
                next = rem * DECPOWERS[3 - cut];
            }
            for i in (0..shift_units).rev() {
                if i < uar.len() {
                    uar[i] = next as u16;
                }
                next = 0;
            }
        }
        digits += shift;
        source_units = &uar;
    } else {
        source_units = &dn.lsu;
    }
    let mut d = digits;
    let mut n = 0usize;
    while d > 0 {
        let bin = if n < source_units.len() { source_units[n] } else { 0 };
        d -= 3;
        n += 1;
        let dpd = BIN2DPDx(bin) as u32;
        if uout_idx < targ.len() {
            targ[uout_idx] |= dpd << uoff;
        }
        uoff += 10;
        if uoff >= 32 {
            uout_idx += 1;
            uoff -= 32;
            if uout_idx < targ.len() {
                targ[uout_idx] |= dpd >> (10 - uoff);
            }
        }
    }
}
/// Convert digits from DPD (Densely Packed Decimal) encoding
///
/// # Arguments
/// * `dn` - The decNumber to fill
/// * `sour` - The source array containing DPD-encoded data
/// * `declets` - Number of declets to decode
pub fn decDigitsFromDPD(dn: &mut DecNumber, sour: &[u32], declets: i32) {
    let mut digits: Vec<u8> = Vec::new();
    let mut bit_pos: i32 = 0;
    for _ in 0..declets {
        let word_idx = (bit_pos / 32) as usize;
        let bit_offset = bit_pos % 32;
        let declet = if word_idx < sour.len() {
            if bit_offset <= 22 {
                (sour[word_idx] >> bit_offset) & 0x3ff
            } else {
                let low_bits = sour[word_idx] >> bit_offset;
                let high_bits = if word_idx + 1 < sour.len() {
                    sour[word_idx + 1] << (32 - bit_offset)
                } else {
                    0
                };
                (low_bits | high_bits) & 0x3ff
            }
        } else {
            0
        };
        let d0 = dpd_to_digit(
            ((declet >> 0) & 0xf).try_into().unwrap(),
            ((declet >> 4) & 0x7).try_into().unwrap(),
        );
        let d1 = dpd_to_digit(
            ((declet >> 4) & 0xf).try_into().unwrap(),
            ((declet >> 7) & 0x7).try_into().unwrap(),
        );
        let d2 = dpd_to_digit(((declet >> 7) & 0x7).try_into().unwrap(), 0);
        digits.push(d2 as u8);
        digits.push(d1 as u8);
        digits.push(d0 as u8);
        bit_pos += 10;
    }
    while digits.len() > 1 && digits[0] == 0 {
        digits.remove(0);
    }
    dn.digits = digits.len() as i32;
    dn.lsu.clear();
    let mut i = digits.len();
    while i > 0 {
        let mut unit: u16 = 0;
        let start = if i >= 3 { i - 3 } else { 0 };
        for j in start..i {
            unit = unit * 10 + digits[j] as u16;
        }
        dn.lsu.push(unit);
        i = start;
    }
}
/// Convert a string to a decimal64
///
/// # Arguments
/// * `result` - The decimal64 to fill
/// * `string` - The string to parse
/// * `set` - The decimal context
///
/// # Returns
/// A mutable reference to the result
pub fn decimal64FromString<'a>(
    result: &'a mut Decimal64,
    string: &'a str,
    set: &'a mut DecContext,
) -> &'a mut Decimal64 {
    let mut dn = DecNumber::default();
    crate::decnumber::decNumberFromString(&mut dn, string, set);
    decimal64FromNumber(result, &dn, set);
    result
}
/// Convert decimal64 to string representation
pub fn decimal64ToString<'a>(d64: &'a Decimal64, string: &'a mut [u8]) -> &'a str {
    let mut c_idx = 0usize;
    let sourar = [
        u32::from_le_bytes([d64.bytes[0], d64.bytes[1], d64.bytes[2], d64.bytes[3]]),
        u32::from_le_bytes([d64.bytes[4], d64.bytes[5], d64.bytes[6], d64.bytes[7]]),
    ];
    if (sourar[1] as i32) < 0 {
        if c_idx < string.len() {
            string[c_idx] = b'-';
            c_idx += 1;
        }
    }
    let comb = ((sourar[1] >> 26) & 0x1f) as usize;
    let msd = COMBMSD[comb];
    let exp_cont = COMBEXP[comb];
    if exp_cont == 3 {
        if msd == 0 {
            let inf = b"Infinity";
            for &b in inf.iter() {
                if c_idx < string.len() {
                    string[c_idx] = b;
                    c_idx += 1;
                }
            }
            if c_idx < string.len() {
                string[c_idx] = 0;
            }
            let len = string.iter().position(|&c| c == 0).unwrap_or(string.len());
            return std::str::from_utf8(&string[..len]).unwrap_or("");
        }
        if sourar[1] & 0x02000000 != 0 {
            if c_idx < string.len() {
                string[c_idx] = b's';
                c_idx += 1;
            }
        }
        let nan = b"NaN";
        for &b in nan.iter() {
            if c_idx < string.len() {
                string[c_idx] = b;
                c_idx += 1;
            }
        }
        if sourar[0] == 0 && (sourar[1] & 0x0003ffff) == 0 {
            if c_idx < string.len() {
                string[c_idx] = 0;
            }
            let len = string.iter().position(|&c| c == 0).unwrap_or(string.len());
            return std::str::from_utf8(&string[..len]).unwrap_or("");
        }
    }
    let exp = if exp_cont == 3 {
        0i32
    } else {
        ((exp_cont << 8) + ((sourar[1] >> 18) & 0xff)) as i32 - DECIMAL64_BIAS
    };
    let cstart = c_idx;
    if msd != 0 && exp_cont != 3 {
        if c_idx < string.len() {
            string[c_idx] = b'0' + msd as u8;
            c_idx += 1;
        }
    }
    let declets = [
        (sourar[1] >> 8) & 0x3ff,
        ((sourar[1] & 0xff) << 2) | (sourar[0] >> 30),
        (sourar[0] >> 20) & 0x3ff,
        (sourar[0] >> 10) & 0x3ff,
        sourar[0] & 0x3ff,
    ];
    for &dpd in &declets {
        let bin = DPD2BINx(dpd as u16) as usize;
        let chars = &BIN2CHAR[bin];
        let sig = chars[0] as usize;
        if c_idx != cstart {
            for i in 1..4 {
                if c_idx < string.len() {
                    string[c_idx] = chars[i];
                    c_idx += 1;
                }
            }
        } else if bin > 0 {
            // Only start writing when we encounter a non-zero declet
            // This prevents outputting leading zeros; for pure zero,
            // the fallback at the end writes a single '0'
            for i in (4 - sig)..4 {
                if c_idx < string.len() {
                    string[c_idx] = chars[i];
                    c_idx += 1;
                }
            }
        }
    }
    if c_idx == cstart {
        if c_idx < string.len() {
            string[c_idx] = b'0';
            c_idx += 1;
        }
    }
    if exp == 0 {
        if c_idx < string.len() {
            string[c_idx] = 0;
        }
        let len = string.iter().position(|&c| c == 0).unwrap_or(string.len());
        return std::str::from_utf8(&string[..len]).unwrap_or("");
    }
    let coeff_len = c_idx - cstart;
    let pre = coeff_len as i32 + exp;
    let need_exp = exp > 0 || pre < -5;
    let (final_exp, pre) = if need_exp { (pre - 1, 1i32) } else { (0, pre) };
    if pre > 0 {
        let dotat = cstart + pre as usize;
        if dotat < c_idx {
            let mut i = c_idx;
            while i > dotat {
                if i < string.len() {
                    string[i] = string[i - 1];
                }
                i -= 1;
            }
            if dotat < string.len() {
                string[dotat] = b'.';
            }
            c_idx += 1;
        }
        if final_exp != 0 {
            if c_idx < string.len() {
                string[c_idx] = b'E';
                c_idx += 1;
            }
            if final_exp >= 0 {
                if c_idx < string.len() {
                    string[c_idx] = b'+';
                    c_idx += 1;
                }
            } else {
                if c_idx < string.len() {
                    string[c_idx] = b'-';
                    c_idx += 1;
                }
            }
            let exp_abs = final_exp.unsigned_abs();
            let exp_chars = &BIN2CHAR[exp_abs as usize];
            let sig = exp_chars[0] as usize;
            for i in (4 - sig)..4 {
                if c_idx < string.len() {
                    string[c_idx] = exp_chars[i];
                    c_idx += 1;
                }
            }
        }
    } else {
        let zeros_needed = (-pre) as usize;
        let new_len = 2 + zeros_needed + coeff_len;
        if cstart + new_len <= string.len() {
            for i in (0..coeff_len).rev() {
                string[cstart + 2 + zeros_needed + i] = string[cstart + i];
            }
        }
        if cstart < string.len() {
            string[cstart] = b'0';
        }
        if cstart + 1 < string.len() {
            string[cstart + 1] = b'.';
        }
        for i in 0..zeros_needed {
            if cstart + 2 + i < string.len() {
                string[cstart + 2 + i] = b'0';
            }
        }
        c_idx = cstart + new_len;
    }
    if c_idx < string.len() {
        string[c_idx] = 0;
    }
    let len = string.iter().position(|&c| c == 0).unwrap_or(string.len());
    std::str::from_utf8(&string[..len]).unwrap_or("")
}
/// Convert decNumber to decimal64
pub fn decimal64FromNumber<'a>(
    d64: &'a mut Decimal64,
    dn: &'a DecNumber,
    set: &'a mut DecContext,
) -> &'a mut Decimal64 {
    let mut status = 0u32;
    let mut targar = [0u32; 2];
    let ae = dn.exponent + dn.digits - 1;
    let dn_to_use: DecNumber;
    let dn_ref: &DecNumber;
    if dn.digits > DECIMAL64_PMAX || ae > DECIMAL64_EMAX || ae < DECIMAL64_EMIN {
        let mut dc = DecContext::default();
        decContextDefault(&mut dc, 64);
        dc.round = set.round;
        let mut dw = DecNumber::default();
        decNumberPlus(&mut dw, dn, &mut dc);
        dw.bits |= dn.bits & DECNEG;
        status = dc.status;
        dn_to_use = dw;
        dn_ref = &dn_to_use;
    } else {
        dn_ref = dn;
    }
    if dn_ref.bits & DECSPECIAL != 0 {
        if dn_ref.bits & DECINF != 0 {
            targar[1] = 0x78 << 24;
        } else {
            if dn_ref.lsu.first().copied().unwrap_or(0) != 0 || dn_ref.digits > 1 {
                if dn_ref.digits < DECIMAL64_PMAX {
                    decDigitsToDPD(dn_ref, &mut targar, 0);
                }
            }
            if dn_ref.bits & DECNAN != 0 {
                targar[1] |= 0x7c << 24;
            } else {
                targar[1] |= 0x7e << 24;
            }
        }
    } else {
        let is_zero = dn_ref.lsu.first().copied().unwrap_or(0) == 0 && dn_ref.digits == 1
            && (dn_ref.bits & DECSPECIAL) == 0;
        let (comb, exp) = if is_zero {
            let exp = if dn_ref.exponent < -DECIMAL64_BIAS {
                status |= DEC_CLAMPED;
                0u32
            } else {
                let e = (dn_ref.exponent + DECIMAL64_BIAS) as u32;
                let max_exp = (DECIMAL64_EMAX + DECIMAL64_BIAS - DECIMAL64_PMAX + 1)
                    as u32;
                if e > max_exp {
                    status |= DEC_CLAMPED;
                    max_exp
                } else {
                    e
                }
            };
            ((exp >> 5) & 0x18, exp)
        } else {
            let mut exp = (dn_ref.exponent + DECIMAL64_BIAS) as u32;
            let max_exp = (DECIMAL64_EMAX + DECIMAL64_BIAS - DECIMAL64_PMAX + 1) as u32;
            let pad = if exp > max_exp {
                let p = exp - max_exp;
                exp = max_exp;
                status |= DEC_CLAMPED;
                p as i32
            } else {
                0
            };
            if pad == 0 {
                let mut dpd = [0u32; 6];
                let mut d = dn_ref.digits;
                let mut i = 0;
                while d > 0 {
                    let unit = dn_ref.lsu.get(i).copied().unwrap_or(0);
                    dpd[i] = BIN2DPDx(unit) as u32;
                    d -= 3;
                    i += 1;
                }
                targar[0] = dpd[0];
                targar[0] |= dpd[1] << 10;
                targar[0] |= dpd[2] << 20;
                if dn_ref.digits > 6 {
                    targar[0] |= dpd[3] << 30;
                    targar[1] = dpd[3] >> 2;
                    targar[1] |= dpd[4] << 8;
                }
            } else {
                decDigitsToDPD(dn_ref, &mut targar, pad);
            }
            let msd = targar[1] >> 18;
            targar[1] &= 0x0003ffff;
            let comb = if msd >= 8 {
                0x18 | ((exp >> 7) & 0x06) | (msd & 0x01)
            } else {
                ((exp >> 5) & 0x18) | msd
            };
            (comb, exp)
        };
        targar[1] |= comb << 26;
        targar[1] |= (exp & 0xff) << 18;
    }
    if dn_ref.bits & DECNEG != 0 {
        targar[1] |= 0x80000000;
    }
    d64.bytes[0..4].copy_from_slice(&targar[0].to_le_bytes());
    d64.bytes[4..8].copy_from_slice(&targar[1].to_le_bytes());
    if status != 0 {
        decContextSetStatus(set, status);
    }
    d64
}
/// Convert a decimal64 to a decNumber
///
/// # Arguments
/// * `d64` - The decimal64 value to convert
/// * `dn` - The decNumber to fill with the converted value
///
/// # Returns
/// A mutable reference to the filled decNumber
pub fn decimal64ToNumber<'a>(
    d64: &'a Decimal64,
    dn: &'a mut DecNumber,
) -> &'a mut DecNumber {
    let mut sourar: [u32; 2] = [0; 2];
    sourar[0] = u32::from_be_bytes([
        d64.bytes[0],
        d64.bytes[1],
        d64.bytes[2],
        d64.bytes[3],
    ]);
    sourar[1] = u32::from_be_bytes([
        d64.bytes[4],
        d64.bytes[5],
        d64.bytes[6],
        d64.bytes[7],
    ]);
    let comb = ((sourar[1] >> 26) & 0x1f) as usize;
    decNumberZero(dn);
    if sourar[1] & DECIMAL64_SIGN != 0 {
        dn.bits = DECNEG;
    }
    let mut msd = COMBMSD[comb];
    let exp = COMBEXP[comb];
    if exp == 3 {
        if msd == 0 {
            dn.bits |= DECFLOAT_INF;
            return dn;
        } else if sourar[1] & 0x02000000 != 0 {
            dn.bits |= DECFLOAT_SNAN;
        } else {
            dn.bits |= DECFLOAT_QNAN;
        }
        msd = 0;
    } else {
        dn.exponent = ((exp << 8) + ((sourar[1] >> 18) & 0xff)) as i32 - DECIMAL64_BIAS;
    }
    sourar[1] &= 0x0003ffff;
    let need: i32;
    if msd != 0 {
        sourar[1] |= msd << 18;
        need = 6;
    } else {
        if sourar[1] == 0 {
            if sourar[0] == 0 {
                return dn;
            }
            need = if sourar[0] & 0xc0000000 != 0 { 4 } else { 3 };
        } else {
            need = if sourar[1] & 0x0003ff00 != 0 { 5 } else { 4 };
        }
    }
    decDigitsFromDPD(dn, &sourar, need);
    dn
}
/// Make a decimal64 canonical
///
/// # Arguments
/// * `result` - The result decimal64
/// * `d64` - The input decimal64
///
/// # Returns
/// A mutable reference to the result
pub fn decimal64Canonical<'a>(
    result: &'a mut Decimal64,
    d64: &'a Decimal64,
) -> &'a mut Decimal64 {
    let mut dn = DecNumber::default();
    decimal64ToNumber(d64, &mut dn);
    let mut set = DecContext::default();
    decimal64FromNumber(result, &dn, &mut set);
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decimal64_zero() {
        let mut d64 = Decimal64::default();
        let mut dn = DecNumber::default();
        let mut dc = DecContext::default();
        decContextDefault(&mut dc, 64);
        dn.digits = 1;
        dn.exponent = 0;
        dn.bits = 0;
        dn.lsu = vec![0];
        decimal64FromNumber(&mut d64, &dn, &mut dc);
        let mut buf = [0u8; 43];
        let s = decimal64ToString(&d64, &mut buf);
        assert_eq!(s, "0");
    }
    #[test]
    fn test_dpd_conversion() {
        for i in 0..1000u16 {
            let dpd = BIN2DPD[i as usize];
            let bin = DPD2BIN[dpd as usize];
            assert_eq!(bin, i, "DPD roundtrip failed for {}", i);
        }
    }
}
/// Sign bit mask
const DECIMAL64_SIGN: u32 = 0x80000000;
/// Infinity indicator
const DECFLOAT_INF: u8 = 0x40;
/// Signaling NaN indicator
const DECFLOAT_SNAN: u8 = 0x10;
/// Quiet NaN indicator
const DECFLOAT_QNAN: u8 = 0x20;
impl std::fmt::Debug for Decimal64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = [0u8; 43];
        let s = decimal64ToString(self, &mut buf);
        write!(f, "Decimal64({})", s)
    }
}
