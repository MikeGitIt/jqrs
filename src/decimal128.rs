//! Module: decimal128
//!
//! Contains 7 transpiled functions:
//! - decimal128FromNumber:5429766842425468467:./src/decNumber/decimal128.c
//! - decimal128ToString:4901615331538694744:./src/decNumber/decimal128.c
//! - decimal128IsCanonical:7333346391738271675:./src/decNumber/decimal128.c
//! - decimal128FromString:14220207272447941604:./src/decNumber/decimal128.c
//! - decimal128Canonical:7806737329668640789:./src/decNumber/decimal128.c
//! - decimal128ToEngString:13996357345695502760:./src/decNumber/decimal128.c
//! - decimal128ToNumber:13098759382835615561:./src/decNumber/decimal128.c
use crate::decnumber::{decNumberPlus, decNumberZero, decNumberToEngString, decNumberFromString};
use crate::types::DecNumber;
use crate::deccontext::{decContextDefault, decContextSetStatus};
use crate::types::{Decimal128, DecContext, Rounding};
const DECIMAL128_BIAS: i32 = 6176;
const DECIMAL128_EMAX: i32 = 6144;
const DECIMAL128_EMIN: i32 = -6143;
const DECIMAL128_PMAX: i32 = 34;
const COMBMSD: [u32; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9,
    8, 9, 0, 0,
];
const COMBEXP: [u32; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 1, 1,
    2, 2, 3, 3,
];
const DPD2BIN: [u16; 1024] = {
    let mut table = [0u16; 1024];
    let mut i = 0;
    while i < 1024 {
        let d0 = ((i >> 7) & 0x7) as u16;
        let d1 = ((i >> 4) & 0x7) as u16;
        let d2 = (i & 0xf) as u16;
        table[i] = d0 * 100 + d1 * 10 + d2;
        i += 1;
    }
    table
};
fn dpd_to_bin(dpd: u32) -> u32 {
    let p = dpd & 0x3ff;
    let mut d0: u32;
    let mut d1: u32;
    let mut d2: u32;
    let bit0 = (p >> 0) & 1;
    let bit1 = (p >> 1) & 1;
    let bit2 = (p >> 2) & 1;
    let bit3 = (p >> 3) & 1;
    let bit4 = (p >> 4) & 1;
    let bit5 = (p >> 5) & 1;
    let bit6 = (p >> 6) & 1;
    let bit7 = (p >> 7) & 1;
    let bit8 = (p >> 8) & 1;
    let bit9 = (p >> 9) & 1;
    if bit3 == 0 {
        d2 = bit2 | (bit1 << 1) | (bit0 << 2);
        if bit7 == 0 {
            d1 = bit6 | (bit5 << 1) | (bit4 << 2);
            d0 = bit9 | (bit8 << 1) | (0 << 2);
        } else {
            d1 = bit6 | (bit5 << 1) | (8);
            d0 = bit9 | (bit8 << 1) | (0 << 2);
        }
    } else {
        if bit7 == 0 {
            d2 = bit2 | (bit1 << 1) | (8);
            d1 = bit6 | (bit5 << 1) | (bit4 << 2);
            d0 = bit9 | (bit8 << 1) | (0 << 2);
        } else {
            d2 = bit2 | (bit1 << 1) | (8);
            d1 = bit6 | (bit5 << 1) | (8);
            d0 = bit9 | (bit8 << 1) | (8);
        }
    }
    d0 * 100 + d1 * 10 + d2
}
fn bin_to_char(bin: u32) -> [u8; 4] {
    let d0 = ((bin / 100) % 10) as u8;
    let d1 = ((bin / 10) % 10) as u8;
    let d2 = (bin % 10) as u8;
    let count = if bin >= 100 {
        3u8
    } else if bin >= 10 {
        2u8
    } else if bin > 0 {
        1u8
    } else {
        0u8
    };
    [count, b'0' + d0, b'0' + d1, b'0' + d2]
}
const DEC_CLAMPED: u32 = 0x00000400;
/// Sign bit for negative numbers
const DECNEG: u8 = 0x80;
/// Special bit for infinity
const DECINF: u8 = 0x40;
/// Quiet NaN bit
const DECNAN: u8 = 0x20;
/// Signaling NaN bit
const DECSNAN: u8 = 0x10;
const DECSPECIAL: u8 = DECINF | DECNAN | DECSNAN;
/// Helper to read u32 from bytes (little-endian architecture assumed)
fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
/// Helper to write u32 to bytes (little-endian architecture assumed)
fn write_u32_le(bytes: &mut [u8], offset: usize, value: u32) {
    let le_bytes = value.to_le_bytes();
    bytes[offset..offset + 4].copy_from_slice(&le_bytes);
}
/// Convert decimal128 to string representation
pub fn decimal128ToString<'a>(d128: &'a Decimal128, string: &'a mut String) -> &'a str {
    string.clear();
    let sourar: [u32; 4] = [
        read_u32_le(&d128.bytes, 0),
        read_u32_le(&d128.bytes, 4),
        read_u32_le(&d128.bytes, 8),
        read_u32_le(&d128.bytes, 12),
    ];
    if (sourar[3] as i32) < 0 {
        string.push('-');
    }
    let comb = (sourar[3] >> 26) & 0x1f;
    let mut msd = COMBMSD[comb as usize];
    let exp_type = COMBEXP[comb as usize];
    if exp_type == 3 {
        if msd == 0 {
            string.push_str("Infinity");
            return string.as_str();
        }
        if sourar[3] & 0x02000000 != 0 {
            string.push('s');
        }
        string.push_str("NaN");
        if sourar[0] == 0 && sourar[1] == 0 && sourar[2] == 0
            && (sourar[3] & 0x0003ffff) == 0
        {
            return string.as_str();
        }
        msd = 0;
    }
    let mut exp = if exp_type == 3 {
        0i32
    } else {
        ((exp_type << 12) + ((sourar[3] >> 14) & 0xfff)) as i32 - DECIMAL128_BIAS
    };
    let cstart = string.len();
    if msd != 0 {
        string.push((b'0' + msd as u8) as char);
    }
    let decode_declet = |
        sourar: &[u32; 4],
        dpd: u32,
        string: &mut String,
        cstart: usize|
    {
        let bin = dpd_to_bin(dpd);
        let chars = bin_to_char(bin);
        if string.len() != cstart {
            string.push((chars[1]) as char);
            string.push((chars[2]) as char);
            string.push((chars[3]) as char);
        } else if chars[0] > 0 {
            for i in (4 - chars[0] as usize)..4 {
                string.push(chars[i] as char);
            }
        }
    };
    let dpd = (sourar[3] >> 4) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = ((sourar[3] & 0xf) << 6) | (sourar[2] >> 26);
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = (sourar[2] >> 16) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = (sourar[2] >> 6) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = ((sourar[2] & 0x3f) << 4) | (sourar[1] >> 28);
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = (sourar[1] >> 18) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = (sourar[1] >> 8) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = ((sourar[1] & 0xff) << 2) | (sourar[0] >> 30);
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = (sourar[0] >> 20) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = (sourar[0] >> 10) & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    let dpd = sourar[0] & 0x3ff;
    decode_declet(&sourar, dpd, string, cstart);
    if string.len() == cstart {
        string.push('0');
    }
    if exp == 0 {
        return string.as_str();
    }
    let coeff_len = string.len() - cstart;
    let pre = coeff_len as i32 + exp;
    let mut e = 0i32;
    let adjusted_pre;
    if exp > 0 || pre < -5 {
        e = pre - 1;
        adjusted_pre = 1;
    } else {
        adjusted_pre = pre;
    }
    if adjusted_pre > 0 {
        let dot_pos = cstart + adjusted_pre as usize;
        if dot_pos < string.len() {
            let suffix: String = string[dot_pos..].to_string();
            string.truncate(dot_pos);
            string.push('.');
            string.push_str(&suffix);
        }
        if e != 0 {
            string.push('E');
            if e >= 0 {
                string.push('+');
            }
            string.push_str(&e.to_string());
        }
    } else {
        let digits: String = string[cstart..].to_string();
        string.truncate(cstart);
        string.push_str("0.");
        for _ in 0..(-adjusted_pre) {
            string.push('0');
        }
        string.push_str(&digits);
    }
    string.as_str()
}
/// Convert decNumber to decimal128
pub fn decimal128FromNumber<'a>(
    d128: &'a mut Decimal128,
    dn: &'a DecNumber,
    set: &'a mut DecContext,
) -> &'a mut Decimal128 {
    let mut status = 0u32;
    let mut working_dn;
    let dn_ref: &DecNumber;
    let ae = dn.exponent + dn.digits - 1;
    if dn.digits > DECIMAL128_PMAX || ae > DECIMAL128_EMAX || ae < DECIMAL128_EMIN {
        let mut dc = DecContext::default();
        decContextDefault(&mut dc, 128);
        dc.round = set.round.clone();
        working_dn = DecNumber::default();
        decNumberPlus(&mut working_dn, dn, &mut dc);
        working_dn.bits |= dn.bits & DECNEG;
        status = dc.status;
        dn_ref = &working_dn;
    } else {
        dn_ref = dn;
    }
    let mut targar: [u32; 4] = [0, 0, 0, 0];
    if dn_ref.bits & DECSPECIAL != 0 {
        if dn_ref.bits & DECINF != 0 {
            targar[3] = 0x78 << 24;
        } else {
            if !dn_ref.lsu.is_empty() && (dn_ref.lsu[0] != 0 || dn_ref.digits > 1)
                && dn_ref.digits < DECIMAL128_PMAX
            {
                encode_coefficient(dn_ref, &mut targar, 0);
            }
            if dn_ref.bits & DECNAN != 0 {
                targar[3] |= 0x7c << 24;
            } else {
                targar[3] |= 0x7e << 24;
            }
        }
    } else {
        let comb: u32;
        let exp: u32;
        let is_zero = dn_ref.lsu.is_empty()
            || (dn_ref.lsu[0] == 0 && dn_ref.digits == 1);
        if is_zero {
            if dn_ref.exponent < -DECIMAL128_BIAS {
                exp = 0;
                status |= DEC_CLAMPED;
            } else {
                let mut e = (dn_ref.exponent + DECIMAL128_BIAS) as u32;
                if e > (DECIMAL128_EMAX + DECIMAL128_BIAS - DECIMAL128_PMAX + 1) as u32 {
                    e = (DECIMAL128_EMAX + DECIMAL128_BIAS - DECIMAL128_PMAX + 1) as u32;
                    status |= DEC_CLAMPED;
                }
                exp = e;
            }
            comb = (exp >> 9) & 0x18;
        } else {
            let mut e = (dn_ref.exponent + DECIMAL128_BIAS) as u32;
            let mut pad = 0i32;
            if e > (DECIMAL128_EMAX + DECIMAL128_BIAS - DECIMAL128_PMAX + 1) as u32 {
                pad = (e
                    - (DECIMAL128_EMAX + DECIMAL128_BIAS - DECIMAL128_PMAX + 1) as u32)
                    as i32;
                e = (DECIMAL128_EMAX + DECIMAL128_BIAS - DECIMAL128_PMAX + 1) as u32;
                status |= DEC_CLAMPED;
            }
            exp = e;
            encode_coefficient(dn_ref, &mut targar, pad);
            let msd = targar[3] >> 14;
            targar[3] &= 0x00003fff;
            if msd >= 8 {
                comb = 0x18 | ((exp >> 11) & 0x06) | (msd & 0x01);
            } else {
                comb = ((exp >> 9) & 0x18) | msd;
            }
        }
        targar[3] |= comb << 26;
        targar[3] |= (exp & 0xfff) << 14;
    }
    if dn_ref.bits & DECNEG != 0 {
        targar[3] |= 0x80000000;
    }
    write_u32_le(&mut d128.bytes, 0, targar[0]);
    write_u32_le(&mut d128.bytes, 4, targar[1]);
    write_u32_le(&mut d128.bytes, 8, targar[2]);
    write_u32_le(&mut d128.bytes, 12, targar[3]);
    if status != 0 {
        decContextSetStatus(set, status);
    }
    d128
}
/// Encode coefficient from decNumber to DPD format
fn encode_coefficient(dn: &DecNumber, targar: &mut [u32; 4], pad: i32) {
    let mut digits = Vec::with_capacity(DECIMAL128_PMAX as usize);
    for &unit in &dn.lsu {
        let mut u = unit;
        for _ in 0..3 {
            digits.push((u % 10) as u8);
            u /= 10;
            if digits.len() >= dn.digits as usize {
                break;
            }
        }
        if digits.len() >= dn.digits as usize {
            break;
        }
    }
    for _ in 0..pad {
        digits.insert(0, 0);
    }
    while digits.len() < DECIMAL128_PMAX as usize {
        digits.push(0);
    }
    digits.reverse();
    let msd = digits[0] as u32;
    let encode_dpd = |d0: u8, d1: u8, d2: u8| -> u32 {
        let d0 = d0 as u32;
        let d1 = d1 as u32;
        let d2 = d2 as u32;
        if d0 < 8 && d1 < 8 && d2 < 8 {
            (d0 << 7) | (d1 << 4) | d2
        } else {
            let mut result = 0u32;
            if d0 >= 8 {
                result |= 0x200;
            }
            if d1 >= 8 {
                result |= 0x80;
            }
            if d2 >= 8 {
                result |= 0x08;
            }
            result |= (d0 & 0x01) << 8;
            result |= (d1 & 0x07) << 4;
            result |= d2 & 0x07;
            result
        }
    };
    let dpd1 = encode_dpd(digits[1], digits[2], digits[3]);
    let dpd2 = encode_dpd(digits[4], digits[5], digits[6]);
    targar[3] |= msd << 14;
    targar[3] |= dpd1 << 4;
    targar[3] |= (dpd2 >> 6) & 0xf;
    let dpd3 = encode_dpd(digits[7], digits[8], digits[9]);
    let dpd4 = encode_dpd(digits[10], digits[11], digits[12]);
    let dpd5 = encode_dpd(digits[13], digits[14], digits[15]);
    targar[2] |= (dpd2 & 0x3f) << 26;
    targar[2] |= dpd3 << 16;
    targar[2] |= dpd4 << 6;
    targar[2] |= (dpd5 >> 4) & 0x3f;
    let dpd6 = encode_dpd(digits[16], digits[17], digits[18]);
    let dpd7 = encode_dpd(digits[19], digits[20], digits[21]);
    let dpd8 = encode_dpd(digits[22], digits[23], digits[24]);
    targar[1] |= (dpd5 & 0xf) << 28;
    targar[1] |= dpd6 << 18;
    targar[1] |= dpd7 << 8;
    targar[1] |= (dpd8 >> 2) & 0xff;
    let dpd9 = encode_dpd(digits[25], digits[26], digits[27]);
    let dpd10 = encode_dpd(digits[28], digits[29], digits[30]);
    let dpd11 = encode_dpd(digits[31], digits[32], digits[33]);
    targar[0] |= (dpd8 & 0x3) << 30;
    targar[0] |= dpd9 << 20;
    targar[0] |= dpd10 << 10;
    targar[0] |= dpd11;
}
/// Convert decimal128 to decNumber
pub fn decimal128ToNumber<'a>(
    d128: &'a Decimal128,
    dn: &'a mut DecNumber,
) -> &'a mut DecNumber {
    let sourar: [u32; 4] = [
        read_u32_le(&d128.bytes, 0),
        read_u32_le(&d128.bytes, 4),
        read_u32_le(&d128.bytes, 8),
        read_u32_le(&d128.bytes, 12),
    ];
    decNumberZero(dn);
    let comb = (sourar[3] >> 26) & 0x1f;
    let msd = COMBMSD[comb as usize];
    let exp_type = COMBEXP[comb as usize];
    if (sourar[3] as i32) < 0 {
        dn.bits |= DECNEG;
    }
    if exp_type == 3 {
        if msd == 0 {
            dn.bits |= DECINF;
        } else if sourar[3] & 0x02000000 != 0 {
            dn.bits |= DECSNAN;
        } else {
            dn.bits |= DECNAN;
        }
        return dn;
    }
    dn.exponent = ((exp_type << 12) + ((sourar[3] >> 14) & 0xfff)) as i32
        - DECIMAL128_BIAS;
    let mut digits = Vec::with_capacity(DECIMAL128_PMAX as usize);
    digits.push(msd as u8);
    let decode_declet = |dpd: u32| -> (u8, u8, u8) {
        let bin = dpd_to_bin(dpd);
        let d0 = ((bin / 100) % 10) as u8;
        let d1 = ((bin / 10) % 10) as u8;
        let d2 = (bin % 10) as u8;
        (d0, d1, d2)
    };
    let dpd = (sourar[3] >> 4) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = ((sourar[3] & 0xf) << 6) | (sourar[2] >> 26);
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = (sourar[2] >> 16) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = (sourar[2] >> 6) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = ((sourar[2] & 0x3f) << 4) | (sourar[1] >> 28);
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = (sourar[1] >> 18) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = (sourar[1] >> 8) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = ((sourar[1] & 0xff) << 2) | (sourar[0] >> 30);
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = (sourar[0] >> 20) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = (sourar[0] >> 10) & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let dpd = sourar[0] & 0x3ff;
    let (d0, d1, d2) = decode_declet(dpd);
    digits.extend_from_slice(&[d0, d1, d2]);
    let first_nonzero = digits.iter().position(|&d| d != 0).unwrap_or(digits.len() - 1);
    let significant_digits = &digits[first_nonzero..];
    dn.digits = significant_digits.len() as i32;
    dn.lsu.clear();
    let mut idx = significant_digits.len();
    while idx > 0 {
        let mut unit = 0u16;
        let mut mult = 1u16;
        for _ in 0..3 {
            if idx > 0 {
                idx -= 1;
                unit += significant_digits[idx] as u16 * mult;
                mult *= 10;
            }
        }
        dn.lsu.push(unit);
    }
    if dn.lsu.is_empty() {
        dn.lsu.push(0);
        dn.digits = 1;
    }
    dn
}
/// Convert decimal128 to engineering string representation
pub fn decimal128ToEngString<'a>(
    d128: &'a Decimal128,
    string: &'a mut String,
) -> &'a mut String {
    let mut dn = DecNumber::default();
    decimal128ToNumber(d128, &mut dn);
    let result = decNumberToEngString(&dn);
    string.clear();
    string.push_str(&result);
    string
}
/// Convert string to decimal128
pub fn decimal128FromString<'a>(
    result: &'a mut Decimal128,
    string: &'a str,
    set: &'a mut DecContext,
) -> &'a mut Decimal128 {
    let mut dn = DecNumber::default();
    let trimmed = string.trim();
    if trimmed.eq_ignore_ascii_case("infinity") || trimmed.eq_ignore_ascii_case("inf") {
        dn.bits |= DECINF;
    } else if trimmed.eq_ignore_ascii_case("-infinity")
        || trimmed.eq_ignore_ascii_case("-inf")
    {
        dn.bits |= DECINF | DECNEG;
    } else if trimmed.eq_ignore_ascii_case("nan") {
        dn.bits |= DECNAN;
    } else if trimmed.eq_ignore_ascii_case("snan") {
        dn.bits |= DECSNAN;
    } else {
        let (sign, rest) = if trimmed.starts_with('-') {
            (true, &trimmed[1..])
        } else if trimmed.starts_with('+') {
            (false, &trimmed[1..])
        } else {
            (false, trimmed)
        };
        if sign {
            dn.bits |= DECNEG;
        }
        let (mantissa, exp_str) = if let Some(e_pos) = rest.to_lowercase().find('e') {
            (&rest[..e_pos], Some(&rest[e_pos + 1..]))
        } else {
            (rest, None)
        };
        let (integer_part, frac_part) = if let Some(dot_pos) = mantissa.find('.') {
            (&mantissa[..dot_pos], Some(&mantissa[dot_pos + 1..]))
        } else {
            (mantissa, None)
        };
        let mut digits: Vec<u8> = Vec::new();
        for c in integer_part.chars() {
            if c.is_ascii_digit() {
                digits.push(c as u8 - b'0');
            }
        }
        let frac_digits = if let Some(frac) = frac_part {
            let mut frac_digits = Vec::new();
            for c in frac.chars() {
                if c.is_ascii_digit() {
                    frac_digits.push(c as u8 - b'0');
                }
            }
            frac_digits
        } else {
            Vec::new()
        };
        digits.extend(frac_digits.iter());
        let mut exp = if let Some(e) = exp_str {
            e.parse::<i32>().unwrap_or(0)
        } else {
            0
        };
        if let Some(frac) = frac_part {
            exp -= frac.len() as i32;
        }
        while digits.len() > 1 && digits[0] == 0 {
            digits.remove(0);
        }
        dn.digits = digits.len() as i32;
        dn.exponent = exp;
        dn.lsu.clear();
        let mut idx = digits.len();
        while idx > 0 {
            let mut unit = 0u16;
            let mut mult = 1u16;
            for _ in 0..3 {
                if idx > 0 {
                    idx -= 1;
                    unit += digits[idx] as u16 * mult;
                    mult *= 10;
                }
            }
            dn.lsu.push(unit);
        }
        if dn.lsu.is_empty() {
            dn.lsu.push(0);
            dn.digits = 1;
        }
    }
    // Call decimal128FromNumber which modifies result in place
    // We need to reborrow result after the call since we can't return
    // a reference tied to the local dn's lifetime
    let _ = decimal128FromNumber(result, &dn, set);
    result
}
/// Check if decimal128 is canonical
pub fn decimal128IsCanonical(d128: &Decimal128) -> u32 {
    let mut dn = DecNumber::default();
    let mut canon = Decimal128::default();
    let mut dc = DecContext::default();
    decContextDefault(&mut dc, 128);
    decimal128ToNumber(d128, &mut dn);
    decimal128FromNumber(&mut canon, &dn, &mut dc);
    if d128.bytes == canon.bytes { 1 } else { 0 }
}
/// Canonicalize a decimal128 value
pub fn decimal128Canonical<'a>(
    result: &'a mut Decimal128,
    d128: &'a Decimal128,
) -> &'a mut Decimal128 {
    let mut dn = DecNumber::default();
    let mut dc = DecContext::default();
    decContextDefault(&mut dc, 128);
    decimal128ToNumber(d128, &mut dn);
    decimal128FromNumber(result, &dn, &mut dc);
    result
}
const DECIMAL128_BYTES: usize = 16;
const DECIMAL128_STRING: usize = 43;
const DECIMAL128_EHIGH: i32 = DECIMAL128_EMAX + DECIMAL128_BIAS - (DECIMAL128_PMAX - 1);
const BIN2DPD: [u16; 1000] = {
    let mut table = [0u16; 1000];
    let mut i = 0u16;
    while i < 1000 {
        let d0 = i / 100;
        let d1 = (i / 10) % 10;
        let d2 = i % 10;
        table[i as usize] = ((d0 & 0x7) << 7) | ((d1 & 0x7) << 4) | (d2 & 0xf);
        i += 1;
    }
    table
};
/// Helper function to calculate how many declets are needed
fn calculate_digits_needed(sourar: &[u32; 4], msd: u32) -> i32 {
    if msd != 0 {
        return 12;
    }
    if sourar[3] != 0 {
        return 11;
    }
    if sourar[2] != 0 {
        return 10;
    }
    if sourar[1] != 0 {
        return 7;
    }
    if sourar[0] != 0 {
        return 4;
    }
    0
}
/// Extract digits from DPD encoding into decNumber
fn decDigitsFromDPD(dn: &mut DecNumber, sourar: &[u32; 4], declets: i32) {
    let mut digits = Vec::with_capacity((declets * 3) as usize);
    for i in 0..declets {
        let shift = (i % 3) * 10;
        let word_idx = (i / 3) as usize;
        let dpd = if word_idx < 4 {
            ((sourar[word_idx] >> shift) & 0x3ff) as usize
        } else {
            0
        };
        let bin = dpd2bin(dpd as u16);
        digits.push((bin / 100) as u8);
        digits.push(((bin / 10) % 10) as u8);
        digits.push((bin % 10) as u8);
    }
    let mut start = 0;
    while start < digits.len() && digits[start] == 0 {
        start += 1;
    }
    if start >= digits.len() {
        dn.digits = 1;
        dn.lsu = vec![0];
        return;
    }
    let significant_digits: Vec<u8> = digits[start..].to_vec();
    dn.digits = significant_digits.len() as i32;
    let mut lsu = Vec::new();
    let mut i = significant_digits.len();
    while i > 0 {
        let end = i;
        let start = if i >= 3 { i - 3 } else { 0 };
        let mut val: u16 = 0;
        for j in start..end {
            val = val * 10 + significant_digits[j] as u16;
        }
        lsu.push(val);
        i = start;
    }
    dn.lsu = lsu;
}
/// Convert DPD to binary (3 decimal digits)
fn dpd2bin(dpd: u16) -> u16 {
    let p = dpd & 0x7;
    let q = (dpd >> 3) & 0x7;
    let r = (dpd >> 6) & 0x7;
    let s = (dpd >> 9) & 0x1;
    let d2: u16;
    let d1: u16;
    let d0: u16;
    let v = (dpd >> 3) & 0x1;
    let w = (dpd >> 4) & 0x1;
    let x = (dpd >> 5) & 0x1;
    if (dpd & 0x8) == 0 {
        d2 = r;
        d1 = q;
        d0 = p;
    } else if (dpd & 0x8) != 0 && (dpd & 0x40) == 0 {
        d2 = r;
        d1 = q;
        d0 = 8 | p;
    } else {
        d2 = (dpd >> 7) & 0x7;
        d1 = (dpd >> 4) & 0x7;
        d0 = dpd & 0x7;
    }
    d2 * 100 + d1 * 10 + d0
}
/// Get the most significant digit
fn get_msd(dn: &DecNumber) -> u32 {
    if dn.lsu.is_empty() {
        return 0;
    }
    let mut val = 0u64;
    for &unit in dn.lsu.iter().rev() {
        val = val * 1000 + unit as u64;
    }
    if val == 0 {
        return 0;
    }
    while val >= 10 {
        val /= 10;
    }
    val as u32
}
/// Convert binary (3 decimal digits) to DPD
fn bin2dpd(bin: u16) -> u16 {
    let d0 = (bin / 100) % 10;
    let d1 = (bin / 10) % 10;
    let d2 = bin % 10;
    if d0 < 8 && d1 < 8 && d2 < 8 {
        (d0 << 7) | (d1 << 4) | d2
    } else {
        let mut dpd: u16 = 0;
        if d0 >= 8 && d1 >= 8 && d2 >= 8 {
            dpd = 0x6e | ((d0 & 1) << 7) | ((d1 & 1) << 4) | (d2 & 1);
        } else if d0 >= 8 && d1 >= 8 {
            dpd = 0x6e | ((d0 & 1) << 7) | ((d1 & 1) << 4) | d2;
        } else if d0 >= 8 && d2 >= 8 {
            dpd = 0x4e | ((d0 & 1) << 7) | (d1 << 4) | (d2 & 1);
        } else if d1 >= 8 && d2 >= 8 {
            dpd = 0x2e | (d0 << 7) | ((d1 & 1) << 4) | (d2 & 1);
        } else if d0 >= 8 {
            dpd = 0x0e | ((d0 & 1) << 7) | (d1 << 4) | d2;
        } else if d1 >= 8 {
            dpd = 0x0a | (d0 << 7) | ((d1 & 1) << 4) | d2;
        } else if d2 >= 8 {
            dpd = 0x08 | (d0 << 7) | (d1 << 4) | (d2 & 1);
        }
        dpd
    }
}
/// Write targar array to decimal128 bytes
fn write_decimal128(d128: &mut Decimal128, targar: &[u32; 4]) {
    let bytes0 = targar[0].to_le_bytes();
    let bytes1 = targar[1].to_le_bytes();
    let bytes2 = targar[2].to_le_bytes();
    let bytes3 = targar[3].to_le_bytes();
    d128.bytes[0..4].copy_from_slice(&bytes0);
    d128.bytes[4..8].copy_from_slice(&bytes1);
    d128.bytes[8..12].copy_from_slice(&bytes2);
    d128.bytes[12..16].copy_from_slice(&bytes3);
}
impl Decimal128 {
    /// Create a new zero decimal128
    pub fn new() -> Self {
        Decimal128 { bytes: [0; 16] }
    }
    /// Create from raw bytes
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Decimal128 { bytes }
    }
    /// Get raw bytes
    pub fn to_bytes(&self) -> [u8; 16] {
        self.bytes
    }
}
impl Default for Decimal128 {
    fn default() -> Self {
        Decimal128 { bytes: [0; 16] }
    }
}
