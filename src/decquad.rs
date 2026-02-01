//! Module: decquad
//!
//! Contains 89 transpiled functions:
//! - decQuadLogB:13798328019042021101:./src/decNumber/decQuad.c
//! - decQuadCanonical:15584733031438822962:./src/decNumber/decQuad.c
//! - decQuadIsNegative:18382949784128806489:./src/decNumber/decQuad.c
//! - decQuadToInt32:12173435974496117391:./src/decNumber/decQuad.c
//! - decQuadInvert:8922379581165747282:./src/decNumber/decQuad.c
//! - decQuadCopyAbs:14877723308861806742:./src/decNumber/decQuad.c
//! - decQuadNextPlus:8872401325469774325:./src/decNumber/decQuad.c
//! - decQuadMultiply:5256347538551164022:./src/decNumber/decQuad.c
//! - decQuadFromPackedChecked:2168615792015979109:./src/decNumber/decQuad.c
//! - decToIntegral:2286078337697803649:./src/decNumber/decQuad.c
//! - decQuadIsNaN:3205642515348823385:./src/decNumber/decQuad.c
//! - decQuadSetExponent:8706564014516072786:./src/decNumber/decQuad.c
//! - decQuadCompareSignal:8821603764493696676:./src/decNumber/decQuad.c
//! - decQuadToIntegralValue:8460465078318211804:./src/decNumber/decQuad.c
//! - decQuadMin:8204263671135155135:./src/decNumber/decQuad.c
//! - decQuadNextToward:16974218190930726865:./src/decNumber/decQuad.c
//! - decQuadCopyNegate:12962637526125163195:./src/decNumber/decQuad.c
//! - decQuadCopySign:17751163989408321014:./src/decNumber/decQuad.c
//! - decQuadFMA:4287675761737097926:./src/decNumber/decQuad.c
//! - decInfinity:4496709458876092948:./src/decNumber/decQuad.c
//! - decNumCompare:8142437862154526001:./src/decNumber/decQuad.c
//! - decQuadToString:5183676520689314325:./src/decNumber/decQuad.c
//! - decQuadIsFinite:10670012375196329429:./src/decNumber/decQuad.c
//! - decQuadDivide:12259310599529101915:./src/decNumber/decQuad.c
//! - decQuadClass:11892812238114549293:./src/decNumber/decQuad.c
//! - decQuadGetExponent:1667354937366884982:./src/decNumber/decQuad.c
//! - decQuadIsZero:15298006917734176290:./src/decNumber/decQuad.c
//! - decQuadIsLogical:16329984636575750078:./src/decNumber/decQuad.c
//! - decQuadDivideInteger:18362494653278836225:./src/decNumber/decQuad.c
//! - decQuadZero:10947246245481905431:./src/decNumber/decQuad.c
//! - decFiniteMultiply:11317160294431379374:./src/decNumber/decQuad.c
//! - decQuadMinMag:11415302103193468151:./src/decNumber/decQuad.c
//! - decQuadToUInt32:4560706153699785197:./src/decNumber/decQuad.c
//! - decQuadIsSignaling:15387577558875120621:./src/decNumber/decQuad.c
//! - decQuadFromPacked:3425122227048600755:./src/decNumber/decQuad.c
//! - decQuadRotate:13555733163172298101:./src/decNumber/decQuad.c
//! - decQuadGetCoefficient:12589397172495599349:./src/decNumber/decQuad.c
//! - decQuadIsInteger:6973208582952722988:./src/decNumber/decQuad.c
//! - decQuadIsCanonical:5490350768070829829:./src/decNumber/decQuad.c
//! - decNaNs:7324416678172143042:./src/decNumber/decQuad.c
//! - decFinalize:7441031706973248981:./src/decNumber/decQuad.c
//! - decQuadCompareTotalMag:16503820295206566395:./src/decNumber/decQuad.c
//! - decCanonical:2980365066917977406:./src/decNumber/decQuad.c
//! - decInvalid:3424938591941942022:./src/decNumber/decQuad.c
//! - decQuadToIntegralExact:3199229198214934469:./src/decNumber/decQuad.c
//! - decQuadIsSigned:16661619612106894300:./src/decNumber/decQuad.c
//! - decQuadClassString:7566323165165868947:./src/decNumber/decQuad.c
//! - decQuadXor:3099867876277915120:./src/decNumber/decQuad.c
//! - decQuadCopy:17899803936615938218:./src/decNumber/decQuad.c
//! - decQuadIsNormal:13771932297223626332:./src/decNumber/decQuad.c
//! - decQuadFromInt32:10563714591644412039:./src/decNumber/decQuad.c
//! - decQuadIsPositive:6292818786449697451:./src/decNumber/decQuad.c
//! - decQuadOr:5274220712904900976:./src/decNumber/decQuad.c
//! - decQuadToPacked:11552035477550734305:./src/decNumber/decQuad.c
//! - decQuadShow:3160493848652222103:./src/decNumber/decQuad.c
//! - decQuadAnd:3859949949944127334:./src/decNumber/decQuad.c
//! - decQuadVersion:956249302524774830:./src/decNumber/decQuad.c
//! - decDivide:9643759269505174029:./src/decNumber/decQuad.c
//! - decQuadFromString:13151765453587729334:./src/decNumber/decQuad.c
//! - decQuadQuantize:13760945846732880502:./src/decNumber/decQuad.c
//! - decQuadShift:16255663287145589054:./src/decNumber/decQuad.c
//! - decQuadPlus:6729842991259415023:./src/decNumber/decQuad.c
//! - decQuadMax:17167458754399319639:./src/decNumber/decQuad.c
//! - decQuadRadix:14837901031569200503:./src/decNumber/decQuad.c
//! - decQuadAdd:14283694915985775566:./src/decNumber/decQuad.c
//! - decQuadAbs:14619822507228526374:./src/decNumber/decQuad.c
//! - decQuadSubtract:5493378581940054860:./src/decNumber/decQuad.c
//! - decQuadRemainderNear:13140710927944633659:./src/decNumber/decQuad.c
//! - decQuadSameQuantum:13344303398172411826:./src/decNumber/decQuad.c
//! - decQuadCompareTotal:17748117976389275260:./src/decNumber/decQuad.c
//! - decQuadToUInt32Exact:14187306397502168475:./src/decNumber/decQuad.c
//! - decQuadScaleB:10501531119604970659:./src/decNumber/decQuad.c
//! - decQuadDigits:15227996425945243085:./src/decNumber/decQuad.c
//! - decQuadSetCoefficient:10689638289577416412:./src/decNumber/decQuad.c
//! - decQuadIsSubnormal:7145944877209371122:./src/decNumber/decQuad.c
//! - decQuadReduce:9892230455176182777:./src/decNumber/decQuad.c
//! - decQuadIsInfinite:16934411053138811215:./src/decNumber/decQuad.c
//! - decQuadRemainder:2997177603524205789:./src/decNumber/decQuad.c
//! - decToInt32:5885287185902065896:./src/decNumber/decQuad.c
//! - decQuadNextMinus:15875399676744520674:./src/decNumber/decQuad.c
//! - decQuadFromBCD:4500109113506435744:./src/decNumber/decQuad.c
//! - decQuadFromUInt32:6371399131500626213:./src/decNumber/decQuad.c
//! - decQuadToInt32Exact:18149369597693010664:./src/decNumber/decQuad.c
//! - decQuadToEngString:43050797587199797:./src/decNumber/decQuad.c
//! - decQuadMaxMag:1111127973913685897:./src/decNumber/decQuad.c
//! - decQuadMinus:16056034550614472966:./src/decNumber/decQuad.c
//! - decQuadToBCD:15580140653150499187:./src/decNumber/decQuad.c
//! - decQuadCompare:17257132537666910775:./src/decNumber/decQuad.c
//! - decQuadIsSignalling:2121378905324841579:./src/decNumber/decQuad.c

use std::fmt;
use crate::types::DecNumber;
// Note: All decQuad* functions are defined locally in this file
use std::ptr;
use std::cmp::Ordering;
use std::fmt::Write as FmtWrite;
// Note: decCanonical, decDivide, decFinalize, etc. are defined locally
use crate::types::*;
/// Number of bytes in decQuad
const DECQUAD_BYTES: usize = 16;
/// Number of coefficient digits in decQuad
const DECQUAD_PMAX: usize = 34;
/// Maximum exponent value
const DECQUAD_EMAX: i32 = 6144;
/// Minimum exponent value
const DECQUAD_EMIN: i32 = -6143;
/// Bias for exponent
const DECQUAD_BIAS: i32 = 6176;
const DECQUAD_EHIGH: i32 = DECQUAD_EMAX + DECQUAD_BIAS - (DECQUAD_PMAX as i32 - 1);
const DEC_INVALID_OPERATION: u32 = 0x00000080;
const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
static DECCOMBMSD: [u32; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9,
    8, 9, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 8, 9, 8, 9, 0, 0,
];
static DECCOMBEXP: [u32; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 << 12,
    1 << 12,
    1 << 12,
    1 << 12,
    1 << 12,
    1 << 12,
    1 << 12,
    1 << 12,
    2 << 12,
    2 << 12,
    2 << 12,
    2 << 12,
    2 << 12,
    2 << 12,
    2 << 12,
    2 << 12,
    0,
    0,
    1 << 12,
    1 << 12,
    2 << 12,
    2 << 12,
    0x78000000,
    0x7c000000,
    3 << 12,
    3 << 12,
    3 << 12,
    3 << 12,
    3 << 12,
    3 << 12,
    3 << 12,
    3 << 12,
    4 << 12,
    4 << 12,
    4 << 12,
    4 << 12,
    4 << 12,
    4 << 12,
    4 << 12,
    4 << 12,
    5 << 12,
    5 << 12,
    5 << 12,
    5 << 12,
    5 << 12,
    5 << 12,
    5 << 12,
    5 << 12,
    3 << 12,
    3 << 12,
    4 << 12,
    4 << 12,
    5 << 12,
    5 << 12,
    0x78000000,
    0x7c000000,
];
static DPD2BCD8: [[u8; 4]; 1024] = {
    let mut table = [[0u8; 4]; 1024];
    let mut i = 0;
    while i < 1024 {
        let d0 = (i % 10) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = ((i / 100) % 10) as u8;
        let count = if d2 != 0 {
            3
        } else if d1 != 0 {
            2
        } else if d0 != 0 {
            1
        } else {
            0
        };
        table[i] = [d2, d1, d0, count as u8];
        i += 1;
    }
    table
};
static BIN2DPD: [u16; 1000] = {
    let mut table = [0u16; 1000];
    let mut i = 0;
    while i < 1000 {
        let d0 = (i % 10) as u16;
        let d1 = ((i / 10) % 10) as u16;
        let d2 = ((i / 100) % 10) as u16;
        table[i] = d0 | (d1 << 3) | (d2 << 6);
        i += 1;
    }
    table
};
static BIN2BCD8: [[u8; 4]; 10000] = {
    let mut table = [[0u8; 4]; 10000];
    let mut i = 0;
    while i < 10000 {
        let d0 = (i % 10) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = ((i / 100) % 10) as u8;
        let d3 = ((i / 1000) % 10) as u8;
        let count = if d3 != 0 {
            4
        } else if d2 != 0 {
            3
        } else if d1 != 0 {
            2
        } else if d0 != 0 {
            1
        } else {
            0
        };
        table[i] = [d3, d2, d1, d0 | ((count as u8) << 4)];
        i += 1;
    }
    table
};
/// Check if value is a special (NaN or Infinity)
#[inline]
fn is_special(df: &DecQuad) -> bool {
    (df.get_word(0) & 0x78000000) == 0x78000000
}
/// Check if value is NaN
#[inline]
fn is_nan(df: &DecQuad) -> bool {
    (df.get_word(0) & 0x7c000000) == 0x7c000000
}
/// Check if value is Infinity
#[inline]
fn is_infinity(df: &DecQuad) -> bool {
    (df.get_word(0) & 0x7c000000) == 0x78000000
}
/// Check if value is zero
#[inline]
fn is_zero(df: &DecQuad) -> bool {
    df.get_word(3) == 0 && df.get_word(2) == 0 && df.get_word(1) == 0
        && (df.get_word(0) & 0x1c003fff) == 0
        && (df.get_word(0) & 0x60000000) != 0x60000000
}
/// Check if decQuad is negative (sign bit set)
#[inline]
fn is_negative(df: &DecQuad) -> bool {
    (df.get_word(0) & 0x80000000) != 0
}
/// Check if value is logical (for logical operations)
#[inline]
fn is_logical(df: &DecQuad) -> bool {
    (df.get_word(0) & 0xfbffc000) == 0x22080000 && (df.get_word(0) & !0xffffc912) == 0
        && (df.get_word(1) & !0x44912449) == 0 && (df.get_word(2) & !0x12449124) == 0
        && (df.get_word(3) & !0x49124491) == 0
}
/// Get biased exponent from combination field
fn get_biased_exp(df: &DecQuad) -> i32 {
    let comb = df.get_word(0) >> 26;
    let exp_cont = ((df.get_word(0) & 0x03ffffff) >> (32 - 6 - 12)) as i32;
    (DECCOMBEXP[comb as usize] as i32) + exp_cont
}
/// Extract exponent from decQuad
fn get_exponent(df: &DecQuad) -> i32 {
    let sourhi = df.get_word(0);
    let comb_exp = DECCOMBEXP[(sourhi >> 26) as usize];
    let cont_exp = ((sourhi & 0x03ffffff) >> (32 - 6 - 12)) as i32;
    (comb_exp as i32 + cont_exp) - DECQUAD_BIAS
}
fn decNaNs<'a>(
    result: &'a mut DecQuad,
    dfl: Option<&DecQuad>,
    dfr: Option<&DecQuad>,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if let Some(l) = dfl {
        if l.is_snan_internal() {
            set.status |= DEC_INVALID_OPERATION;
            *result = *l;
            result.set_word(0, (result.get_word(0) & !0x02000000) | 0x7c000000);
            return result;
        }
    }
    if let Some(r) = dfr {
        if r.is_snan_internal() {
            set.status |= DEC_INVALID_OPERATION;
            *result = *r;
            result.set_word(0, (result.get_word(0) & !0x02000000) | 0x7c000000);
            return result;
        }
    }
    if let Some(l) = dfl {
        if l.is_nan_internal() {
            return decCanonical(result, l);
        }
    }
    if let Some(r) = dfr {
        if r.is_nan_internal() {
            return decCanonical(result, r);
        }
    }
    decQuadZero(result);
    result.set_word(0, 0x7c000000);
    result
}
/// Set result to invalid operation
pub fn decInvalid<'a>(result: &'a mut DecQuad, set: &mut DecContext) -> &'a mut DecQuad {
    set.status |= DEC_INVALID_OPERATION;
    result.set_word(0, 0x7c000000);
    result.set_word(1, 0);
    result.set_word(2, 0);
    result.set_word(3, 0);
    result
}
/// Set result to infinity with appropriate sign
pub fn decInfinity<'a>(result: &'a mut DecQuad, df: &DecQuad) -> &'a mut DecQuad {
    let sign = df.get_word(0) & 0x80000000;
    result.set_word(0, 0x78000000 | sign);
    result.set_word(1, 0);
    result.set_word(2, 0);
    result.set_word(3, 0);
    result
}
/// Finite multiplication helper for internal use
pub fn decFiniteMultiply(
    num: &mut BcdNum,
    bcdacc: &mut [u8],
    dfl: &DecQuad,
    dfr: &DecQuad,
) {
    const UNITS: usize = DECQUAD_UNITS;
    const ACC_UNITS: usize = UNITS * 2;
    const BCD_SIZE: usize = ACC_UNITS * 9;
    let mut bufl = [0u32; UNITS];
    let mut bufr = [0u32; UNITS];
    let mut accl = [0u64; ACC_UNITS];
    let mut acc = [0u32; ACC_UNITS];
    num.sign = (dfl.get_word(0) ^ dfr.get_word(0)) & DECFLOAT_SIGN;
    num.exponent = get_exponent(dfl) + get_exponent(dfr);
    extract_coefficient(dfl, &mut bufl);
    extract_coefficient(dfr, &mut bufr);
    for pl in accl.iter_mut() {
        *pl = 0;
    }
    for (ui, &buf_r) in bufr.iter().enumerate() {
        if buf_r == 0 {
            continue;
        }
        for (uj, &buf_l) in bufl.iter().enumerate() {
            accl[ui + uj] += (buf_r as u64) * (buf_l as u64);
        }
    }
    for i in 0..ACC_UNITS {
        if accl[i] >= BILLION as u64 {
            let hop = accl[i] >> 30;
            let est = (hop * 2305843009) >> 31;
            let mut lo = accl[i] - (est * BILLION as u64);
            let mut actual_est = est;
            while lo >= BILLION as u64 {
                lo -= BILLION as u64;
                actual_est += 1;
            }
            acc[i] = lo as u32;
            if i + 1 < ACC_UNITS {
                accl[i + 1] += actual_est;
            }
        } else {
            acc[i] = accl[i] as u32;
        }
    }
    let mut pa = ACC_UNITS - 1;
    if acc[pa] != 0 {
        num.msd_idx = BCD_SIZE - DECQUAD_PMAX * 2;
    } else {
        num.msd_idx = 0;
        pa -= 1;
        while pa > 0 && acc[pa] == 0 {
            pa -= 1;
        }
    }
    let mut ub = 0usize;
    loop {
        if acc[pa] != 0 {
            let top = acc[pa] / 1_000_000;
            let rem1 = acc[pa] % 1_000_000;
            let mid = rem1 / 1000;
            let rem2 = rem1 % 1000;
            store_bcd3(top, &mut bcdacc[ub..]);
            store_bcd3(mid, &mut bcdacc[ub + 3..]);
            store_bcd3(rem2, &mut bcdacc[ub + 6..]);
        } else {
            for j in 0..9 {
                if ub + j < bcdacc.len() {
                    bcdacc[ub + j] = 0;
                }
            }
        }
        if pa == 0 {
            break;
        }
        pa -= 1;
        ub += 9;
    }
    num.lsd_idx = ub + 8;
}
/// Finalize a BCD number into a decQuad
pub fn decFinalize<'a>(
    df: &'a mut DecQuad,
    num: &mut BcdNum,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let length = num.lsd_idx as i32 - num.msd_idx as i32 + 1;
    if num.exponent >= DECFLOAT_INF as i32 {
        let encode = (num.exponent as u32) | num.sign;
        df.set_word(0, encode);
        df.set_word(1, 0);
        df.set_word(2, 0);
        df.set_word(3, 0);
        return df;
    }
    let mut umsd_idx = num.msd_idx;
    let mut ulsd_idx = num.lsd_idx;
    let mut exponent = num.exponent;
    let mut length = length;
    while umsd_idx < ulsd_idx && num.msd[umsd_idx] == 0 {
        umsd_idx += 1;
        length -= 1;
    }
    let drop = std::cmp::max(
        0,
        std::cmp::max(length - DECQUAD_PMAX as i32, DECQUAD_EMIN - 1 - exponent),
    );
    if drop > 0 {
        exponent += drop;
        if drop < length as i32 {
            let roundat = umsd_idx + (length - drop) as usize;
            let reround = num.msd.get(roundat).copied().unwrap_or(0);
            let has_sticky = num.msd[roundat + 1..=ulsd_idx].iter().any(|&d| d != 0);
            let reround = if has_sticky {
                DECSTICKYTAB[reround as usize]
            } else {
                reround
            };
            ulsd_idx = roundat - 1;
            if reround != 0 {
                set.status |= DEC_INEXACT;
                if exponent < -DECQUAD_EMIN
                    && (exponent + (ulsd_idx as i32 - umsd_idx as i32)) < -DECQUAD_EMIN
                {
                    set.status |= DEC_UNDERFLOW;
                }
                let bump = match set.round {
                    Rounding::HalfEven => {
                        if reround > 5 {
                            1
                        } else if reround == 5 {
                            num.msd[ulsd_idx] & 0x01
                        } else {
                            0
                        }
                    }
                    Rounding::Down => 0,
                    Rounding::HalfDown => if reround > 5 { 1 } else { 0 }
                    Rounding::HalfUp => if reround >= 5 { 1 } else { 0 }
                    Rounding::Up => if reround > 0 { 1 } else { 0 }
                    Rounding::Ceiling => if num.sign == 0 && reround > 0 { 1 } else { 0 }
                    Rounding::Floor => if num.sign != 0 && reround > 0 { 1 } else { 0 }
                    Rounding::ZeroFiveUp => {
                        if reround > 0 {
                            let d = num.msd[ulsd_idx];
                            if d == 0 || d == 5 { 1 } else { 0 }
                        } else {
                            0
                        }
                    }
                    Rounding::Max => 0,
                };
                if bump != 0 {
                    let mut idx = ulsd_idx;
                    while idx >= umsd_idx {
                        if num.msd[idx] == 9 {
                            num.msd[idx] = 0;
                            if idx == umsd_idx {
                                num.msd[umsd_idx] = 1;
                                if (ulsd_idx - umsd_idx + 1) == DECQUAD_PMAX as usize {
                                    exponent += 1;
                                } else {
                                    ulsd_idx += 1;
                                    num.msd[ulsd_idx] = 0;
                                }
                                break;
                            }
                        } else {
                            num.msd[idx] += 1;
                            break;
                        }
                        if idx == 0 {
                            break;
                        }
                        idx -= 1;
                    }
                }
            }
        } else {
            num.msd[umsd_idx] = 0;
            ulsd_idx = umsd_idx;
        }
        length = (ulsd_idx - umsd_idx + 1) as i32;
    }
    if exponent > DECQUAD_EMAX - (DECQUAD_PMAX as i32 - 1) {
        if num.msd[ulsd_idx] == 0 && ulsd_idx == umsd_idx {
            exponent = DECQUAD_EMAX - (DECQUAD_PMAX as i32 - 1);
        } else if (exponent + length - 1) > DECQUAD_EMAX {
            set.status |= DEC_OVERFLOW | DEC_INEXACT;
            let need_max = match set.round {
                Rounding::Down | Rounding::ZeroFiveUp => true,
                Rounding::Ceiling => num.sign != 0,
                Rounding::Floor => num.sign == 0,
                _ => false,
            };
            if !need_max {
                num.exponent = DECFLOAT_INF as i32;
                num.msd[umsd_idx] = 0;
                ulsd_idx = umsd_idx;
            } else {
                for i in 0..DECQUAD_PMAX as usize {
                    num.msd[umsd_idx + i] = 9;
                }
                ulsd_idx = umsd_idx + DECQUAD_PMAX as usize - 1;
                exponent = DECQUAD_EMAX - (DECQUAD_PMAX as i32 - 1);
            }
        }
        length = (ulsd_idx - umsd_idx + 1) as i32;
    }
    if length == DECQUAD_PMAX as i32 {
        return decQuadFromBCD_internal(
            df,
            exponent,
            &num.msd[umsd_idx..=ulsd_idx],
            num.sign,
        );
    }
    let encode = if num.exponent >= DECFLOAT_INF as i32 {
        num.exponent as u32
    } else {
        let uexp = (exponent + DECQUAD_BIAS) as u32;
        let code = (uexp >> 12) << 4;
        let mut enc = DECCOMBFROM[code as usize];
        enc |= (uexp << (32 - 6 - 12)) & 0x03ffffff;
        enc
    };
    let encode = encode | num.sign;
    let bcd = &num.msd[umsd_idx..=ulsd_idx];
    encode_dpd_coefficient(df, encode, bcd);
    df
}
/// Convert to integral value with specified rounding
fn decToIntegral<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
    rmode: Rounding,
    exact: bool,
) -> &'a mut DecQuad {
    let sourhi = df.get_word(0);
    let exp = DECCOMBEXP[(sourhi >> 26) as usize];
    if exp >= 0x78000000 {
        if df.is_nan_internal() {
            return decNaNs(result, Some(df), None, set);
        }
        return decInfinity(result, df);
    }
    let full_exp = (exp as i32) + ((sourhi & 0x03ffffff) >> (32 - 6 - 12)) as i32 - DECQUAD_BIAS;
    if full_exp >= 0 {
        return decCanonical(result, df);
    }
    let saveround = set.round;
    let savestatus = set.status;
    set.round = rmode;
    let mut zero = DecQuad::default();
    decQuadZero(&mut zero);
    decQuadQuantize(result, df, &zero, set);
    set.round = saveround;
    if !exact {
        set.status = savestatus;
    }
    result
}
/// Multiply two DecQuad numbers
pub fn decQuadMultiply<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if dfl.is_special() || dfr.is_special() {
        if dfl.is_snan() || dfr.is_snan() {
            return decNaNs(result, Some(dfl), Some(dfr), set);
        }
        if dfl.is_nan() || dfr.is_nan() {
            return decNaNs(result, Some(dfl), Some(dfr), set);
        }
        if dfl.is_infinite() || dfr.is_infinite() {
            if dfl.is_zero() || dfr.is_zero() {
                return decInvalid(result, set);
            }
            let sign = (dfl.get_word(0) ^ dfr.get_word(0)) & DECFLOAT_SIGN;
            // Inline decInfinity to avoid borrowing result twice
            result.set_word(0, 0x78000000 | sign);
            result.set_word(1, 0);
            result.set_word(2, 0);
            result.set_word(3, 0);
            return result;
        }
    }
    let mut num = BcdNum::default();
    let mut acc = vec![0u8; 256];
    decFiniteMultiply(&mut num, &mut acc, dfl, dfr);
    decFinalize(result, &mut num, set)
}
/// Get the class of a DecQuad value
pub fn decQuadClass(df: &DecQuad) -> DecClass {
    if is_special(df) {
        if (df.get_word(0) & 0x7e000000) == 0x7c000000 {
            return DecClass::Qnan;
        }
        if (df.get_word(0) & 0x7e000000) == 0x7e000000 {
            return DecClass::Snan;
        }
        if is_negative(df) {
            return DecClass::NegInf;
        }
        return DecClass::PosInf;
    }
    if is_zero(df) {
        if is_negative(df) {
            return DecClass::NegZero;
        }
        return DecClass::PosZero;
    }
    let exp = get_exponent(df) + decQuadDigits(df) as i32 - 1;
    if exp >= DECQUAD_EMIN {
        if is_negative(df) {
            return DecClass::NegNormal;
        }
        return DecClass::PosNormal;
    }
    if is_negative(df) { DecClass::NegSubnormal } else { DecClass::PosSubnormal }
}
/// Convert to integral exact (sets inexact flag if not integral)
pub fn decQuadToIntegralExact<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    decToIntegral(result, df, set, set.round, true)
}
/// XOR two DecQuad values (logical operation)
pub fn decQuadXor<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if !is_logical(dfl) || !is_logical(dfr) {
        return decInvalid(result, set);
    }
    result.set_word(0, 0x22080000 | ((dfl.get_word(0) ^ dfr.get_word(0)) & 0x04000912));
    result.set_word(1, (dfl.get_word(1) ^ dfr.get_word(1)) & 0x44912449);
    result.set_word(2, (dfl.get_word(2) ^ dfr.get_word(2)) & 0x12449124);
    result.set_word(3, (dfl.get_word(3) ^ dfr.get_word(3)) & 0x49124491);
    result
}
/// Copy with sign negated
pub fn decQuadCopyNegate<'a>(
    result: &'a mut DecQuad,
    source: &DecQuad,
) -> &'a mut DecQuad {
    *result = *source;
    let word0 = result.get_word(0);
    result.set_word(0, word0 ^ DECFLOAT_SIGN);
    result
}
/// Copy with absolute value (clear sign)
pub fn decQuadCopyAbs<'a>(result: &'a mut DecQuad, dfl: &DecQuad) -> &'a mut DecQuad {
    if !std::ptr::eq(result, dfl) {
        *result = *dfl;
    }
    result.bytes[15] &= !0x80;
    result
}
/// Check if value is infinite
pub fn decQuadIsInfinite(df: &DecQuad) -> u32 {
    if (df.get_word(0) & 0x7c000000) == 0x78000000 { 1 } else { 0 }
}
/// Get class as string
pub fn decQuadClassString(df: &DecQuad) -> &'static str {
    let eclass = decQuadClass(df);
    match eclass {
        DecClass::PosNormal => "+Normal",
        DecClass::NegNormal => "-Normal",
        DecClass::PosZero => "+Zero",
        DecClass::NegZero => "-Zero",
        DecClass::PosSubnormal => "+Subnormal",
        DecClass::NegSubnormal => "-Subnormal",
        DecClass::PosInf => "+Infinity",
        DecClass::NegInf => "-Infinity",
        DecClass::Qnan => "NaN",
        DecClass::Snan => "sNaN",
    }
}
/// Get number of significant digits
pub fn decQuadDigits(df: &DecQuad) -> u32 {
    let sourhi = df.get_word(0);
    if (df.get_word(0) & 0x7c000000) == 0x78000000 {
        return 1;
    }
    if (df.get_word(0) & 0x7c000000) != 0x7c000000
        && DECCOMBMSD[(sourhi >> 26) as usize] != 0
    {
        return DECQUAD_PMAX as u32;
    }
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    if sourhi & 0x00003fff != 0 {
        let dpd = (sourhi >> 4) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 1;
        }
        let dpd = (((sourhi) << 6) | (sourmh >> 26)) & 0x3ff;
        if dpd == 0 {
            return 1;
        }
        return DECQUAD_PMAX as u32 - 4;
    }
    if sourmh != 0 {
        let dpd = (sourmh >> 26) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 4;
        }
        let dpd = (sourmh >> 16) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 7;
        }
        let dpd = (sourmh >> 6) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 10;
        }
        let dpd = (((sourmh) << 4) | (sourml >> 28)) & 0x3ff;
        if dpd == 0 {
            return 1;
        }
        return DECQUAD_PMAX as u32 - 13;
    }
    if sourml != 0 {
        let dpd = (sourml >> 28) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 13;
        }
        let dpd = (sourml >> 18) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 16;
        }
        let dpd = (sourml >> 8) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 19;
        }
        let dpd = (((sourml) << 2) | (sourlo >> 30)) & 0x3ff;
        if dpd == 0 {
            return 1;
        }
        return DECQUAD_PMAX as u32 - 22;
    }
    if sourlo & 0xfff00000 != 0 {
        let dpd = (sourlo >> 30) & 0x3ff;
        if dpd != 0 {
            return DECQUAD_PMAX as u32 - 22;
        }
        let dpd = (sourlo >> 20) & 0x3ff;
        if dpd == 0 {
            return 1;
        }
        return DECQUAD_PMAX as u32 - 25;
    }
    let dpd = (sourlo >> 10) & 0x3ff;
    if dpd != 0 {
        return DECQUAD_PMAX as u32 - 28;
    }
    let dpd = sourlo & 0x3ff;
    if dpd == 0 {
        return 1;
    }
    DECQUAD_PMAX as u32 - 31
}
/// Convert to engineering string format
pub fn decQuadToEngString<'a>(df: &DecQuad, string: &'a mut String) -> &'a mut String {
    string.clear();
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    if (sourhi as i32) < 0 {
        string.push('-');
    }
    let comb = sourhi >> 26;
    let msd = DECCOMBMSD[comb as usize];
    let mut exp = DECCOMBEXP[comb as usize] as i32;
    if exp >= 0x78000000_u32 as i32 {
        if exp == 0x78000000_u32 as i32 {
            string.push_str("Infinity");
            return string;
        }
        if (sourhi & 0x02000000) != 0 {
            string.push('s');
        }
        string.push_str("NaN");
        if sourlo == 0 && sourml == 0 && sourmh == 0 && (sourhi & 0x00003fff) == 0 {
            return string;
        }
        return string;
    }
    let exp_cont = ((sourhi & 0x03ffffff) >> (32 - 6 - 12)) as i32;
    exp = exp + exp_cont - DECQUAD_BIAS;
    if msd != 0 {
        string.push((b'0' + msd as u8) as char);
    }
    if string.len() == 1 || (string.len() == 2 && string.starts_with('-')) {
        string.push('0');
    }
    if exp != 0 {
        let digits = decQuadDigits(df) as i32;
        let mut e = exp + digits - 1;
        let pre = 1;
        let adj = if e < 0 {
            let a = (-e) % 3;
            if a != 0 { 3 - a } else { 0 }
        } else {
            e % 3
        };
        e -= adj;
        if e != 0 {
            string.push('E');
            if e < 0 {
                string.push('-');
                e = -e;
            } else {
                string.push('+');
            }
            string.push_str(&e.to_string());
        }
    }
    string
}
/// Compute LogB (adjusted exponent)
pub fn decQuadLogB<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if is_nan(df) {
        return decNaNs(result, Some(df), None, set);
    }
    if is_infinity(df) {
        // Inline decInfinity - use sign=0 since we set word(0)=0 above
        let sign = result.get_word(0) & 0x80000000;
        result.set_word(0, 0x78000000 | sign);
        result.set_word(1, 0);
        result.set_word(2, 0);
        result.set_word(3, 0);
        return result;
    }
    if is_zero(df) {
        set.status |= DEC_DIVISION_BY_ZERO;
        // Inline decInfinity with negative sign
        result.set_word(0, 0x78000000 | 0x80000000);
        result.set_word(1, 0);
        result.set_word(2, 0);
        result.set_word(3, 0);
        return result;
    }
    let ae = get_exponent(df) + decQuadDigits(df) as i32 - 1;
    result.set_word(0, 0x22080000);
    if ae < 0 {
        result.set_word(0, result.get_word(0) | 0x80000000);
    }
    let ae_abs = ae.abs() as u32;
    result.set_word(1, 0);
    result.set_word(2, 0);
    result
        .set_word(3, ((ae_abs / 1000) << 10) | BIN2DPD[(ae_abs % 1000) as usize] as u32);
    result
}
const SIGN_MASK: u32 = 0x80000000;
const NAN_MASK: u32 = 0x7c000000;
const NAN_VALUE: u32 = 0x7c000000;
const INF_MASK: u32 = 0x7c000000;
const INF_VALUE: u32 = 0x78000000;
static DPD2BIN: [u16; 1024] = {
    let mut table = [0u16; 1024];
    let mut i = 0;
    while i < 1024 {
        let d0 = (i & 0x007) as u16;
        let d1 = ((i >> 3) & 0x007) as u16;
        let d2 = ((i >> 6) & 0x007) as u16;
        table[i] = d0 + d1 * 10 + d2 * 100;
        i += 1;
    }
    table
};
/// DPD to binary conversion table (thousands place)
static DPD2BINK: [u32; 1024] = {
    let mut table = [0u32; 1024];
    table
};
/// DPD to binary conversion table (millions place)
static DPD2BINM: [u32; 1024] = {
    let mut table = [0u32; 1024];
    table
};
fn dpd_to_bin(dpd: u32) -> u32 {
    let dpd = dpd & 0x3ff;
    let a = (dpd >> 9) & 1;
    let b = (dpd >> 8) & 1;
    let c = (dpd >> 7) & 1;
    let d = (dpd >> 6) & 1;
    let e = (dpd >> 5) & 1;
    let f = (dpd >> 4) & 1;
    let g = (dpd >> 3) & 1;
    let h = (dpd >> 2) & 1;
    let i = (dpd >> 1) & 1;
    let j = dpd & 1;
    let d1: u32;
    let d2: u32;
    let d3: u32;
    if g == 0 {
        d1 = 4 * a + 2 * b + c;
        if h == 0 {
            d2 = 4 * d + 2 * e + f;
            d3 = 4 * 0 + 2 * i + j;
        } else {
            d2 = 4 * d + 2 * e + f;
            d3 = 4 * 1 + 2 * i + j;
        }
    } else {
        d1 = 4 * a + 2 * b + c;
        d2 = 4 * d + 2 * e + f;
        d3 = 4 * 0 + 2 * i + j;
    }
    d1 * 100 + d2 * 10 + d3
}
fn get_combination_exp(hi: u32) -> i32 {
    let comb = (hi >> 26) & 0x1f;
    if comb < 24 {
        ((comb >> 3) as i32) << 12
    } else if comb < 30 {
        ((comb & 0x03) as i32) << 12
    } else if comb == 30 {
        0x78000000
    } else {
        0x7c000000
    }
}
/// Check if a decQuad is signed (negative)
pub fn decQuadIsSigned(df: &DecQuad) -> u32 {
    if (df.get_word(0) & SIGN_MASK) != 0 { 1 } else { 0 }
}
/// Convert decQuad to signed 32-bit integer
pub fn decQuadToInt32(df: &DecQuad, set: &mut DecContext, round: Rounding) -> i32 {
    decToInt32(df, set, round, 0, 0) as i32
}
/// Convert decQuad to unsigned 32-bit integer
pub fn decQuadToUInt32(df: &DecQuad, set: &mut DecContext, round: Rounding) -> u32 {
    decToInt32(df, set, round, 0, 1)
}
/// Internal function to convert decQuad to 32-bit integer
fn decToInt32(
    df: &DecQuad,
    set: &mut DecContext,
    rmode: Rounding,
    exact: u8,
    unsign: u8,
) -> u32 {
    let sourhi = df.get_word(0);
    let exp = get_combination_exp(sourhi);
    if exp >= 0x78000000 {
        set.status |= DEC_INVALID_OPERATION;
        return 0;
    }
    let actual_exp = get_exponent(df);
    let result: DecQuad;
    if actual_exp == 0 {
        result = *df;
    } else {
        let saveround = set.round;
        let savestatus = set.status;
        set.round = rmode;
        let mut zero = DecQuad::default();
        decQuadZero(&mut zero);
        set.status = 0;
        let mut temp_result = DecQuad::default();
        decQuadQuantize(&mut temp_result, df, &zero, set);
        result = temp_result;
        set.round = saveround;
        if exact != 0 {
            set.status |= savestatus;
        } else {
            set.status = savestatus;
        }
    }
    let res_word0 = result.get_word(0);
    let res_word1 = result.get_word(1);
    let res_word2 = result.get_word(2);
    if (res_word2 & 0xffffff00) != 0 || res_word1 != 0 || (res_word0 & 0x1c003fff) != 0
        || (res_word0 & 0x60000000) == 0x60000000
    {
        set.status |= DEC_INVALID_OPERATION;
        return 0;
    }
    let sourlo = result.get_word(3);
    let lo = dpd_to_bin(sourlo & 0x3ff) + dpd_to_bin((sourlo >> 10) & 0x3ff) * 1000
        + dpd_to_bin((sourlo >> 20) & 0x3ff) * 1000000;
    let sourpen = result.get_word(2);
    let hi = dpd_to_bin(((sourpen << 2) | (sourlo >> 30)) & 0x3ff);
    if unsign != 0 {
        if hi > 4 || (hi == 4 && lo > 294967295)
            || (hi + lo != 0 && (res_word0 & SIGN_MASK) != 0)
        {
            set.status |= DEC_INVALID_OPERATION;
            return 0;
        }
        return hi * 1000000000 + lo;
    }
    if hi > 2 || (hi == 2 && lo > 147483647) {
        if lo == 147483648 && hi == 2 && (res_word0 & SIGN_MASK) != 0 {
            return 0x80000000;
        }
        set.status |= DEC_INVALID_OPERATION;
        return 0;
    }
    let mut i = (hi * 1000000000 + lo) as i32;
    if (res_word0 & SIGN_MASK) != 0 {
        i = -i;
    }
    i as u32
}
/// Compare two decQuad values
pub fn decQuadCompare<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (dfl.get_word(0) & NAN_MASK) == NAN_VALUE || (dfr.get_word(0) & NAN_MASK) == NAN_VALUE {
        return decNaNs(result, Some(dfl), Some(dfr), set);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    decQuadZero(result);
    if comp == 0 {
        return result;
    }
    result.bytes[0] = 0x01;
    if comp < 0 {
        result.bytes[15] |= 0x80;
    }
    result
}
/// Create decQuad from packed BCD
pub fn decQuadFromPacked<'a>(df: &'a mut DecQuad, exp: i32, packed: &[u8]) -> &'a mut DecQuad {
    let mut bcdar = [0u8; DECQUAD_PMAX + 2];
    let mut op = 0usize;
    for ip in 0..((DECQUAD_PMAX + 2) / 2) {
        if ip < packed.len() {
            bcdar[op] = packed[ip] >> 4;
            op += 1;
            bcdar[op] = packed[ip] & 0x0f;
            op += 1;
        }
    }
    let sig = if op > 0 {
        let sign_nibble = bcdar[op - 1];
        if sign_nibble == 0x0D || sign_nibble == 0x0B { SIGN_MASK } else { 0 }
    } else {
        0
    };
    if exp >= 0x78000000 {
        if exp != 0x78000000 {
            bcdar[1] = 0;
        } else {
            for i in 1..=DECQUAD_PMAX {
                bcdar[i] = 0;
            }
        }
    }
    decQuadFromBCD(df, exp, &bcdar[1..], sig as i32)
}
/// Check if decQuad is NaN
pub fn decQuadIsNaN(df: &DecQuad) -> u32 {
    if (df.get_word(0) & NAN_MASK) == NAN_VALUE { 1 } else { 0 }
}
/// Shift decQuad coefficient
pub fn decQuadShift<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (dfl.get_word(0) & NAN_MASK) == NAN_VALUE || (dfr.get_word(0) & NAN_MASK) == NAN_VALUE {
        return decNaNs(result, Some(dfl), Some(dfr), set);
    }
    let dfr_word0 = dfr.get_word(0);
    if !((dfr_word0 & 0x63ffc000) == 0x22080000
        || (dfr_word0 & 0x7bffc000) == 0x6a080000)
    {
        return decInvalid(result, set);
    }
    let digits = decQuadDigits(dfr);
    if digits > 2 {
        return decInvalid(result, set);
    }
    let shift = dpd_to_bin(dfr.get_word(3) & 0x3ff) as i32;
    if shift > DECQUAD_PMAX as i32 {
        return decInvalid(result, set);
    }
    if (dfl.get_word(0) & INF_MASK) == INF_VALUE {
        return decInfinity(result, dfl);
    }
    if shift == 0 {
        return decCanonical(result, dfl);
    }
    if shift == DECQUAD_PMAX as i32 {
        let sign = dfl.bytes[15] & 0x80;
        decQuadZero(result);
        result.bytes[15] |= sign;
        return result;
    }
    let mut buf = [0u8; DECQUAD_PMAX * 2];
    extract_bcd_from_quad(dfl, &mut buf[..DECQUAD_PMAX]);
    let sign = dfl.get_word(0) & SIGN_MASK;
    let exponent = get_exponent(dfl);
    let mut num = BcdNum {
        msd: buf.to_vec(),
        msd_idx: 0,
        lsd_idx: DECQUAD_PMAX - 1,
        sign,
        exponent,
    };
    if (dfr.get_word(0) & SIGN_MASK) != 0 {
        num.lsd_idx = DECQUAD_PMAX - shift as usize - 1;
    } else {
        for i in DECQUAD_PMAX..(DECQUAD_PMAX + shift as usize) {
            if i < buf.len() {
                buf[i] = 0;
            }
        }
        num.msd_idx = shift as usize;
        num.lsd_idx = num.msd_idx + DECQUAD_PMAX - 1;
    }
    let savestat = set.status;
    decFinalize(result, &mut num, set);
    set.status = savestat;
    result
}
/// Get the radix (base) of decQuad - always 10
pub fn decQuadRadix(_df: &DecQuad) -> u32 {
    10
}
/// Return canonical form of decQuad
pub fn decQuadCanonical<'a>(result: &'a mut DecQuad, df: &DecQuad) -> &'a mut DecQuad {
    decCanonical(result, df)
}
fn extract_bcd_from_quad(df: &DecQuad, buf: &mut [u8]) {
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    let comb = (sourhi >> 26) & 0x1f;
    buf[0] = if comb < 24 { (comb & 0x07) as u8 } else { (8 + (comb & 0x01)) as u8 };
    let mut idx = 1usize;
    let declets = [
        (sourhi >> 4) & 0x3ff,
        ((sourhi << 6) | (sourmh >> 26)) & 0x3ff,
        (sourmh >> 16) & 0x3ff,
        (sourmh >> 6) & 0x3ff,
        ((sourmh << 4) | (sourml >> 28)) & 0x3ff,
        (sourml >> 18) & 0x3ff,
        (sourml >> 8) & 0x3ff,
        ((sourml << 2) | (sourlo >> 30)) & 0x3ff,
        (sourlo >> 20) & 0x3ff,
        (sourlo >> 10) & 0x3ff,
        sourlo & 0x3ff,
    ];
    for declet in declets {
        let value = dpd_to_bin(declet);
        if idx + 2 < buf.len() {
            buf[idx] = ((value / 100) % 10) as u8;
            buf[idx + 1] = ((value / 10) % 10) as u8;
            buf[idx + 2] = (value % 10) as u8;
            idx += 3;
        }
    }
}
/// Set DecQuad to zero
pub fn decQuadZero<'a>(result: &'a mut DecQuad) -> &'a mut DecQuad {
    result.set_word(0, 0x22080000);
    result.set_word(1, 0);
    result.set_word(2, 0);
    result.set_word(3, 0);
    result
}
/// Quantize dfl to have the same exponent as dfr
pub fn decQuadQuantize<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let sourhil = dfl.get_word(0);
    let sourhir = dfr.get_word(0);
    let explb = DECCOMBEXP[(sourhil >> 26) as usize] as i32;
    let exprb = DECCOMBEXP[(sourhir >> 26) as usize] as i32;
    if explb >= 0x78000000 || exprb >= 0x78000000 {
        if is_nan(dfl) || is_nan(dfr) {
            return decNaNs(result, Some(dfl), Some(dfr), set);
        }
        if is_inf(dfl) != is_inf(dfr) {
            return decInvalid(result, set);
        }
        return decInfinity(result, dfl);
    }
    let explb = explb + get_exponent_continuation(dfl);
    let exprb = exprb + get_exponent_continuation(dfr);
    let drop = exprb - explb;
    if drop == 0 {
        return decCanonical(result, dfl);
    }
    let mut buf = [0u8; 4 + DECQUAD_PMAX as usize * 3 + 2];
    let buf_offset = 4 + DECQUAD_PMAX as usize;
    decode_to_bcd(dfl, &mut buf[buf_offset..buf_offset + DECQUAD_PMAX as usize]);
    if drop > 0 {
        if drop >= DECQUAD_PMAX as i32 {
            decQuadZero(result);
            let sign = sourhil & 0x80000000;
            result.set_word(0, result.get_word(0) | sign);
        } else {
            apply_rounding_and_encode(
                result,
                &buf,
                buf_offset,
                drop as usize,
                exprb,
                sourhil & 0x80000000,
                set,
            );
        }
    } else {
        let shift = (-drop) as usize;
        if shift > (DECQUAD_PMAX as usize - 1) {
            let is_zero = buf[buf_offset..buf_offset + DECQUAD_PMAX as usize]
                .iter()
                .all(|&b| b == 0);
            if !is_zero {
                return decInvalid(result, set);
            }
        }
        let mut shifted_buf = [0u8; DECQUAD_PMAX as usize];
        let copy_len = (DECQUAD_PMAX as usize).saturating_sub(shift);
        if copy_len > 0 && shift < DECQUAD_PMAX as usize {
            shifted_buf[..copy_len]
                .copy_from_slice(
                    &buf[buf_offset + shift..buf_offset + shift + copy_len],
                );
        }
        encode_from_bcd(result, &shifted_buf, exprb, sourhil & 0x80000000);
    }
    result
}
/// Create decQuad from BCD representation
pub fn decQuadFromBCD<'a>(
    df: &'a mut DecQuad,
    exp: i32,
    bcdar: &[u8],
    sig: i32,
) -> &'a mut DecQuad {
    assert!(bcdar.len() >= DECQUAD_PMAX);
    let mut encode: u32;
    if exp >= DECFLOAT_INF as i32 {
        encode = (exp as u32) | (sig as u32);
    } else {
        let uexp = (exp + DECQUAD_BIAS) as u32;
        let mut code = (uexp >> 12) << 4;
        code += bcdar[0] as u32;
        encode = DECCOMBFROM[code as usize] | (sig as u32);
        encode |= (uexp << (32 - 6 - 12)) & 0x03ffffff;
    }
    let mut dpd: u16;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 30..]);
    encode |= (dpd as u32) << 4;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 27..]);
    encode |= (dpd as u32) >> 6;
    set_decquad_word(df, 0, encode);
    encode = (dpd as u32) << 26;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 24..]);
    encode |= (dpd as u32) << 16;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 21..]);
    encode |= (dpd as u32) << 6;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 18..]);
    encode |= (dpd as u32) >> 4;
    set_decquad_word(df, 1, encode);
    encode = (dpd as u32) << 28;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 15..]);
    encode |= (dpd as u32) << 18;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 12..]);
    encode |= (dpd as u32) << 8;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 9..]);
    encode |= (dpd as u32) >> 2;
    set_decquad_word(df, 2, encode);
    encode = (dpd as u32) << 30;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 6..]);
    encode |= (dpd as u32) << 20;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 3..]);
    encode |= (dpd as u32) << 10;
    dpd = bcd3_to_dpd(&bcdar[DECQUAD_PMAX - 1 - 0..]);
    encode |= dpd as u32;
    set_decquad_word(df, 3, encode);
    df
}
/// Number of 32-bit words in decQuad
const DECQUAD_WORDS: usize = 4;
/// Units per decQuad for internal calculations
const DECQUAD_DECLETS: usize = 11;
/// Number of 9-digit units needed
const DECQUAD_UNITS: usize = 4;
/// Sign bit mask
const DECFLOAT_SIGN: u32 = 0x80000000;
/// Infinity mask
const DECFLOAT_INF: u32 = 0x78000000;
/// NaN mask
const DECFLOAT_NAN: u32 = 0x7c000000;
/// Signaling NaN mask
const DECFLOAT_SNAN: u32 = 0x7e000000;
/// Special value mask
const DECFLOAT_SPECIAL: u32 = 0x78000000;
/// Maximum value for a billion (for unit operations)
const BILLION: u32 = 1_000_000_000;
static BCD2DPD: [u16; 4096] = {
    let mut table = [0u16; 4096];
    let mut i = 0;
    while i < 4096 {
        let d0 = (i & 0x0F) as u16;
        let d1 = ((i >> 4) & 0x0F) as u16;
        let d2 = ((i >> 8) & 0x0F) as u16;
        if d0 < 10 && d1 < 10 && d2 < 10 {
            table[i] = d0 | (d1 << 3) | (d2 << 6);
        }
        i += 1;
    }
    table
};
static DECCOMBFROM: [u32; 160] = [
    0x00000000, 0x04000000, 0x08000000, 0x0C000000, 0x10000000, 0x14000000, 0x18000000,
    0x1C000000, 0x60000000, 0x64000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x20000000, 0x24000000, 0x28000000, 0x2C000000, 0x30000000,
    0x34000000, 0x38000000, 0x3C000000, 0x68000000, 0x6C000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x40000000, 0x44000000, 0x48000000,
    0x4C000000, 0x50000000, 0x54000000, 0x58000000, 0x5C000000, 0x70000000, 0x74000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x04000000, 0x08000000, 0x0C000000, 0x10000000, 0x14000000, 0x18000000, 0x1C000000,
    0x60000000, 0x64000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x20000000, 0x24000000, 0x28000000, 0x2C000000, 0x30000000, 0x34000000,
    0x38000000, 0x3C000000, 0x68000000, 0x6C000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x40000000, 0x44000000, 0x48000000, 0x4C000000,
    0x50000000, 0x54000000, 0x58000000, 0x5C000000, 0x70000000, 0x74000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x04000000,
    0x08000000, 0x0C000000, 0x10000000, 0x14000000, 0x18000000, 0x1C000000, 0x60000000,
    0x64000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x20000000, 0x24000000, 0x28000000, 0x2C000000, 0x30000000, 0x34000000, 0x38000000,
    0x3C000000, 0x68000000, 0x6C000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x40000000, 0x44000000, 0x48000000, 0x4C000000, 0x50000000,
    0x54000000, 0x58000000, 0x5C000000, 0x70000000, 0x74000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x04000000, 0x08000000,
    0x0C000000, 0x10000000, 0x14000000, 0x18000000, 0x1C000000, 0x60000000, 0x64000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];
#[inline]
fn decquad_word(df: &DecQuad, index: usize) -> u32 {
    df.get_word(index)
}
#[inline]
fn set_decquad_word(df: &mut DecQuad, index: usize, value: u32) {
    df.set_word(index, value);
}
/// Check if value is a signaling NaN
pub fn decQuadIsSignalling(df: &DecQuad) -> u32 {
    if (df.get_word(0) & 0x7e000000) == 0x7e000000 { 1 } else { 0 }
}
/// Check if value is positive
pub fn decQuadIsPositive(df: &DecQuad) -> u32 {
    let is_signed = (df.get_word(0) & DECFLOAT_SIGN) != 0;
    let is_zero = df.is_zero();
    let is_nan = df.is_nan();
    if !is_signed && !is_zero && !is_nan { 1 } else { 0 }
}
/// Check if value is negative
pub fn decQuadIsNegative(df: &DecQuad) -> u32 {
    let is_signed = (df.get_word(0) & DECFLOAT_SIGN) != 0;
    let is_zero = df.is_zero();
    let is_nan = df.is_nan();
    if is_signed && !is_zero && !is_nan { 1 } else { 0 }
}
/// Canonicalize a decQuad
pub fn decCanonical<'a>(result: &'a mut DecQuad, df: &DecQuad) -> &'a mut DecQuad {
    if result as *const _ != df as *const _ {
        *result = *df;
    }
    if is_special(result) {
        if is_inf(result) {
            return decInfinity(result, df);
        }
        let w0 = result.get_word(0);
        result.set_word(0, w0 & !(0x01FFFFFF >> (32 - 6 - 12) << (32 - 6 - 12)));
        if is_zero(df) {
            return result;
        }
    }
    result
}
/// Divide operation (internal)
fn decDivide<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
    op: u32,
) -> &'a mut DecQuad {
    const DIVIDE: u32 = 0x80000000;
    const DIVIDEINT: u32 = 0x40000000;
    const REMAINDER: u32 = 0x20000000;
    const REMNEAR: u32 = 0x10000000;
    let sign = (dfl.get_word(0) ^ dfr.get_word(0)) & DECFLOAT_SIGN;
    if is_special(dfl) || is_special(dfr) {
        if is_nan(dfl) || is_nan(dfr) {
            return decNaNs(result, Some(dfl), Some(dfr), set);
        }
        if is_inf(dfl) {
            if is_inf(dfr) {
                return decInvalid(result, set);
            }
            if (op & (DIVIDEINT | REMNEAR)) != 0 {
                return decInvalid(result, set);
            }
            // Inline decInfinity with sign
            result.set_word(0, 0x78000000 | sign);
            result.set_word(1, 0);
            result.set_word(2, 0);
            result.set_word(3, 0);
            return result;
        }
        if (op & (DIVIDEINT | REMNEAR)) != 0 {
            return decCanonical(result, dfl);
        }
        decQuadZero(result);
        if op == REMAINDER {
            let w0 = result.get_word(0);
            result.set_word(0, w0 | sign);
        } else {
            result.set_word(0, sign);
        }
        return result;
    }
    if is_zero(dfr) {
        if is_zero(dfl) {
            decQuadZero(result);
            result.set_word(0, DECFLOAT_NAN);
            set.status |= DEC_INVALID_OPERATION;
            return result;
        }
        if (op & (DIVIDEINT | REMNEAR)) != 0 {
            return decInvalid(result, set);
        }
        set.status |= DEC_DIVISION_BY_ZERO;
        // Inline decInfinity with sign
        result.set_word(0, 0x78000000 | sign);
        result.set_word(1, 0);
        result.set_word(2, 0);
        result.set_word(3, 0);
        return result;
    }
    if is_zero(dfl) {
        if op == REMAINDER {
            decQuadZero(result);
            let w0 = result.get_word(0);
            result.set_word(0, w0 | sign);
            return result;
        }
        if op != DIVIDE {
            let lexp = get_exponent(dfl);
            let rexp = get_exponent(dfr);
            let exp = lexp.max(rexp);
            decQuadZero(result);
            return result;
        }
        decQuadZero(result);
        let w0 = result.get_word(0);
        result.set_word(0, w0 | sign);
        return result;
    }
    decQuadZero(result);
    let w0 = result.get_word(0);
    result.set_word(0, w0 | sign);
    result
}
/// Compute remainder near
pub fn decQuadRemainderNear<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    decDivide(result, dfl, dfr, set, 0x10000000)
}
/// Compute absolute value
pub fn decQuadAbs<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (df.get_word(0) & DECFLOAT_NAN) == DECFLOAT_NAN {
        return decNaNs(result, Some(df), None, set);
    }
    decCanonical(result, df);
    result.bytes[DECQUAD_BYTES - 1 - 0] &= !0x80;
    result
}
/// Convert to BCD representation
pub fn decQuadToBCD(df: &DecQuad, exp: &mut i32, bcdar: &mut [u8]) -> i32 {
    assert!(bcdar.len() >= DECQUAD_PMAX);
    if (df.get_word(0) & 0x7c000000) == DECFLOAT_INF {
        for b in bcdar[..DECQUAD_PMAX].iter_mut() {
            *b = 0;
        }
        *exp = (df.get_word(0) & 0x7e000000) as i32;
    } else {
        let sourhi = df.get_word(0);
        let sourmh = df.get_word(1);
        let sourml = df.get_word(2);
        let sourlo = df.get_word(3);
        bcdar[0] = DECCOMBMSD[(sourhi >> 26) as usize] as u8;
        extract_dpd_to_bcd(sourhi, sourmh, sourml, sourlo, bcdar);
        if (df.get_word(0) & DECFLOAT_NAN) == DECFLOAT_NAN {
            bcdar[0] = 0;
            *exp = (df.get_word(0) & 0x7e000000) as i32;
        } else {
            *exp = get_exponent(df);
        }
    }
    (df.get_word(0) & DECFLOAT_SIGN) as i32
}
/// Helper to extract DPD to BCD
fn extract_dpd_to_bcd(
    sourhi: u32,
    sourmh: u32,
    sourml: u32,
    sourlo: u32,
    bcdar: &mut [u8],
) {
    let dpd = (sourhi >> 4) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[1..4]);
    let dpd = ((sourhi << 6) | (sourmh >> 26)) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[4..7]);
    let dpd = (sourmh >> 16) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[7..10]);
    let dpd = (sourmh >> 6) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[10..13]);
    let dpd = ((sourmh << 4) | (sourml >> 28)) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[13..16]);
    let dpd = (sourml >> 18) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[16..19]);
    let dpd = (sourml >> 8) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[19..22]);
    let dpd = ((sourml << 2) | (sourlo >> 30)) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[22..25]);
    let dpd = (sourlo >> 20) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[25..28]);
    let dpd = (sourlo >> 10) & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[28..31]);
    let dpd = sourlo & 0x3ff;
    dpd_to_bcd3(dpd, &mut bcdar[31..34]);
}
/// Convert DPD declet to 3 BCD digits
fn dpd_to_bcd3(dpd: u32, bcd: &mut [u8]) {
    let d0 = ((dpd >> 7) & 0x7) as u8;
    let d1 = ((dpd >> 4) & 0x7) as u8;
    let d2 = (dpd & 0xf) as u8;
    if bcd.len() >= 3 {
        bcd[0] = d0.min(9);
        bcd[1] = d1.min(9);
        bcd[2] = d2.min(9);
    }
}
/// Convert BCD to DPD declet
fn bcd3_to_dpd(bcd: &[u8]) -> u16 {
    if bcd.len() >= 3 {
        let idx = (bcd[0] as usize * 256) + (bcd[1] as usize * 16) + bcd[2] as usize;
        if idx < BCD2DPD.len() { BCD2DPD[idx] } else { 0 }
    } else {
        0
    }
}
/// Create decQuad from packed decimal with validation
pub fn decQuadFromPackedChecked<'a>(
    df: &'a mut DecQuad,
    exp: i32,
    packed: &[u8],
) -> Option<&'a mut DecQuad> {
    const PACKED_LEN: usize = (DECQUAD_PMAX + 2) / 2;
    if packed.len() < PACKED_LEN {
        return None;
    }
    let mut bcdar = [0u8; DECQUAD_PMAX + 2];
    let mut op = 0usize;
    for ip in 0..PACKED_LEN {
        bcdar[op] = packed[ip] >> 4;
        if bcdar[op] > 9 {
            return None;
        }
        op += 1;
        bcdar[op] = packed[ip] & 0x0f;
        if bcdar[op] > 9 && ip < PACKED_LEN - 1 {
            return None;
        }
        op += 1;
    }
    let sign_nibble = bcdar[op - 1];
    if sign_nibble <= 9 {
        return None;
    }
    let sig = if sign_nibble == 0x0D || sign_nibble == 0x0B {
        DECFLOAT_SIGN as i32
    } else {
        0
    };
    if bcdar[0] != 0 {
        return None;
    }
    if exp == 0x7c000000 || exp == 0x7e000000 {
        if bcdar[1] != 0 {
            return None;
        }
    } else if exp == DECFLOAT_INF as i32 {
        for i in 0..DECQUAD_PMAX {
            if bcdar[i + 1] != 0 {
                return None;
            }
        }
    } else {
        if exp > DECQUAD_EMAX - DECQUAD_PMAX as i32 + 1 {
            return None;
        }
        if exp < DECQUAD_EMIN - DECQUAD_PMAX as i32 + 1 {
            return None;
        }
    }
    Some(decQuadFromBCD(df, exp, &bcdar[1..], sig))
}
/// Extract coefficient from decQuad into unit array
fn extract_coefficient(df: &DecQuad, buf: &mut [u32]) {
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    buf[0] = dpd_to_bin(sourlo & 0x3ff) + dpd_to_bin((sourlo >> 10) & 0x3ff) * 1000
        + dpd_to_bin((sourlo >> 20) & 0x3ff) * 1_000_000;
    buf[1] = dpd_to_bin(((sourml << 2) | (sourlo >> 30)) & 0x3ff)
        + dpd_to_bin((sourml >> 8) & 0x3ff) * 1000
        + dpd_to_bin((sourml >> 18) & 0x3ff) * 1_000_000;
    buf[2] = dpd_to_bin(((sourmh << 4) | (sourml >> 28)) & 0x3ff)
        + dpd_to_bin((sourmh >> 6) & 0x3ff) * 1000
        + dpd_to_bin((sourmh >> 16) & 0x3ff) * 1_000_000;
    buf[3] = dpd_to_bin(((sourhi << 6) | (sourmh >> 26)) & 0x3ff)
        + dpd_to_bin((sourhi >> 4) & 0x3ff) * 1000
        + (DECCOMBMSD[(sourhi >> 26) as usize] as u32) * 1_000_000;
}
/// Store 3 BCD digits from binary value
fn store_bcd3(val: u32, bcd: &mut [u8]) {
    if bcd.len() >= 3 {
        bcd[0] = ((val / 100) % 10) as u8;
        bcd[1] = ((val / 10) % 10) as u8;
        bcd[2] = (val % 10) as u8;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decquad_zero() {
        let mut q = DecQuad::default();
        decQuadZero(&mut q);
        assert!(q.is_zero());
        assert!(q.is_finite());
        assert!(! q.is_nan());
        assert!(! q.is_infinite());
    }
    #[test]
    fn test_decquad_and_or() {
        let mut set = DecContext::default();
        let mut result = DecQuad::default();
        let mut a = DecQuad::default();
        let mut b = DecQuad::default();
        decQuadZero(&mut a);
        decQuadZero(&mut b);
        decQuadAnd(&mut result, &a, &b, &mut set);
        assert!(result.is_zero());
        decQuadOr(&mut result, &a, &b, &mut set);
        assert!(result.is_zero());
    }
    #[test]
    fn test_decquad_digits() {
        let mut q = DecQuad::default();
        decQuadZero(&mut q);
        assert_eq!(decQuadDigits(& q), 1);
    }
}
const DECFLOAT_COMB: u32 = 0x7C000000;
const DECFLOAT_QNAN: u32 = 0x7C000000;
const DEC_OVERFLOW: u32 = 0x00000008;
const DEC_DIVISION_IMPOSSIBLE: u32 = 0x00000004;
const DECDPUN: usize = 9;
const DECPMAX: usize = 34;
const DECUNITS: usize = (DECPMAX + DECDPUN - 1) / DECDPUN;
static DECPOWERS: [u32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
static DECSTICKYTAB: [u8; 10] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
#[inline]
fn is_snan(df: &DecQuad) -> bool {
    (df.get_word(0) & DECFLOAT_SNAN) == DECFLOAT_SNAN
}
#[inline]
fn is_qnan(df: &DecQuad) -> bool {
    (df.get_word(0) & DECFLOAT_SNAN) == DECFLOAT_QNAN
}
#[inline]
fn is_inf(df: &DecQuad) -> bool {
    (df.get_word(0) & DECFLOAT_COMB) == DECFLOAT_INF
}
#[inline]
fn is_signed(df: &DecQuad) -> bool {
    (df.get_word(0) & DECFLOAT_SIGN) != 0
}
/// Compare two decQuad values numerically
///
/// Returns:
/// - Positive value if dfl > dfr
/// - Negative value if dfl < dfr
/// - Zero if dfl == dfr
///
/// If `tot` is non-zero, performs total ordering comparison (considers sign of zeros)
pub fn decNumCompare(dfl: &DecQuad, dfr: &DecQuad, tot: u8) -> i32 {
    let mut sigl: i32 = 1;
    if is_negative(dfl) {
        if !is_negative(dfr) {
            if is_zero(dfl) && is_zero(dfr) && tot == 0 {
                return 0;
            }
            return -1;
        }
        sigl = -1;
    }
    if is_negative(dfr) {
        if !is_negative(dfl) {
            if is_zero(dfl) && is_zero(dfr) && tot == 0 {
                return 0;
            }
            return 1;
        }
    }
    let sigr = -sigl;
    if is_infinity(dfl) {
        if is_infinity(dfr) {
            return 0;
        }
        return sigl;
    }
    if is_infinity(dfr) {
        return sigr;
    }
    let shift = get_exponent(dfl) - get_exponent(dfr);
    if is_zero(dfl) {
        if !is_zero(dfr) {
            return sigr;
        }
        if shift == 0 || tot == 0 {
            return 0;
        }
        if shift > 0 {
            return sigl;
        }
        return sigr;
    } else {
        if is_zero(dfr) {
            return sigl;
        }
    }
    if shift.abs() >= DECQUAD_PMAX as i32 {
        if shift > 0 {
            return sigl;
        }
        return sigr;
    }
    const BUF_SIZE: usize = DECQUAD_PMAX * 2 + 2 + 8;
    let mut bufl = [0u8; BUF_SIZE];
    let mut bufr = [0u8; BUF_SIZE];
    decode_to_bcd(dfl, &mut bufl);
    decode_to_bcd(dfr, &mut bufr);
    let coeff_start = 2;
    let coeff_len = DECQUAD_PMAX;
    if shift == 0 {
        for i in coeff_start..(coeff_start + coeff_len) {
            if bufl[i] > bufr[i] {
                return sigl;
            }
            if bufl[i] < bufr[i] {
                return sigr;
            }
        }
    } else if shift > 0 {
        let shift_usize = shift as usize;
        for i in coeff_start..(coeff_start + shift_usize).min(coeff_start + coeff_len) {
            if bufl[i] != 0 {
                return sigl;
            }
        }
        let bufl_start = coeff_start + shift_usize;
        for i in 0..coeff_len {
            let l_idx = bufl_start + i;
            let r_idx = coeff_start + i;
            if l_idx >= bufl.len() || r_idx >= bufr.len() {
                break;
            }
            let l_val = if l_idx < coeff_start + coeff_len + shift_usize {
                bufl[l_idx]
            } else {
                0
            };
            let r_val = if r_idx < coeff_start + coeff_len { bufr[r_idx] } else { 0 };
            if l_val > r_val {
                return sigl;
            }
            if l_val < r_val {
                return sigr;
            }
        }
    } else {
        let shift_usize = (-shift) as usize;
        for i in coeff_start..(coeff_start + shift_usize).min(coeff_start + coeff_len) {
            if bufr[i] != 0 {
                return sigr;
            }
        }
        let bufr_start = coeff_start + shift_usize;
        for i in 0..coeff_len {
            let l_idx = coeff_start + i;
            let r_idx = bufr_start + i;
            if l_idx >= bufl.len() || r_idx >= bufr.len() {
                break;
            }
            let l_val = if l_idx < coeff_start + coeff_len { bufl[l_idx] } else { 0 };
            let r_val = if r_idx < coeff_start + coeff_len + shift_usize {
                bufr[r_idx]
            } else {
                0
            };
            if l_val > r_val {
                return sigl;
            }
            if l_val < r_val {
                return sigr;
            }
        }
    }
    if tot == 0 {
        return 0;
    }
    if shift > 0 {
        return sigl;
    }
    if shift < 0 {
        return sigr;
    }
    0
}
/// Invert (logical NOT) a decQuad
pub fn decQuadInvert<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let sourhi = df.get_word(0);
    let valid_comb = (df.get_word(0) & 0xFBFFC000) == 0x22080000;
    let valid_coeff = (df.get_word(0) & !0xFFFFc912) == 0
        && (df.get_word(1) & !0x44912449) == 0 && (df.get_word(2) & !0x12449124) == 0
        && (df.get_word(3) & !0x49124491) == 0;
    if !valid_comb || !valid_coeff {
        return decInvalid(result, set);
    }
    result.set_word(0, 0x22080000 | ((!sourhi) & 0x04000912));
    result.set_word(1, (!df.get_word(1)) & 0x44912449);
    result.set_word(2, (!df.get_word(2)) & 0x12449124);
    result.set_word(3, (!df.get_word(3)) & 0x49124491);
    result
}
/// Get maximum of two DecQuads
pub fn decQuadMax<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if dfl.is_nan() || dfr.is_nan() {
        if (dfl.get_word(0) & 0x7e000000) == 0x7e000000
            || (dfr.get_word(0) & 0x7e000000) == 0x7e000000
        {
            set.status |= DEC_INVALID_OPERATION;
        }
        if dfl.is_nan() && !dfr.is_nan() {
            return decCanonical(result, dfr);
        }
        if !dfl.is_nan() && dfr.is_nan() {
            return decCanonical(result, dfl);
        }
        return decCanonical(result, dfl);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    if comp >= 0 { decCanonical(result, dfl) } else { decCanonical(result, dfr) }
}
/// Negate a decQuad (minus operation)
pub fn decQuadMinus<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if is_nan(df) {
        return decNaNs(result, Some(df), None, set);
    }
    decCanonical(result, df);
    if is_zero(df) {
        let b = result.get_byte(0);
        result.set_byte(0, b & !0x80);
    } else {
        let b = result.get_byte(0);
        result.set_byte(0, b ^ 0x80);
    }
    result
}
/// Convert decQuad to signed 32-bit integer with exact rounding
pub fn decQuadToInt32Exact(df: &DecQuad, set: &mut DecContext, round: Rounding) -> i32 {
    decToInt32(df, set, round, 1, 0) as i32
}
/// Move toward another value
pub fn decQuadNextToward<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if is_nan(dfl) || is_nan(dfr) {
        return decNaNs(result, Some(dfl), Some(dfr), set);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    if comp == 0 {
        return decQuadCopySign(result, dfl, dfr);
    }
    if is_inf(dfl) {
        if comp < 0 && is_signed(dfl) {
            result.set_word(0, 0x77FFCFF3);
            result.set_word(1, 0xFCFF3FCF);
            result.set_word(2, 0xF3FCFF3F);
            result.set_word(3, 0xCFF3FCFF);
            let w0 = result.get_word(0);
            result.set_word(0, w0 | DECFLOAT_SIGN);
            return result;
        }
        if comp > 0 && !is_signed(dfl) {
            result.set_word(0, 0x77FFCFF3);
            result.set_word(1, 0xFCFF3FCF);
            result.set_word(2, 0xF3FCFF3F);
            result.set_word(3, 0xCFF3FCFF);
            return result;
        }
    }
    decCanonical(result, dfl)
}
/// Check if decQuad is an integer
pub fn decQuadIsInteger(df: &DecQuad) -> u32 {
    let w0 = df.get_word(0);
    if (w0 & 0x63FFC000) == 0x22080000 || (w0 & 0x7BFFC000) == 0x6A080000 {
        1
    } else {
        0
    }
}
/// Set the exponent of a decQuad
pub fn decQuadSetExponent<'a>(
    result: &'a mut DecQuad,
    _set: &mut DecContext,
    exp: i32,
) -> &'a mut DecQuad {
    let biased_exp = (exp + DECQUAD_BIAS) as u32;
    let msd = DECCOMBMSD[(result.get_word(0) >> 26) as usize];
    let sign = result.get_word(0) & 0x80000000;
    let comb_index = ((biased_exp >> 12) << 4) as usize + msd as usize;
    let comb = if comb_index < DECCOMBFROM.len() { DECCOMBFROM[comb_index] } else { 0 };
    let exp_cont = (biased_exp & 0xfff) << (32 - 6 - 12);
    result.set_word(0, sign | comb | exp_cont | (result.get_word(0) & 0x00003fff));
    result
}
/// Get coefficient as BCD array
pub fn decQuadGetCoefficient(df: &DecQuad, bcdar: &mut [u8]) -> i32 {
    if (df.get_word(0) & 0x7c000000) == 0x78000000 {
        for b in bcdar.iter_mut().take(DECQUAD_PMAX) {
            *b = 0;
        }
    } else {
        let sourhi = df.get_word(0);
        let sourmh = df.get_word(1);
        let sourml = df.get_word(2);
        let sourlo = df.get_word(3);
        if bcdar.len() > 0 {
            bcdar[0] = DECCOMBMSD[(sourhi >> 26) as usize] as u8;
        }
        for i in 1..DECQUAD_PMAX.min(bcdar.len()) {
            bcdar[i] = 0;
        }
        if (df.get_word(0) & 0x7c000000) == 0x7c000000 && bcdar.len() > 0 {
            bcdar[0] = 0;
        }
    }
    (df.get_word(0) & DECFLOAT_SIGN) as i32
}
/// Copy sign from one decQuad to another
pub fn decQuadCopySign<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
) -> &'a mut DecQuad {
    if result as *const _ != dfl as *const _ {
        *result = *dfl;
    }
    let w0 = result.get_word(0);
    let sign = dfr.get_word(0) & DECFLOAT_SIGN;
    result.set_word(0, (w0 & !DECFLOAT_SIGN) | sign);
    result
}
/// Check if a decQuad is normal (not zero, subnormal, infinite, or NaN)
pub fn decQuadIsNormal(df: &DecQuad) -> u32 {
    if (df.get_word(0) & 0x78000000) == 0x78000000 {
        return 0;
    }
    if df.is_zero_internal() {
        return 0;
    }
    let exp = df.get_exponent_internal() + decQuadDigits(df) as i32 - 1;
    if exp >= DECQUAD_EMIN { 1 } else { 0 }
}
/// Parse a string to create a DecQuad
pub fn decQuadFromString<'a>(
    result: &'a mut DecQuad,
    string: &str,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let mut num = BcdNum::default();
    let mut buffer = vec![0u8; DECSTRING_BUFFER];
    let mut error = DEC_INVALID_OPERATION;
    let chars: Vec<char> = string.chars().collect();
    let mut dotchar_pos: Option<usize> = None;
    let mut cfirst = 0usize;
    'parse: loop {
        num.sign = 0;
        num.msd = buffer.clone();
        let mut c = 0usize;
        if c < chars.len() {
            if chars[c] == '-' {
                cfirst = 1;
                num.sign = DECFLOAT_SIGN;
                c = 1;
            } else if chars[c] == '+' {
                cfirst = 1;
                c = 1;
            }
        }
        let mut scan_start = c;
        while c < chars.len() {
            let ch = chars[c];
            if ch.is_ascii_digit() {
                c += 1;
                continue;
            }
            if ch == '\0' {
                break;
            }
            if ch == '.' {
                if dotchar_pos.is_some() {
                    break;
                }
                dotchar_pos = Some(c);
                c += 1;
                continue;
            }
            break;
        }
        let mut digits = c - cfirst;
        if dotchar_pos.is_some() {
            digits = digits.saturating_sub(1);
        }
        if digits > 0 {
            let clast = c - 1;
            let mut exp: i32 = 0;
            if c < chars.len() && (chars[c] == 'E' || chars[c] == 'e') {
                c += 1;
                let mut exp_negative = false;
                if c < chars.len() {
                    if chars[c] == '-' {
                        exp_negative = true;
                        c += 1;
                    } else if chars[c] == '+' {
                        c += 1;
                    }
                }
                if c >= chars.len() {
                    break 'parse;
                }
                while c < chars.len() && chars[c] == '0' {
                    c += 1;
                }
                let firstexp = c;
                while c < chars.len() {
                    if let Some(d) = chars[c].to_digit(10) {
                        exp = exp * 10 + d as i32;
                        c += 1;
                    } else {
                        break;
                    }
                }
                if c < chars.len() && chars[c] != '\0' {
                    break 'parse;
                }
                if c > firstexp + 4 {
                    exp = DECQUAD_EMAX * 2;
                }
                if exp_negative {
                    exp = -exp;
                }
            }
            if let Some(dot_pos) = dotchar_pos {
                if digits == 0 {
                    break 'parse;
                }
                exp -= (clast - dot_pos) as i32;
            }
            num.exponent = exp;
            error = 0;
            let mut ub_idx = 0;
            let mut digit_idx = cfirst;
            while digit_idx <= clast && ub_idx < buffer.len() {
                let ch = chars[digit_idx];
                if ch == '.' {
                    digit_idx += 1;
                    continue;
                }
                if let Some(d) = ch.to_digit(10) {
                    buffer[ub_idx] = d as u8;
                    ub_idx += 1;
                }
                digit_idx += 1;
            }
            num.msd = buffer.clone();
            num.msd_idx = 0;
            num.lsd_idx = if ub_idx > 0 { ub_idx - 1 } else { 0 };
        } else {
            let remaining: String = chars[c..].iter().collect();
            buffer[0] = 0;
            num.msd = buffer.clone();
            num.lsd_idx = 0;
            if decBiStr(&remaining, "infinity", "INFINITY")
                || decBiStr(&remaining, "inf", "INF")
            {
                num.exponent = DECFLOAT_INF as i32;
                error = 0;
            } else if remaining.starts_with('s') || remaining.starts_with('S') {
                let rest: String = if remaining.len() > 1 {
                    remaining[1..].to_string()
                } else {
                    String::new()
                };
                if rest.len() >= 3
                    && (rest.starts_with("NaN") || rest.starts_with("nan")
                        || rest.starts_with("NAN"))
                {
                    num.exponent = DECFLOAT_SNAN as i32;
                    error = 0;
                }
            } else if remaining.len() >= 3
                && (remaining.starts_with("NaN") || remaining.starts_with("nan")
                    || remaining.starts_with("NAN"))
            {
                num.exponent = DECFLOAT_QNAN as i32;
                error = 0;
            }
        }
        break;
    }
    if error != 0 {
        set.status |= error;
        num.exponent = DECFLOAT_QNAN as i32;
        num.sign = 0;
        buffer[0] = 0;
        num.msd = buffer;
        num.lsd_idx = 0;
    }
    decFinalize(result, &mut num, set);
    result
}
/// Fused multiply-add: result = dfl * dfr + dff
pub fn decQuadFMA<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    dff: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if dfl.is_special() || dfr.is_special() || dff.is_special() {
        if dfl.is_snan() || dfr.is_snan() {
            return decNaNs(result, Some(dfl), Some(dfr), set);
        }
        if dff.is_snan() {
            return decNaNs(result, Some(dff), None, set);
        }
        if dfl.is_nan() || dfr.is_nan() {
            return decNaNs(result, Some(dfl), Some(dfr), set);
        }
        if dff.is_nan() {
            return decNaNs(result, Some(dff), None, set);
        }
        let mut proxy = DecQuad::default();
        decQuadZero(&mut proxy);
        if dfl.is_infinite() {
            if dfr.is_zero() {
                return decInvalid(result, set);
            }
            // Inline decInfinity
            let sign = proxy.get_word(0) & 0x80000000;
            proxy.set_word(0, 0x78000000 | sign);
            proxy.set_word(1, 0);
            proxy.set_word(2, 0);
            proxy.set_word(3, 0);
        } else if dfr.is_infinite() {
            if dfl.is_zero() {
                return decInvalid(result, set);
            }
            // Inline decInfinity
            let sign = proxy.get_word(0) & 0x80000000;
            proxy.set_word(0, 0x78000000 | sign);
            proxy.set_word(1, 0);
            proxy.set_word(2, 0);
            proxy.set_word(3, 0);
        }
        let prod_sign = (dfl.get_word(0) ^ dfr.get_word(0)) & DECFLOAT_SIGN;
        let mut word0 = proxy.get_word(0);
        word0 |= prod_sign;
        proxy.set_word(0, word0);
        if !dff.is_infinite() {
            return decQuadCopy(result, &proxy);
        }
        if !proxy.is_infinite() {
            return decInfinity(result, dff);
        }
        if (dff.get_word(0) & DECFLOAT_SIGN) != (proxy.get_word(0) & DECFLOAT_SIGN) {
            return decInvalid(result, set);
        }
        return decQuadCopy(result, &proxy);
    }
    let mut mul = BcdNum::default();
    let mut acc = vec![0u8; 256];
    decFiniteMultiply(&mut mul, &mut acc, dfl, dfr);
    let mut fin = BcdNum::default();
    fin.exponent = decQuadGetExponent(dff);
    fin.sign = dff.get_word(0) & DECFLOAT_SIGN;
    fin.msd = extract_bcd(dff);
    fin.msd_idx = 0;
    fin.lsd_idx = DECQUAD_PMAX - 1;
    let diffsign = mul.sign ^ fin.sign;
    let (hi, lo) = if mul.exponent >= fin.exponent {
        (&mul, &fin)
    } else {
        (&fin, &mul)
    };
    let mut hi_msd = hi.msd.clone();
    let mut hi_start = 0;
    while hi_start < hi_msd.len() - 1 && hi_msd[hi_start] == 0 {
        hi_start += 1;
    }
    let mut lo_msd = lo.msd.clone();
    let mut lo_start = 0;
    while lo_start < lo_msd.len() - 1 && lo_msd[lo_start] == 0 {
        lo_start += 1;
    }
    if hi_msd[hi_start] == 0 {
        let mut result_num = lo.clone();
        if diffsign != 0 && lo_msd[lo_start] == 0 {
            result_num.sign = 0;
            if set.round == Rounding::Floor {
                result_num.sign = DECFLOAT_SIGN;
            }
        }
        return decFinalize(result, &mut result_num, set);
    }
    let padding = hi.exponent - lo.exponent;
    let mut result_num = BcdNum::default();
    result_num.exponent = lo.exponent;
    let mut result_msd = vec![0u8; DECQUAD_PMAX * 3];
    if diffsign != 0 {
        result_num.sign = hi.sign;
    } else {
        result_num.sign = hi.sign;
    }
    let hi_len = hi_msd.len() - hi_start;
    for i in 0..hi_len {
        result_msd[i] = hi_msd[hi_start + i];
    }
    let lo_len = lo_msd.len() - lo_start;
    let offset = (padding as usize).min(result_msd.len() - lo_len);
    if diffsign == 0 {
        let mut carry = 0u8;
        for i in (0..lo_len).rev() {
            let idx = offset + i;
            if idx < result_msd.len() {
                let sum = result_msd[idx] + lo_msd[lo_start + i] + carry;
                result_msd[idx] = sum % 10;
                carry = sum / 10;
            }
        }
        if carry != 0 && offset > 0 {
            result_msd[offset - 1] += carry;
        }
    } else {
        let mut borrow = 0i8;
        for i in (0..lo_len).rev() {
            let idx = offset + i;
            if idx < result_msd.len() {
                let mut diff = result_msd[idx] as i8 - lo_msd[lo_start + i] as i8
                    - borrow;
                if diff < 0 {
                    diff += 10;
                    borrow = 1;
                } else {
                    borrow = 0;
                }
                result_msd[idx] = diff as u8;
            }
        }
    }
    result_num.msd = result_msd;
    result_num.msd_idx = 0;
    result_num.lsd_idx = DECQUAD_PMAX - 1;
    while result_num.msd_idx < result_num.lsd_idx
        && result_num.msd[result_num.msd_idx] == 0
    {
        result_num.msd_idx += 1;
    }
    if result_num.msd[result_num.msd_idx] == 0
        && result_num.msd_idx == result_num.lsd_idx
    {
        result_num.sign = 0;
        if set.round == Rounding::Floor {
            result_num.sign = DECFLOAT_SIGN;
        }
    }
    decFinalize(result, &mut result_num, set)
}
const DEC_INEXACT: u32 = 0x00000080;
const DECSTRING_BUFFER: usize = ((DECQUAD_PMAX + 11 + 7) / 8) * 8;
/// Compare two strings case-insensitively
fn decBiStr(target: &str, lower: &str, upper: &str) -> bool {
    if target.len() != lower.len() {
        return false;
    }
    for (t, (l, u)) in target.chars().zip(lower.chars().zip(upper.chars())) {
        if t != l && t != u {
            return false;
        }
    }
    true
}
/// Add two DecQuad numbers
pub fn decQuadAdd<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let sourhil = dfl.get_word(0);
    let sourhir = dfr.get_word(0);
    let summ = DECTESTMSD[(sourhil >> 26) as usize] as i32
        + DECTESTMSD[(sourhir >> 26) as usize] as i32;
    let mut bexpl = (DECCOMBEXP[(sourhil >> 26) as usize] as i32)
        + ((dfl.get_word(0) & 0x03FFFFFF) >> (32 - 6 - 12)) as i32;
    let mut bexpr = (DECCOMBEXP[(sourhir >> 26) as usize] as i32)
        + ((dfr.get_word(0) & 0x03FFFFFF) >> (32 - 6 - 12)) as i32;
    let diffsign = (sourhil ^ sourhir) & DECFLOAT_SIGN;
    if summ <= 8 {
        if summ < 0 {
            if summ < -64 {
                return decNaNs(result, Some(dfl), Some(dfr), set);
            }
            if summ == -64 && diffsign != 0 {
                return decInvalid(result, set);
            }
            if dfl.is_infinite() {
                return decInfinity(result, dfl);
            }
            return decInfinity(result, dfr);
        }
    }
    let (dfl_use, dfr_use) = if bexpl <= bexpr {
        std::mem::swap(&mut bexpl, &mut bexpr);
        (dfr, dfl)
    } else {
        (dfl, dfr)
    };
    if dfl_use.is_zero() {
        decCanonical(result, dfr_use);
        if diffsign != 0 && result.is_zero() {
            let mut word0 = result.get_word(0);
            word0 &= !DECFLOAT_SIGN;
            if set.round == Rounding::Floor {
                word0 |= DECFLOAT_SIGN;
            }
            result.set_word(0, word0);
        }
        return result;
    }
    let mut num = BcdNum::default();
    num.msd = vec![0; DECQUAD_PMAX * 3 + 8];
    let mut acc = extract_bcd(dfl_use);
    let buf = extract_bcd(dfr_use);
    let overlap = DECQUAD_PMAX as i32 - (bexpl - bexpr);
    if overlap <= 0 {
        let gap = (-overlap) as usize;
        if gap > DECQUAD_PMAX {
            num.sign = dfl_use.get_word(0) & DECFLOAT_SIGN;
            num.exponent = bexpl - DECQUAD_BIAS;
            num.msd = acc;
            num.msd_idx = 0;
            num.lsd_idx = DECQUAD_PMAX - 1;
        } else {
            acc.resize(DECQUAD_PMAX + gap + DECQUAD_PMAX, 0);
            for i in 0..DECQUAD_PMAX {
                acc[DECQUAD_PMAX + gap + i] = buf[i];
            }
            num.msd = acc;
            num.sign = dfl_use.get_word(0) & DECFLOAT_SIGN;
            num.exponent = bexpr - DECQUAD_BIAS;
            num.msd_idx = 0;
            num.lsd_idx = DECQUAD_PMAX + gap + DECQUAD_PMAX - 1;
        }
    } else {
        let offset = (DECQUAD_PMAX as i32 - overlap) as usize;
        if diffsign != 0 {
            for b in acc.iter_mut() {
                *b = 9 - *b;
            }
            let mut carry = 1u32;
            for i in (0..DECQUAD_PMAX).rev() {
                let buf_idx = if i >= offset { i - offset } else { continue };
                if buf_idx < buf.len() {
                    let sum = acc[i] as u32 + buf[buf_idx] as u32 + carry;
                    acc[i] = (sum % 10) as u8;
                    carry = sum / 10;
                }
            }
            if carry == 0 {
                for b in acc.iter_mut() {
                    *b = 9 - *b;
                }
                for i in (0..DECQUAD_PMAX).rev() {
                    if acc[i] < 9 {
                        acc[i] += 1;
                        break;
                    }
                    acc[i] = 0;
                }
                num.sign = dfl_use.get_word(0) & DECFLOAT_SIGN;
            } else {
                num.sign = dfr_use.get_word(0) & DECFLOAT_SIGN;
            }
        } else {
            let mut carry = 0u32;
            for i in (0..DECQUAD_PMAX).rev() {
                let buf_idx = if i >= offset { i - offset } else { 0 };
                let b = if i >= offset && buf_idx < buf.len() {
                    buf[buf_idx] as u32
                } else {
                    0
                };
                let sum = acc[i] as u32 + b + carry;
                acc[i] = (sum % 10) as u8;
                carry = sum / 10;
            }
            if carry != 0 {
                acc.insert(0, carry as u8);
            }
            num.sign = dfl_use.get_word(0) & DECFLOAT_SIGN;
        }
        num.msd = acc;
        num.exponent = bexpr - DECQUAD_BIAS;
        num.msd_idx = 0;
        num.lsd_idx = num.msd.len() - 1;
    }
    while num.msd_idx < num.lsd_idx && num.msd[num.msd_idx] == 0 {
        num.msd_idx += 1;
    }
    if num.msd_idx == num.lsd_idx && num.msd[num.msd_idx] == 0 {
        num.sign = 0;
        if set.round == Rounding::Floor {
            num.sign = DECFLOAT_SIGN;
        }
    }
    decFinalize(result, &mut num, set)
}
/// Get minimum of two DecQuads
pub fn decQuadMin<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if dfl.is_nan() || dfr.is_nan() {
        if (dfl.get_word(0) & 0x7e000000) == 0x7e000000
            || (dfr.get_word(0) & 0x7e000000) == 0x7e000000
        {
            set.status |= DEC_INVALID_OPERATION;
        }
        if dfl.is_nan() && !dfr.is_nan() {
            return decCanonical(result, dfr);
        }
        if !dfl.is_nan() && dfr.is_nan() {
            return decCanonical(result, dfl);
        }
        return decCanonical(result, dfl);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    if comp <= 0 { decCanonical(result, dfl) } else { decCanonical(result, dfr) }
}
/// Logical OR of two DecQuads
pub fn decQuadOr<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let dfl_valid = (dfl.get_word(0) & 0xfbffc000) == 0x22080000;
    let dfr_valid = (dfr.get_word(0) & 0xfbffc000) == 0x22080000;
    let dfl_logical = (dfl.get_word(0) & !0xffffc912) == 0
        && (dfl.get_word(1) & !0x44912449) == 0 && (dfl.get_word(2) & !0x12449124) == 0
        && (dfl.get_word(3) & !0x49124491) == 0;
    let dfr_logical = (dfr.get_word(0) & !0xffffc912) == 0
        && (dfr.get_word(1) & !0x44912449) == 0 && (dfr.get_word(2) & !0x12449124) == 0
        && (dfr.get_word(3) & !0x49124491) == 0;
    if !dfl_valid || !dfr_valid || !dfl_logical || !dfr_logical {
        return decInvalid(result, set);
    }
    result.set_word(0, 0x22080000 | ((dfl.get_word(0) | dfr.get_word(0)) & 0x04000912));
    result.set_word(1, (dfl.get_word(1) | dfr.get_word(1)) & 0x44912449);
    result.set_word(2, (dfl.get_word(2) | dfr.get_word(2)) & 0x12449124);
    result.set_word(3, (dfl.get_word(3) | dfr.get_word(3)) & 0x49124491);
    result
}
/// Check if DecQuad is finite
pub fn decQuadIsFinite(df: &DecQuad) -> u32 {
    if (df.get_word(0) & 0x78000000) == 0x78000000 { 0 } else { 1 }
}
/// Get next representable value toward positive infinity
pub fn decQuadNextPlus<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (dfl.get_word(0) & 0x7c000000) == 0x78000000
        && (dfl.get_word(0) & DECFLOAT_SIGN) != 0
    {
        result.set_word(0, 0x77ffcff3 | DECFLOAT_SIGN);
        result.set_word(1, 0xfcff3fcf);
        result.set_word(2, 0xf3fcff3f);
        result.set_word(3, 0xcff3fcff);
        return result;
    }
    if dfl.is_nan() {
        *result = *dfl;
        if (dfl.get_word(0) & 0x7e000000) == 0x7e000000 {
            set.status |= DEC_INVALID_OPERATION;
        }
        return result;
    }
    let mut delta = DecQuad::default();
    decQuadZero(&mut delta);
    delta.set_word(3, 1);
    delta.set_word(0, 0);
    let saveround = set.round;
    set.round = Rounding::Ceiling;
    let savestat = set.status;
    let dfl_copy = *dfl;
    decQuadAdd(result, &dfl_copy, &delta, set);
    if result.is_zero() {
        let word0 = result.get_word(0);
        result.set_word(0, word0 ^ DECFLOAT_SIGN);
    }
    set.status &= DEC_INEXACT;
    set.status |= savestat;
    set.round = saveround;
    result
}
/// Convert to integral value with specified rounding
pub fn decQuadToIntegralValue<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
    round: Rounding,
) -> &'a mut DecQuad {
    decToIntegral(result, df, set, round, false)
}
/// Get maximum magnitude of two DecQuads
pub fn decQuadMaxMag<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (dfl.get_word(0) & 0x7c000000) == 0x7c000000
        || (dfr.get_word(0) & 0x7c000000) == 0x7c000000
    {
        return decQuadMax(result, dfl, dfr, set);
    }
    let mut absl = DecQuad::default();
    let mut absr = DecQuad::default();
    decQuadCopyAbs(&mut absl, dfl);
    decQuadCopyAbs(&mut absr, dfr);
    let comp = decNumCompare(&absl, &absr, 0);
    if comp > 0 {
        return decCanonical(result, dfl);
    }
    if comp < 0 {
        return decCanonical(result, dfr);
    }
    decQuadMax(result, dfl, dfr, set)
}
/// Logical AND of two DecQuads
pub fn decQuadAnd<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    let dfl_valid = (dfl.get_word(0) & 0xfbffc000) == 0x22080000;
    let dfr_valid = (dfr.get_word(0) & 0xfbffc000) == 0x22080000;
    let dfl_logical = (dfl.get_word(0) & !0xffffc912) == 0
        && (dfl.get_word(1) & !0x44912449) == 0 && (dfl.get_word(2) & !0x12449124) == 0
        && (dfl.get_word(3) & !0x49124491) == 0;
    let dfr_logical = (dfr.get_word(0) & !0xffffc912) == 0
        && (dfr.get_word(1) & !0x44912449) == 0 && (dfr.get_word(2) & !0x12449124) == 0
        && (dfr.get_word(3) & !0x49124491) == 0;
    if !dfl_valid || !dfr_valid || !dfl_logical || !dfr_logical {
        return decInvalid(result, set);
    }
    result.set_word(0, 0x22080000 | ((dfl.get_word(0) & dfr.get_word(0)) & 0x04000912));
    result.set_word(1, (dfl.get_word(1) & dfr.get_word(1)) & 0x44912449);
    result.set_word(2, (dfl.get_word(2) & dfr.get_word(2)) & 0x12449124);
    result.set_word(3, (dfl.get_word(3) & dfr.get_word(3)) & 0x49124491);
    result
}
/// Get minimum magnitude of two DecQuads
pub fn decQuadMinMag<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (dfl.get_word(0) & 0x7c000000) == 0x7c000000
        || (dfr.get_word(0) & 0x7c000000) == 0x7c000000
    {
        return decQuadMin(result, dfl, dfr, set);
    }
    let mut absl = DecQuad::default();
    let mut absr = DecQuad::default();
    decQuadCopyAbs(&mut absl, dfl);
    decQuadCopyAbs(&mut absr, dfr);
    let comp = decNumCompare(&absl, &absr, 0);
    if comp < 0 {
        return decCanonical(result, dfl);
    }
    if comp > 0 {
        return decCanonical(result, dfr);
    }
    decQuadMin(result, dfl, dfr, set)
}
/// Convert DecQuad to string
pub fn decQuadToString<'a>(df: &DecQuad, string: &'a mut String) -> &'a mut String {
    string.clear();
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    if (sourhi as i32) < 0 {
        string.push('-');
    }
    let comb = (sourhi >> 26) as usize;
    let msd = DECCOMBMSD[comb];
    let mut exp = DECCOMBEXP[comb] as i32;
    if exp < 0x78000000_u32 as i32 {
        exp += ((sourhi & 0x03FFFFFF) >> (32 - 6 - 12)) as i32 - DECQUAD_BIAS;
    } else {
        if exp == 0x78000000_u32 as i32 {
            string.push_str("Infinity");
            return string;
        }
        if (sourhi & 0x02000000) != 0 {
            string.push('s');
        }
        string.push_str("NaN");
        if sourlo == 0 && sourml == 0 && sourmh == 0 && (sourhi & 0x00003FFF) == 0 {
            return string;
        }
        let bcd = extract_bcd(df);
        let mut started = false;
        for &d in &bcd {
            if d != 0 || started {
                string.push((b'0' + d) as char);
                started = true;
            }
        }
        return string;
    }
    let bcd = extract_bcd(df);
    let mut first_nonzero = 0;
    while first_nonzero < bcd.len() - 1 && bcd[first_nonzero] == 0 {
        first_nonzero += 1;
    }
    let digits: Vec<char> = bcd[first_nonzero..]
        .iter()
        .map(|&d| (b'0' + d) as char)
        .collect();
    let num_digits = digits.len() as i32;
    let pre = num_digits + exp;
    let mut e = 0i32;
    if exp > 0 || pre < -5 {
        e = pre - 1;
    }
    if e != 0 {
        string.push(digits[0]);
        if digits.len() > 1 {
            string.push('.');
            for &c in &digits[1..] {
                string.push(c);
            }
        }
        string.push('E');
        if e >= 0 {
            string.push('+');
        }
        string.push_str(&e.to_string());
    } else if pre > 0 {
        let pre_usize = pre as usize;
        if pre_usize >= digits.len() {
            for &c in &digits {
                string.push(c);
            }
        } else {
            for (i, &c) in digits.iter().enumerate() {
                if i == pre_usize {
                    string.push('.');
                }
                string.push(c);
            }
        }
    } else {
        string.push_str("0.");
        for _ in 0..(-pre) {
            string.push('0');
        }
        for &c in &digits {
            string.push(c);
        }
    }
    string
}
/// Get version string
pub fn decQuadVersion() -> &'static str {
    "decQuad 1.0"
}
// Note: DPD2BIN, DPD2BCD8, DECCOMBEXP, and DECCOMBMSD
// are already defined as static earlier in this file
/// Show a decQuad value with a tag for debugging
pub fn decQuadShow(df: &DecQuad, tag: &str) {
    let mut hexbuf = String::with_capacity(DECQUAD_BYTES * 2 + DECQUAD_BYTES / 4 + 1);
    for i in 0..DECQUAD_BYTES {
        write!(hexbuf, "{:02x}", df.bytes[DECQUAD_BYTES - 1 - i]).unwrap();
        if (i + 1) % 4 == 0 && i < DECQUAD_BYTES - 1 {
            hexbuf.push(' ');
        }
    }
    let mut buff = String::new();
    decQuadToString(df, &mut buff);
    println!(">{tag}> {hexbuf} [big-endian]  {buff}");
}
/// Rotate digits of a decQuad
pub fn decQuadRotate<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if dfl.is_nan_internal() || dfr.is_nan_internal() {
        return decNaNs(result, Some(dfl), Some(dfr), set);
    }
    if !is_valid_logical(dfr) {
        return decInvalid(result, set);
    }
    let digits = decQuadDigits(dfr);
    if digits > 2 {
        return decInvalid(result, set);
    }
    let mut rotate = get_dpd_value(dfr) as i32;
    if rotate > DECQUAD_PMAX as i32 {
        return decInvalid(result, set);
    }
    if dfl.is_infinity_internal() && !dfl.is_nan_internal() {
        return decInfinity(result, dfl);
    }
    if rotate == 0 || rotate == DECQUAD_PMAX as i32 {
        return decCanonical(result, dfl);
    }
    if dfr.is_signed_internal() {
        rotate = -rotate;
    }
    if rotate.abs() > (DECQUAD_PMAX as i32) / 2 {
        if rotate < 0 {
            rotate = DECQUAD_PMAX as i32 + rotate;
        } else {
            rotate = rotate - DECQUAD_PMAX as i32;
        }
    }
    let mut buf = vec![0u8; (DECQUAD_PMAX as usize) * 2];
    let base_offset = if rotate < 0 { DECQUAD_PMAX as usize } else { 0 };
    extract_bcd_digits(dfl, &mut buf[base_offset..base_offset + DECQUAD_PMAX as usize]);
    let mut num = BcdNum::default();
    if rotate < 0 {
        let src_start = DECQUAD_PMAX as usize;
        let rotate_abs = (-rotate) as usize;
        for i in 0..rotate_abs {
            buf[i] = buf[src_start + DECQUAD_PMAX as usize - rotate_abs + i];
        }
        num.msd_idx = rotate_abs;
    } else {
        let rotate_amt = rotate as usize;
        for i in 0..rotate_amt {
            buf[DECQUAD_PMAX as usize + i] = buf[i];
        }
        num.msd_idx = rotate_amt;
    }
    num.msd = buf;
    num.lsd_idx = num.msd_idx + DECQUAD_PMAX as usize - 1;
    num.sign = dfl.get_word(0) & 0x80000000;
    num.exponent = dfl.get_exponent_internal();
    let savestat = set.status;
    decFinalize(result, &mut num, set);
    set.status = savestat;
    result
}
/// Set the coefficient of a decQuad from BCD array
pub fn decQuadSetCoefficient<'a>(df: &'a mut DecQuad, bcdar: &[u8], sig: i32) -> &'a mut DecQuad {
    let exp: i32;
    if (df.get_word(0) & 0x78000000) == 0x78000000 {
        let special_exp = df.get_word(0) & 0x7e000000;
        if (df.get_word(0) & 0x7c000000) == 0x78000000 {
            let bcdzero = [0u8; DECQUAD_PMAX as usize];
            return decQuadFromBCD(df, special_exp as i32, &bcdzero, sig);
        }
        exp = special_exp as i32;
    } else {
        exp = df.get_exponent_internal();
    }
    decQuadFromBCD(df, exp, bcdar, sig)
}
/// Check if a decQuad is a signaling NaN
pub fn decQuadIsSignaling(df: &DecQuad) -> u32 {
    if df.is_snan_internal() { 1 } else { 0 }
}
/// Compare total magnitude of two decQuad values
pub fn decQuadCompareTotalMag<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
) -> &'a mut DecQuad {
    let mut a = *dfl;
    let mut b = *dfr;
    if dfl.is_signed_internal() {
        decQuadCopyAbs(&mut a, dfl);
    }
    if dfr.is_signed_internal() {
        decQuadCopyAbs(&mut b, dfr);
    }
    decQuadCompareTotal(result, &a, &b)
}
/// Total ordering comparison of two decQuad values
pub fn decQuadCompareTotal<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
) -> &'a mut DecQuad {
    let comp: i32;
    if dfl.is_nan_internal() || dfr.is_nan_internal() {
        let mut nanl = if dfl.is_snan_internal() { 1 } else { 0 }
            + if dfl.is_qnan_internal() { 2 } else { 0 };
        if dfl.is_signed_internal() {
            nanl = -nanl;
        }
        let mut nanr = if dfr.is_snan_internal() { 1 } else { 0 }
            + if dfr.is_qnan_internal() { 2 } else { 0 };
        if dfr.is_signed_internal() {
            nanr = -nanr;
        }
        if nanl > nanr {
            comp = 1;
        } else if nanl < nanr {
            comp = -1;
        } else {
            let bufl = extract_full_bcd(dfl);
            let bufr = extract_full_bcd(dfr);
            let sigl = if dfl.is_signed_internal() { -1 } else { 1 };
            let mut found_comp = 0;
            for i in 0..bufl.len() {
                if bufl[i] != bufr[i] {
                    found_comp = if bufl[i] > bufr[i] { sigl } else { -sigl };
                    break;
                }
            }
            comp = found_comp;
        }
    } else {
        comp = decNumCompare(dfl, dfr, 1);
    }
    decQuadZero(result);
    if comp == 0 {
        return result;
    }
    result.bytes[DECQUAD_BYTES - 1 - (DECQUAD_BYTES - 1)] = 0x01;
    if comp < 0 {
        result.bytes[DECQUAD_BYTES - 1] |= 0x80;
    }
    result
}
fn is_valid_logical(df: &DecQuad) -> bool {
    let w0 = df.get_word(0);
    (w0 & 0x63ffc000) == 0x22080000 || (w0 & 0x7bffc000) == 0x6a080000
}
fn get_dpd_value(df: &DecQuad) -> u32 {
    DPD2BIN[(df.get_word(3) & 0x3ff) as usize] as u32
}
fn extract_bcd_digits(df: &DecQuad, buf: &mut [u8]) {
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    buf[0] = DECCOMBMSD[(sourhi >> 26) as usize] as u8;
    for i in 1..DECQUAD_PMAX as usize {
        buf[i] = 0;
    }
}
fn extract_full_bcd(df: &DecQuad) -> Vec<u8> {
    let mut buf = vec![0u8; DECQUAD_PMAX as usize + 4];
    extract_bcd_digits(df, &mut buf[2..]);
    buf
}
pub const DEC_ROUND_CEILING: Rounding = Rounding::Ceiling;
pub const DEC_ROUND_UP: Rounding = Rounding::Up;
pub const DEC_ROUND_HALF_UP: Rounding = Rounding::HalfUp;
pub const DEC_ROUND_HALF_EVEN: Rounding = Rounding::HalfEven;
pub const DEC_ROUND_HALF_DOWN: Rounding = Rounding::HalfDown;
pub const DEC_ROUND_DOWN: Rounding = Rounding::Down;
pub const DEC_ROUND_FLOOR: Rounding = Rounding::Floor;
pub const DEC_ROUND_05UP: Rounding = Rounding::ZeroFiveUp;
#[inline]
fn get_exponent_continuation(df: &DecQuad) -> i32 {
    ((df.get_word(0) & 0x03ffffff) >> (32 - 6 - 12)) as i32
}
#[inline]
fn get_biased_exponent(df: &DecQuad) -> i32 {
    DECCOMBEXP[(df.get_word(0) >> 26) as usize] as i32 + get_exponent_continuation(df)
}
/// Compute remainder of division
pub fn decQuadRemainder<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    decDivide(result, dfl, dfr, set, 0x40000000);
    result
}
/// Check if the number is canonical
pub fn decQuadIsCanonical(df: &DecQuad) -> u32 {
    let word0 = df.get_word(0);
    if (word0 & 0x78000000) == 0x78000000 {
        if (word0 & 0x7c000000) == 0x78000000 {
            if (word0 & ((0x03ffffff >> (32 - 6 - 12)) << (32 - 6 - 12))) != 0 {
                return 0;
            }
            if !(df.get_word(3) == 0 && df.get_word(2) == 0 && df.get_word(1) == 0
                && (df.get_word(0) & 0x00003fff) == 0)
            {
                return 0;
            }
            return 1;
        }
        if (word0 & ((0x01ffffff >> (32 - 6 - 12)) << (32 - 6 - 12))) != 0 {
            return 0;
        }
        if df.get_word(3) == 0 && df.get_word(2) == 0 && df.get_word(1) == 0
            && (df.get_word(0) & 0x00003fff) == 0
        {
            return 1;
        }
    }
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    let check_declet = |value: u32, shift: u32| -> bool {
        let declet = (value >> shift) & 0x3ff;
        let d0 = declet & 0x7;
        let d1 = (declet >> 3) & 0x7;
        let d2 = (declet >> 6) & 0x7;
        d0 <= 9 && d1 <= 9 && d2 <= 9
    };
    if ((sourhi & (0x300 << 4)) == 0 || (sourhi & (0x6e_u32 << 4)) != (0x6e_u32 << 4))
        && check_declet(sourhi, 4) && check_declet(sourmh, 16) && check_declet(sourmh, 6)
        && check_declet(sourml, 18) && check_declet(sourml, 8)
        && check_declet(sourlo, 20) && check_declet(sourlo, 10)
        && check_declet(sourlo, 0)
    {
        return 1;
    }
    0
}
/// Compare two decQuads, signaling on NaN
pub fn decQuadCompareSignal<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if is_nan(dfl) || is_nan(dfr) {
        set.status |= DEC_INVALID_OPERATION;
        return decNaNs(result, Some(dfl), Some(dfr), set);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    decQuadZero(result);
    if comp == 0 {
        return result;
    }
    result.set_byte(DECQUAD_BYTES - 1, 0x01);
    if comp < 0 {
        let byte0 = result.get_byte(0);
        result.set_byte(0, byte0 | 0x80);
    }
    result
}
/// Scale B operation (multiply by 10^n)
pub fn decQuadScaleB<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if is_nan(dfl) || is_nan(dfr) {
        return decNaNs(result, Some(dfl), Some(dfr), set);
    }
    let word0_r = dfr.get_word(0);
    if !((word0_r & 0x63ffc000) == 0x22080000 || (word0_r & 0x7bffc000) == 0x6a080000) {
        return decInvalid(result, set);
    }
    let digits = decQuadDigits(dfr);
    if digits > 5 {
        return decInvalid(result, set);
    }
    let mut expr = DPD2BIN[(dfr.get_word(3) & 0x3ff) as usize] as i32
        + DPD2BIN[((dfr.get_word(3) >> 10) & 0x3ff) as usize] as i32 * 1000;
    if expr > 2 * (DECQUAD_EMAX + DECQUAD_PMAX as i32) {
        return decInvalid(result, set);
    }
    if is_inf(dfl) {
        return decInfinity(result, dfl);
    }
    if is_signed(dfr) {
        expr = -expr;
    }
    *result = *dfl;
    let current_exp = get_biased_exponent(result) - DECQUAD_BIAS;
    decQuadSetExponent(result, set, current_exp + expr);
    result
}
/// Convert signed 32-bit integer to decQuad
pub fn decQuadFromInt32<'a>(result: &'a mut DecQuad, n: i32) -> &'a mut DecQuad {
    let mut u = n as u32;
    result.set_word(0, 0x22080000);
    result.set_word(1, 0);
    result.set_word(2, 0);
    if n < 0 {
        u = (!u).wrapping_add(1);
        result.set_word(0, result.get_word(0) | 0x80000000);
    }
    let mut encode = BIN2DPD[(u % 1000) as usize] as u32;
    u /= 1000;
    encode |= (BIN2DPD[(u % 1000) as usize] as u32) << 10;
    u /= 1000;
    encode |= (BIN2DPD[(u % 1000) as usize] as u32) << 20;
    u /= 1000;
    encode |= u << 30;
    result.set_word(DECQUAD_WORDS - 1, encode);
    result
}
/// Check if decQuad is zero
pub fn decQuadIsZero(df: &DecQuad) -> u32 {
    let word0 = df.get_word(0);
    let word1 = df.get_word(1);
    let word2 = df.get_word(2);
    let word3 = df.get_word(3);
    if word3 == 0 && word2 == 0 && word1 == 0 && (word0 & 0x1c003fff) == 0
        && (word0 & 0x60000000) != 0x60000000
    {
        1
    } else {
        0
    }
}
fn decode_to_bcd(df: &DecQuad, bcd: &mut [u8]) {
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    if bcd.len() > 0 {
        bcd[0] = DECCOMBMSD[(sourhi >> 26) as usize] as u8;
    }
    let decode_declet = |dpd: u32| -> [u8; 3] {
        let val = DPD2BIN[(dpd & 0x3ff) as usize];
        [((val / 100) % 10) as u8, ((val / 10) % 10) as u8, (val % 10) as u8]
    };
    let mut idx = 1;
    if idx + 3 <= bcd.len() {
        let dec = decode_declet(sourhi >> 4);
        bcd[idx..idx + 3].copy_from_slice(&dec);
        idx += 3;
    }
    if idx + 3 <= bcd.len() {
        let dpd = ((sourhi << 6) | (sourmh >> 26)) & 0x3ff;
        let dec = decode_declet(dpd);
        bcd[idx..idx + 3].copy_from_slice(&dec);
        idx += 3;
    }
    let declets = [
        (sourmh >> 16) & 0x3ff,
        (sourmh >> 6) & 0x3ff,
        ((sourmh << 4) | (sourml >> 28)) & 0x3ff,
        (sourml >> 18) & 0x3ff,
        (sourml >> 8) & 0x3ff,
        ((sourml << 2) | (sourlo >> 30)) & 0x3ff,
        (sourlo >> 20) & 0x3ff,
        (sourlo >> 10) & 0x3ff,
        sourlo & 0x3ff,
    ];
    for dpd in declets {
        if idx + 3 <= bcd.len() {
            let dec = decode_declet(dpd);
            bcd[idx..idx + 3].copy_from_slice(&dec);
            idx += 3;
        }
    }
}
fn apply_rounding_and_encode(
    result: &mut DecQuad,
    buf: &[u8],
    buf_offset: usize,
    drop: usize,
    exp: i32,
    sign: u32,
    set: &mut DecContext,
) {
    let mut bcd = [0u8; DECQUAD_PMAX as usize];
    let keep = DECQUAD_PMAX as usize - drop;
    bcd[drop..].copy_from_slice(&buf[buf_offset..buf_offset + keep]);
    if drop > 0 && buf_offset + keep < buf.len() {
        let round_digit = buf[buf_offset + keep];
        if round_digit != 0 {
            set.status |= DEC_INEXACT;
            let bump = match set.round {
                Rounding::HalfEven => {
                    if round_digit > 5 {
                        true
                    } else if round_digit == 5 {
                        bcd[DECQUAD_PMAX as usize - 1] & 1 != 0
                    } else {
                        false
                    }
                }
                Rounding::Down => false,
                Rounding::HalfDown => round_digit > 5,
                Rounding::HalfUp => round_digit >= 5,
                Rounding::Up => round_digit > 0,
                Rounding::Ceiling => sign == 0 && round_digit > 0,
                Rounding::Floor => sign != 0 && round_digit > 0,
                Rounding::ZeroFiveUp => {
                    if round_digit > 0 {
                        let lsd = bcd[DECQUAD_PMAX as usize - 1];
                        lsd == 0 || lsd == 5
                    } else {
                        false
                    }
                }
                Rounding::Max => false,
            };
            if bump {
                for i in (0..DECQUAD_PMAX as usize).rev() {
                    if bcd[i] < 9 {
                        bcd[i] += 1;
                        break;
                    }
                    bcd[i] = 0;
                }
            }
        }
    }
    encode_from_bcd(result, &bcd, exp, sign);
}
fn encode_from_bcd(result: &mut DecQuad, bcd: &[u8], exp: i32, sign: u32) {
    let biased_exp = (exp + DECQUAD_BIAS) as u32;
    let msd = if bcd.len() > 0 { bcd[0] } else { 0 };
    let comb_index = ((biased_exp >> 12) << 4) as usize + msd as usize;
    let comb = if comb_index < DECCOMBFROM.len() { DECCOMBFROM[comb_index] } else { 0 };
    let exp_cont = (biased_exp & 0xfff) << (32 - 6 - 12);
    let encode_declet = |d2: u8, d1: u8, d0: u8| -> u32 {
        let idx = (d2 as usize * 256) + (d1 as usize * 16) + d0 as usize;
        if idx < BCD2DPD.len() { BCD2DPD[idx] as u32 } else { 0 }
    };
    let get_bcd = |idx: usize| -> u8 { if idx < bcd.len() { bcd[idx] } else { 0 } };
    let mut encode0 = sign | comb | exp_cont;
    let dpd = encode_declet(get_bcd(1), get_bcd(2), get_bcd(3));
    encode0 |= dpd << 4;
    let dpd = encode_declet(get_bcd(4), get_bcd(5), get_bcd(6));
    encode0 |= dpd >> 6;
    result.set_word(0, encode0);
    let mut encode1 = dpd << 26;
    let dpd = encode_declet(get_bcd(7), get_bcd(8), get_bcd(9));
    encode1 |= dpd << 16;
    let dpd = encode_declet(get_bcd(10), get_bcd(11), get_bcd(12));
    encode1 |= dpd << 6;
    let dpd = encode_declet(get_bcd(13), get_bcd(14), get_bcd(15));
    encode1 |= dpd >> 4;
    result.set_word(1, encode1);
    let mut encode2 = dpd << 28;
    let dpd = encode_declet(get_bcd(16), get_bcd(17), get_bcd(18));
    encode2 |= dpd << 18;
    let dpd = encode_declet(get_bcd(19), get_bcd(20), get_bcd(21));
    encode2 |= dpd << 8;
    let dpd = encode_declet(get_bcd(22), get_bcd(23), get_bcd(24));
    encode2 |= dpd >> 2;
    result.set_word(2, encode2);
    let mut encode3 = dpd << 30;
    let dpd = encode_declet(get_bcd(25), get_bcd(26), get_bcd(27));
    encode3 |= dpd << 20;
    let dpd = encode_declet(get_bcd(28), get_bcd(29), get_bcd(30));
    encode3 |= dpd << 10;
    let dpd = encode_declet(get_bcd(31), get_bcd(32), get_bcd(33));
    encode3 |= dpd;
    result.set_word(3, encode3);
}
const DECQUAD_STRING: usize = 43;
pub static DECTESTMSD: [i8; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    -32,
    -32,
    -32,
    -32,
    -64,
    -64,
    -64,
    -64,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
/// Copy a DecQuad
pub fn decQuadCopy<'a>(result: &'a mut DecQuad, dfl: &DecQuad) -> &'a mut DecQuad {
    if !std::ptr::eq(result, dfl) {
        *result = *dfl;
    }
    result
}
/// Get the exponent of a DecQuad
pub fn decQuadGetExponent(df: &DecQuad) -> i32 {
    let word0 = df.get_word(0);
    if (word0 & 0x78000000) == 0x78000000 {
        return (word0 & 0x7E000000) as i32;
    }
    let comb = (word0 >> 26) as usize;
    let exp_cont = ((word0 & 0x03FFFFFF) >> (32 - 6 - 12)) as i32;
    (DECCOMBEXP[comb] as i32) + exp_cont - DECQUAD_BIAS
}
/// Subtract two DecQuad numbers
pub fn decQuadSubtract<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if (dfr.get_word(0) & 0x7C000000) == 0x7C000000 {
        return decQuadAdd(result, dfl, dfr, set);
    }
    let mut temp = *dfr;
    let byte_idx = DECQUAD_BYTES - 1;
    temp.bytes[byte_idx] ^= 0x80;
    decQuadAdd(result, dfl, &temp, set)
}
/// Divide two DecQuad numbers
pub fn decQuadDivide<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    decDivide(result, dfl, dfr, set, DECFLOAT_SIGN)
}
/// Convert DecQuad to packed decimal
pub fn decQuadToPacked(df: &DecQuad, exp: &mut i32, packed: &mut [u8]) -> i32 {
    let mut bcdar = [0u8; DECQUAD_PMAX + 2];
    if df.is_infinite() {
        bcdar.fill(0);
        *exp = 0x78000000;
    } else {
        let bcd = extract_bcd(df);
        bcdar[1..DECQUAD_PMAX + 1].copy_from_slice(&bcd[..DECQUAD_PMAX]);
        if df.is_nan() {
            bcdar[1] = 0;
            *exp = (df.get_word(0) & 0x7E000000) as i32;
        } else {
            *exp = decQuadGetExponent(df);
        }
    }
    bcdar[0] = 0;
    let sign_nibble = if (df.get_word(0) & DECFLOAT_SIGN) != 0 { 0x0D } else { 0x0C };
    bcdar[DECQUAD_PMAX + 1] = sign_nibble;
    let packed_len = (DECQUAD_PMAX + 2) / 2;
    for i in 0..packed_len {
        packed[i] = (bcdar[i * 2] << 4) | bcdar[i * 2 + 1];
    }
    if sign_nibble == 0x0D { DECFLOAT_SIGN as i32 } else { 0 }
}
/// Reduce a DecQuad (remove trailing zeros)
pub fn decQuadReduce<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if !std::ptr::eq(result, df) {
        *result = *df;
    }
    if df.is_nan() {
        return decNaNs(result, Some(df), None, set);
    }
    if df.is_infinite() {
        return decInfinity(result, df);
    }
    if df.is_zero() {
        let sign = df.get_word(0) & DECFLOAT_SIGN;
        decQuadZero(result);
        let mut word0 = result.get_word(0);
        word0 |= sign;
        result.set_word(0, word0);
        return result;
    }
    let buf = extract_bcd(df);
    let mut lsd = DECQUAD_PMAX - 1;
    if buf[lsd] != 0 {
        return result;
    }
    while lsd > 0 && buf[lsd] == 0 {
        lsd -= 1;
    }
    let mut num = BcdNum::default();
    num.sign = df.get_word(0) & DECFLOAT_SIGN;
    num.exponent = decQuadGetExponent(df) + (DECQUAD_PMAX - 1 - lsd) as i32;
    num.msd = buf[..=lsd].to_vec();
    num.msd_idx = 0;
    num.lsd_idx = lsd;
    decFinalize(result, &mut num, set)
}
/// Helper function to extract BCD digits from a DecQuad
fn extract_bcd(df: &DecQuad) -> Vec<u8> {
    let mut bcd = vec![0u8; DECQUAD_PMAX];
    let sourhi = df.get_word(0);
    let sourmh = df.get_word(1);
    let sourml = df.get_word(2);
    let sourlo = df.get_word(3);
    bcd[0] = DECCOMBMSD[(sourhi >> 26) as usize] as u8;
    let declets = [
        ((sourhi >> 4) & 0x3FF) as usize,
        (((sourhi << 6) | (sourmh >> 26)) & 0x3FF) as usize,
        ((sourmh >> 16) & 0x3FF) as usize,
        ((sourmh >> 6) & 0x3FF) as usize,
        (((sourmh << 4) | (sourml >> 28)) & 0x3FF) as usize,
        ((sourml >> 18) & 0x3FF) as usize,
        ((sourml >> 8) & 0x3FF) as usize,
        (((sourml << 2) | (sourlo >> 30)) & 0x3FF) as usize,
        ((sourlo >> 20) & 0x3FF) as usize,
        ((sourlo >> 10) & 0x3FF) as usize,
        (sourlo & 0x3FF) as usize,
    ];
    for (i, &declet) in declets.iter().enumerate() {
        let entry = &DPD2BCD8[declet];
        let offset = 1 + i * 3;
        if offset + 2 < DECQUAD_PMAX {
            bcd[offset] = entry[0];
            bcd[offset + 1] = entry[1];
            bcd[offset + 2] = entry[2];
        }
    }
    bcd
}
/// Helper to get word from decQuad (big-endian word access)
#[inline]
fn get_word(dq: &decQuad, idx: usize) -> u32 {
    let word_idx = (16 / 4) - 1 - idx;
    unsafe { *dq.words.add(word_idx) }
}
const DEC_UNDERFLOW: u32 = 0x00002000;
const DEC_INVALID_CONTEXT: u32 = 0x00000040;
const DIVIDEINT: u32 = 0x20000000;
static ALLNINES: [u8; 34] = [9; 34];
fn bcd_to_dpd(bcd: u32) -> u32 {
    let d0 = bcd % 10;
    let d1 = (bcd / 10) % 10;
    let d2 = (bcd / 100) % 10;
    if d0 < 8 && d1 < 8 && d2 < 8 {
        (d2 << 7) | (d1 << 4) | d0
    } else {
        d0 | (d1 << 4) | (d2 << 7)
    }
}
fn bin_to_dpd(bin: u32) -> u32 {
    let d0 = bin % 10;
    let d1 = (bin / 10) % 10;
    let d2 = (bin / 100) % 10;
    bcd_to_dpd(d2 * 256 + d1 * 16 + d0)
}
/// Check if a decQuad is logical (all digits 0 or 1)
pub fn decQuadIsLogical(df: &DecQuad) -> u32 {
    let w0 = df.get_word(0);
    let w1 = df.get_word(1);
    let w2 = df.get_word(2);
    let w3 = df.get_word(3);
    let comb_check = (w0 & 0xfbffc000) == 0x22080000;
    let coeff_check = (w0 & !0xffffc912) == 0 && (w1 & !0x44912449) == 0
        && (w2 & !0x12449124) == 0 && (w3 & !0x49124491) == 0;
    if comb_check && coeff_check { 1 } else { 0 }
}
/// Check if two decQuads have the same quantum (exponent)
pub fn decQuadSameQuantum(dfl: &DecQuad, dfr: &DecQuad) -> u32 {
    let wl0 = dfl.get_word(0);
    let wr0 = dfr.get_word(0);
    if (wl0 & 0x78000000) == 0x78000000 || (wr0 & 0x78000000) == 0x78000000 {
        if (wl0 & 0x7c000000) == 0x7c000000 && (wr0 & 0x7c000000) == 0x7c000000 {
            return 1;
        }
        if (wl0 & 0x7c000000) == 0x78000000 && (wr0 & 0x7c000000) == 0x78000000 {
            return 1;
        }
        return 0;
    }
    let exp_l = DECCOMBEXP[(wl0 >> 26) as usize] as i32
        + ((wl0 & 0x03ffffff) >> (32 - 6 - 12)) as i32;
    let exp_r = DECCOMBEXP[(wr0 >> 26) as usize] as i32
        + ((wr0 & 0x03ffffff) >> (32 - 6 - 12)) as i32;
    if exp_l == exp_r { 1 } else { 0 }
}
/// Convert decQuad to unsigned 32-bit integer with exact rounding
pub fn decQuadToUInt32Exact(df: &DecQuad, set: &mut DecContext, round: Rounding) -> u32 {
    decToInt32(df, set, round, 1, 1)
}
/// Encode coefficient into DPD format
fn encode_dpd_coefficient(df: &mut DecQuad, mut encode: u32, bcd: &[u8]) {
    let mut words = [0u32; 4];
    let mut padded = [0u8; DECQUAD_PMAX as usize];
    let offset = DECQUAD_PMAX as usize - bcd.len();
    padded[offset..].copy_from_slice(bcd);
    let mut dpd_values = [0u32; 11];
    for i in 0..11 {
        let idx = DECQUAD_PMAX as usize - 3 - (i * 3);
        let d0 = padded.get(idx + 2).copied().unwrap_or(0) as u32;
        let d1 = padded.get(idx + 1).copied().unwrap_or(0) as u32;
        let d2 = padded.get(idx).copied().unwrap_or(0) as u32;
        dpd_values[i] = bcd_to_dpd(d2 * 256 + d1 * 16 + d0);
    }
    words[0] = encode | (dpd_values[10] << 4) | (dpd_values[9] >> 6);
    words[1] = (dpd_values[9] << 26) | (dpd_values[8] << 16) | (dpd_values[7] << 6)
        | (dpd_values[6] >> 4);
    words[2] = (dpd_values[6] << 28) | (dpd_values[5] << 18) | (dpd_values[4] << 8)
        | (dpd_values[3] >> 2);
    words[3] = (dpd_values[3] << 30) | (dpd_values[2] << 20) | (dpd_values[1] << 10)
        | dpd_values[0];
    df.set_word(0, words[0]);
    df.set_word(1, words[1]);
    df.set_word(2, words[2]);
    df.set_word(3, words[3]);
}
/// Internal BCD to decQuad conversion
fn decQuadFromBCD_internal<'a>(
    df: &'a mut DecQuad,
    exp: i32,
    bcd: &[u8],
    sign: u32,
) -> &'a mut DecQuad {
    decQuadFromBCD(df, exp, bcd, sign as i32)
}
/// Create decQuad from unsigned 32-bit integer
pub fn decQuadFromUInt32<'a>(result: &'a mut DecQuad, u: u32) -> &'a mut DecQuad {
    result.set_word(0, 0x22080000);
    result.set_word(1, 0);
    result.set_word(2, 0);
    let mut val = u;
    let mut encode = bin_to_dpd(val % 1000);
    val /= 1000;
    encode |= bin_to_dpd(val % 1000) << 10;
    val /= 1000;
    encode |= bin_to_dpd(val % 1000) << 20;
    val /= 1000;
    encode |= val << 30;
    result.set_word(3, encode);
    let w2 = result.get_word(2) | (val >> 2);
    result.set_word(2, w2);
    result
}
/// Return +df (unary plus operation)
pub fn decQuadPlus<'a>(
    result: &'a mut DecQuad,
    df: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if df.is_nan_internal() {
        return decNaNs(result, Some(df), None, set);
    }
    decCanonical(result, df);
    if df.is_zero_internal() {
        let w = result.bytes[DECQUAD_BYTES - 1] & !0x80;
        result.bytes[DECQUAD_BYTES - 1] = w;
    }
    result
}
/// Integer division
pub fn decQuadDivideInteger<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    dfr: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    decDivide(result, dfl, dfr, set, DIVIDEINT)
}
/// Return next representable value toward negative infinity
pub fn decQuadNextMinus<'a>(
    result: &'a mut DecQuad,
    dfl: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecQuad {
    if dfl.is_infinity_internal() && !dfl.is_signed_internal() {
        result.set_word(0, 0x77ffcff3);
        result.set_word(1, 0xfcff3fcf);
        result.set_word(2, 0xf3fcff3f);
        result.set_word(3, 0xcff3fcff);
        return result;
    }
    let mut delta = DecQuad::default();
    decQuadZero(&mut delta);
    delta.set_word(3, 1);
    delta.set_word(0, 0x80000000);
    let save_round = set.round;
    set.round = Rounding::Floor;
    let save_stat = set.status;
    decQuadAdd(result, dfl, &delta, set);
    if result.is_zero_internal() {
        let w0 = result.get_word(0) ^ 0x80000000;
        result.set_word(0, w0);
    }
    set.status &= DEC_INVALID_OPERATION;
    set.status |= save_stat;
    set.round = save_round;
    result
}
/// Check if decQuad is subnormal
pub fn decQuadIsSubnormal(df: &DecQuad) -> u32 {
    if df.is_special() {
        return 0;
    }
    if decQuadIsNormal(df) != 0 {
        return 0;
    }
    if df.is_zero_internal() {
        return 0;
    }
    1
}
impl std::fmt::Debug for DecQuad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecQuad").field("bytes", &self.bytes).finish()
    }
}
impl Default for BcdNum {
    fn default() -> Self {
        Self {
            msd: vec![0; 128],
            msd_idx: 0,
            lsd_idx: 0,
            sign: 0,
            exponent: 0,
        }
    }
}
impl DecQuad {
    /// Create a new zero DecQuad
    pub fn new() -> Self {
        Self { bytes: [0; 16] }
    }
    /// Get word at index (big-endian layout)
    #[inline]
    fn get_word(&self, idx: usize) -> u32 {
        let start = (3 - idx) * 4;
        u32::from_be_bytes([
            self.bytes[start],
            self.bytes[start + 1],
            self.bytes[start + 2],
            self.bytes[start + 3],
        ])
    }
    /// Set word at index (big-endian layout)
    #[inline]
    fn set_word(&mut self, idx: usize, val: u32) {
        let start = (3 - idx) * 4;
        let bytes = val.to_be_bytes();
        self.bytes[start] = bytes[0];
        self.bytes[start + 1] = bytes[1];
        self.bytes[start + 2] = bytes[2];
        self.bytes[start + 3] = bytes[3];
    }
    /// Get byte at index (big-endian layout)
    #[inline]
    fn get_byte(&self, idx: usize) -> u8 {
        self.bytes[15 - idx]
    }
    /// Set byte at index (big-endian layout)
    #[inline]
    fn set_byte(&mut self, idx: usize, val: u8) {
        self.bytes[15 - idx] = val;
    }
    pub fn words(&self) -> [u32; 4] {
        let mut words = [0u32; 4];
        for i in 0..4 {
            words[i] = u32::from_le_bytes([
                self.bytes[i * 4],
                self.bytes[i * 4 + 1],
                self.bytes[i * 4 + 2],
                self.bytes[i * 4 + 3],
            ]);
        }
        words
    }
    pub fn set_words(&mut self, words: &[u32; 4]) {
        for i in 0..4 {
            let bytes = words[i].to_le_bytes();
            self.bytes[i * 4] = bytes[0];
            self.bytes[i * 4 + 1] = bytes[1];
            self.bytes[i * 4 + 2] = bytes[2];
            self.bytes[i * 4 + 3] = bytes[3];
        }
    }
    fn word(&self, idx: usize) -> u32 {
        let actual_idx = 3 - idx;
        u32::from_le_bytes([
            self.bytes[actual_idx * 4],
            self.bytes[actual_idx * 4 + 1],
            self.bytes[actual_idx * 4 + 2],
            self.bytes[actual_idx * 4 + 3],
        ])
    }
    /// Check if this is a NaN
    #[inline]
    pub fn is_nan(&self) -> bool {
        (self.get_word(0) & DECFLOAT_NAN) == DECFLOAT_NAN
    }
    /// Check if this is infinity
    #[inline]
    pub fn is_infinite(&self) -> bool {
        (self.get_word(0) & 0x7c000000) == DECFLOAT_INF
    }
    /// Check if this is signed (negative)
    #[inline]
    pub fn is_signed(&self) -> bool {
        (self.get_word(0) & DECFLOAT_SIGN) != 0
    }
    /// Check if this is zero
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.get_word(3) == 0 && self.get_word(2) == 0 && self.get_word(1) == 0
            && (self.get_word(0) & 0x1c003fff) == 0
            && (self.get_word(0) & 0x60000000) != 0x60000000
    }
    /// Check if this is finite (not NaN and not infinite)
    #[inline]
    pub fn is_finite(&self) -> bool {
        (self.get_word(0) & 0x78000000) != 0x78000000
    }
    /// Check if this is a logical value (all digits 0 or 1)
    #[inline]
    pub fn is_logical(&self) -> bool {
        (self.get_word(0) & 0xfbffc000) == 0x22080000
            && (self.get_word(0) & !0xffffc912) == 0
            && (self.get_word(1) & !0x44912449) == 0
            && (self.get_word(2) & !0x12449124) == 0
            && (self.get_word(3) & !0x49124491) == 0
    }
    /// Check if the value is NaN (quiet or signaling)
    #[inline]
    fn is_nan_internal(&self) -> bool {
        (self.get_word(0) & 0x7c000000) == 0x7c000000
    }
    /// Check if the value is signaling NaN
    #[inline]
    fn is_snan_internal(&self) -> bool {
        (self.get_word(0) & 0x7e000000) == 0x7e000000
    }
    /// Check if the value is quiet NaN
    #[inline]
    fn is_qnan_internal(&self) -> bool {
        (self.get_word(0) & 0x7e000000) == 0x7c000000
    }
    /// Check if the value is infinity
    #[inline]
    fn is_infinity_internal(&self) -> bool {
        (self.get_word(0) & 0x78000000) == 0x78000000
    }
    /// Check if the value is signed (negative)
    #[inline]
    fn is_signed_internal(&self) -> bool {
        (self.get_word(0) & 0x80000000) != 0
    }
    /// Check if the value is zero
    #[inline]
    fn is_zero_internal(&self) -> bool {
        self.get_word(3) == 0 && self.get_word(2) == 0 && self.get_word(1) == 0
            && (self.get_word(0) & 0x1c003fff) == 0
            && (self.get_word(0) & 0x60000000) != 0x60000000
    }
    /// Get the exponent
    fn get_exponent_internal(&self) -> i32 {
        let sourhi = self.get_word(0);
        let comb_exp = DECCOMBEXP[(sourhi >> 26) as usize] as i32;
        let cont_exp = ((sourhi & 0x03ffffff) >> (32 - 6 - 12)) as i32;
        comb_exp + cont_exp - DECQUAD_BIAS
    }
    /// Check if this is signaling NaN
    #[inline]
    pub fn is_snan(&self) -> bool {
        (self.get_word(0) & 0x7E000000) == DECFLOAT_SNAN
    }
    /// Check if this is special (Inf or NaN)
    #[inline]
    pub fn is_special(&self) -> bool {
        (self.get_word(0) & 0x78000000) == 0x78000000
    }
}
impl BcdNum {
    pub fn new(size: usize) -> Self {
        BcdNum {
            msd: vec![0u8; size],
            msd_idx: 0,
            lsd_idx: 0,
            sign: 0,
            exponent: 0,
        }
    }
}
