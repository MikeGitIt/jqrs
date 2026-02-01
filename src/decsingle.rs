//! Module: decsingle
//!
//! Contains 20 transpiled functions:
//! - decSingleShow:7335899324020232498:./src/decNumber/decSingle.c
//! - decSingleFromPacked:2760608566214198675:./src/decNumber/decSingle.c
//! - decSingleFromBCD:14863223581950522146:./src/decNumber/decSingle.c
//! - decSingleToEngString:15635844755682014439:./src/decNumber/decSingle.c
//! - decSingleFromPackedChecked:13606121005238398989:./src/decNumber/decSingle.c
//! - decBiStr:2508719278762932362:./src/decNumber/decSingle.c
//! - decSingleFromWider:7228028510653631282:./src/decNumber/decSingle.c
//! - decSingleRadix:4333875330472234014:./src/decNumber/decSingle.c
//! - decSingleFromString:10349688442288931563:./src/decNumber/decSingle.c
//! - decSingleZero:17965690611028358415:./src/decNumber/decSingle.c
//! - decSingleGetExponent:10413000853104488183:./src/decNumber/decSingle.c
//! - decSingleToBCD:12492793819603558322:./src/decNumber/decSingle.c
//! - decSingleVersion:14225568325396865313:./src/decNumber/decSingle.c
//! - decSingleToString:11116252373133408450:./src/decNumber/decSingle.c
//! - decFinalize:52824653885468409:./src/decNumber/decSingle.c
//! - decSingleToWider:9032044823759457257:./src/decNumber/decSingle.c
//! - decSingleGetCoefficient:6249883842161221952:./src/decNumber/decSingle.c
//! - decSingleSetExponent:9401771462469309140:./src/decNumber/decSingle.c
//! - decSingleSetCoefficient:10278393480053143803:./src/decNumber/decSingle.c
//! - decSingleToPacked:9653188497827703353:./src/decNumber/decSingle.c

// Note: All decSingle* functions are defined locally in this file
use std::fmt;
use crate::types::*;
const DECSINGLE_PMAX: i32 = 7;
const DECSINGLE_EMAX: i32 = 96;
const DECSINGLE_EMIN: i32 = -95;
const DECSINGLE_BIAS: i32 = 101;
const DECSINGLE_STRING: usize = 16;
const DECFLOAT_INF: i32 = 0x78000000_u32 as i32;
const DECFLOAT_QNAN: i32 = 0x7c000000_u32 as i32;
const DECFLOAT_SNAN: i32 = 0x7e000000_u32 as i32;
const DEC_INEXACT: u32 = 0x00000020;
const DEC_UNDERFLOW: u32 = 0x00002000;
const DEC_OVERFLOW: u32 = 0x00000200;
const DEC_INVALID_OPERATION: u32 = 0x00000040;
const DEC_ROUND_HALF_EVEN: Rounding = Rounding::HalfEven;
const DEC_ROUND_DOWN: Rounding = Rounding::Down;
const DEC_ROUND_HALF_DOWN: Rounding = Rounding::HalfDown;
const DEC_ROUND_HALF_UP: Rounding = Rounding::HalfUp;
const DEC_ROUND_UP: Rounding = Rounding::Up;
const DEC_ROUND_CEILING: Rounding = Rounding::Ceiling;
const DEC_ROUND_FLOOR: Rounding = Rounding::Floor;
const DEC_ROUND_05UP: Rounding = Rounding::ZeroFiveUp;
/// Combination field to exponent high bits mapping
static DECCOMBEXP: [i32; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    0,
    0,
    1,
    1,
    2,
    2,
    DECFLOAT_INF,
    DECFLOAT_QNAN,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    0,
    0,
    1,
    1,
    2,
    2,
    DECFLOAT_INF,
    DECFLOAT_QNAN,
];
/// Combination field to MSD mapping (64 entries)
static DECCOMBMSD: [u8; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9,
    8, 9, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 8, 9, 8, 9, 0, 0,
];
/// Combination field encoding table (MSD + exp high bits -> combination)
/// Indexed by expTopTwoBits*16 + msd (0-47)
static DECCOMBFROM: [u32; 48] = [
    // exp_high=0: indices 0-9 (msd 0-9), then 10-15 unused
    0x00000000, 0x04000000, 0x08000000, 0x0C000000, 0x10000000, 0x14000000,
    0x18000000, 0x1C000000, 0x60000000, 0x64000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    // exp_high=1: indices 16-25 (msd 0-9), then 26-31 unused
    0x20000000, 0x24000000, 0x28000000, 0x2C000000, 0x30000000, 0x34000000,
    0x38000000, 0x3C000000, 0x68000000, 0x6C000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    // exp_high=2: indices 32-41 (msd 0-9), then 42-47 unused
    0x40000000, 0x44000000, 0x48000000, 0x4C000000, 0x50000000, 0x54000000,
    0x58000000, 0x5C000000, 0x70000000, 0x74000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
/// Proper DPD encoding for a single declet (3 BCD digits -> 10-bit DPD)
const fn bin_to_dpd(d2: u8, d1: u8, d0: u8) -> u32 {
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
    let (p, q, r, s, t, u, v, w, x, y): (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    if a == 0 && e == 0 && i == 0 {
        p = b as u32; q = c as u32; r = d as u32;
        s = f as u32; t = g as u32; u = h as u32;
        v = 0; w = j as u32; x = k as u32; y = m as u32;
    } else if a == 0 && e == 0 && i == 1 {
        p = b as u32; q = c as u32; r = d as u32;
        s = f as u32; t = g as u32; u = h as u32;
        v = 1; w = 0; x = 0; y = m as u32;
    } else if a == 0 && e == 1 && i == 0 {
        p = b as u32; q = c as u32; r = d as u32;
        s = j as u32; t = k as u32; u = h as u32;
        v = 1; w = 0; x = 1; y = m as u32;
    } else if a == 0 && e == 1 && i == 1 {
        p = b as u32; q = c as u32; r = d as u32;
        s = 0; t = 0; u = h as u32;
        v = 1; w = 1; x = 1; y = m as u32;
    } else if a == 1 && e == 0 && i == 0 {
        p = j as u32; q = k as u32; r = d as u32;
        s = f as u32; t = g as u32; u = h as u32;
        v = 1; w = 1; x = 0; y = m as u32;
    } else if a == 1 && e == 0 && i == 1 {
        p = f as u32; q = g as u32; r = d as u32;
        s = 0; t = 1; u = h as u32;
        v = 1; w = 1; x = 1; y = m as u32;
    } else if a == 1 && e == 1 && i == 0 {
        p = j as u32; q = k as u32; r = d as u32;
        s = 1; t = 0; u = h as u32;
        v = 1; w = 1; x = 1; y = m as u32;
    } else {
        p = 0; q = 0; r = d as u32;
        s = 1; t = 1; u = h as u32;
        v = 1; w = 1; x = 1; y = m as u32;
    }
    (p << 9) | (q << 8) | (r << 7) | (s << 6) | (t << 5) | (u << 4) | (v << 3) | (w << 2) | (x << 1) | y
}
static BCD2DPD: [u32; 1000] = {
    let mut table = [0u32; 1000];
    let mut i = 0usize;
    while i < 1000 {
        let d0 = (i % 10) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = (i / 100) as u8;
        table[i] = bin_to_dpd(d2, d1, d0);
        i += 1;
    }
    table
};
static DPD2BCD8: [[u8; 4]; 1024] = {
    let mut table = [[0u8; 4]; 1024];
    let mut i = 0;
    while i < 1024 {
        let d0 = ((i >> 7) & 0x7) as u8;
        let d1 = ((i >> 4) & 0x7) as u8;
        let d2 = (i & 0xF) as u8;
        table[i] = [d0, d1, d2, 3];
        i += 1;
    }
    table
};
static BIN2BCD8: [[u8; 4]; 1000] = {
    let mut table = [[0u8; 4]; 1000];
    let mut i = 0;
    while i < 1000 {
        let d0 = (i / 100) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = (i % 10) as u8;
        let count = if i >= 100 { 3 } else if i >= 10 { 2 } else { 1 };
        table[i] = [d0, d1, d2, count];
        i += 1;
    }
    table
};
/// Sticky digit table for rounding
static DECSTICKYTAB: [u8; 10] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static ALL_NINES: [u8; 7] = [9, 9, 9, 9, 9, 9, 9];
/// Check if exponent indicates special value (Inf/NaN)
#[inline]
fn is_special(exp: i32) -> bool {
    exp >= 0x78000000_u32 as i32
}
/// Display a decSingle value with tag for debugging
pub fn decSingleShow(df: &DecSingle, tag: &str) {
    let mut hexbuf = String::with_capacity(13);
    for i in 0..4 {
        hexbuf.push_str(&format!("{:02x}", df.bytes[4 - 1 - i]));
        if (i + 1) % 4 == 0 {
            hexbuf.push(' ');
        }
    }
    let mut buff = [0u8; 16];
    decSingleToString(df, &mut buff);
    let buff_str = std::str::from_utf8(&buff).unwrap_or("").trim_end_matches('\0');
    println!(">{tag}> {hexbuf} [big-endian]  {buff_str}");
}
/// Compare target string against two alternative strings (case-insensitive match)
fn decBiStr(targ: &str, str1: &str, str2: &str) -> bool {
    let targ_bytes = targ.as_bytes();
    let str1_bytes = str1.as_bytes();
    let str2_bytes = str2.as_bytes();
    if targ_bytes.len() != str1_bytes.len() || targ_bytes.len() != str2_bytes.len() {
        return false;
    }
    for i in 0..targ_bytes.len() {
        if targ_bytes[i] != str1_bytes[i] && targ_bytes[i] != str2_bytes[i] {
            return false;
        }
    }
    true
}
/// Return the version string for decSingle
pub fn decSingleVersion() -> &'static str {
    "decNumber 3.68"
}
/// Return the radix (always 10 for decimal)
pub fn decSingleRadix(_df: &DecSingle) -> u32 {
    10
}
/// Finalize a BCD number into a decSingle
pub fn decFinalize<'a>(
    df: &'a mut DecSingle,
    num: &mut BcdNumInternal,
    set: &mut DecContext,
) -> &'a mut DecSingle {
    let mut umsd_idx = num.msd_idx;
    let mut ulsd_idx = num.lsd_idx;
    let mut length = (ulsd_idx - umsd_idx + 1) as i32;
    if !is_special(num.exponent) {
        while umsd_idx < ulsd_idx && num.data[umsd_idx] == 0 {
            umsd_idx += 1;
        }
        length = (ulsd_idx - umsd_idx + 1) as i32;
        let drop = std::cmp::max(
            (length - DECSINGLE_PMAX).max((-DECSINGLE_BIAS) - num.exponent),
            0,
        );
        if drop > 0 {
            num.exponent += drop;
            let (roundat_idx, mut reround) = if drop < length {
                let roundat = umsd_idx + (length - drop) as usize;
                let mut rr = num.data[roundat];
                for ub in (roundat + 1)..=ulsd_idx {
                    if num.data[ub] != 0 {
                        rr = DECSTICKYTAB[rr as usize];
                        break;
                    }
                }
                ulsd_idx = roundat - 1;
                (Some(roundat), rr)
            } else if drop == length {
                let roundat = umsd_idx;
                let mut rr = num.data[roundat];
                for ub in (roundat + 1)..=ulsd_idx {
                    if num.data[ub] != 0 {
                        rr = DECSTICKYTAB[rr as usize];
                        break;
                    }
                }
                num.data[umsd_idx] = 0;
                ulsd_idx = umsd_idx;
                (Some(roundat), rr)
            } else {
                let mut rr = 0u8;
                for ub in umsd_idx..=ulsd_idx {
                    if num.data[ub] != 0 {
                        rr = DECSTICKYTAB[rr as usize];
                        break;
                    }
                }
                num.data[umsd_idx] = 0;
                ulsd_idx = umsd_idx;
                (None, rr)
            };
            if reround != 0 {
                let mut bump = 0u32;
                set.status |= DEC_INEXACT;
                if num.exponent < -DECSINGLE_EMIN
                    && (num.exponent + (ulsd_idx - umsd_idx + 1) as i32 - 1)
                        < -DECSINGLE_EMIN
                {
                    set.status |= DEC_UNDERFLOW;
                }
                match set.round {
                    DEC_ROUND_HALF_EVEN => {
                        if reround > 5 {
                            bump = 1;
                        } else if reround == 5 {
                            bump = (num.data[ulsd_idx] & 0x01) as u32;
                        }
                    }
                    DEC_ROUND_DOWN => {}
                    DEC_ROUND_HALF_DOWN => {
                        if reround > 5 {
                            bump = 1;
                        }
                    }
                    DEC_ROUND_HALF_UP => {
                        if reround >= 5 {
                            bump = 1;
                        }
                    }
                    DEC_ROUND_UP => {
                        if reround > 0 {
                            bump = 1;
                        }
                    }
                    DEC_ROUND_CEILING => {
                        if num.sign == 0 && reround > 0 {
                            bump = 1;
                        }
                    }
                    DEC_ROUND_FLOOR => {
                        if num.sign != 0 && reround > 0 {
                            bump = 1;
                        }
                    }
                    DEC_ROUND_05UP => {
                        if reround > 0 {
                            let last = num.data[ulsd_idx];
                            if last == 0 || last == 5 {
                                bump = 1;
                            }
                        }
                    }
                    _ => {
                        set.status |= DEC_INVALID_OPERATION;
                    }
                }
                if bump != 0 {
                    let mut ub = ulsd_idx;
                    while ub >= umsd_idx {
                        if num.data[ub] == 9 {
                            num.data[ub] = 0;
                            if ub == umsd_idx {
                                break;
                            }
                            ub -= 1;
                        } else {
                            num.data[ub] += 1;
                            break;
                        }
                    }
                    if ub < umsd_idx || (ub == umsd_idx && num.data[umsd_idx] == 0) {
                        num.data[umsd_idx] = 1;
                        if (ulsd_idx - umsd_idx + 1) as i32 == DECSINGLE_PMAX {
                            num.exponent += 1;
                        } else {
                            ulsd_idx += 1;
                            num.data[ulsd_idx] = 0;
                        }
                    }
                }
            }
            length = (ulsd_idx - umsd_idx + 1) as i32;
        }
        if num.exponent > DECSINGLE_EMAX - (DECSINGLE_PMAX - 1) {
            if num.data[ulsd_idx] == 0 && ulsd_idx == umsd_idx {
                num.exponent = DECSINGLE_EMAX - (DECSINGLE_PMAX - 1);
            } else if (num.exponent + length - 1) > DECSINGLE_EMAX {
                let mut needmax = false;
                set.status |= DEC_OVERFLOW | DEC_INEXACT;
                match set.round {
                    DEC_ROUND_DOWN | DEC_ROUND_05UP => needmax = true,
                    DEC_ROUND_CEILING => {
                        if num.sign != 0 {
                            needmax = true;
                        }
                    }
                    DEC_ROUND_FLOOR => {
                        if num.sign == 0 {
                            needmax = true;
                        }
                    }
                    _ => {}
                }
                if !needmax {
                    num.exponent = DECFLOAT_INF;
                    num.data[umsd_idx] = 0;
                    ulsd_idx = umsd_idx;
                } else {
                    for i in 0..DECSINGLE_PMAX as usize {
                        if umsd_idx + i < num.data.len() {
                            num.data[umsd_idx + i] = 9;
                        }
                    }
                    ulsd_idx = umsd_idx + DECSINGLE_PMAX as usize - 1;
                    num.exponent = DECSINGLE_EMAX - (DECSINGLE_PMAX - 1);
                }
            }
            length = (ulsd_idx - umsd_idx + 1) as i32;
        }
    }
    // For special values (Inf, NaN, sNaN), encode directly without coefficient
    if is_special(num.exponent) {
        let encode = (num.exponent as u32) | num.sign;
        df.set_word(0, encode);
        return df;
    }
    // For normal numbers (any length), pad BCD to PMAX digits and encode
    let mut bcd = vec![0u8; DECSINGLE_PMAX as usize];
    let actual_len = (ulsd_idx - umsd_idx + 1).min(DECSINGLE_PMAX as usize);
    let start_pos = DECSINGLE_PMAX as usize - actual_len;
    for i in 0..actual_len {
        if umsd_idx + i < num.data.len() {
            bcd[start_pos + i] = num.data[umsd_idx + i];
        }
    }
    decSingleFromBCD(df, num.exponent, &bcd, num.sign as i32)
}
/// Convert decSingle to wider decDouble format
pub fn decSingleToWider<'a>(source: &DecSingle, wider: &'a mut DecDouble) -> &'a mut DecDouble {
    let sourhi = source.get_word(0);
    let msd = if (sourhi & 0x78000000) == 0x78000000 {
        wider.set_word(0, sourhi & 0xfe000000);
        0
    } else {
        let comb = sourhi >> 26;
        let exp_part = DECCOMBEXP[comb as usize]
            + ((sourhi & 0x03ffffff) >> (32 - 6 - 6)) as i32;
        let exp = (exp_part - DECSINGLE_BIAS + 398) as u32;
        let code = (exp >> 8) << 29;
        let mut enc = code | ((exp << (32 - 6 - 8)) & 0x03ffffff);
        enc |= sourhi & 0x80000000;
        wider.set_word(0, enc);
        DECCOMBMSD[comb as usize]
    };
    wider.set_word(1, (sourhi & 0x000fffff) | ((msd as u32) << 20));
    wider
}
/// Convert decSingle to BCD representation
pub fn decSingleToBCD(df: &DecSingle, exp: &mut i32, bcdar: &mut [u8]) -> i32 {
    let sourhi = df.get_word(0);
    if (sourhi & 0x7c000000) == 0x78000000 {
        for b in bcdar.iter_mut().take(DECSINGLE_PMAX as usize) {
            *b = 0;
        }
        *exp = (sourhi & 0x7e000000) as i32;
    } else {
        let msd = DECCOMBMSD[(sourhi >> 26) as usize] as u8;
        bcdar[0] = msd;
        let dpd1 = ((sourhi >> 10) & 0x3ff) as usize;
        let dpd0 = (sourhi & 0x3ff) as usize;
        let bcd1 = &DPD2BCD8[dpd1];
        let bcd0 = &DPD2BCD8[dpd0];
        bcdar[1] = bcd1[0];
        bcdar[2] = bcd1[1];
        bcdar[3] = bcd1[2];
        bcdar[4] = bcd0[0];
        bcdar[5] = bcd0[1];
        bcdar[6] = bcd0[2];
        if (sourhi & 0x7c000000) == 0x7c000000 {
            bcdar[0] = 0;
            *exp = (sourhi & 0x7e000000) as i32;
        } else {
            let comb = sourhi >> 26;
            let exp_high = DECCOMBEXP[comb as usize];
            // Reconstruct biased exponent: (exp_high << 6) | exp_continuation
            *exp = (exp_high << 6) + ((sourhi >> 20) & 0x3f) as i32 - DECSINGLE_BIAS;
        }
    }
    (sourhi & 0x80000000) as i32
}
/// Convert packed decimal to decSingle
pub fn decSingleFromPacked<'a>(
    df: &'a mut DecSingle,
    exp: i32,
    packed: &[u8],
) -> Option<&'a mut DecSingle> {
    let mut bcdar = [0u8; DECSINGLE_PMAX as usize + 2];
    let mut op = 1usize;
    let pack_len = (DECSINGLE_PMAX as usize + 2) / 2;
    for &byte in packed.iter().take(pack_len) {
        bcdar[op] = byte >> 4;
        op += 1;
        bcdar[op] = byte & 0x0f;
        op += 1;
    }
    op -= 1;
    let sig = if bcdar[op] == 0x0D || bcdar[op] == 0x0B {
        0x80000000_u32 as i32
    } else {
        0
    };
    if is_special(exp) {
        if exp != DECFLOAT_INF {
            bcdar[1] = 0;
        } else {
            for b in bcdar[1..=DECSINGLE_PMAX as usize].iter_mut() {
                *b = 0;
            }
        }
    }
    Some(decSingleFromBCD(df, exp, &bcdar[1..=DECSINGLE_PMAX as usize], sig))
}
/// Get the exponent of a decSingle
pub fn decSingleGetExponent(df: &DecSingle) -> i32 {
    let sourhi = df.get_word(0);
    if (sourhi & 0x78000000) == 0x78000000 {
        (sourhi & 0x7e000000) as i32
    } else {
        let comb = sourhi >> 26;
        let exp_high = DECCOMBEXP[comb as usize];
        // Reconstruct biased exponent: (exp_high << 6) | exp_continuation
        (exp_high << 6) + ((sourhi >> 20) & 0x3f) as i32 - DECSINGLE_BIAS
    }
}
/// Convert decSingle to engineering string format
pub fn decSingleToEngString<'a>(df: &DecSingle, string: &'a mut [u8]) -> &'a mut [u8] {
    let sourhi = df.get_word(0);
    let mut c = 0usize;
    if (sourhi as i32) < 0 {
        string[c] = b'-';
        c += 1;
    }
    let comb = sourhi >> 26;
    let msd = DECCOMBMSD[comb as usize];
    let mut exp = DECCOMBEXP[comb as usize];
    if is_special(exp) {
        if exp == (DECFLOAT_INF >> 26) {
            string[c..c + 8].copy_from_slice(b"Infinity");
            c += 8;
            string[c] = 0;
            return string;
        }
        if (sourhi & 0x02000000) != 0 {
            string[c] = b's';
            c += 1;
        }
        string[c..c + 3].copy_from_slice(b"NaN");
        c += 3;
        if (sourhi & 0x000fffff) == 0 {
            string[c] = 0;
            return string;
        }
        exp = 0;
    } else {
        // Reconstruct biased exponent: (exp_high << 6) | exp_continuation
        exp = (exp << 6) + ((sourhi >> 20) & 0x3f) as i32 - DECSINGLE_BIAS;
    }
    let cstart = c;
    if msd != 0 {
        string[c] = b'0' + msd as u8;
        c += 1;
    }
    let dpd1 = ((sourhi >> 10) & 0x3ff) as usize;
    let dpd0 = (sourhi & 0x3ff) as usize;
    let bcd1 = &DPD2BCD8[dpd1];
    let bcd0 = &DPD2BCD8[dpd0];
    if c != cstart {
        string[c] = b'0' + bcd1[0];
        string[c + 1] = b'0' + bcd1[1];
        string[c + 2] = b'0' + bcd1[2];
        c += 3;
    } else {
        let cnt = bcd1[3] as usize;
        if cnt > 0 {
            for i in 0..cnt {
                string[c + i] = b'0' + bcd1[3 - cnt + i];
            }
            c += cnt;
        }
    }
    if c != cstart {
        string[c] = b'0' + bcd0[0];
        string[c + 1] = b'0' + bcd0[1];
        string[c + 2] = b'0' + bcd0[2];
        c += 3;
    } else {
        let cnt = bcd0[3] as usize;
        if cnt > 0 {
            for i in 0..cnt {
                string[c + i] = b'0' + bcd0[3 - cnt + i];
            }
            c += cnt;
        }
    }
    if c == cstart {
        string[c] = b'0';
        c += 1;
    }
    if exp == 0 {
        string[c] = 0;
        return string;
    }
    let mut e = 0i32;
    let pre = (c - cstart) as i32 + exp;
    if exp > 0 || pre < -5 {
        e = pre - 1;
        let mut pre_adj = 1i32;
        if e != 0 {
            let adj = if e < 0 {
                let a = (-e) % 3;
                if a != 0 { 3 - a } else { 0 }
            } else {
                e % 3
            };
            e -= adj;
            pre_adj += adj;
        }
        if pre_adj > 0 && (pre_adj as usize) < (c - cstart) {
            let dotat = cstart + pre_adj as usize;
            for i in (dotat..c).rev() {
                string[i + 1] = string[i];
            }
            string[dotat] = b'.';
            c += 1;
        }
        if e != 0 {
            string[c] = b'E';
            c += 1;
            if e < 0 {
                string[c] = b'-';
                e = -e;
            } else {
                string[c] = b'+';
            }
            c += 1;
            let e_abs = e as usize;
            if e_abs >= 100 {
                string[c] = b'0' + (e_abs / 100) as u8;
                c += 1;
            }
            if e_abs >= 10 {
                string[c] = b'0' + ((e_abs / 10) % 10) as u8;
                c += 1;
            }
            string[c] = b'0' + (e_abs % 10) as u8;
            c += 1;
        }
    } else if pre > 0 {
        let dotat = cstart + pre as usize;
        if dotat < c {
            for i in (dotat..c).rev() {
                string[i + 1] = string[i];
            }
            string[dotat] = b'.';
            c += 1;
        } else {
            while c < dotat {
                string[c] = b'0';
                c += 1;
            }
        }
    } else {
        let shift = (-pre + 2) as usize;
        for i in (cstart..c).rev() {
            string[i + shift] = string[i];
        }
        string[cstart] = b'0';
        string[cstart + 1] = b'.';
        for i in 2..shift {
            string[cstart + i] = b'0';
        }
        c += shift;
    }
    string[c] = 0;
    string
}
pub fn decSingleSetExponent<'a>(
    df: &'a mut DecSingle,
    _set: &mut DecContext,
    exp: i32,
) -> &'a mut DecSingle {
    let sourhi = df.get_word(0);
    if is_special(exp) {
        df.set_word(0, (sourhi & 0x80000000) | (exp as u32 & 0x7e000000));
    } else {
        let uexp = (exp + DECSINGLE_BIAS) as u32;
        let code = (uexp >> 6) << 4;
        let mut encode = DECCOMBFROM[code as usize];
        encode |= (uexp << (32 - 6 - 6)) & 0x03ffffff;
        encode |= sourhi & 0x80000000;
        encode |= sourhi & 0x000fffff;
        df.set_word(0, encode);
    }
    df
}
pub fn decSingleFromBCD<'a>(
    df: &'a mut DecSingle,
    exp: i32,
    bcdar: &[u8],
    sig: i32,
) -> &'a mut DecSingle {
    let sign = if sig != 0 { 0x80000000_u32 } else { 0 };
    let encode = if is_special(exp) {
        sign | (exp as u32 & 0x7e000000)
    } else {
        let uexp = (exp + DECSINGLE_BIAS) as u32;
        let msd = bcdar.first().copied().unwrap_or(0) as u32;
        let code = ((uexp >> 6) << 4) | msd;
        let mut enc = DECCOMBFROM.get(code as usize).copied().unwrap_or(0);
        enc |= (uexp << (32 - 6 - 6)) & 0x03ffffff;
        let d1 = bcdar.get(1).copied().unwrap_or(0) as usize;
        let d2 = bcdar.get(2).copied().unwrap_or(0) as usize;
        let d3 = bcdar.get(3).copied().unwrap_or(0) as usize;
        let d4 = bcdar.get(4).copied().unwrap_or(0) as usize;
        let d5 = bcdar.get(5).copied().unwrap_or(0) as usize;
        let d6 = bcdar.get(6).copied().unwrap_or(0) as usize;
        let dpd1 = BCD2DPD.get(d1 * 100 + d2 * 10 + d3).copied().unwrap_or(0);
        let dpd0 = BCD2DPD.get(d4 * 100 + d5 * 10 + d6).copied().unwrap_or(0);
        enc |= dpd1 << 10;
        enc |= dpd0;
        enc | sign
    };
    df.set_word(0, encode);
    df
}
/// Set a decSingle to zero
pub fn decSingleZero<'a>(df: &'a mut DecSingle) -> &'a mut DecSingle {
    df.set_word(0, 0x22500000);
    df
}
/// Parse a decimal string into a decSingle
pub fn decSingleFromString<'a>(
    result: &'a mut DecSingle,
    string: &str,
    set: &mut DecContext,
) -> &'a mut DecSingle {
    let mut num = BcdNumInternal {
        data: Vec::new(),
        msd_idx: 0,
        lsd_idx: 0,
        sign: 0,
        exponent: 0,
    };
    let mut buffer = vec![0u8; ((DECSINGLE_PMAX as usize + 11 + 7) / 8) * 8];
    let mut error = DEC_CONVERSION_SYNTAX;
    let chars: Vec<char> = string.chars().collect();
    let mut idx = 0;
    while idx < chars.len() && chars[idx].is_whitespace() {
        idx += 1;
    }
    if idx >= chars.len() {
        set.status |= error;
        num.exponent = DECFLOAT_QNAN;
        num.sign = 0;
        buffer[0] = 0;
        num.data = buffer;
        num.msd_idx = 0;
        num.lsd_idx = 0;
        decFinalize(result, &mut num, set);
        return result;
    }
    num.sign = 0;
    if chars[idx] == '-' {
        num.sign = DECFLOAT_SIGN;
        idx += 1;
    } else if chars[idx] == '+' {
        idx += 1;
    }
    let cfirst = idx;
    let mut dotchar: Option<usize> = None;
    while idx < chars.len() {
        let c = chars[idx];
        if c.is_ascii_digit() {
            idx += 1;
            continue;
        }
        if c == '.' {
            if dotchar.is_some() {
                break;
            }
            dotchar = Some(idx);
            idx += 1;
            continue;
        }
        break;
    }
    let c_end = idx;
    let mut digits = c_end - cfirst - if dotchar.is_some() { 1 } else { 0 };
    if digits > 0 {
        let clast = c_end - 1;
        let mut exp: i32 = 0;
        if idx < chars.len() {
            let c = chars[idx];
            if c == 'E' || c == 'e' {
                idx += 1;
                let mut exp_sign = 1;
                if idx < chars.len() {
                    if chars[idx] == '-' {
                        exp_sign = -1;
                        idx += 1;
                    } else if chars[idx] == '+' {
                        idx += 1;
                    }
                }
                while idx < chars.len() && chars[idx] == '0' {
                    idx += 1;
                }
                let exp_start = idx;
                let mut exp_val: i32 = 0;
                while idx < chars.len() && chars[idx].is_ascii_digit() {
                    let digit = chars[idx].to_digit(10).unwrap() as i32;
                    exp_val = exp_val * 10 + digit;
                    idx += 1;
                }
                if idx < chars.len() && !chars[idx].is_whitespace() {
                    error = DEC_CONVERSION_SYNTAX;
                } else {
                    if idx > exp_start + 3 {
                        exp_val = DECSINGLE_EMAX * 2;
                    }
                    exp = exp_val * exp_sign;
                    error = 0;
                }
            } else if idx < chars.len() && !chars[idx].is_whitespace() {
                error = DEC_CONVERSION_SYNTAX;
            } else {
                error = 0;
            }
        } else {
            error = 0;
        }
        if let Some(dot_pos) = dotchar {
            digits -= 0;
            exp -= (clast - dot_pos) as i32;
        }
        num.exponent = exp;
        let mut buf_idx = 0;
        for i in cfirst..c_end {
            if chars[i] == '.' {
                continue;
            }
            if buf_idx < buffer.len() {
                buffer[buf_idx] = chars[i].to_digit(10).unwrap_or(0) as u8;
                buf_idx += 1;
            }
        }
        let pmax = DECSINGLE_PMAX as usize;
        if buf_idx > pmax {
            let extra = buf_idx - pmax;
            let mut sticky = false;
            for i in pmax..buf_idx {
                if buffer[i] != 0 {
                    sticky = true;
                    break;
                }
            }
            if sticky {
                buffer[pmax - 1] = DECSTICKYTAB[buffer[pmax - 1]
                    as usize];
            }
            buf_idx = pmax;
            num.exponent += extra as i32;
        }
        num.data = buffer[..buf_idx.max(1)].to_vec();
        num.msd_idx = 0;
        num.lsd_idx = if buf_idx > 0 { buf_idx - 1 } else { 0 };
    } else {
        let remaining: String = chars[idx..].iter().collect();
        buffer[0] = 0;
        num.data = buffer;
        num.msd_idx = 0;
        num.lsd_idx = 0;
        if decBiStr(&remaining, "infinity", "INFINITY")
            || decBiStr(&remaining, "inf", "INF")
        {
            num.exponent = DECFLOAT_INF;
            error = 0;
        } else if remaining.to_lowercase().starts_with("snan") {
            num.exponent = DECFLOAT_SNAN;
            error = 0;
            let payload_str: String = remaining.chars().skip(4).collect();
            if !payload_str.is_empty() {
                let mut payload_digits = Vec::new();
                for c in payload_str.chars() {
                    if c.is_ascii_digit() {
                        payload_digits.push(c.to_digit(10).unwrap() as u8);
                    } else {
                        break;
                    }
                }
                if !payload_digits.is_empty() {
                    num.data = payload_digits;
                    num.lsd_idx = num.data.len() - 1;
                }
            }
        } else if remaining.to_lowercase().starts_with("nan") {
            num.exponent = DECFLOAT_QNAN;
            error = 0;
            let payload_str: String = remaining.chars().skip(3).collect();
            if !payload_str.is_empty() {
                let mut payload_digits = Vec::new();
                for c in payload_str.chars() {
                    if c.is_ascii_digit() {
                        payload_digits.push(c.to_digit(10).unwrap() as u8);
                    } else {
                        break;
                    }
                }
                if !payload_digits.is_empty() {
                    num.data = payload_digits;
                    num.lsd_idx = num.data.len() - 1;
                }
            }
        }
    }
    if error != 0 {
        set.status |= error;
        num.exponent = DECFLOAT_QNAN;
        num.sign = 0;
        num.data = vec![0];
        num.msd_idx = 0;
        num.lsd_idx = 0;
    }
    decFinalize(result, &mut num, set);
    result
}
/// Convert a decSingle to a string
pub fn decSingleToString<'a>(df: &DecSingle, string: &'a mut [u8]) -> &'a str {
    let sourhi = df.get_word(0);
    let mut c_idx = 0;
    if (sourhi as i32) < 0 {
        string[c_idx] = b'-';
        c_idx += 1;
    }
    let comb = (sourhi >> 26) as usize;
    let msd = DECCOMBMSD[comb];
    let mut exp = DECCOMBEXP[comb];
    if !is_special_exp(exp) {
        // Reconstruct biased exponent: (exp_high << 6) | exp_continuation
        exp = (exp << 6) + ((sourhi >> 20) & 0x3f) as i32 - DECSINGLE_BIAS;
    } else {
        if exp == DECFLOAT_INF {
            let s = b"Infinity";
            string[c_idx..c_idx + s.len()].copy_from_slice(s);
            let result = std::str::from_utf8(&string[..c_idx + s.len()]).unwrap_or("");
            return result;
        }
        if (sourhi & 0x02000000) != 0 {
            string[c_idx] = b's';
            c_idx += 1;
        }
        let s = b"NaN";
        string[c_idx..c_idx + s.len()].copy_from_slice(s);
        c_idx += s.len();
        if (sourhi & 0x000fffff) == 0 {
            let result = std::str::from_utf8(&string[..c_idx]).unwrap_or("");
            return result;
        }
        exp = 0;
    }
    let cstart = c_idx;
    if msd != 0 {
        string[c_idx] = b'0' + msd;
        c_idx += 1;
    }
    let dpd1 = ((sourhi >> 10) & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd1);
    if c_idx > cstart || d0 != 0 {
        string[c_idx] = b'0' + d0;
        c_idx += 1;
    }
    if c_idx > cstart || d1 != 0 {
        string[c_idx] = b'0' + d1;
        c_idx += 1;
    }
    if c_idx > cstart || d2 != 0 {
        string[c_idx] = b'0' + d2;
        c_idx += 1;
    }
    let dpd0 = (sourhi & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd0);
    if c_idx > cstart || d0 != 0 {
        string[c_idx] = b'0' + d0;
        c_idx += 1;
    }
    if c_idx > cstart || d1 != 0 {
        string[c_idx] = b'0' + d1;
        c_idx += 1;
    }
    string[c_idx] = b'0' + d2;
    c_idx += 1;
    if c_idx == cstart {
        string[c_idx] = b'0';
        c_idx += 1;
    }
    let digits = c_idx - cstart;
    let pre = digits as i32 + exp;
    let mut e = 0;
    let use_exp_notation = exp > 0 || pre < -5;
    if use_exp_notation {
        e = pre - 1;
        if digits > 1 {
            let dot_pos = cstart + 1;
            for i in (dot_pos..c_idx).rev() {
                string[i + 1] = string[i];
            }
            string[dot_pos] = b'.';
            c_idx += 1;
        }
        string[c_idx] = b'E';
        c_idx += 1;
        if e >= 0 {
            string[c_idx] = b'+';
        } else {
            string[c_idx] = b'-';
            e = -e;
        }
        c_idx += 1;
        let bcd = bin_to_bcd(e);
        for i in (3 - bcd[3] as usize)..3 {
            string[c_idx] = b'0' + bcd[i];
            c_idx += 1;
        }
    } else if pre > 0 && (pre as usize) < digits {
        let dot_pos = cstart + pre as usize;
        for i in (dot_pos..c_idx).rev() {
            string[i + 1] = string[i];
        }
        string[dot_pos] = b'.';
        c_idx += 1;
    } else if pre <= 0 {
        let zeros_needed = (-pre) as usize + 2;
        for i in (cstart..c_idx).rev() {
            string[i + zeros_needed] = string[i];
        }
        string[cstart] = b'0';
        string[cstart + 1] = b'.';
        for i in 0..(-pre as usize) {
            string[cstart + 2 + i] = b'0';
        }
        c_idx += zeros_needed;
    }
    string[c_idx] = 0;
    std::str::from_utf8(&string[..c_idx]).unwrap_or("")
}
/// Get the coefficient of a decSingle as BCD
pub fn decSingleGetCoefficient(df: &DecSingle, bcdar: &mut [u8]) -> i32 {
    let word = df.get_word(0);
    if is_infinity(word) {
        bcdar[..DECSINGLE_PMAX as usize].fill(0);
    } else {
        bcdar[0] = DECCOMBMSD[(word >> 26) as usize];
        let dpd1 = ((word >> 10) & 0x3ff) as u16;
        let (d0, d1, d2) = decode_dpd_declet(dpd1);
        bcdar[1] = d0;
        bcdar[2] = d1;
        bcdar[3] = d2;
        let dpd0 = (word & 0x3ff) as u16;
        let (d0, d1, d2) = decode_dpd_declet(dpd0);
        bcdar[4] = d0;
        bcdar[5] = d1;
        bcdar[6] = d2;
        if is_nan(word) {
            bcdar[0] = 0;
        }
    }
    (word & DECFLOAT_SIGN) as i32
}
/// Set the coefficient of a decSingle
pub fn decSingleSetCoefficient<'a>(
    df: &'a mut DecSingle,
    bcdar: &[u8],
    sig: i32,
) -> &'a mut DecSingle {
    let word = df.get_word(0);
    let mut bcdzero = [0u8; DECSINGLE_PMAX as usize];
    let exp = if is_special(word as i32) {
        let special_exp = (word & 0x7e000000) as i32;
        if is_infinity(word) {
            return decSingleFromBCD(df, special_exp, &bcdzero, sig);
        }
        special_exp
    } else {
        let comb = (word >> 26) as usize;
        let exp_high = DECCOMBEXP[comb];
        let exp_cont = ((word >> 20) & 0x3f) as i32;
        // Reconstruct biased exponent: (exp_high << 6) | exp_continuation
        (exp_high << 6) + exp_cont - DECSINGLE_BIAS
    };
    decSingleFromBCD(df, exp, bcdar, sig)
}
/// Convert a decSingle to packed BCD
pub fn decSingleToPacked(df: &DecSingle, exp: &mut i32, packed: &mut [u8]) -> i32 {
    let mut bcdar = [0u8; DECSINGLE_PMAX as usize + 2];
    let word = df.get_word(0);
    if is_infinity(word) {
        bcdar.fill(0);
        *exp = DECFLOAT_INF;
    } else {
        bcdar[1] = DECCOMBMSD[(word >> 26) as usize];
        let dpd1 = ((word >> 10) & 0x3ff) as u16;
        let (d0, d1, d2) = decode_dpd_declet(dpd1);
        bcdar[2] = d0;
        bcdar[3] = d1;
        bcdar[4] = d2;
        let dpd0 = (word & 0x3ff) as u16;
        let (d0, d1, d2) = decode_dpd_declet(dpd0);
        bcdar[5] = d0;
        bcdar[6] = d1;
        bcdar[7] = d2;
        if is_nan(word) {
            bcdar[1] = 0;
            *exp = (word & 0x7e000000) as i32;
        } else {
            let comb = (word >> 26) as usize;
            let exp_high = DECCOMBEXP[comb];
            let exp_cont = ((word >> 20) & 0x3f) as i32;
            // Reconstruct biased exponent: (exp_high << 6) | exp_continuation
            *exp = (exp_high << 6) + exp_cont - DECSINGLE_BIAS;
        }
    }
    let sign_nibble = if (word & DECFLOAT_SIGN) != 0 { 0x0D } else { 0x0C };
    bcdar[DECSINGLE_PMAX as usize + 1] = sign_nibble;
    for i in 0..(DECSINGLE_PMAX as usize + 2) / 2 {
        packed[i] = (bcdar[i * 2 + 1] << 4) | bcdar[i * 2 + 2];
    }
    if sign_nibble == 0x0D { DECFLOAT_SIGN as i32 } else { 0 }
}
/// Convert from wider format (decDouble) to decSingle
pub fn decSingleFromWider<'a>(
    result: &'a mut DecSingle,
    wider: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecSingle {
    let mut bcdar = [0u8; 16];
    let widerhi = wider.get_word(1);
    let widerlo = wider.get_word(0);
    bcdar[0] = DECCOMBMSD[(widerhi >> 26) as usize];
    let dpd1 = ((widerhi >> 8) & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd1);
    bcdar[1] = d0;
    bcdar[2] = d1;
    bcdar[3] = d2;
    let dpd2 = (((widerhi << 2) | (widerlo >> 30)) & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd2);
    bcdar[4] = d0;
    bcdar[5] = d1;
    bcdar[6] = d2;
    let dpd3 = ((widerlo >> 20) & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd3);
    bcdar[7] = d0;
    bcdar[8] = d1;
    bcdar[9] = d2;
    let dpd4 = ((widerlo >> 10) & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd4);
    bcdar[10] = d0;
    bcdar[11] = d1;
    bcdar[12] = d2;
    let dpd5 = (widerlo & 0x3ff) as u16;
    let (d0, d1, d2) = decode_dpd_declet(dpd5);
    bcdar[13] = d0;
    bcdar[14] = d1;
    bcdar[15] = d2;
    let mut exp = DECCOMBWEXP[(widerhi >> 26) as usize];
    if is_special_exp(exp) {
        exp = (widerhi & 0x7e000000) as i32;
    } else {
        let exp_cont = ((widerhi & 0x03ffffff) >> (32 - 6 - 8)) as i32;
        exp += exp_cont - 398;
    }
    let mut num = BcdNumInternal {
        data: bcdar.to_vec(),
        msd_idx: 0,
        lsd_idx: 15,
        sign: widerhi & DECFLOAT_SIGN,
        exponent: exp,
    };
    decFinalize(result, &mut num, set)
}
/// Convert from packed BCD with checking
pub fn decSingleFromPackedChecked<'a>(
    df: &'a mut DecSingle,
    exp: i32,
    packed: &[u8],
) -> Option<&'a mut DecSingle> {
    let pmax = DECSINGLE_PMAX as usize;
    let mut bcdar = [0u8; DECSINGLE_PMAX as usize + 2];
    if packed.len() < (pmax + 2) / 2 {
        return None;
    }
    let mut op_idx = 1;
    for i in 0..(pmax + 2) / 2 {
        bcdar[op_idx] = packed[i] >> 4;
        if bcdar[op_idx] > 9 {
            return None;
        }
        op_idx += 1;
        bcdar[op_idx] = packed[i] & 0x0f;
        if bcdar[op_idx] > 9 && i < (pmax + 2) / 2 - 1 {
            return None;
        }
        op_idx += 1;
    }
    let sign_nibble = bcdar[op_idx - 1];
    if sign_nibble <= 9 {
        return None;
    }
    let sig = if sign_nibble == 0x0D || sign_nibble == 0x0B {
        DECFLOAT_SIGN as i32
    } else {
        0
    };
    if exp == DECFLOAT_QNAN || exp == DECFLOAT_SNAN {
        if bcdar[1] != 0 {
            return None;
        }
    } else if exp == DECFLOAT_INF {
        for i in 0..pmax {
            if bcdar[i + 1] != 0 {
                return None;
            }
        }
    } else {
        if exp > DECSINGLE_EMAX - DECSINGLE_PMAX + 1 {
            return None;
        }
        if exp < DECSINGLE_EMIN - DECSINGLE_PMAX + 1 {
            return None;
        }
    }
    Some(decSingleFromBCD(df, exp, &bcdar[1..], sig))
}
const DECFLOAT_SIGN: u32 = 0x80000000;
const DEC_CONVERSION_SYNTAX: u32 = 0x00000001;
/// Wider combination field to exponent mapping
static DECCOMBWEXP: [i32; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    0,
    0,
    1,
    1,
    2,
    2,
    DECFLOAT_INF,
    DECFLOAT_QNAN,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    0,
    0,
    1,
    1,
    2,
    2,
    DECFLOAT_INF,
    DECFLOAT_QNAN,
];
/// DPD to BCD conversion table (simplified - 1024 * 4 bytes in full)
/// Each entry is [d0, d1, d2, count] where count is significant digits
fn dpd_to_bcd(dpd: u32) -> [u8; 4] {
    let d0 = ((dpd >> 7) & 0x7) as u8;
    let d1 = ((dpd >> 4) & 0x7) as u8;
    let d2 = (dpd & 0xf) as u8;
    let (b0, b1, b2) = decode_dpd_declet(dpd as u16);
    let count = if b0 != 0 { 3 } else if b1 != 0 { 2 } else if b2 != 0 { 1 } else { 0 };
    [b0, b1, b2, count]
}
/// Decode a single DPD declet (10 bits) to 3 BCD digits
/// DPD bits: p(9) q(8) r(7) s(6) t(5) u(4) v(3) w(2) x(1) y(0)
fn decode_dpd_declet(dpd: u16) -> (u8, u8, u8) {
    let dpd = dpd & 0x3ff;
    // Extract individual bits
    let p = ((dpd >> 9) & 1) as u8;
    let q = ((dpd >> 8) & 1) as u8;
    let r = ((dpd >> 7) & 1) as u8;
    let s = ((dpd >> 6) & 1) as u8;
    let t = ((dpd >> 5) & 1) as u8;
    let u = ((dpd >> 4) & 1) as u8;
    let v = ((dpd >> 3) & 1) as u8;
    let w = ((dpd >> 2) & 1) as u8;
    let x = ((dpd >> 1) & 1) as u8;
    let y = (dpd & 1) as u8;

    if v == 0 {
        // Simple case: all digits < 8
        let d2 = (p << 2) | (q << 1) | r;
        let d1 = (s << 2) | (t << 1) | u;
        let d0 = (w << 2) | (x << 1) | y;
        (d2, d1, d0)
    } else {
        // Complex case: at least one digit >= 8
        // Use w,x bits to determine which case
        match (w, x) {
            (0, 0) => {
                // d0 and d1 >= 8, d2 < 8
                let d2 = (p << 2) | (q << 1) | r;
                let d1 = 8 + u;
                let d0 = 8 + y;
                (d2, d1, d0)
            }
            (0, 1) => {
                // d0 >= 8, d1 < 8, d2 < 8  OR  d0 < 8, d1 >= 8, d2 < 8
                if s == 0 && t == 0 {
                    // Only d0 >= 8
                    let d2 = (p << 2) | (q << 1) | r;
                    let d1 = (s << 2) | (t << 1) | u;
                    let d0 = 8 + y;
                    (d2, d1, d0)
                } else {
                    // Only d1 >= 8
                    let d2 = (p << 2) | (q << 1) | r;
                    let d1 = 8 + u;
                    let d0 = (s << 2) | (t << 1) | y;
                    (d2, d1, d0)
                }
            }
            (1, 0) => {
                // d2 >= 8, others may vary
                if s == 0 && t == 0 {
                    // Only d2 >= 8
                    let d2 = 8 + r;
                    let d1 = (p << 2) | (q << 1) | u;
                    let d0 = (s << 2) | (t << 1) | y;
                    (d2, d1, d0)
                } else {
                    // d1 and d2 >= 8, d0 < 8
                    let d2 = 8 + r;
                    let d1 = 8 + u;
                    let d0 = (p << 2) | (q << 1) | y;
                    (d2, d1, d0)
                }
            }
            (1, 1) => {
                // Multiple >= 8 cases, use s,t to distinguish
                if s == 0 && t == 0 {
                    // d0 and d1 >= 8, d2 < 8
                    let d2 = (p << 2) | (q << 1) | r;
                    let d1 = 8 + u;
                    let d0 = 8 + y;
                    (d2, d1, d0)
                } else if s == 0 && t == 1 {
                    // d0 and d2 >= 8, d1 < 8
                    let d2 = 8 + r;
                    let d1 = (p << 2) | (q << 1) | u;
                    let d0 = 8 + y;
                    (d2, d1, d0)
                } else if s == 1 && t == 0 {
                    // d1 and d2 >= 8, d0 < 8
                    let d2 = 8 + r;
                    let d1 = 8 + u;
                    let d0 = (p << 2) | (q << 1) | y;
                    (d2, d1, d0)
                } else {
                    // All three >= 8
                    let d2 = 8 + r;
                    let d1 = 8 + u;
                    let d0 = 8 + y;
                    (d2, d1, d0)
                }
            }
            _ => (0, 0, 0),
        }
    }
}
/// Encode 3 BCD digits to DPD declet
fn encode_dpd_declet(d0: u8, d1: u8, d2: u8) -> u16 {
    let d0 = d0 & 0xf;
    let d1 = d1 & 0xf;
    let d2 = d2 & 0xf;
    if d0 < 8 && d1 < 8 && d2 < 8 {
        return ((d0 as u16) << 7) | ((d1 as u16) << 4) | (d2 as u16);
    }
    let mut dpd: u16 = 0;
    if d0 >= 8 {
        dpd |= 0x100 | (((d0 & 1) as u16) << 7);
    } else {
        dpd |= (d0 as u16) << 7;
    }
    if d1 >= 8 {
        dpd |= 0x80 | (((d1 & 1) as u16) << 4);
    } else {
        dpd |= (d1 as u16) << 4;
    }
    if d2 >= 8 {
        dpd |= 0x8 | (d2 & 1) as u16;
    } else {
        dpd |= d2 as u16;
    }
    dpd
}
/// BCD to DPD lookup (simplified)
fn bcd_to_dpd(d0: u8, d1: u8, d2: u8) -> u16 {
    encode_dpd_declet(d0, d1, d2)
}
/// Binary to BCD conversion for exponents
fn bin_to_bcd(val: i32) -> [u8; 4] {
    let val = val.unsigned_abs();
    let d0 = ((val / 100) % 10) as u8;
    let d1 = ((val / 10) % 10) as u8;
    let d2 = (val % 10) as u8;
    let count = if d0 != 0 { 3 } else if d1 != 0 { 2 } else { 1 };
    [d0, d1, d2, count]
}
/// Check if exponent indicates special value (Inf, NaN)
#[inline]
fn is_special_exp(exp: i32) -> bool {
    exp >= DECFLOAT_INF
}
/// Check if value is infinity
#[inline]
fn is_infinity(word: u32) -> bool {
    (word & 0x7c000000) == 0x78000000
}
/// Check if value is NaN (quiet or signaling)
#[inline]
fn is_nan(word: u32) -> bool {
    (word & 0x7c000000) == 0x7c000000
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero() {
        let mut ds = DecSingle::new();
        decSingleZero(&mut ds);
        let mut buf = [0u8; 32];
        let s = decSingleToString(&ds, &mut buf);
        assert_eq!(s, "0");
    }
    #[test]
    fn test_from_string() {
        let mut ds = DecSingle::new();
        let mut ctx = DecContext::default();
        decSingleFromString(&mut ds, "123.45", &mut ctx);
        let mut buf = [0u8; 32];
        let s = decSingleToString(&ds, &mut buf);
        assert_eq!(s, "123.45");
        // Also test simpler values
        let mut ds2 = DecSingle::new();
        decSingleFromString(&mut ds2, "1", &mut ctx);
        let mut buf2 = [0u8; 32];
        let s2 = decSingleToString(&ds2, &mut buf2);
        assert_eq!(s2, "1");
        // Test 12345
        let mut ds3 = DecSingle::new();
        decSingleFromString(&mut ds3, "12345", &mut ctx);
        let mut buf3 = [0u8; 32];
        let s3 = decSingleToString(&ds3, &mut buf3);
        assert_eq!(s3, "12345");
    }
    #[test]
    fn test_infinity() {
        let mut ds = DecSingle::new();
        let mut ctx = DecContext::default();
        decSingleFromString(&mut ds, "Infinity", &mut ctx);
        let mut buf = [0u8; 32];
        let s = decSingleToString(&ds, &mut buf);
        assert!(s.contains("Inf"));
    }
    #[test]
    fn test_nan() {
        let mut ds = DecSingle::new();
        let mut ctx = DecContext::default();
        decSingleFromString(&mut ds, "NaN", &mut ctx);
        let mut buf = [0u8; 32];
        let s = decSingleToString(&ds, &mut buf);
        assert!(s.contains("NaN"));
    }
}
impl fmt::Debug for DecSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; DECSINGLE_STRING];
        let s = decSingleToString(self, &mut buf);
        write!(f, "DecSingle({})", s)
    }
}
impl DecSingle {
    /// Get the word at index (for big-endian access)
    #[inline]
    pub fn get_word(&self, idx: usize) -> u32 {
        let start = (1 - 1 - idx) * 4;
        u32::from_be_bytes([
            self.bytes[start],
            self.bytes[start + 1],
            self.bytes[start + 2],
            self.bytes[start + 3],
        ])
    }
    /// Set the word at index (for big-endian access)
    #[inline]
    pub fn set_word(&mut self, idx: usize, value: u32) {
        let start = (1 - 1 - idx) * 4;
        let bytes = value.to_be_bytes();
        self.bytes[start] = bytes[0];
        self.bytes[start + 1] = bytes[1];
        self.bytes[start + 2] = bytes[2];
        self.bytes[start + 3] = bytes[3];
    }
    /// Create a new zero-initialized DecSingle
    pub fn new() -> Self {
        DecSingle { bytes: [0; 4] }
    }
}
impl DecDouble {
    #[inline]
    fn get_word(&self, idx: usize) -> u32 {
        let start = idx * 4;
        u32::from_le_bytes([
            self.bytes[start],
            self.bytes[start + 1],
            self.bytes[start + 2],
            self.bytes[start + 3],
        ])
    }
    #[inline]
    fn set_word(&mut self, idx: usize, value: u32) {
        let start = idx * 4;
        let bytes = value.to_le_bytes();
        self.bytes[start] = bytes[0];
        self.bytes[start + 1] = bytes[1];
        self.bytes[start + 2] = bytes[2];
        self.bytes[start + 3] = bytes[3];
    }
}
