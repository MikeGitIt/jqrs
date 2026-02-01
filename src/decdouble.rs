//! Module: decdouble
//!
//! Contains 89 transpiled functions:
//! - decNaNs:17749432081802947218:./src/decNumber/decDouble.c
//! - decCanonical:17142465090312332316:./src/decNumber/decDouble.c
//! - decDoubleFromWider:362522921258399172:./src/decNumber/decDouble.c
//! - decDoubleIsCanonical:10596112529964294178:./src/decNumber/decDouble.c
//! - decDoubleIsLogical:15073832029141056279:./src/decNumber/decDouble.c
//! - decDoubleRemainderNear:15292643537711274216:./src/decNumber/decDouble.c
//! - decDoubleVersion:5957435936521089324:./src/decNumber/decDouble.c
//! - decDoubleDigits:8676830158695629361:./src/decNumber/decDouble.c
//! - decDivide:1340564579812564026:./src/decNumber/decDouble.c
//! - decDoubleFMA:2296742438025918713:./src/decNumber/decDouble.c
//! - decDoubleToPacked:1113351252649685851:./src/decNumber/decDouble.c
//! - decDoubleToBCD:15230003957305610857:./src/decNumber/decDouble.c
//! - decDoubleScaleB:2220532594790945368:./src/decNumber/decDouble.c
//! - decInfinity:10643065395196952983:./src/decNumber/decDouble.c
//! - decDoubleClass:15938678006649697664:./src/decNumber/decDouble.c
//! - decDoubleGetCoefficient:16713750851309327744:./src/decNumber/decDouble.c
//! - decDoubleIsPositive:16155252009887114136:./src/decNumber/decDouble.c
//! - decDoubleToWider:16811426647296564485:./src/decNumber/decDouble.c
//! - decDoubleAnd:235104082565813369:./src/decNumber/decDouble.c
//! - decDoubleNextToward:5710503231710604232:./src/decNumber/decDouble.c
//! - decDoubleGetExponent:13177050864222243737:./src/decNumber/decDouble.c
//! - decDoubleShow:1834111963549856259:./src/decNumber/decDouble.c
//! - decDoubleOr:10382835826416049349:./src/decNumber/decDouble.c
//! - decDoubleDivide:3596324649214296840:./src/decNumber/decDouble.c
//! - decDoubleDivideInteger:7836513731375900796:./src/decNumber/decDouble.c
//! - decDoubleCompare:15592450702267828135:./src/decNumber/decDouble.c
//! - decDoubleToUInt32:9149771945402925878:./src/decNumber/decDouble.c
//! - decDoubleIsNegative:7477659575525203779:./src/decNumber/decDouble.c
//! - decDoubleIsSignalling:12643960325218904486:./src/decNumber/decDouble.c
//! - decDoubleMultiply:4692523101803029959:./src/decNumber/decDouble.c
//! - decDoubleNextMinus:13914711624530568911:./src/decNumber/decDouble.c
//! - decDoublePlus:16455477506984229492:./src/decNumber/decDouble.c
//! - decDoubleIsInteger:7497539146005191296:./src/decNumber/decDouble.c
//! - decDoubleIsSubnormal:11531981394285835843:./src/decNumber/decDouble.c
//! - decDoubleSameQuantum:10596086896507792543:./src/decNumber/decDouble.c
//! - decDoubleCanonical:4321248788805069635:./src/decNumber/decDouble.c
//! - decDoubleIsSignaling:2609950549972352049:./src/decNumber/decDouble.c
//! - decDoubleXor:2960587828609524988:./src/decNumber/decDouble.c
//! - decDoubleCopySign:9753497167704670215:./src/decNumber/decDouble.c
//! - decDoubleMinMag:14145022465103056650:./src/decNumber/decDouble.c
//! - decDoubleCopy:10230044084945676448:./src/decNumber/decDouble.c
//! - decDoubleSetCoefficient:6416143160995269385:./src/decNumber/decDouble.c
//! - decFinalize:17864086006616444042:./src/decNumber/decDouble.c
//! - decDoubleMin:9090549548095805745:./src/decNumber/decDouble.c
//! - decDoubleZero:1870020910290259323:./src/decNumber/decDouble.c
//! - decDoubleIsNaN:16731079366099543829:./src/decNumber/decDouble.c
//! - decDoubleCompareTotal:17557633653977969128:./src/decNumber/decDouble.c
//! - decDoubleToInt32Exact:13519665437282892918:./src/decNumber/decDouble.c
//! - decDoubleRotate:9280312058206873189:./src/decNumber/decDouble.c
//! - decDoubleFromInt32:13000570140857793221:./src/decNumber/decDouble.c
//! - decDoubleCopyAbs:14247590573847841979:./src/decNumber/decDouble.c
//! - decDoubleToUInt32Exact:9301190642679008750:./src/decNumber/decDouble.c
//! - decDoubleIsSigned:10787811596747458362:./src/decNumber/decDouble.c
//! - decDoubleToEngString:16674621028669661674:./src/decNumber/decDouble.c
//! - decToIntegral:16451784145593058120:./src/decNumber/decDouble.c
//! - decDoubleLogB:2555062517918987815:./src/decNumber/decDouble.c
//! - decDoubleSubtract:12556607687152540345:./src/decNumber/decDouble.c
//! - decDoubleInvert:18041348356339175316:./src/decNumber/decDouble.c
//! - decDoubleMaxMag:6030035621264323800:./src/decNumber/decDouble.c
//! - decDoubleIsInfinite:15412072957446545154:./src/decNumber/decDouble.c
//! - decDoubleCompareSignal:6898310639489355123:./src/decNumber/decDouble.c
//! - decDoubleSetExponent:1919093557958173187:./src/decNumber/decDouble.c
//! - decDoubleMax:2597842413181145993:./src/decNumber/decDouble.c
//! - decDoubleCompareTotalMag:12133642497774427531:./src/decNumber/decDouble.c
//! - decDoubleIsZero:1944080950415303145:./src/decNumber/decDouble.c
//! - decDoubleFromBCD:15443596688186839017:./src/decNumber/decDouble.c
//! - decNumCompare:8142437862154526001:./src/decNumber/decDouble.c
//! - decDoubleAdd:12459430857671918259:./src/decNumber/decDouble.c
//! - decDoubleToIntegralValue:591664437422198497:./src/decNumber/decDouble.c
//! - decDoubleToIntegralExact:15164253795784091936:./src/decNumber/decDouble.c
//! - decDoubleFromUInt32:161074967165641084:./src/decNumber/decDouble.c
//! - decDoubleMinus:1516666449470972532:./src/decNumber/decDouble.c
//! - decDoubleToString:6250843483191567097:./src/decNumber/decDouble.c
//! - decDoubleReduce:14930309365263415914:./src/decNumber/decDouble.c
//! - decDoubleIsNormal:18388419653468951545:./src/decNumber/decDouble.c
//! - decDoubleClassString:14088806639560890592:./src/decNumber/decDouble.c
//! - decDoubleIsFinite:8614175974289136021:./src/decNumber/decDouble.c
//! - decDoubleRemainder:15391382074073828874:./src/decNumber/decDouble.c
//! - decDoubleFromPackedChecked:15545070560489535086:./src/decNumber/decDouble.c
//! - decDoubleCopyNegate:2745643386141685706:./src/decNumber/decDouble.c
//! - decInvalid:8995817802493243291:./src/decNumber/decDouble.c
//! - decDoubleNextPlus:9709880431467013786:./src/decNumber/decDouble.c
//! - decDoubleFromPacked:13422197107618567181:./src/decNumber/decDouble.c
//! - decDoubleRadix:3702443068527164891:./src/decNumber/decDouble.c
//! - decDoubleQuantize:3625191406970802634:./src/decNumber/decDouble.c
//! - decDoubleShift:12870257891366409595:./src/decNumber/decDouble.c
//! - decDoubleAbs:13033965534140960278:./src/decNumber/decDouble.c
//! - decDoubleToInt32:10518162030431759359:./src/decNumber/decDouble.c
//! - decDoubleFromString:3833716947781545116:./src/decNumber/decDouble.c

use std::ptr;
use crate::types::{DecDouble, DecQuad, DecContext, DecClass, Rounding, BcdNum};
use std::cmp::Ordering;
// Note: All decDouble* functions are defined locally in this file
use std::fmt;
use crate::decquad::DECTESTMSD;
const DECDPUN: usize = 3;
const DECPMAX: usize = 16;
const DECEMIN: i32 = -383;
const DECEMAX: i32 = 384;
const DECBIAS: i32 = 398;
const DEC_INVALID_OPERATION: u32 = 0x00000080;
const DEC_INEXACT: u32 = 0x00000020;
const DEC_INVALID_CONTEXT: u32 = 0x00000040;
const DECSPECIAL_MASK: u32 = 0x78000000;
const DECSPECIAL_VALUE: u32 = 0x78000000;
const DECNAN_MASK: u32 = 0x7c000000;
const DECNAN_VALUE: u32 = 0x7c000000;
const DECSNAN_MASK: u32 = 0x7e000000;
const DECSNAN_VALUE: u32 = 0x7e000000;
const DECINF_MASK: u32 = 0x7c000000;
const DECINF_VALUE: u32 = 0x78000000;
const DECSIGN_MASK: u32 = 0x80000000;
const DECLOGICAL_COMB_MASK: u32 = 0xfbfc0000;
const DECLOGICAL_COMB_VALUE: u32 = 0x22380000;
const DECLOGICAL_HI_MASK: u32 = 0xfffc9124;
const DECLOGICAL_LO_MASK: u32 = 0x49124491;
const DECZERO_LO_MASK: u32 = 0x1c03ffff;
const DECZERO_SPECIAL_MASK: u32 = 0x60000000;
const DECZERO_SPECIAL_VALUE: u32 = 0x60000000;
const DECEXP_CONT_MASK: u32 = 0x03ffffff;
const DECEXP_SHIFT: u32 = 32 - 6 - 8;
const ACC_SIZE: usize = ((1 + ((((DECPMAX + 9 - 1) / 9) * 9) / 9) * 18 + 1 + DECPMAX + 2
    + 3) & !3);
const BUF_SIZE: usize = 4 + DECPMAX * 3 + 2 * 0;
/// Combined field to exponent lookup table
static DECCOMBEXP: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 3,
    0, 1, 2, 3, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000,
    0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000,
    0x78000000, 0x78000000, 0x78000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000,
    0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000,
    0x7c000000, 0x7e000000, 0x7e000000, 0x7e000000, 0x7e000000,
];
/// Combined field to MSD (most significant digit) lookup table
static DECCOMBMSD: [u8; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 8,
    8, 8, 8, 8, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7,
    9, 9, 9, 9, 9, 9, 9, 9,
];
static DECCOMBFROM: [u32; 160] = [
    0x00000000, 0x04000000, 0x08000000, 0x0c000000, 0x10000000, 0x14000000, 0x18000000,
    0x1c000000, 0x60000000, 0x64000000, 0x00000000, 0x04000000, 0x08000000, 0x0c000000,
    0x10000000, 0x14000000, 0x20000000, 0x24000000, 0x28000000, 0x2c000000, 0x30000000,
    0x34000000, 0x38000000, 0x3c000000, 0x68000000, 0x6c000000, 0x20000000, 0x24000000,
    0x28000000, 0x2c000000, 0x30000000, 0x34000000, 0x40000000, 0x44000000, 0x48000000,
    0x4c000000, 0x50000000, 0x54000000, 0x58000000, 0x5c000000, 0x70000000, 0x74000000,
    0x40000000, 0x44000000, 0x48000000, 0x4c000000, 0x50000000, 0x54000000, 0x00000000,
    0x04000000, 0x08000000, 0x0c000000, 0x10000000, 0x14000000, 0x18000000, 0x1c000000,
    0x60000000, 0x64000000, 0x00000000, 0x04000000, 0x08000000, 0x0c000000, 0x10000000,
    0x14000000, 0x00000000, 0x04000000, 0x08000000, 0x0c000000, 0x10000000, 0x14000000,
    0x18000000, 0x1c000000, 0x60000000, 0x64000000, 0x00000000, 0x04000000, 0x08000000,
    0x0c000000, 0x10000000, 0x14000000, 0x20000000, 0x24000000, 0x28000000, 0x2c000000,
    0x30000000, 0x34000000, 0x38000000, 0x3c000000, 0x68000000, 0x6c000000, 0x20000000,
    0x24000000, 0x28000000, 0x2c000000, 0x30000000, 0x34000000, 0x40000000, 0x44000000,
    0x48000000, 0x4c000000, 0x50000000, 0x54000000, 0x58000000, 0x5c000000, 0x70000000,
    0x74000000, 0x40000000, 0x44000000, 0x48000000, 0x4c000000, 0x50000000, 0x54000000,
    0x00000000, 0x04000000, 0x08000000, 0x0c000000, 0x10000000, 0x14000000, 0x18000000,
    0x1c000000, 0x60000000, 0x64000000, 0x00000000, 0x04000000, 0x08000000, 0x0c000000,
    0x10000000, 0x14000000, 0x00000000, 0x04000000, 0x08000000, 0x0c000000, 0x10000000,
    0x14000000, 0x18000000, 0x1c000000, 0x60000000, 0x64000000, 0x00000000, 0x04000000,
    0x08000000, 0x0c000000, 0x10000000, 0x14000000, 0x20000000, 0x24000000, 0x28000000,
    0x2c000000, 0x30000000, 0x34000000, 0x38000000, 0x3c000000, 0x68000000, 0x6c000000,
    0x20000000, 0x24000000, 0x28000000, 0x2c000000, 0x30000000, 0x34000000,
];
static DECSTICKYTAB: [u8; 10] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
/// Compute DPD encoding from 3 BCD digits (const fn for use in static initialization)
const fn bcd_to_dpd_const(a: u8, b: u8, c: u8) -> u16 {
    // DPD encoding algorithm for 3 BCD digits (0-9 each)
    // Based on IEEE 754-2008 densely packed decimal encoding
    let a = a as u16;
    let b = b as u16;
    let c = c as u16;

    // Extract high bit of each digit (determines if >= 8)
    let ah = (a >> 3) & 1;
    let bh = (b >> 3) & 1;
    let ch = (c >> 3) & 1;

    // Low 3 bits of each digit
    let al = a & 7;
    let bl = b & 7;
    let cl = c & 7;

    // DPD encoding rules from IEEE 754-2008
    if ah == 0 && bh == 0 && ch == 0 {
        // All small (0-7): direct encoding
        (al << 7) | (bl << 4) | cl
    } else if ah == 0 && bh == 0 && ch == 1 {
        // c large (8-9)
        (al << 7) | (bl << 4) | (cl & 1) | 0x08
    } else if ah == 0 && bh == 1 && ch == 0 {
        // b large (8-9)
        (al << 7) | ((bl & 1) << 4) | cl | 0x0A
    } else if ah == 0 && bh == 1 && ch == 1 {
        // b and c large
        (al << 7) | 0x4E | ((bl & 1) << 4) | (cl & 1)
    } else if ah == 1 && bh == 0 && ch == 0 {
        // a large (8-9)
        ((al & 1) << 7) | (bl << 4) | cl | 0x0C
    } else if ah == 1 && bh == 0 && ch == 1 {
        // a and c large
        (bl << 4) | 0x2E | ((al & 1) << 7) | (cl & 1)
    } else if ah == 1 && bh == 1 && ch == 0 {
        // a and b large
        ((bl & 1) << 4) | cl | 0x0E | ((al & 1) << 7)
    } else {
        // all large
        0x6E | ((al & 1) << 7) | ((bl & 1) << 4) | (cl & 1)
    }
}
/// Convert 3 BCD digits to DPD
fn bcd_to_dpd(bcd: &[u8]) -> u32 {
    let a = *bcd.first().unwrap_or(&0);
    let b = *bcd.get(1).unwrap_or(&0);
    let c = *bcd.get(2).unwrap_or(&0);
    bcd_to_dpd_const(a, b, c) as u32
}
fn dpd_to_bcd(dpd: u32) -> [u8; 4] {
    let val = dpd & 0x3ff;
    let d0 = ((val / 100) % 10) as u8;
    let d1 = ((val / 10) % 10) as u8;
    let d2 = (val % 10) as u8;
    [0, d0, d1, d2]
}
/// Copy a decDouble value
pub fn decDoubleCopy<'a>(result: &'a mut DecDouble, dfl: &DecDouble) -> &'a mut DecDouble {
    if !ptr::eq(dfl as *const _, result as *const _) {
        *result = *dfl;
    }
    result
}
/// Set DecDouble to zero
pub fn decDoubleZero<'a>(df: &'a mut DecDouble) -> &'a mut DecDouble {
    df.bytes = [0; 8];
    df.dd_set_word(0, 0x22380000);
    df
}
/// Check if DecDouble is finite (not NaN or Infinity)
pub fn decDoubleIsFinite(df: &DecDouble) -> u32 {
    if df.is_special() { 0 } else { 1 }
}
/// Handle NaN propagation
fn decNaNs<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: Option<&DecDouble>,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let source = if let Some(r) = dfr {
        if r.is_snan() && !dfl.is_snan() { r } else { dfl }
    } else {
        dfl
    };
    if source.is_snan() {
        decCanonical(result, source);
        let w0 = result.dd_get_word(0) & !(DECNAN_MASK ^ DECSNAN_MASK);
        result.dd_set_word(0, w0);
        set.status |= DEC_INVALID_OPERATION;
        return result;
    }
    let actual_source = if !dfl.is_nan_internal() { dfr.unwrap_or(dfl) } else { dfl };
    decCanonical(result, actual_source)
}
/// Canonicalize a DecDouble value
fn decCanonical<'a>(result: &'a mut DecDouble, df: &DecDouble) -> &'a mut DecDouble {
    if !std::ptr::eq(result, df) {
        *result = *df;
    }
    let hi = result.get_word_hi();
    if (hi & 0x78000000) == 0x78000000 {
        if (hi & 0x7c000000) == 0x78000000 {
            return decInfinity(result, df);
        }
        let new_hi = hi & !((0x01ffffff >> (32 - 6 - 8)) << (32 - 6 - 8));
        result.set_word_hi(new_hi);
        let lo = df.get_word_lo();
        if lo == 0 && (df.get_word_hi() & 0x0003ffff) == 0 {
            return result;
        }
    }
    let sourhi = df.get_word_hi();
    let sourlo = df.get_word_lo();
    if is_canonical_dpd(sourhi, sourlo) {
        return result;
    }
    canonicalize_declets(result);
    result
}
/// Return invalid operation result
pub fn decInvalid<'a>(
    result: &'a mut DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    set.status |= DEC_INVALID_OPERATION;
    result.dd_set_word(0, 0x7c000000);
    result.dd_set_word(1, 0);
    result
}
/// Return infinity with sign from df
fn decInfinity<'a>(result: &'a mut DecDouble, df: &DecDouble) -> &'a mut DecDouble {
    result.bytes = [0; 8];
    let sign = df.dd_get_word(0) & DECSIGN_MASK;
    result.dd_set_word(0, 0x78000000 | sign);
    result
}
/// Get coefficient as BCD array, returns sign
pub fn decDoubleGetCoefficient(df: &DecDouble, bcdar: &mut [u8]) -> i32 {
    if bcdar.len() < DECPMAX {
        return 0;
    }
    let sourhi = df.dd_get_word(0);
    let sourlo = df.dd_get_word(1);
    bcdar[0] = DECCOMBMSD[(sourhi >> 26) as usize];
    let dpd1 = (sourhi >> 8) & 0x3ff;
    let dpd2 = ((sourhi << 2) | (sourlo >> 30)) & 0x3ff;
    let dpd3 = (sourlo >> 20) & 0x3ff;
    let dpd4 = (sourlo >> 10) & 0x3ff;
    let dpd5 = sourlo & 0x3ff;
    let bcd1 = dpd_to_bcd(dpd1);
    let bcd2 = dpd_to_bcd(dpd2);
    let bcd3 = dpd_to_bcd(dpd3);
    let bcd4 = dpd_to_bcd(dpd4);
    let bcd5 = dpd_to_bcd(dpd5);
    bcdar[1..4].copy_from_slice(&bcd1[1..4]);
    bcdar[4..7].copy_from_slice(&bcd2[1..4]);
    bcdar[7..10].copy_from_slice(&bcd3[1..4]);
    bcdar[10..13].copy_from_slice(&bcd4[1..4]);
    bcdar[13..16].copy_from_slice(&bcd5[1..4]);
    if (sourhi & DECSIGN_MASK) != 0 { DECSIGN_MASK as i32 } else { 0 }
}
/// Finalize a BCD number into DecDouble format
fn decFinalize<'a>(
    df: &'a mut DecDouble,
    num: &mut BcdNum,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let msd_idx = num.msd_idx;
    let lsd_idx = num.lsd_idx;
    if lsd_idx < msd_idx {
        *df = DecDouble::zero();
        return df;
    }
    let mut length = lsd_idx - msd_idx + 1;
    if num.exponent < DECFLOAT_INF {
        let mut umsd = msd_idx;
        while umsd < lsd_idx && num.msd[umsd] == 0 {
            umsd += 1;
        }
        length = lsd_idx - umsd + 1;
        let drop = std::cmp::max(
            (length as i32 - DECDOUBLE_PMAX) as i32,
            (DECDOUBLE_EMIN - num.exponent) as i32,
        );
        if drop > 0 {
            let (new_length, new_exp, new_msd) = perform_rounding(
                num,
                umsd,
                lsd_idx,
                drop as usize,
                set,
            );
            length = new_length;
            num.exponent = new_exp;
            let _ = new_msd;
        }
        if num.exponent > DECDOUBLE_EMAX - (DECDOUBLE_PMAX - 1) {
            handle_overflow(df, num, set);
            return df;
        }
    }
    if length == DECDOUBLE_PMAX as usize {
        return decDoubleFromBCD(df, num.exponent, &num.msd[msd_idx..], num.sign as i32);
    }
    encode_short_coefficient(df, num, msd_idx, lsd_idx, length)
}
/// Finite multiplication helper
fn decFiniteMultiply(
    num: &mut BcdNum,
    bcdacc: &mut [u8],
    dfl: &DecDouble,
    dfr: &DecDouble,
) {
    let mut lhs = [0u8; DECPMAX];
    let mut rhs = [0u8; DECPMAX];
    let lsign = decDoubleGetCoefficient(dfl, &mut lhs);
    let rsign = decDoubleGetCoefficient(dfr, &mut rhs);
    let sourhi_l = dfl.dd_get_word(0);
    let sourhi_r = dfr.dd_get_word(0);
    let exp_l = DECCOMBEXP[(sourhi_l >> 26) as usize]
        + ((sourhi_l & DECEXP_CONT_MASK) >> DECEXP_SHIFT) as i32 - DECBIAS;
    let exp_r = DECCOMBEXP[(sourhi_r >> 26) as usize]
        + ((sourhi_r & DECEXP_CONT_MASK) >> DECEXP_SHIFT) as i32 - DECBIAS;
    let mut result = [0u32; DECPMAX * 2];
    for i in 0..DECPMAX {
        for j in 0..DECPMAX {
            let pos = i + j + 1;
            if pos < result.len() {
                result[pos]
                    += (lhs[DECPMAX - 1 - i] as u32) * (rhs[DECPMAX - 1 - j] as u32);
            }
        }
    }
    for i in (1..result.len()).rev() {
        if result[i] >= 10 {
            result[i - 1] += result[i] / 10;
            result[i] %= 10;
        }
    }
    let acc_len = bcdacc.len().min(result.len());
    for i in 0..acc_len {
        bcdacc[i] = result[i] as u8;
    }
    num.exponent = exp_l + exp_r;
    num.sign = if (lsign != 0) != (rsign != 0) { DECSIGN_MASK } else { 0 };
    num.msd = bcdacc.to_vec();
    num.msd_idx = 0;
    num.lsd_idx = acc_len.saturating_sub(1);
    while num.msd_idx < num.lsd_idx && num.msd[num.msd_idx] == 0 {
        num.msd_idx += 1;
    }
}
/// Compare two DecDoubles with optional total ordering
fn decNumCompare(dfl: &DecDouble, dfr: &DecDouble, tot: u8) -> i32 {
    let mut sigl: i32 = 1;
    if dfl.is_signed_internal() {
        if !dfr.is_signed_internal() {
            if dfl.is_zero_internal() && dfr.is_zero_internal() && tot == 0 {
                return 0;
            }
            return -1;
        }
        sigl = -1;
    }
    if dfr.is_signed_internal() {
        if !dfl.is_signed_internal() {
            if dfl.is_zero_internal() && dfr.is_zero_internal() && tot == 0 {
                return 0;
            }
            return 1;
        }
    }
    let sigr = -sigl;
    if dfl.is_infinity_internal() {
        if dfr.is_infinity_internal() {
            return 0;
        }
        return sigl;
    }
    if dfr.is_infinity_internal() {
        return sigr;
    }
    let shift = dfl.get_biased_exponent() - dfr.get_biased_exponent();
    if dfl.is_zero_internal() {
        if !dfr.is_zero_internal() {
            return sigr;
        }
        if shift == 0 || tot == 0 {
            return 0;
        }
        if shift > 0 {
            return sigl;
        }
        return sigr;
    } else if dfr.is_zero_internal() {
        return sigl;
    }
    if shift.abs() >= DECDOUBLE_PMAX as i32 {
        if shift > 0 {
            return sigl;
        }
        return sigr;
    }
    let coeff_l = extract_coefficient(dfl);
    let coeff_r = extract_coefficient(dfr);
    if shift == 0 {
        for i in 0..16 {
            if coeff_l[i] > coeff_r[i] {
                return sigl;
            }
            if coeff_l[i] < coeff_r[i] {
                return sigr;
            }
        }
    } else if shift > 0 {
        for i in 0..(shift as usize) {
            if i < 16 && coeff_l[i] != 0 {
                return sigl;
            }
        }
        let shift_usize = shift as usize;
        for i in 0..(16 - shift_usize).min(16) {
            let l_idx = shift_usize + i;
            if l_idx < 16 {
                if coeff_l[l_idx] > coeff_r[i] {
                    return sigl;
                }
                if coeff_l[l_idx] < coeff_r[i] {
                    return sigr;
                }
            }
        }
    } else {
        let neg_shift = (-shift) as usize;
        for i in 0..neg_shift {
            if i < 16 && coeff_r[i] != 0 {
                return sigr;
            }
        }
        for i in 0..(16 - neg_shift).min(16) {
            let r_idx = neg_shift + i;
            if r_idx < 16 {
                if coeff_l[i] > coeff_r[r_idx] {
                    return sigl;
                }
                if coeff_l[i] < coeff_r[r_idx] {
                    return sigr;
                }
            }
        }
    }
    if tot == 0 {
        return 0;
    }
    if shift > 0 { sigl } else if shift < 0 { sigr } else { 0 }
}
/// Fused multiply-add: result = dfl * dfr + dff
pub fn decDoubleFMA<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    dff: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let mut acc = vec![0u8; ACC_SIZE];
    let mut mul = BcdNum::new(ACC_SIZE);
    let mut fin = BcdNum::new(DECPMAX);
    let mut coe = vec![0u8; (DECPMAX + 3) & ! 3];
    if dfl.is_special() || dfr.is_special() || dff.is_special() {
        let mut proxy = DecDouble::new();
        if dfl.is_snan() || dfr.is_snan() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        if dff.is_snan() {
            return decNaNs(result, dff, None, set);
        }
        if dfl.is_nan_internal() || dfr.is_nan_internal() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        if dff.is_nan_internal() {
            return decNaNs(result, dff, None, set);
        }
        decDoubleZero(&mut proxy);
        if dfl.is_infinity() {
            if dfr.is_zero_internal() {
                return decInvalid(result, set);
            }
            // Set infinity with sign 0 (positive)
            proxy.bytes = [0; 8];
            proxy.dd_set_word(0, 0x78000000);
        } else if dfr.is_infinity() {
            if dfl.is_zero_internal() {
                return decInvalid(result, set);
            }
            // Set infinity with sign 0 (positive)
            proxy.bytes = [0; 8];
            proxy.dd_set_word(0, 0x78000000);
        }
        let sign = (dfl.dd_get_word(0) ^ dfr.dd_get_word(0)) & DECSIGN_MASK;
        let w0 = proxy.dd_get_word(0) | sign;
        proxy.dd_set_word(0, w0);
        if !dff.is_infinity() {
            return decDoubleCopy(result, &proxy);
        }
        if !proxy.is_infinity() {
            return decInfinity(result, dff);
        }
        if (dff.dd_get_word(0) & DECSIGN_MASK) != (proxy.dd_get_word(0) & DECSIGN_MASK) {
            return decInvalid(result, set);
        }
        return decDoubleCopy(result, &proxy);
    }
    decFiniteMultiply(&mut mul, &mut acc[1..], dfl, dfr);
    let sourhi = dff.dd_get_word(0);
    let sourlo = dff.dd_get_word(1);
    fin.exponent = DECCOMBEXP[(sourhi >> 26) as usize]
        + ((sourhi & DECEXP_CONT_MASK) >> DECEXP_SHIFT) as i32 - DECBIAS;
    fin.sign = sourhi & DECSIGN_MASK;
    let diffsign = mul.sign ^ fin.sign;
    coe[0] = DECCOMBMSD[(sourhi >> 26) as usize];
    let dpd1 = (sourhi >> 8) & 0x3ff;
    let dpd2 = ((sourhi << 2) | (sourlo >> 30)) & 0x3ff;
    let dpd3 = (sourlo >> 20) & 0x3ff;
    let dpd4 = (sourlo >> 10) & 0x3ff;
    let dpd5 = sourlo & 0x3ff;
    let bcd1 = dpd_to_bcd(dpd1);
    let bcd2 = dpd_to_bcd(dpd2);
    let bcd3 = dpd_to_bcd(dpd3);
    let bcd4 = dpd_to_bcd(dpd4);
    let bcd5 = dpd_to_bcd(dpd5);
    coe[1..4].copy_from_slice(&bcd1[1..4]);
    coe[4..7].copy_from_slice(&bcd2[1..4]);
    coe[7..10].copy_from_slice(&bcd3[1..4]);
    coe[10..13].copy_from_slice(&bcd4[1..4]);
    coe[13..16].copy_from_slice(&bcd5[1..4]);
    fin.msd = coe;
    fin.msd_idx = 0;
    fin.lsd_idx = DECPMAX - 1;
    let (hi, lo) = if mul.exponent >= fin.exponent {
        (&mut mul, &mut fin)
    } else {
        (&mut fin, &mut mul)
    };
    while hi.msd_idx < hi.lsd_idx && hi.msd[hi.msd_idx] == 0 {
        hi.msd_idx += 1;
    }
    while lo.msd_idx < lo.lsd_idx && lo.msd[lo.msd_idx] == 0 {
        lo.msd_idx += 1;
    }
    if hi.msd_idx <= hi.lsd_idx && hi.msd[hi.msd_idx] == 0 {
        if diffsign != 0 {
            if lo.msd_idx <= lo.lsd_idx && lo.msd[lo.msd_idx] == 0 {
                lo.sign = 0;
                if set.round == Rounding::Floor {
                    lo.sign = DECSIGN_MASK;
                }
            }
        }
        return decFinalize(result, lo, set);
    }
    let mut result_num = if diffsign == 0 { lo.clone() } else { lo.clone() };
    decFinalize(result, &mut result_num, set)
}
/// Create DecDouble from BCD array
pub fn decDoubleFromBCD<'a>(
    df: &'a mut DecDouble,
    exp: i32,
    bcdar: &[u8],
    sig: i32,
) -> &'a mut DecDouble {
    if bcdar.len() < DECPMAX {
        return df;
    }
    let encode: u32;
    let sign = (sig as u32) & DECSIGN_MASK;
    if exp >= 0x78000000 {
        encode = (exp as u32) | sign;
    } else {
        let uexp = (exp + DECBIAS) as u32;
        let code = ((uexp >> 8) << 4) + (bcdar[0] as u32);
        let code_idx = (code as usize).min(DECCOMBFROM.len() - 1);
        encode = DECCOMBFROM[code_idx] | sign
            | ((uexp << DECEXP_SHIFT) & DECEXP_CONT_MASK);
    }
    let dpd4 = bcd_to_dpd(&bcdar[1..4]);
    let dpd3 = bcd_to_dpd(&bcdar[4..7]);
    let dpd2 = bcd_to_dpd(&bcdar[7..10]);
    let dpd1 = bcd_to_dpd(&bcdar[10..13]);
    let dpd0 = bcd_to_dpd(&bcdar[13..16]);
    let hi = encode | (dpd4 << 8) | (dpd3 >> 2);
    let lo = (dpd3 << 30) | (dpd2 << 20) | (dpd1 << 10) | dpd0;
    df.dd_set_word(0, hi);
    df.dd_set_word(1, lo);
    df
}
/// Quantize dfl to the exponent of dfr
pub fn decDoubleQuantize<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let sourhi_l = dfl.dd_get_word(0);
    let sourhi_r = dfr.dd_get_word(0);
    let exp_l = DECCOMBEXP[(sourhi_l >> 26) as usize];
    let exp_r = DECCOMBEXP[(sourhi_r >> 26) as usize];
    if exp_l >= 0x78000000 || exp_r >= 0x78000000 {
        if dfl.is_nan_internal() || dfr.is_nan_internal() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        if dfl.is_infinity() != dfr.is_infinity() {
            return decInvalid(result, set);
        }
        return decInfinity(result, dfl);
    }
    let full_exp_l = exp_l + ((sourhi_l & DECEXP_CONT_MASK) >> DECEXP_SHIFT) as i32;
    let full_exp_r = exp_r + ((sourhi_r & DECEXP_CONT_MASK) >> DECEXP_SHIFT) as i32;
    let drop = full_exp_r - full_exp_l;
    if drop == 0 {
        return decCanonical(result, dfl);
    }
    let mut buf = vec![0u8; BUF_SIZE];
    let bcd_start = 4 + DECPMAX;
    let sourlo_l = dfl.dd_get_word(1);
    buf[bcd_start] = DECCOMBMSD[(sourhi_l >> 26) as usize];
    let dpd1 = (sourhi_l >> 8) & 0x3ff;
    let dpd2 = ((sourhi_l << 2) | (sourlo_l >> 30)) & 0x3ff;
    let dpd3 = (sourlo_l >> 20) & 0x3ff;
    let dpd4 = (sourlo_l >> 10) & 0x3ff;
    let dpd5 = sourlo_l & 0x3ff;
    let bcd1 = dpd_to_bcd(dpd1);
    let bcd2 = dpd_to_bcd(dpd2);
    let bcd3 = dpd_to_bcd(dpd3);
    let bcd4 = dpd_to_bcd(dpd4);
    let bcd5 = dpd_to_bcd(dpd5);
    buf[bcd_start + 1..bcd_start + 4].copy_from_slice(&bcd1[1..4]);
    buf[bcd_start + 4..bcd_start + 7].copy_from_slice(&bcd2[1..4]);
    buf[bcd_start + 7..bcd_start + 10].copy_from_slice(&bcd3[1..4]);
    buf[bcd_start + 10..bcd_start + 13].copy_from_slice(&bcd4[1..4]);
    buf[bcd_start + 13..bcd_start + 16].copy_from_slice(&bcd5[1..4]);
    let ulsd_idx: usize;
    if drop > 0 {
        let drop_usize = drop as usize;
        if drop_usize < DECPMAX {
            let roundat = bcd_start + DECPMAX - drop_usize;
            let mut reround = buf[roundat];
            for i in (roundat + 1)..(bcd_start + DECPMAX) {
                if buf[i] != 0 {
                    reround = DECSTICKYTAB[reround as usize];
                    break;
                }
            }
            ulsd_idx = roundat - 1;
            if reround != 0 {
                let mut bump = 0u32;
                set.status |= DEC_INEXACT;
                match set.round {
                    Rounding::HalfEven => {
                        if reround > 5 {
                            bump = 1;
                        } else if reround == 5 {
                            bump = (buf[ulsd_idx] & 0x01) as u32;
                        }
                    }
                    Rounding::Down => {}
                    Rounding::HalfDown => {
                        if reround > 5 {
                            bump = 1;
                        }
                    }
                    Rounding::HalfUp => {
                        if reround >= 5 {
                            bump = 1;
                        }
                    }
                    Rounding::Up => {
                        if reround > 0 {
                            bump = 1;
                        }
                    }
                    Rounding::Ceiling => {
                        if (sourhi_l & DECSIGN_MASK) == 0 && reround > 0 {
                            bump = 1;
                        }
                    }
                    Rounding::Floor => {
                        if (sourhi_l & DECSIGN_MASK) != 0 && reround > 0 {
                            bump = 1;
                        }
                    }
                    Rounding::ZeroFiveUp => {
                        if reround > 0 && (buf[ulsd_idx] == 0 || buf[ulsd_idx] == 5) {
                            bump = 1;
                        }
                    }
                    _ => {
                        set.status |= DEC_INVALID_CONTEXT;
                    }
                }
                if bump != 0 {
                    let mut idx = ulsd_idx;
                    while idx > 0 && buf[idx] == 9 {
                        buf[idx] = 0;
                        idx -= 1;
                    }
                    buf[idx] += 1;
                }
            }
        } else {
            ulsd_idx = bcd_start;
        }
    } else {
        let shift = (-drop) as usize;
        if shift > DECPMAX - 1 {
            let mut all_zero = true;
            for i in bcd_start..(bcd_start + DECPMAX) {
                if buf[i] != 0 {
                    all_zero = false;
                    break;
                }
            }
            if !all_zero {
                return decInvalid(result, set);
            }
            ulsd_idx = bcd_start + DECPMAX - 1;
        } else {
            ulsd_idx = bcd_start + DECPMAX + shift - 1;
            for i in (bcd_start + DECPMAX)..=ulsd_idx {
                if i < buf.len() {
                    buf[i] = 0;
                }
            }
        }
    }
    let msd_idx = ulsd_idx + 1 - DECPMAX;
    let code = ((full_exp_r as u32 >> 8) << 4) + (buf[msd_idx] as u32);
    let code_idx = (code as usize).min(DECCOMBFROM.len() - 1);
    let mut encode = DECCOMBFROM[code_idx];
    encode |= sourhi_r & ((DECEXP_CONT_MASK >> DECEXP_SHIFT) << DECEXP_SHIFT);
    encode |= sourhi_l & DECSIGN_MASK;
    let dpd4 = bcd_to_dpd(&buf[ulsd_idx - 14..ulsd_idx - 11]);
    let dpd3 = bcd_to_dpd(&buf[ulsd_idx - 11..ulsd_idx - 8]);
    let dpd2 = bcd_to_dpd(&buf[ulsd_idx - 8..ulsd_idx - 5]);
    let dpd1 = bcd_to_dpd(&buf[ulsd_idx - 5..ulsd_idx - 2]);
    let dpd0 = bcd_to_dpd(&buf[ulsd_idx - 2..=ulsd_idx]);
    let hi = encode | (dpd4 << 8) | (dpd3 >> 2);
    let lo = (dpd3 << 30) | (dpd2 << 20) | (dpd1 << 10) | dpd0;
    result.dd_set_word(0, hi);
    result.dd_set_word(1, lo);
    result
}
/// Logical invert of DecDouble
pub fn decDoubleInvert<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let sourhi = df.dd_get_word(0);
    let sourlo = df.dd_get_word(1);
    if (sourhi & DECLOGICAL_COMB_MASK) != DECLOGICAL_COMB_VALUE
        || (sourhi & !DECLOGICAL_HI_MASK) != 0 || (sourlo & !DECLOGICAL_LO_MASK) != 0
    {
        return decInvalid(result, set);
    }
    result.dd_set_word(0, 0x22380000 | ((!sourhi) & 0x04009124));
    result.dd_set_word(1, (!sourlo) & DECLOGICAL_LO_MASK);
    result
}
/// Multiply two DecDouble values
pub fn decDoubleMultiply<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let mut num = BcdNum::new(ACC_SIZE);
    let mut bcdacc = vec![0u8; ((DECPMAX + 9 - 1) / 9 * 9 / 9) * 18 + 1];
    if dfl.is_special() || dfr.is_special() {
        if dfl.is_nan_internal() || dfr.is_nan_internal() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        if dfl.is_infinity() && dfr.is_zero_internal() {
            return decInvalid(result, set);
        }
        if dfr.is_infinity() && dfl.is_zero_internal() {
            return decInvalid(result, set);
        }
        let sign = (dfl.dd_get_word(0) ^ dfr.dd_get_word(0)) & DECSIGN_MASK;
        // Set infinity in-place preserving sign
        result.bytes = [0; 8];
        result.dd_set_word(0, 0x78000000 | sign);
        return result;
    }
    decFiniteMultiply(&mut num, &mut bcdacc, dfl, dfr);
    decFinalize(result, &mut num, set)
}
/// Set exponent of DecDouble
pub fn decDoubleSetExponent<'a>(
    df: &'a mut DecDouble,
    set: &mut DecContext,
    exp: i32,
) -> &'a mut DecDouble {
    let mut bcdcopy = [0u8; DECPMAX];
    let sign = decDoubleGetCoefficient(df, &mut bcdcopy);
    let mut num = BcdNum::new(DECPMAX);
    num.exponent = exp;
    num.sign = sign as u32;
    if df.is_special() {
        if df.is_infinity() {
            bcdcopy.fill(0);
        }
        bcdcopy[0] = 0;
    }
    num.msd = bcdcopy.to_vec();
    num.msd_idx = 0;
    num.lsd_idx = DECPMAX - 1;
    decFinalize(df, &mut num, set)
}
/// Return next representable value toward -Infinity
pub fn decDoubleNextMinus<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let mut delta = DecDouble::new();
    if dfl.is_infinity() && !dfl.is_negative_internal() {
        result.dd_set_word(0, 0x77fcff3f);
        result.dd_set_word(1, 0xcff3fcff);
        return result;
    }
    decDoubleZero(&mut delta);
    delta.dd_set_word(1, 1);
    delta.dd_set_word(0, delta.dd_get_word(0) | DECSIGN_MASK);
    let saveround = set.round;
    set.round = Rounding::Floor;
    let savestat = set.status;
    decDoubleAdd(result, dfl, &delta, set);
    if result.is_zero_internal() {
        let w0 = result.dd_get_word(0) ^ DECSIGN_MASK;
        result.dd_set_word(0, w0);
    }
    set.status &= DEC_INVALID_OPERATION;
    set.status |= savestat;
    set.round = saveround;
    result
}
/// Return maximum of two DecDouble values
pub fn decDoubleMax<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan_internal() {
        if dfr.is_nan_internal() || dfl.is_snan() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        return decCanonical(result, dfr);
    }
    if dfr.is_nan_internal() {
        if dfr.is_snan() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        return decCanonical(result, dfl);
    }
    let comp = decNumCompare(dfl, dfr, 1);
    if comp >= 0 { decCanonical(result, dfl) } else { decCanonical(result, dfr) }
}
/// Add two decDouble values
pub fn decDoubleAdd<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let mut num = BcdNum::default();
    let sourhil = dfl.hi();
    let sourhir = dfr.hi();
    let summ = DECTESTMSD[(sourhil >> 26) as usize]
        + DECTESTMSD[(sourhir >> 26) as usize];
    let mut bexpl = DECCOMBEXP[(sourhil >> 26) as usize]
        + ((sourhil & 0x03ffffff) >> (32 - 6 - 8)) as i32;
    let mut bexpr = DECCOMBEXP[(sourhir >> 26) as usize]
        + ((sourhir & 0x03ffffff) >> (32 - 6 - 8)) as i32;
    let diffsign = (sourhil ^ sourhir) & 0x80000000;
    if summ <= 8 {
        if summ < 0 {
            if summ < -64 {
                return decNaNs(result, dfl, Some(dfr), set);
            }
            if summ == -64 && diffsign != 0 {
                return decInvalid(result, set);
            }
            if dfl.is_infinity() {
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
            result.bytes[7] &= !0x80;
            if set.round == Rounding::Floor {
                result.bytes[7] |= 0x80;
            }
        }
        return result;
    }
    let acc_size = 4 + 2 + DECPMAX * 3 + 8;
    let mut acc = vec![0u8; acc_size];
    let hi = dfl_use.hi();
    let lo = dfl_use.lo();
    acc[4] = DECCOMBMSD[(hi >> 26) as usize] as u8;
    let declets = [
        ((hi >> 8) & 0x3ff) as usize,
        (((hi << 2) | (lo >> 30)) & 0x3ff) as usize,
        ((lo >> 20) & 0x3ff) as usize,
        ((lo >> 10) & 0x3ff) as usize,
        (lo & 0x3ff) as usize,
    ];
    let mut idx = 5;
    for &declet in &declets {
        let bcd = &DPD2BCD8[declet];
        if idx + 3 <= acc.len() {
            acc[idx] = bcd[1];
            acc[idx + 1] = bcd[2];
            acc[idx + 2] = bcd[3];
            idx += 3;
        }
    }
    num.sign = dfl_use.hi() & 0x80000000;
    num.msd = acc;
    num.msd_idx = 4;
    num.lsd_idx = 4 + DECPMAX - 1;
    num.exponent = bexpr - DECEBIAS;
    decFinalize(result, &mut num, set)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dec_double_zero() {
        let mut df = DecDouble::new();
        decDoubleZero(&mut df);
        assert_eq!(decDoubleIsZero(& df), 1);
    }
    #[test]
    fn test_dec_double_copy() {
        let mut src = DecDouble::new();
        decDoubleZero(&mut src);
        let mut dst = DecDouble::new();
        decDoubleCopy(&mut dst, &src);
        assert_eq!(src.dd_get_word(0), dst.dd_get_word(0));
        assert_eq!(src.dd_get_word(1), dst.dd_get_word(1));
    }
    #[test]
    fn test_dec_double_is_nan() {
        let mut df = DecDouble::new();
        df.dd_set_word(0, 0x7c000000);
        assert_eq!(decDoubleIsNaN(& df), 1);
        decDoubleZero(&mut df);
        assert_eq!(decDoubleIsNaN(& df), 0);
    }
}
const DECDOUBLE_BYTES: usize = 8;
const DECDOUBLE_PMAX: i32 = 16;
const DECDOUBLE_EMAX: i32 = 384;
const DECDOUBLE_EMIN: i32 = -383;
const DECDOUBLE_BIAS: i32 = 398;
const DECDOUBLE_STRING: usize = 25;
const DEC_UNDERFLOW: u32 = 0x00002000;
const DEC_OVERFLOW: u32 = 0x00000200;
const DECFLOAT_INF: i32 = 0x78000000_u32 as i32;
const DECFLOAT_QNAN: i32 = 0x7c000000_u32 as i32;
const DECFLOAT_SNAN: i32 = 0x7e000000_u32 as i32;
const DECFLOAT_SIGN: u32 = 0x80000000;
const DECFLOAT_COMB: u32 = 0x7c000000;
const DECFLOAT_NAN_MASK: u32 = 0x7c000000;
const DECFLOAT_NAN_VALUE: u32 = 0x7c000000;
const DECFLOAT_INF_MASK: u32 = 0x7c000000;
const DECFLOAT_INF_VALUE: u32 = 0x78000000;
const DECFLOAT_SNAN_MASK: u32 = 0x7e000000;
const DECFLOAT_SNAN_VALUE: u32 = 0x7e000000;
const DECFLOAT_LOG_MASK_HI: u32 = 0xfbfc0000;
const DECFLOAT_LOG_VALUE_HI: u32 = 0x22380000;
const DECFLOAT_LOG_CHECK_HI: u32 = 0xfffc9124;
const DECFLOAT_LOG_CHECK_LO: u32 = 0x49124491;
const DECFLOAT_XOR_MASK_HI: u32 = 0x04009124;
const DECFLOAT_XOR_MASK_LO: u32 = 0x49124491;
static BIN2DPD: [u32; 1000] = {
    let mut table = [0u32; 1000];
    let mut i = 0;
    while i < 1000 {
        let d0 = (i % 10) as u32;
        let d1 = ((i / 10) % 10) as u32;
        let d2 = ((i / 100) % 10) as u32;
        let dpd = if d0 <= 7 && d1 <= 7 && d2 <= 7 {
            (d2 << 7) | (d1 << 4) | d0
        } else if d0 <= 7 && d1 <= 7 && d2 >= 8 {
            (d2 & 1) << 7 | (d1 << 4) | d0 | 0x400
        } else if d0 <= 7 && d1 >= 8 && d2 <= 7 {
            (d2 << 7) | ((d1 & 1) << 4) | d0 | 0x200
        } else if d0 >= 8 && d1 <= 7 && d2 <= 7 {
            (d2 << 7) | (d1 << 4) | (d0 & 1) | 0x100
        } else if d0 >= 8 && d1 >= 8 && d2 <= 7 {
            (d2 << 7) | ((d1 & 1) << 4) | (d0 & 1) | 0x300
        } else if d0 >= 8 && d1 <= 7 && d2 >= 8 {
            ((d2 & 1) << 7) | (d1 << 4) | (d0 & 1) | 0x500
        } else if d0 <= 7 && d1 >= 8 && d2 >= 8 {
            ((d2 & 1) << 7) | ((d1 & 1) << 4) | d0 | 0x600
        } else {
            ((d2 & 1) << 7) | ((d1 & 1) << 4) | (d0 & 1) | 0x700
        };
        table[i] = dpd;
        i += 1;
    }
    table
};
static DPD2BIN: [u16; 1024] = {
    let mut table = [0u16; 1024];
    let mut i = 0;
    while i < 1000 {
        let d0 = (i % 10) as u16;
        let d1 = ((i / 10) % 10) as u16;
        let d2 = ((i / 100) % 10) as u16;
        let dpd = if d0 <= 7 && d1 <= 7 && d2 <= 7 {
            (d2 << 7) | (d1 << 4) | d0
        } else {
            i as u16
        };
        if (dpd as usize) < 1024 {
            table[dpd as usize] = i as u16;
        }
        i += 1;
    }
    table
};
/// BCD to DPD lookup table
static BCD2DPD: [u16; 2458] = {
    let mut table = [0u16; 2458];
    let mut i = 0;
    while i < 1000 {
        let d0 = (i / 100) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = (i % 10) as u8;
        table[d0 as usize * 256 + d1 as usize * 16 + d2 as usize] = bcd_to_dpd_const(d0, d1, d2);
        i += 1;
    }
    table
};
static DPD2BCD8: [[u8; 4]; 1024] = {
    let mut table = [[0u8; 4]; 1024];
    let mut dpd = 0usize;
    while dpd < 1024 {
        let val = DPD2BIN[dpd] as usize;
        let d0 = (val % 10) as u8;
        let d1 = ((val / 10) % 10) as u8;
        let d2 = ((val / 100) % 10) as u8;
        table[dpd][0] = d2;
        table[dpd][1] = d1;
        table[dpd][2] = d0;
        if d2 != 0 {
            table[dpd][3] = 3;
        } else if d1 != 0 {
            table[dpd][3] = 2;
        } else if d0 != 0 {
            table[dpd][3] = 1;
        } else {
            table[dpd][3] = 0;
        }
        dpd += 1;
    }
    table
};
static BIN2BCD8: [[u8; 4]; 1000] = {
    let mut table = [[0u8; 4]; 1000];
    let mut i = 0usize;
    while i < 1000 {
        let d0 = (i % 10) as u8;
        let d1 = ((i / 10) % 10) as u8;
        let d2 = ((i / 100) % 10) as u8;
        table[i][0] = d2;
        table[i][1] = d1;
        table[i][2] = d0;
        if d2 != 0 {
            table[i][3] = 3;
        } else if d1 != 0 {
            table[i][3] = 2;
        } else {
            table[i][3] = 1;
        }
        i += 1;
    }
    table
};
static ALL_NINES: [u8; DECDOUBLE_PMAX as usize] = [9; DECDOUBLE_PMAX as usize];
/// Convert DecDouble to integral value with exact flag
pub fn decDoubleToIntegralExact<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    decToIntegral(result, df, set, set.round, true)
}
/// Perform logical XOR operation on two DecDoubles
pub fn decDoubleXor<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if !dfl.is_logical() || !dfr.is_logical() {
        return decInvalid(result, set);
    }
    let dfl_hi = dfl.get_word_hi();
    let dfl_lo = dfl.get_word_lo();
    let dfr_hi = dfr.get_word_hi();
    let dfr_lo = dfr.get_word_lo();
    let result_hi = DECFLOAT_LOG_VALUE_HI | ((dfl_hi ^ dfr_hi) & DECFLOAT_XOR_MASK_HI);
    let result_lo = (dfl_lo ^ dfr_lo) & DECFLOAT_XOR_MASK_LO;
    result.set_word_hi(result_hi);
    result.set_word_lo(result_lo);
    result
}
/// Check if DPD encoding is canonical
fn is_canonical_dpd(sourhi: u32, sourlo: u32) -> bool {
    let check_declet = |bits: u32, shift: u32| -> bool {
        let mask = 0x300u32 << shift;
        let check = 0x6eu32 << shift;
        (bits & mask) == 0 || (bits & check) != check
    };
    check_declet(sourhi, 8) && check_declet(sourhi >> (32 - 30), 0)
        && check_declet(sourlo, 30) && check_declet(sourlo, 20)
        && check_declet(sourlo, 10) && check_declet(sourlo, 0)
}
/// Canonicalize individual declets
fn canonicalize_declets(result: &mut DecDouble) {
    for declet_idx in 0..5 {
        let (word_idx, bit_offset) = get_declet_position(declet_idx);
        let dpd = extract_declet(result, word_idx, bit_offset);
        if dpd >= 0x16e {
            let bin = DPD2BIN[dpd as usize];
            let canon = BIN2DPD[bin as usize];
            if canon as u32 != dpd {
                set_declet(result, word_idx, bit_offset, canon as u32);
            }
        }
    }
}
/// Get declet position (word index and bit offset)
fn get_declet_position(idx: usize) -> (usize, u32) {
    match idx {
        0 => (1, 0),
        1 => (1, 10),
        2 => (1, 20),
        3 => (0, 8),
        4 => (0, 18),
        _ => (0, 0),
    }
}
/// Extract a 10-bit declet from the encoding
fn extract_declet(df: &DecDouble, word_idx: usize, bit_offset: u32) -> u32 {
    let word = if word_idx == 0 { df.get_word_hi() } else { df.get_word_lo() };
    (word >> bit_offset) & 0x3ff
}
/// Set a 10-bit declet in the encoding
fn set_declet(df: &mut DecDouble, word_idx: usize, bit_offset: u32, value: u32) {
    let word = if word_idx == 0 { df.get_word_hi() } else { df.get_word_lo() };
    let mask = !(0x3ffu32 << bit_offset);
    let new_word = (word & mask) | ((value & 0x3ff) << bit_offset);
    if word_idx == 0 {
        df.set_word_hi(new_word);
    } else {
        df.set_word_lo(new_word);
    }
}
/// Find maximum magnitude of two DecDoubles
pub fn decDoubleMaxMag<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan() || dfr.is_nan() {
        return decDoubleMax(result, dfl, dfr, set);
    }
    let mut absl = DecDouble::zero();
    let mut absr = DecDouble::zero();
    decDoubleCopyAbs(&mut absl, dfl);
    decDoubleCopyAbs(&mut absr, dfr);
    let comp = decNumCompare(&absl, &absr, 0);
    if comp > 0 {
        return decCanonical(result, dfl);
    }
    if comp < 0 {
        return decCanonical(result, dfr);
    }
    decDoubleMax(result, dfl, dfr, set)
}
/// Perform rounding on BCD number
fn perform_rounding(
    num: &mut BcdNum,
    umsd: usize,
    ulsd: usize,
    drop: usize,
    set: &mut DecContext,
) -> (usize, i32, usize) {
    let length = ulsd - umsd + 1;
    num.exponent += drop as i32;
    if drop >= length {
        num.msd[umsd] = 0;
        return (1, num.exponent, umsd);
    }
    let new_lsd = ulsd - drop;
    let roundat = new_lsd + 1;
    let mut reround = if roundat <= ulsd { num.msd[roundat] } else { 0 };
    for i in roundat + 1..=ulsd {
        if num.msd[i] != 0 {
            reround = DECSTICKYTAB[reround as usize];
            break;
        }
    }
    if reround != 0 {
        set.status |= DEC_INEXACT;
        if num.exponent < DECDOUBLE_EMIN {
            let adjusted = num.exponent + (new_lsd - umsd) as i32;
            if adjusted < DECDOUBLE_EMIN {
                set.status |= DEC_UNDERFLOW;
            }
        }
        let bump = should_round_up(reround, num.msd[new_lsd], num.sign != 0, set.round);
        if bump {
            let overflow = increment_bcd(&mut num.msd, umsd, new_lsd);
            if overflow {
                num.msd[umsd] = 1;
                if new_lsd - umsd + 1 == DECDOUBLE_PMAX as usize {
                    num.exponent += 1;
                }
            }
        }
    }
    let new_length = new_lsd - umsd + 1;
    (new_length, num.exponent, umsd)
}
/// Determine if rounding should increment
fn should_round_up(reround: u8, lsd: u8, is_negative: bool, round: Rounding) -> bool {
    match round {
        Rounding::HalfEven => {
            if reround > 5 {
                true
            } else if reround == 5 {
                (lsd & 1) != 0
            } else {
                false
            }
        }
        Rounding::Down => false,
        Rounding::HalfDown => reround > 5,
        Rounding::HalfUp => reround >= 5,
        Rounding::Up => reround > 0,
        Rounding::Ceiling => !is_negative && reround > 0,
        Rounding::Floor => is_negative && reround > 0,
        Rounding::ZeroFiveUp => if reround > 0 { lsd == 0 || lsd == 5 } else { false }
        _ => false,
    }
}
/// Increment BCD coefficient, return true if overflow
fn increment_bcd(digits: &mut [u8], msd: usize, lsd: usize) -> bool {
    for i in (msd..=lsd).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return false;
        }
        digits[i] = 0;
    }
    true
}
/// Handle overflow condition
fn handle_overflow(df: &mut DecDouble, num: &mut BcdNum, set: &mut DecContext) {
    set.status |= DEC_OVERFLOW | DEC_INEXACT;
    let need_max = match set.round {
        Rounding::Down | Rounding::ZeroFiveUp => true,
        Rounding::Ceiling => num.sign != 0,
        Rounding::Floor => num.sign == 0,
        _ => false,
    };
    if need_max {
        num.exponent = DECDOUBLE_EMAX - (DECDOUBLE_PMAX - 1);
        for i in 0..DECDOUBLE_PMAX as usize {
            num.msd[i] = 9;
        }
    } else {
        num.exponent = DECFLOAT_INF;
        num.msd[0] = 0;
    }
}
/// Encode short coefficient into DecDouble
fn encode_short_coefficient<'a>(
    df: &'a mut DecDouble,
    num: &BcdNum,
    msd_idx: usize,
    lsd_idx: usize,
    _length: usize,
) -> &'a mut DecDouble {
    let encode = if num.exponent < DECFLOAT_INF {
        let uexp = (num.exponent + DECDOUBLE_BIAS) as u32;
        let code = (uexp >> 8) << 4;
        let mut enc = DECCOMBFROM[code as usize];
        enc |= (uexp << (32 - 6 - 8)) & 0x03ffffff;
        enc | num.sign
    } else {
        (num.exponent as u32) | num.sign
    };
    let mut hi = encode;
    let mut lo = 0u32;
    let digits = &num.msd[msd_idx..=lsd_idx];
    let num_digits = digits.len();
    for (i, chunk) in digits.chunks(3).enumerate() {
        let dpd = bcd_to_dpd(chunk);
        let shift = match i {
            0 => {
                hi |= dpd << 8;
                continue;
            }
            1 => 30,
            2 => 20,
            3 => 10,
            4 => 0,
            _ => continue,
        };
        if i == 1 {
            hi |= dpd >> 2;
            lo |= dpd << 30;
        } else {
            lo |= dpd << shift;
        }
    }
    df.set_word_hi(hi);
    df.set_word_lo(lo);
    let _ = num_digits;
    df
}
/// Check if DecDouble is a signaling NaN (alternate spelling)
pub fn decDoubleIsSignalling(df: &DecDouble) -> u32 {
    if (df.get_word_hi() & 0x7e000000) == 0x7e000000 { 1 } else { 0 }
}
/// Compare two DecDoubles with signal on NaN
pub fn decDoubleCompareSignal<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan() || dfr.is_nan() {
        set.status |= DEC_INVALID_OPERATION;
        return decNaNs(result, dfl, Some(dfr), set);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    decDoubleZero(result);
    if comp == 0 {
        return result;
    }
    result.bytes[DECDOUBLE_BYTES - 1] = 0x01;
    if comp < 0 {
        result.bytes[0] |= 0x80;
    }
    result
}
/// Convert DecDouble to packed decimal format
pub fn decDoubleToPacked(df: &DecDouble, exp: &mut i32, packed: &mut [u8]) -> i32 {
    let mut bcdar = [0u8; DECDOUBLE_PMAX as usize + 2];
    if df.is_infinite() {
        bcdar.fill(0);
        *exp = DECFLOAT_INF;
    } else {
        extract_bcd_digits(df, &mut bcdar[1..]);
        if df.is_nan() {
            bcdar[1] = 0;
            *exp = (df.get_word_hi() & 0x7e000000) as i32;
        } else {
            *exp = get_exponent(df);
        }
    }
    bcdar[0] = 0;
    bcdar[DECDOUBLE_PMAX as usize + 1] = if df.is_signed() { 0x0D } else { 0x0C };
    let mut ip = 0usize;
    for op in 0..((DECDOUBLE_PMAX as usize + 2) / 2) {
        packed[op] = (bcdar[ip] << 4) | bcdar[ip + 1];
        ip += 2;
    }
    if bcdar[DECDOUBLE_PMAX as usize + 1] == 0x0D { 0x80000000u32 as i32 } else { 0 }
}
/// Extract BCD digits from DecDouble
fn extract_bcd_digits(df: &DecDouble, bcdar: &mut [u8]) {
    let sourhi = df.get_word_hi();
    let sourlo = df.get_word_lo();
    bcdar[0] = DECCOMBMSD[(sourhi >> 26) as usize];
    let declets = [
        (sourhi >> 8) & 0x3ff,
        ((sourhi << 2) | (sourlo >> 30)) & 0x3ff,
        (sourlo >> 20) & 0x3ff,
        (sourlo >> 10) & 0x3ff,
        sourlo & 0x3ff,
    ];
    let mut idx = 1usize;
    for declet in declets {
        let bcd = &DPD2BCD8[declet as usize];
        if idx + 3 <= bcdar.len() {
            bcdar[idx] = bcd[0];
            bcdar[idx + 1] = bcd[1];
            bcdar[idx + 2] = bcd[2];
            idx += 3;
        }
    }
}
/// Get the exponent from a DecDouble
fn get_exponent(df: &DecDouble) -> i32 {
    let sourhi = df.get_word_hi();
    let comb = (sourhi >> 26) as usize;
    let exp_cont = ((sourhi & 0x03ffffff) >> (32 - 6 - 8)) as i32;
    DECCOMBEXP[comb] + exp_cont - DECDOUBLE_BIAS
}
/// Convert DecDouble to string representation
pub fn decDoubleToString(df: &DecDouble, string: &mut [u8]) -> usize {
    let mut c = 0usize;
    let sourhi = df.get_word_hi();
    let sourlo = df.get_word_lo();
    if (sourhi as i32) < 0 {
        string[c] = b'-';
        c += 1;
    }
    let comb = (sourhi >> 26) as usize;
    let msd = DECCOMBMSD[comb];
    let mut exp = DECCOMBEXP[comb];
    if exp < DECFLOAT_INF {
        exp += ((sourhi & 0x03ffffff) >> (32 - 6 - 8)) as i32 - DECDOUBLE_BIAS;
    } else {
        if exp == DECFLOAT_INF {
            copy_str(&mut string[c..], b"Infinity");
            return c + 8;
        }
        if sourhi & 0x02000000 != 0 {
            string[c] = b's';
            c += 1;
        }
        copy_str(&mut string[c..], b"NaN");
        c += 3;
        if sourlo == 0 && (sourhi & 0x0003ffff) == 0 {
            return c;
        }
        exp = 0;
    }
    let cstart = c;
    if msd != 0 {
        string[c] = b'0' + msd;
        c += 1;
    }
    let declets = [
        (sourhi >> 8) & 0x3ff,
        ((sourhi << 2) | (sourlo >> 30)) & 0x3ff,
        (sourlo >> 20) & 0x3ff,
        (sourlo >> 10) & 0x3ff,
        sourlo & 0x3ff,
    ];
    for declet in declets {
        let bcd = &DPD2BCD8[declet as usize];
        if c != cstart {
            string[c] = b'0' + bcd[0];
            string[c + 1] = b'0' + bcd[1];
            string[c + 2] = b'0' + bcd[2];
            c += 3;
        } else if bcd[3] != 0 {
            let skip = 3 - bcd[3] as usize;
            for i in skip..3 {
                string[c] = b'0' + bcd[i];
                c += 1;
            }
        }
    }
    if c == cstart {
        string[c] = b'0';
        c += 1;
    }
    let coeff_len = c - cstart;
    let pre = coeff_len as i32 + exp;
    let mut e = 0i32;
    if exp > 0 || pre < -5 {
        e = pre - 1;
    }
    if pre > 0 && (pre as usize) < coeff_len {
        let dotat = cstart + pre as usize;
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
        c += write_decimal(e as u32, &mut string[c..]);
    }
    if pre <= 0 {
        let zeros_needed = (-pre + 1) as usize;
        let total_shift = zeros_needed + 2;
        for i in (cstart..c).rev() {
            string[i + total_shift] = string[i];
        }
        string[cstart] = b'0';
        string[cstart + 1] = b'.';
        for i in 0..zeros_needed {
            string[cstart + 2 + i] = b'0';
        }
        c += total_shift;
    }
    string[c] = 0;
    c
}
/// Write decimal number to buffer, return number of bytes written
fn write_decimal(mut n: u32, buf: &mut [u8]) -> usize {
    if n == 0 {
        buf[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        tmp[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    for j in 0..i {
        buf[j] = tmp[i - 1 - j];
    }
    i
}
/// Copy string bytes
fn copy_str(dest: &mut [u8], src: &[u8]) {
    for (d, s) in dest.iter_mut().zip(src.iter()) {
        *d = *s;
    }
}
/// Get the class (category) of a DecDouble as a string
pub fn decDoubleClassString(df: &DecDouble) -> &'static str {
    let eclass = decDoubleClass(df);
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
/// Get the decimal class of a DecDouble
pub fn decDoubleClass(df: &DecDouble) -> DecClass {
    let hi = df.get_word_hi();
    let is_negative = (hi & DECFLOAT_SIGN) != 0;
    if (hi & 0x7c000000) == 0x7c000000 {
        if (hi & 0x7e000000) == 0x7e000000 {
            return DecClass::Snan;
        }
        return DecClass::Qnan;
    }
    if (hi & 0x7c000000) == 0x78000000 {
        return if is_negative { DecClass::NegInf } else { DecClass::PosInf };
    }
    if df.is_zero() {
        return if is_negative { DecClass::NegZero } else { DecClass::PosZero };
    }
    let exp = get_exponent(df);
    let digits = decDoubleDigits(df) as i32;
    if exp + digits - 1 < DECDOUBLE_EMIN {
        return if is_negative { DecClass::NegSubnormal } else { DecClass::PosSubnormal };
    }
    if is_negative { DecClass::NegNormal } else { DecClass::PosNormal }
}
/// Get the number of significant digits
pub fn decDoubleDigits(df: &DecDouble) -> u32 {
    let sourhi = df.get_hi();
    if (sourhi & 0x7c000000) == DECFLOAT_INF as u32 {
        return 1;
    }
    if (sourhi & DECFLOAT_NAN as u32) != DECFLOAT_NAN as u32 {
        let msd = DECCOMBMSD[(sourhi >> 26) as usize];
        if msd != 0 {
            return DECDOUBLE_PMAX as u32;
        }
    }
    let sourlo = df.get_lo();
    if (sourhi & 0x0003ffff) != 0 {
        let dpd = ((sourhi >> 8) & 0x3ff) as usize;
        if dpd != 0 {
            return (DECDOUBLE_PMAX - 1 - 3 * 0) as u32 - (3 - DPD2BCD8[dpd][3] as u32);
        }
        let dpd = (((sourhi << 2) | (sourlo >> 30)) & 0x3ff) as usize;
        if dpd == 0 {
            return 1;
        }
        return (DECDOUBLE_PMAX - 1 - 3 * 1) as u32 - (3 - DPD2BCD8[dpd][3] as u32);
    }
    if (sourlo & 0xfff00000) != 0 {
        let dpd = ((sourlo >> 30) & 0x3ff) as usize;
        if dpd != 0 {
            return (DECDOUBLE_PMAX - 1 - 3 * 1) as u32 - (3 - DPD2BCD8[dpd][3] as u32);
        }
        let dpd = ((sourlo >> 20) & 0x3ff) as usize;
        if dpd == 0 {
            return 1;
        }
        return (DECDOUBLE_PMAX - 1 - 3 * 2) as u32 - (3 - DPD2BCD8[dpd][3] as u32);
    }
    let dpd = ((sourlo >> 10) & 0x3ff) as usize;
    if dpd != 0 {
        return (DECDOUBLE_PMAX - 1 - 3 * 3) as u32 - (3 - DPD2BCD8[dpd][3] as u32);
    }
    let dpd = (sourlo & 0x3ff) as usize;
    if dpd == 0 {
        return 1;
    }
    (DECDOUBLE_PMAX - 1 - 3 * 4) as u32 - (3 - DPD2BCD8[dpd][3] as u32)
}
/// Convert to integral value
fn decToIntegral<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
    rmode: Rounding,
    exact: bool,
) -> &'a mut DecDouble {
    *result = *df;
    if df.is_nan() || df.is_infinite() {
        if df.is_nan() {
            return decNaNs(result, df, Some(df), set);
        }
        return result;
    }
    let exp = get_exponent(df);
    if exp >= 0 {
        return result;
    }
    let mut num = BcdNum {
        msd: vec![0; DECDOUBLE_PMAX as usize + 4],
        msd_idx: 0,
        lsd_idx: 0,
        sign: if df.is_signed() { DECFLOAT_SIGN } else { 0 },
        exponent: exp,
    };
    extract_bcd_digits(df, &mut num.msd);
    num.lsd_idx = DECDOUBLE_PMAX as usize - 1;
    let drop = (-exp) as usize;
    if drop >= DECDOUBLE_PMAX as usize {
        num.msd[0] = 0;
        num.lsd_idx = 0;
        num.exponent = 0;
    } else {
        let roundat = num.lsd_idx - drop + 1;
        let reround = num.msd[roundat];
        let mut has_sticky = false;
        for i in roundat + 1..=num.lsd_idx {
            if num.msd[i] != 0 {
                has_sticky = true;
                break;
            }
        }
        let sticky_reround = if has_sticky {
            DECSTICKYTAB[reround as usize]
        } else {
            reround
        };
        num.lsd_idx = roundat - 1;
        num.exponent = 0;
        if sticky_reround != 0 {
            if exact {
                set.status |= DEC_INEXACT;
            }
            let bump = should_round_up(
                sticky_reround,
                num.msd[num.lsd_idx],
                num.sign != 0,
                rmode,
            );
            if bump {
                let overflow = increment_bcd(&mut num.msd, 0, num.lsd_idx);
                if overflow {
                    num.msd.insert(0, 1);
                    num.lsd_idx += 1;
                }
            }
        }
    }
    decFinalize(result, &mut num, set)
}
/// Compare magnitudes of two DecDoubles
fn compare_magnitude(dfl: &DecDouble, dfr: &DecDouble) -> i32 {
    let exp_l = get_exponent(dfl);
    let exp_r = get_exponent(dfr);
    let adj_l = exp_l + decDoubleDigits(dfl) as i32 - 1;
    let adj_r = exp_r + decDoubleDigits(dfr) as i32 - 1;
    if adj_l != adj_r {
        return adj_l.cmp(&adj_r) as i32;
    }
    let hi_l = dfl.get_word_hi();
    let lo_l = dfl.get_word_lo();
    let hi_r = dfr.get_word_hi();
    let lo_r = dfr.get_word_lo();
    let coeff_hi_l = hi_l & 0x0003ffff;
    let coeff_hi_r = hi_r & 0x0003ffff;
    if coeff_hi_l != coeff_hi_r {
        return coeff_hi_l.cmp(&coeff_hi_r) as i32;
    }
    lo_l.cmp(&lo_r) as i32
}
/// Copy absolute value
pub fn decDoubleCopyAbs<'a>(result: &'a mut DecDouble, dfl: &DecDouble) -> &'a mut DecDouble {
    *result = *dfl;
    let hi = result.get_word_hi() & !DECFLOAT_SIGN;
    result.set_word_hi(hi);
    result
}
const DECQUAD_BYTES: usize = 16;
const DECQUAD_PMAX: usize = 34;
const DECQUAD_BIAS: i32 = 6176;
const MASK_NAN: u32 = 0x7c000000;
const MASK_SNAN: u32 = 0x7e000000;
const MASK_QNAN: u32 = 0x7c000000;
const MASK_INF: u32 = 0x78000000;
const MASK_SIGN: u32 = 0x80000000;
const MASK_SPECIAL: u32 = 0x78000000;
const MASK_LOGICAL_HI: u32 = 0xfbfc0000;
const MASK_LOGICAL_VAL_HI: u32 = 0x22380000;
const MASK_LOGICAL_ZERO_HI: u32 = 0xfffc9124;
const MASK_LOGICAL_ZERO_LO: u32 = 0x49124491;
/// Combined field to exponent lookup table for wider (quad) format
static DECCOMBWEXP: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 3,
    0, 1, 2, 3, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000,
    0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000, 0x78000000,
    0x78000000, 0x78000000, 0x78000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000,
    0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000, 0x7c000000,
    0x7c000000, 0x7e000000, 0x7e000000, 0x7e000000, 0x7e000000,
];
const fn dpd_to_bcd_digit0(dpd: u16) -> u8 {
    let p = (dpd >> 7) & 0x7;
    let q = (dpd >> 4) & 0x7;
    let r = dpd & 0xf;
    if (r & 0x8) == 0 {
        ((p & 0x4) | ((r >> 1) & 0x2) | (r & 0x1)) as u8
    } else if (q & 0x1) == 0 && (r & 0x6) == 0x6 {
        (8 | (p & 0x1)) as u8
    } else if (q & 0x1) == 1 && (r & 0x6) == 0x6 {
        (8 | ((p >> 2) & 0x1)) as u8
    } else {
        (p & 0x7) as u8
    }
}
const fn dpd_to_bcd_digit1(dpd: u16) -> u8 {
    let q = (dpd >> 4) & 0x7;
    let s = (dpd >> 1) & 0x7;
    let r = dpd & 0xf;
    if (r & 0x8) == 0 {
        ((q & 0x4) | ((r >> 2) & 0x2) | ((s >> 2) & 0x1)) as u8
    } else if (q & 0x1) == 0 && (r & 0x6) == 0x6 {
        (8 | ((q >> 2) & 0x1)) as u8
    } else if (q & 0x1) == 1 && (r & 0x6) == 0x6 {
        (8 | ((s >> 2) & 0x1)) as u8
    } else {
        (q & 0x7) as u8
    }
}
const fn dpd_to_bcd_digit2(dpd: u16) -> u8 {
    let r = (dpd >> 1) & 0x7;
    let s = dpd & 0x1;
    let t = (dpd >> 3) & 0x1;
    let w = (dpd >> 2) & 0x1;
    if (dpd & 0x8) == 0 {
        r as u8
    } else if t == 0 && w == 0 {
        (8 | s) as u8
    } else if t == 0 {
        r as u8
    } else if w == 0 {
        r as u8
    } else {
        (8 | s) as u8
    }
}
fn decToInt32(
    df: &DecDouble,
    set: &mut DecContext,
    round: Rounding,
    exact: u8,
    unsign: u8,
) -> u32 {
    if df.is_nan() {
        set.status |= 0x00000001;
        return 0x80000000;
    }
    if df.is_infinite() {
        set.status |= 0x00000001;
        return if df.is_negative() { 0x80000000 } else { 0x7fffffff };
    }
    let mut bcd = [0u8; 16];
    df.get_coefficient_bcd(&mut bcd);
    let exp = df.get_exponent();
    let neg = df.is_negative();
    let mut value: i64 = 0;
    for &d in &bcd {
        value = value * 10 + d as i64;
    }
    if exp > 0 {
        for _ in 0..exp {
            value *= 10;
            if value > 0x7fffffff_i64 * 2 {
                set.status |= 0x00000001;
                return if neg { 0x80000000 } else { 0x7fffffff };
            }
        }
    } else if exp < 0 {
        let mut divisor: i64 = 1;
        for _ in 0..(-exp) {
            divisor *= 10;
        }
        let remainder = value % divisor;
        value /= divisor;
        if remainder != 0 {
            if exact != 0 {
                set.status |= 0x00000020;
            }
            let half = divisor / 2;
            match round {
                Rounding::Up => {
                    if !neg {
                        value += 1;
                    }
                }
                Rounding::Down => {}
                Rounding::Ceiling => {
                    if !neg {
                        value += 1;
                    }
                }
                Rounding::Floor => {
                    if neg {
                        value += 1;
                    }
                }
                Rounding::HalfUp => {
                    if remainder >= half {
                        value += 1;
                    }
                }
                Rounding::HalfDown => {
                    if remainder > half {
                        value += 1;
                    }
                }
                Rounding::HalfEven => {
                    if remainder > half || (remainder == half && (value & 1) != 0) {
                        value += 1;
                    }
                }
                _ => {}
            }
        }
    }
    if unsign != 0 {
        if neg && value != 0 {
            set.status |= 0x00000001;
            return 0;
        }
        if value > 0xffffffff {
            set.status |= 0x00000001;
            return 0xffffffff;
        }
    } else {
        if neg {
            if value > 0x80000000 {
                set.status |= 0x00000001;
                return 0x80000000;
            }
            return (-(value as i64)) as u32;
        }
        if value > 0x7fffffff {
            set.status |= 0x00000001;
            return 0x7fffffff;
        }
    }
    value as u32
}
/// Reduce a decDouble to its simplest form
pub fn decDoubleReduce<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if result as *const _ != df as *const _ {
        *result = *df;
    }
    if df.is_nan() {
        return decNaNs(result, df, None, set);
    }
    if df.is_infinite() {
        return decInfinity(result, df);
    }
    if df.is_zero() {
        let sign = df.get_sign();
        *result = DecDouble::new();
        result.dd_set_word(0, result.dd_get_word(0) | sign | 0x22380000);
        return result;
    }
    let mut buf = [0u8; 16];
    df.get_coefficient_bcd(&mut buf);
    let mut ub = 15;
    if buf[ub] != 0 {
        return result;
    }
    while ub > 0 && buf[ub - 1] == 0 {
        ub -= 1;
    }
    let mut num = BcdNum::default();
    num.sign = df.get_sign();
    num.exponent = df.get_exponent() + (15 - ub) as i32;
    num.msd = buf.to_vec();
    num.msd_idx = 0;
    num.lsd_idx = ub;
    decFinalize(result, &mut num, set)
}
/// Convert decDouble to 32-bit signed integer
pub fn decDoubleToInt32(df: &DecDouble, set: &mut DecContext, round: Rounding) -> i32 {
    decToInt32(df, set, round, 0, 0) as i32
}
/// Logical AND of two decDoubles
pub fn decDoubleAnd<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let w0_l = dfl.dd_get_word(0);
    let w1_l = dfl.dd_get_word(1);
    let w0_r = dfr.dd_get_word(0);
    let w1_r = dfr.dd_get_word(1);
    let valid_l = (w0_l & MASK_LOGICAL_HI) == MASK_LOGICAL_VAL_HI
        && (w0_l & !MASK_LOGICAL_ZERO_HI) == 0 && (w1_l & !MASK_LOGICAL_ZERO_LO) == 0;
    let valid_r = (w0_r & MASK_LOGICAL_HI) == MASK_LOGICAL_VAL_HI
        && (w0_r & !MASK_LOGICAL_ZERO_HI) == 0 && (w1_r & !MASK_LOGICAL_ZERO_LO) == 0;
    if !valid_l || !valid_r {
        return decInvalid(result, set);
    }
    result.dd_set_word(0, MASK_LOGICAL_VAL_HI | ((w0_l & w0_r) & 0x04009124));
    result.dd_set_word(1, (w1_l & w1_r) & MASK_LOGICAL_ZERO_LO);
    result
}
/// Check if two decDoubles have the same quantum (exponent)
pub fn decDoubleSameQuantum(dfl: &DecDouble, dfr: &DecDouble) -> u32 {
    let w0_l = dfl.dd_get_word(0);
    let w0_r = dfr.dd_get_word(0);
    if (w0_l & MASK_SPECIAL) == MASK_SPECIAL || (w0_r & MASK_SPECIAL) == MASK_SPECIAL {
        if (w0_l & MASK_NAN) == MASK_NAN && (w0_r & MASK_NAN) == MASK_NAN {
            return 1;
        }
        if (w0_l & MASK_NAN) == MASK_INF && (w0_r & MASK_NAN) == MASK_INF {
            return 1;
        }
        return 0;
    }
    if dfl.get_biased_exp() == dfr.get_biased_exp() { 1 } else { 0 }
}
/// Get word from DecQuad at index (big-endian order)
#[inline]
fn decquad_get_word(q: &DecQuad, idx: usize) -> u32 {
    let word_idx = (DECQUAD_BYTES / 4) - 1 - idx;
    let byte_idx = word_idx * 4;
    u32::from_be_bytes([
        q.bytes[byte_idx],
        q.bytes[byte_idx + 1],
        q.bytes[byte_idx + 2],
        q.bytes[byte_idx + 3],
    ])
}
/// Get biased exponent from DecQuad
#[inline]
fn decquad_get_biased_exp(q: &DecQuad) -> i32 {
    let w0 = decquad_get_word(q, 0);
    let comb = (w0 >> 26) as usize;
    DECCOMBWEXP[comb] + ((w0 & 0x03ffffff) >> (32 - 6 - 12)) as i32
}
/// Set word in DecQuad at index
#[inline]
fn decquad_set_word(q: &mut DecQuad, idx: usize, value: u32) {
    let word_idx = (16 / 4) - 1 - idx;
    let byte_idx = word_idx * 4;
    let bytes = value.to_ne_bytes();
    q.bytes[byte_idx..byte_idx + 4].copy_from_slice(&bytes);
}
/// Convert from wider (quad) format
pub fn decDoubleFromWider<'a>(
    result: &'a mut DecDouble,
    wider: &DecQuad,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let mut bcdar = [0u8; 34];
    let widerhi = decquad_get_word(wider, 0);
    let w0 = decquad_get_word(wider, 0);
    let w1 = decquad_get_word(wider, 1);
    let w2 = decquad_get_word(wider, 2);
    let w3 = decquad_get_word(wider, 3);
    bcdar[0] = DECCOMBMSD[(w0 >> 26) as usize];
    let declets = [
        ((w0 >> 4) & 0x3ff) as usize,
        (((w0 << 6) | (w1 >> 26)) & 0x3ff) as usize,
        ((w1 >> 16) & 0x3ff) as usize,
        ((w1 >> 6) & 0x3ff) as usize,
        (((w1 << 4) | (w2 >> 28)) & 0x3ff) as usize,
        ((w2 >> 18) & 0x3ff) as usize,
        ((w2 >> 8) & 0x3ff) as usize,
        (((w2 << 2) | (w3 >> 30)) & 0x3ff) as usize,
        ((w3 >> 20) & 0x3ff) as usize,
        ((w3 >> 10) & 0x3ff) as usize,
        (w3 & 0x3ff) as usize,
    ];
    for (i, &declet) in declets.iter().enumerate() {
        bcdar[1 + i * 3..1 + i * 3 + 3].copy_from_slice(&DPD2BCD8[declet][..3]);
    }
    let mut num = BcdNum::default();
    num.msd = bcdar.to_vec();
    num.msd_idx = 0;
    num.lsd_idx = 33;
    num.sign = widerhi & MASK_SIGN;
    let exp = DECCOMBWEXP[(widerhi >> 26) as usize];
    if exp >= 0x78000000 {
        num.exponent = (widerhi & 0x7e000000) as i32;
    } else {
        num.exponent = exp + decquad_get_biased_exp(wider) - DECQUAD_BIAS;
    }
    decFinalize(result, &mut num, set)
}
/// Check if decDouble is canonical
pub fn decDoubleIsCanonical(df: &DecDouble) -> u32 {
    let w0 = df.dd_get_word(0);
    let w1 = df.dd_get_word(1);
    if (w0 & MASK_SPECIAL) == MASK_SPECIAL {
        if (w0 & MASK_NAN) == MASK_INF {
            if (w0 & ((0x03ffffff >> (32 - 6 - 8)) << (32 - 6 - 8))) != 0 {
                return 0;
            }
            if w1 != 0 || (w0 & 0x0003ffff) != 0 {
                return 0;
            }
            return 1;
        }
        if (w0 & ((0x01ffffff >> (32 - 6 - 8)) << (32 - 6 - 8))) != 0 {
            return 0;
        }
        if w1 == 0 && (w0 & 0x0003ffff) == 0 {
            return 1;
        }
    }
    let check_declet = |val: u32, shift: u32| -> bool {
        let d = (val >> shift) & 0x3ff;
        (d & 0x300) == 0 || (d & 0x6e) != 0x6e
    };
    if !check_declet(w0, 8) {
        return 0;
    }
    let cross = ((w0 << 2) | (w1 >> 30)) & 0x3ff;
    if (cross & 0x300) != 0 && (cross & 0x6e) == 0x6e {
        return 0;
    }
    if !check_declet(w1, 20) || !check_declet(w1, 10) || !check_declet(w1, 0) {
        return 0;
    }
    1
}
/// Return minimum of two decDoubles
pub fn decDoubleMin<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan() {
        if dfr.is_nan() || dfl.is_snan() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        return decCanonical(result, dfr);
    }
    if dfr.is_nan() {
        if dfr.is_snan() {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        return decCanonical(result, dfl);
    }
    let comp = decNumCompare(dfl, dfr, 1);
    if comp <= 0 { decCanonical(result, dfl) } else { decCanonical(result, dfr) }
}
/// Total ordering comparison
pub fn decDoubleCompareTotal<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
) -> &'a mut DecDouble {
    let comp: i32;
    if dfl.is_nan() || dfr.is_nan() {
        let mut nanl = if dfl.is_snan() { 1 } else { 0 }
            + if dfl.is_qnan() { 2 } else { 0 };
        if dfl.is_negative() {
            nanl = -nanl;
        }
        let mut nanr = if dfr.is_snan() { 1 } else { 0 }
            + if dfr.is_qnan() { 2 } else { 0 };
        if dfr.is_negative() {
            nanr = -nanr;
        }
        if nanl > nanr {
            comp = 1;
        } else if nanl < nanr {
            comp = -1;
        } else {
            let mut bufl = [0u8; 16];
            let mut bufr = [0u8; 16];
            dfl.get_coefficient_bcd(&mut bufl);
            dfr.get_coefficient_bcd(&mut bufr);
            let sigl = if dfl.is_negative() { -1 } else { 1 };
            let mut c = 0;
            for i in 0..16 {
                if bufl[i] != bufr[i] {
                    c = if bufl[i] > bufr[i] { sigl } else { -sigl };
                    break;
                }
            }
            comp = c;
        }
    } else {
        comp = decNumCompare(dfl, dfr, 1);
    }
    *result = DecDouble::new();
    result.dd_set_word(0, 0x22380000);
    if comp == 0 {
        return result;
    }
    result.bytes[DECDOUBLE_BYTES - 1] = 0x01;
    if comp < 0 {
        result.bytes[0] |= 0x80;
    }
    result
}
// Note: DPD2BIN, BIN2DPD, DECCOMBMSD, DECCOMBEXP, DECCOMBFROM, and DPD2BCD8
// are already defined as static earlier in this file
const DECEBIAS: i32 = 398;
const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
/// Convert decDouble to signed 32-bit integer with exact rounding
pub fn decDoubleToInt32Exact(
    df: &DecDouble,
    set: &mut DecContext,
    round: Rounding,
) -> i32 {
    decToInt32(df, set, round, 1, 0) as i32
}
/// Create decDouble from packed decimal
pub fn decDoubleFromPacked<'a>(
    df: &'a mut DecDouble,
    exp: i32,
    packed: &[u8],
) -> &'a mut DecDouble {
    let mut bcdar = [0u8; DECPMAX + 2];
    let mut op = 0;
    for i in 0..((DECPMAX + 2) / 2) {
        if i < packed.len() {
            bcdar[op] = packed[i] >> 4;
            op += 1;
            bcdar[op] = packed[i] & 0x0f;
            op += 1;
        }
    }
    let sig = if op > 0 && (bcdar[op - 1] == 0x0D || bcdar[op - 1] == 0x0B) {
        0x80000000u32
    } else {
        0
    };
    if exp >= 0x78000000 {
        if exp != 0x78000000 {
            bcdar[1] = 0;
        } else {
            bcdar[1..DECPMAX + 1].fill(0);
        }
    }
    decDoubleFromBCD(df, exp, &bcdar[1..], sig as i32)
}
/// Set decDouble coefficient from BCD array
pub fn decDoubleSetCoefficient<'a>(
    df: &'a mut DecDouble,
    bcdar: &[u8],
    sig: i32,
) -> &'a mut DecDouble {
    let exp = if df.is_infinity() || df.is_nan() {
        let exp_val = df.hi() & 0x7e000000;
        if df.is_infinity() {
            let bcdzero = [0u8; DECPMAX];
            return decDoubleFromBCD(df, exp_val as i32, &bcdzero, sig);
        }
        exp_val as i32
    } else {
        df.get_exponent()
    };
    decDoubleFromBCD(df, exp, bcdar, sig)
}
/// Check if decDouble is a logical value (0 or 1 digits only)
pub fn decDoubleIsLogical(df: &DecDouble) -> u32 {
    let hi = df.hi();
    let lo = df.lo();
    let exp_ok = (hi & 0xfbfc0000) == 0x22380000;
    let coeff_ok = (hi & !0xfffc9124) == 0 && (lo & !0x49124491) == 0;
    (exp_ok && coeff_ok) as u32
}
/// Get the adjusted exponent (logB)
pub fn decDoubleLogB<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if df.is_nan() {
        return decNaNs(result, df, None, set);
    }
    if df.is_infinity() {
        // Set infinity with sign 0 (positive)
        result.bytes = [0; 8];
        result.dd_set_word(0, 0x78000000);
        return result;
    }
    if df.is_zero() {
        set.status |= DEC_DIVISION_BY_ZERO;
        // Set infinity with negative sign
        result.bytes = [0; 8];
        result.dd_set_word(0, 0x78000000 | 0x80000000);
        return result;
    }
    let ae = df.get_exponent() + decDoubleDigits(df) as i32 - 1;
    result.set_hi(0x22380000);
    if ae < 0 {
        result.bytes[7] |= 0x80;
        let abs_ae = (-ae) as u32;
        result.set_lo(BIN2DPD[abs_ae as usize]);
    } else {
        result.set_lo(BIN2DPD[ae as usize]);
    }
    result
}
/// Shift decDouble coefficient left or right
pub fn decDoubleShift<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan() || dfr.is_nan() {
        return decNaNs(result, dfl, Some(dfr), set);
    }
    let dfr_hi = dfr.hi();
    if !((dfr_hi & 0x63fc0000) == 0x22380000 || (dfr_hi & 0x7bfc0000) == 0x6a380000) {
        return decInvalid(result, set);
    }
    let digits = decDoubleDigits(dfr);
    if digits > 2 {
        return decInvalid(result, set);
    }
    let shift = DPD2BIN[(dfr.lo() & 0x3ff) as usize] as i32;
    if shift > DECPMAX as i32 {
        return decInvalid(result, set);
    }
    if dfl.is_infinity() {
        return decInfinity(result, dfl);
    }
    if shift == 0 {
        return decCanonical(result, dfl);
    }
    if shift == DECPMAX as i32 {
        let sign = dfl.bytes[7] & 0x80;
        decDoubleZero(result);
        result.bytes[7] |= sign;
        return result;
    }
    let mut buf = vec![0u8; DECPMAX * 2];
    let hi = dfl.hi();
    let lo = dfl.lo();
    buf[0] = DECCOMBMSD[(hi >> 26) as usize] as u8;
    let declets = [
        ((hi >> 8) & 0x3ff) as usize,
        (((hi << 2) | (lo >> 30)) & 0x3ff) as usize,
        ((lo >> 20) & 0x3ff) as usize,
        ((lo >> 10) & 0x3ff) as usize,
        (lo & 0x3ff) as usize,
    ];
    let mut idx = 1;
    for &declet in &declets {
        let bcd = &DPD2BCD8[declet];
        if idx + 3 <= buf.len() {
            buf[idx] = bcd[1];
            buf[idx + 1] = bcd[2];
            buf[idx + 2] = bcd[3];
            idx += 3;
        }
    }
    let mut num = BcdNum::default();
    num.sign = dfl.hi() & 0x80000000;
    num.exponent = dfl.get_exponent();
    if dfr.is_negative() {
        num.msd = buf;
        num.msd_idx = 0;
        num.lsd_idx = DECPMAX - shift as usize - 1;
    } else {
        let shift_usize = shift as usize;
        for i in 0..DECPMAX {
            if i + shift_usize < buf.len() {
                num.msd.push(buf[i + shift_usize]);
            } else {
                num.msd.push(0);
            }
        }
        num.msd_idx = 0;
        num.lsd_idx = DECPMAX - 1;
    }
    let save_status = set.status;
    decFinalize(result, &mut num, set);
    set.status = save_status;
    result
}
/// Get absolute value
pub fn decDoubleAbs<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if df.is_nan() {
        return decNaNs(result, df, None, set);
    }
    decCanonical(result, df);
    result.bytes[7] &= !0x80;
    result
}
/// Convert decDouble to wider decQuad
pub fn decDoubleToWider<'a>(source: &DecDouble, wider: &'a mut DecQuad) -> &'a mut DecQuad {
    let source_hi = source.hi();
    if source.is_infinity() || source.is_nan() {
        decquad_set_word(wider, 0, source_hi & 0xfe000000);
        decquad_set_word(wider, 1, 0);
        decquad_set_word(wider, 2, source_hi & 0x0003ffff);
        decquad_set_word(wider, 3, source.lo());
    } else {
        let exp = source.get_exponent() + 6176;
        let code = ((exp >> 12) as u32) << 29;
        let code = code | ((exp as u32) << (32 - 6 - 12)) & 0x03ffffff;
        let code = code | (source_hi & 0x80000000);
        decquad_set_word(wider, 0, code);
        let msd = DECCOMBMSD[(source_hi >> 26) as usize] as u32;
        decquad_set_word(wider, 2, (source_hi & 0x0003ffff) | (msd << 18));
        decquad_set_word(wider, 3, source.lo());
        decquad_set_word(wider, 1, 0);
    }
    wider
}
/// Convert decDouble to unsigned 32-bit integer with exact rounding
pub fn decDoubleToUInt32Exact(
    df: &DecDouble,
    set: &mut DecContext,
    round: Rounding,
) -> u32 {
    decToInt32(df, set, round, 1, 1)
}
// DecDouble is imported from crate::types
const DEC_CONVERSION_SYNTAX: u32 = 0x00000001;
const fn dpd_to_bin(dpd: u16) -> u16 {
    let d0 = ((dpd >> 7) & 0x7) as u16;
    let d1 = ((dpd >> 4) & 0x7) as u16;
    let d2 = (dpd & 0xF) as u16;
    if d2 < 10 { d0 * 100 + d1 * 10 + d2 } else { 0 }
}
/// Check if a string matches either of two strings (case-sensitive comparison)
pub fn decBiStr(targ: &str, str1: &str, str2: &str) -> bool {
    targ == str1 || targ == str2
}
/// Copy sign from one decDouble to another
pub fn decDoubleCopySign<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
) -> &'a mut DecDouble {
    let sign = dfr.get_byte(0) & 0x80;
    if !ptr::eq(dfl as *const _, result as *const _) {
        *result = *dfl;
    }
    let byte0 = result.get_byte(0) & !0x80;
    result.set_byte(0, byte0 | sign);
    result
}
/// Convert decDouble to BCD representation
pub fn decDoubleToBCD(df: &DecDouble, exp: &mut i32, bcdar: &mut [u8; DECPMAX]) -> i32 {
    let word0 = df.dd_get_word(0);
    let word1 = df.dd_get_word(1);
    if (word0 & 0x7c000000) == 0x78000000 {
        bcdar.fill(0);
        *exp = (word0 & 0x7e000000) as i32;
    } else {
        let sourhi = word0;
        let sourlo = word1;
        bcdar[0] = DECCOMBMSD[(sourhi >> 26) as usize];
        let declet1 = ((sourhi >> 8) & 0x3ff) as usize;
        let declet2 = (((sourhi << 2) | (sourlo >> 30)) & 0x3ff) as usize;
        let declet3 = ((sourlo >> 20) & 0x3ff) as usize;
        let declet4 = ((sourlo >> 10) & 0x3ff) as usize;
        let declet5 = (sourlo & 0x3ff) as usize;
        bcdar[1..5].copy_from_slice(&DPD2BCD8[declet1]);
        bcdar[4..8].copy_from_slice(&DPD2BCD8[declet2]);
        bcdar[7..11].copy_from_slice(&DPD2BCD8[declet3]);
        bcdar[10..14].copy_from_slice(&DPD2BCD8[declet4]);
        bcdar[13..16].copy_from_slice(&DPD2BCD8[declet5][0..3]);
        if (word0 & 0x7c000000) == 0x7c000000 {
            bcdar[0] = 0;
            *exp = (word0 & 0x7e000000) as i32;
        } else {
            let comb_exp = DECCOMBEXP[(word0 >> 26) as usize];
            let exp_cont = (word0 & 0x03ffffff) >> (32 - 6 - 8);
            *exp = (comb_exp as i32 + exp_cont as i32) - DECBIAS;
        }
    }
    (word0 & 0x80000000) as i32
}
/// Remainder near operation (IEEE remainder)
pub fn decDoubleRemainderNear<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    decDivide(result, dfl, dfr, set, 0x10000000)
}
/// Next representable number in positive direction
pub fn decDoubleNextPlus<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let word0 = dfl.dd_get_word(0);
    if (word0 & 0x7c000000) == 0x78000000 && (word0 & 0x80000000) != 0 {
        result.dd_set_word(0, 0x77fcff3f | 0x80000000);
        result.dd_set_word(1, 0xcff3fcff);
        return result;
    }
    let mut delta = DecDouble::new();
    delta.dd_set_word(1, 1);
    delta.dd_set_word(0, 0);
    let saveround = set.round;
    let savestat = set.status;
    set.round = Rounding::Ceiling;
    decDoubleAdd(result, dfl, &delta, set);
    let rword0 = result.dd_get_word(0);
    let rword1 = result.dd_get_word(1);
    if rword1 == 0 && (rword0 & 0x1c03ffff) == 0 && (rword0 & 0x60000000) != 0x60000000 {
        result.dd_set_word(0, rword0 ^ 0x80000000);
    }
    set.status &= DEC_INVALID_OPERATION;
    set.status |= savestat;
    set.round = saveround;
    result
}
/// Check if decDouble is NaN
pub fn decDoubleIsNaN(df: &DecDouble) -> u32 {
    if (df.dd_get_word(0) & 0x7c000000) == 0x7c000000 { 1 } else { 0 }
}
/// Scale B operation (multiply by power of 10)
pub fn decDoubleScaleB<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let word0_l = dfl.dd_get_word(0);
    let word0_r = dfr.dd_get_word(0);
    if (word0_l & 0x7c000000) == 0x7c000000 || (word0_r & 0x7c000000) == 0x7c000000 {
        return decNaNs(result, dfl, Some(dfr), set);
    }
    if !((word0_r & 0x63fc0000) == 0x22380000 || (word0_r & 0x7bfc0000) == 0x6a380000) {
        return decInvalid(result, set);
    }
    let digits = decDoubleDigits(dfr);
    if digits > 3 {
        return decInvalid(result, set);
    }
    let mut expr = DPD2BIN[(dfr.dd_get_word(1) & 0x3ff) as usize] as i32;
    if expr > 2 * (DECEMAX + DECPMAX as i32) {
        return decInvalid(result, set);
    }
    if (word0_l & 0x7c000000) == 0x78000000 {
        return decInfinity(result, dfl);
    }
    if (word0_r & 0x80000000) != 0 {
        expr = -expr;
    }
    *result = *dfl;
    let comb_exp = DECCOMBEXP[(result.dd_get_word(0) >> 26) as usize];
    let exp_cont = (result.dd_get_word(0) & 0x03ffffff) >> (32 - 6 - 8);
    let current_exp = (comb_exp as i32 + exp_cont as i32) - DECBIAS;
    decDoubleSetExponent(result, set, current_exp + expr)
}
/// Minimum magnitude operation
pub fn decDoubleMinMag<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let word0_l = dfl.dd_get_word(0);
    let word0_r = dfr.dd_get_word(0);
    if (word0_l & 0x7c000000) == 0x7c000000 || (word0_r & 0x7c000000) == 0x7c000000 {
        return decDoubleMin(result, dfl, dfr, set);
    }
    let mut absl = DecDouble::new();
    let mut absr = DecDouble::new();
    decDoubleCopyAbs(&mut absl, dfl);
    decDoubleCopyAbs(&mut absr, dfr);
    let comp = decNumCompare(&absl, &absr, 0);
    if comp < 0 {
        return decCanonical(result, dfl);
    }
    if comp > 0 {
        return decCanonical(result, dfr);
    }
    decDoubleMin(result, dfl, dfr, set)
}
/// Check if decDouble is an integer
pub fn decDoubleIsInteger(df: &DecDouble) -> u32 {
    let word0 = df.dd_get_word(0);
    if (word0 & 0x63fc0000) == 0x22380000 || (word0 & 0x7bfc0000) == 0x6a380000 {
        1
    } else {
        0
    }
}
/// Parse a string to create a decDouble
pub fn decDoubleFromString<'a>(
    result: &'a mut DecDouble,
    string: &str,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    let mut num = BcdNum::default();
    let mut error = DEC_CONVERSION_SYNTAX;
    let mut buffer = vec![0u8; ((DECPMAX + 11 + 7) / 8) * 8];
    let bytes = string.as_bytes();
    let mut idx = 0;
    let mut dotchar: Option<usize> = None;
    let mut cfirst = 0;
    num.sign = 0;
    num.msd_idx = 0;
    if idx < bytes.len() {
        if bytes[idx] == b'-' {
            cfirst = 1;
            idx = 1;
            num.sign = DECSIGN_MASK;
        } else if bytes[idx] == b'+' {
            cfirst = 1;
            idx = 1;
        }
    }
    let mut scan_idx = idx;
    while scan_idx < bytes.len() {
        let c = bytes[scan_idx];
        if c.is_ascii_digit() {
            scan_idx += 1;
            continue;
        }
        if c == b'.' {
            if dotchar.is_some() {
                break;
            }
            dotchar = Some(scan_idx);
            scan_idx += 1;
            continue;
        }
        break;
    }
    let mut digits = scan_idx - cfirst;
    if dotchar.is_some() {
        digits -= 1;
    }
    if digits > 0 {
        let clast = scan_idx - 1;
        let mut exp = 0i32;
        if scan_idx < bytes.len() {
            let c = bytes[scan_idx];
            if c == b'E' || c == b'e' {
                scan_idx += 1;
                let mut neg_exp = false;
                if scan_idx < bytes.len() {
                    if bytes[scan_idx] == b'-' {
                        neg_exp = true;
                        scan_idx += 1;
                    } else if bytes[scan_idx] == b'+' {
                        scan_idx += 1;
                    }
                }
                while scan_idx < bytes.len() && bytes[scan_idx] == b'0' {
                    scan_idx += 1;
                }
                let firstexp = scan_idx;
                while scan_idx < bytes.len() {
                    let c = bytes[scan_idx];
                    if !c.is_ascii_digit() {
                        break;
                    }
                    let edig = (c - b'0') as i32;
                    exp = exp * 10 + edig;
                    scan_idx += 1;
                }
                if scan_idx > firstexp + 3 {
                    exp = DECEMAX * 2;
                }
                if neg_exp {
                    exp = -exp;
                }
            }
        }
        if let Some(dot_idx) = dotchar {
            exp -= (clast - dot_idx) as i32;
        }
        num.exponent = exp;
        error = 0;
        let mut ub_idx = 0;
        let mut c_idx = cfirst;
        while c_idx <= clast && ub_idx < buffer.len() {
            let c = bytes[c_idx];
            if c == b'.' {
                c_idx += 1;
                continue;
            }
            if c.is_ascii_digit() {
                buffer[ub_idx] = c - b'0';
                ub_idx += 1;
            }
            c_idx += 1;
        }
        num.lsd_idx = if ub_idx > 0 { ub_idx - 1 } else { 0 };
        num.msd = buffer.clone();
    } else if scan_idx < bytes.len() {
        buffer[0] = 0;
        num.lsd_idx = 0;
        let remaining = &string[scan_idx..];
        if remaining.eq_ignore_ascii_case("infinity")
            || remaining.eq_ignore_ascii_case("inf")
        {
            num.exponent = DECFLOAT_INF;
            error = 0;
        } else if remaining.starts_with('s') || remaining.starts_with('S') {
            let rest = &remaining[1..];
            if rest.len() >= 3 && rest[..3].eq_ignore_ascii_case("nan") {
                num.exponent = DECFLOAT_SNAN;
                error = 0;
            }
        } else if remaining.len() >= 3 && remaining[..3].eq_ignore_ascii_case("nan") {
            num.exponent = DECFLOAT_QNAN;
            error = 0;
            let payload = &remaining[3..];
            if !payload.is_empty() {
                let mut ub_idx = 0;
                for c in payload.bytes() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    if ub_idx >= DECPMAX - 1 {
                        break;
                    }
                    buffer[ub_idx] = c - b'0';
                    ub_idx += 1;
                }
                if ub_idx > 0 {
                    num.lsd_idx = ub_idx - 1;
                }
            }
        }
        num.msd = buffer.clone();
    }
    if error != 0 {
        set.status |= error;
        num.exponent = DECFLOAT_QNAN;
        num.sign = 0;
        buffer[0] = 0;
        num.lsd_idx = 0;
        num.msd = buffer;
    }
    decFinalize(result, &mut num, set);
    result
}
/// Internal division operation
fn decDivide<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
    op: u32,
) -> &'a mut DecDouble {
    let mut num = BcdNum::default();
    num.sign = (dfl.hi() ^ dfr.hi()) & DECFLOAT_SIGN;
    if is_special(dfl) || is_special(dfr) {
        if is_nan(dfl) || is_nan(dfr) {
            return decNaNs(result, dfl, Some(dfr), set);
        }
        if is_infinity(dfl) {
            if is_infinity(dfr) {
                return decInvalid(result, set);
            }
            if (op & (DECFLOAT_OP_REMAINDER | DECFLOAT_OP_DIVIDEINT)) != 0 {
                return decInvalid(result, set);
            }
            // Set infinity preserving sign
            let sign = num.sign & DECSIGN_MASK;
            result.bytes = [0; 8];
            result.dd_set_word(0, 0x78000000 | sign);
            return result;
        }
        if (op & (DECFLOAT_OP_REMAINDER | DECFLOAT_OP_DIVIDEINT)) != 0 {
            return decCanonical(result, dfl);
        }
        decDoubleZero(result);
        if op == DECFLOAT_OP_REMNEAR {
            let hi = result.hi() | num.sign;
            result.set_hi(hi);
        } else {
            result.set_hi(num.sign);
        }
        return result;
    }
    if is_zero(dfr) {
        if is_zero(dfl) {
            decDoubleZero(result);
            result.set_hi(DECFLOAT_QNAN as u32);
            set.status |= DEC_DIVISION_UNDEFINED;
            return result;
        }
        if (op & (DECFLOAT_OP_REMAINDER | DECFLOAT_OP_DIVIDEINT)) != 0 {
            return decInvalid(result, set);
        }
        set.status |= DEC_DIVISION_BY_ZERO;
        // Set infinity preserving sign
        let sign = num.sign & DECSIGN_MASK;
        result.bytes = [0; 8];
        result.dd_set_word(0, 0x78000000 | sign);
        return result;
    }
    let lexp = get_exponent(dfl);
    let rexp = get_exponent(dfr);
    num.exponent = lexp - rexp;
    if is_zero(dfl) {
        if op == DECFLOAT_OP_REMNEAR {
            decDoubleZero(result);
            let hi = result.hi() | num.sign;
            result.set_hi(hi);
            return result;
        }
        if (op & DECFLOAT_OP_DIVIDE) == 0 {
            num.exponent = lexp.max(rexp);
            num.sign = dfl.hi() & DECFLOAT_SIGN;
        }
        num.msd[0] = 0;
        num.msd_idx = 0;
        num.lsd_idx = 0;
        return decFinalize(result, &mut num, set);
    }
    decFinalize(result, &mut num, set)
}
/// Check if decDouble is zero
pub fn decDoubleIsZero(df: &DecDouble) -> u32 {
    let word0 = df.dd_get_word(0);
    let word1 = df.dd_get_word(1);
    if (word0 & 0x7c000000) >= 0x78000000 {
        return 0;
    }
    if DECCOMBMSD[(word0 >> 26) as usize] != 0 {
        return 0;
    }
    if ((word0 >> 8) & 0x3ff) != 0 {
        return 0;
    }
    if (((word0 << 2) | (word1 >> 30)) & 0x3ff) != 0 {
        return 0;
    }
    if ((word1 >> 20) & 0x3ff) != 0 {
        return 0;
    }
    if ((word1 >> 10) & 0x3ff) != 0 {
        return 0;
    }
    if (word1 & 0x3ff) != 0 {
        return 0;
    }
    1
}
const SIGN_MASK: u32 = 0x80000000;
const INF_MASK: u32 = 0x78000000;
const NAN_MASK: u32 = 0x7c000000;
const SNAN_MASK: u32 = 0x7e000000;
const SPECIAL_MASK: u32 = 0x78000000;
const COMB_MASK: u32 = 0xfbfc0000;
const COMB_ZERO_EXP: u32 = 0x22380000;
const LOGICAL_MASK_HI: u32 = 0xfffc9124;
const LOGICAL_MASK_LO: u32 = 0x49124491;
const LOGICAL_RESULT_MASK_HI: u32 = 0x04009124;
/// Helper to get high word (big-endian layout in little-endian storage)
#[inline]
fn get_hi_word(df: &DecDouble) -> u32 {
    u32::from_le_bytes([df.bytes[4], df.bytes[5], df.bytes[6], df.bytes[7]])
}
/// Helper to get low word
#[inline]
fn get_lo_word(df: &DecDouble) -> u32 {
    u32::from_le_bytes([df.bytes[0], df.bytes[1], df.bytes[2], df.bytes[3]])
}
/// Helper to set high word
#[inline]
fn set_hi_word(df: &mut DecDouble, val: u32) {
    let bytes = val.to_le_bytes();
    df.bytes[4] = bytes[0];
    df.bytes[5] = bytes[1];
    df.bytes[6] = bytes[2];
    df.bytes[7] = bytes[3];
}
/// Helper to set low word
#[inline]
fn set_lo_word(df: &mut DecDouble, val: u32) {
    let bytes = val.to_le_bytes();
    df.bytes[0] = bytes[0];
    df.bytes[1] = bytes[1];
    df.bytes[2] = bytes[2];
    df.bytes[3] = bytes[3];
}
/// Check if value is a special (NaN or Infinity)
#[inline]
fn is_special(df: &DecDouble) -> bool {
    (get_hi_word(df) & SPECIAL_MASK) == SPECIAL_MASK
}
/// Helper to check if a DecDouble is NaN
#[inline]
fn is_nan(df: &DecDouble) -> bool {
    (df.hi() & 0x7c000000) == 0x7c000000
}
/// Check if value is zero
#[inline]
fn is_zero(df: &DecDouble) -> bool {
    let hi = get_hi_word(df);
    let lo = get_lo_word(df);
    lo == 0 && (hi & 0x1c03ffff) == 0 && (hi & 0x60000000) != 0x60000000
}
/// Check if value is logical (all digits 0 or 1)
#[inline]
fn is_logical(df: &DecDouble) -> bool {
    let hi = get_hi_word(df);
    let lo = get_lo_word(df);
    (hi & COMB_MASK) == COMB_ZERO_EXP && (hi & !LOGICAL_MASK_HI) == 0
        && (lo & !LOGICAL_MASK_LO) == 0
}
/// Compute remainder of division
pub fn decDoubleRemainder<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    decDivide(result, dfl, dfr, set, 0x40000000)
}
/// Check if value is signed (negative)
pub fn decDoubleIsSigned(df: &DecDouble) -> u32 {
    if (get_hi_word(df) & SIGN_MASK) != 0 { 1 } else { 0 }
}
/// Logical OR of two decimal values
pub fn decDoubleOr<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if !is_logical(dfl) || !is_logical(dfr) {
        return decInvalid(result, set);
    }
    let dfl_hi = get_hi_word(dfl);
    let dfl_lo = get_lo_word(dfl);
    let dfr_hi = get_hi_word(dfr);
    let dfr_lo = get_lo_word(dfr);
    set_hi_word(result, COMB_ZERO_EXP | ((dfl_hi | dfr_hi) & LOGICAL_RESULT_MASK_HI));
    set_lo_word(result, (dfl_lo | dfr_lo) & LOGICAL_MASK_LO);
    result
}
/// Check if value is a normal number
pub fn decDoubleIsNormal(df: &DecDouble) -> u32 {
    let hi = get_hi_word(df);
    if (hi & SPECIAL_MASK) == SPECIAL_MASK {
        return 0;
    }
    if is_zero(df) {
        return 0;
    }
    let comb_idx = (hi >> 26) as usize;
    let exp_cont = ((hi & 0x03ffffff) >> (32 - 6 - 8)) as i32;
    let exp = DECCOMBEXP[comb_idx] + exp_cont - DECDOUBLE_BIAS
        + decDoubleDigits(df) as i32 - 1;
    if exp >= DECDOUBLE_EMIN { 1 } else { 0 }
}
/// Subtract two decimal values
pub fn decDoubleSubtract<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if is_nan(dfr) {
        return decDoubleAdd(result, dfl, dfr, set);
    }
    let mut temp = *dfr;
    temp.bytes[7] ^= 0x80;
    decDoubleAdd(result, dfl, &temp, set)
}
/// Convert unsigned 32-bit integer to decimal
pub fn decDoubleFromUInt32<'a>(result: &'a mut DecDouble, u: u32) -> &'a mut DecDouble {
    set_hi_word(result, COMB_ZERO_EXP);
    let mut val = u;
    let mut encode: u32 = 0;
    encode = BIN2DPD[(val % 1000) as usize];
    val /= 1000;
    encode |= BIN2DPD[(val % 1000) as usize] << 10;
    val /= 1000;
    encode |= BIN2DPD[(val % 1000) as usize] << 20;
    val /= 1000;
    encode |= val << 30;
    set_lo_word(result, encode);
    let hi = get_hi_word(result) | (val >> 2);
    set_hi_word(result, hi);
    result
}
/// Return library version string
pub fn decDoubleVersion() -> &'static str {
    "decNumber 3.68"
}
/// Copy with negated sign
pub fn decDoubleCopyNegate<'a>(result: &'a mut DecDouble, dfl: &DecDouble) -> &'a mut DecDouble {
    if !std::ptr::eq(dfl, result) {
        *result = *dfl;
    }
    result.bytes[7] ^= 0x80;
    result
}
/// Return negation of decimal value
pub fn decDoubleMinus<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if is_nan(df) {
        return decNaNs(result, df, None, set);
    }
    decCanonical(result, df);
    if is_zero(df) {
        result.bytes[7] &= !0x80;
    } else {
        result.bytes[7] ^= 0x80;
    }
    result
}
const DECFLOAT_NAN: u32 = 0x7c000000;
/// Canonicalize a decDouble value
pub fn decDoubleCanonical<'a>(result: &'a mut DecDouble, df: &DecDouble) -> &'a mut DecDouble {
    decCanonical(result, df)
}
/// Convert decDouble to engineering string representation
pub fn decDoubleToEngString<'a>(df: &DecDouble, string: &'a mut String) -> &'a mut String {
    string.clear();
    let sourhi = df.get_hi();
    let sourlo = df.get_lo();
    if (sourhi as i32) < 0 {
        string.push('-');
    }
    let comb = sourhi >> 26;
    let mut msd = DECCOMBMSD[comb as usize];
    let mut exp = DECCOMBEXP[comb as usize] as i32;
    if exp >= 0x78000000_u32 as i32 / (1 << 24) || (sourhi & 0x78000000) >= 0x78000000 {
        if (sourhi & 0x7c000000) == 0x78000000 {
            string.push_str("Infinity");
            return string;
        }
        if (sourhi & 0x02000000) != 0 {
            string.push('s');
        }
        string.push_str("NaN");
        if sourlo == 0 && (sourhi & 0x0003ffff) == 0 {
            return string;
        }
        exp = 0;
        msd = 0;
    } else {
        let biased_exp = ((sourhi & 0x03ffffff) >> (32 - 6 - 8)) as i32;
        exp = (exp << 8) + biased_exp - DECDOUBLE_BIAS;
    }
    let cstart = string.len();
    if msd != 0 {
        string.push((b'0' + msd) as char);
    }
    let dpd1 = ((sourhi >> 8) & 0x3ff) as usize;
    let dpd2 = (((sourhi << 2) | (sourlo >> 30)) & 0x3ff) as usize;
    let dpd3 = ((sourlo >> 20) & 0x3ff) as usize;
    let dpd4 = ((sourlo >> 10) & 0x3ff) as usize;
    let dpd5 = (sourlo & 0x3ff) as usize;
    for &dpd in &[dpd1, dpd2, dpd3, dpd4, dpd5] {
        let bcd = &DPD2BCD8[dpd];
        if string.len() != cstart {
            string.push((b'0' + bcd[0]) as char);
            string.push((b'0' + bcd[1]) as char);
            string.push((b'0' + bcd[2]) as char);
        } else if bcd[3] > 0 {
            let start = 3 - bcd[3] as usize;
            for i in start..3 {
                string.push((b'0' + bcd[i]) as char);
            }
        }
    }
    if string.len() == cstart || (string.len() == cstart + 1 && string.ends_with('-')) {
        string.push('0');
    }
    if exp == 0 {
        return string;
    }
    let coeff_len = string.len() - cstart - if string.starts_with('-') { 1 } else { 0 };
    let pre = coeff_len as i32 + exp;
    let mut e = 0i32;
    if exp > 0 || pre < -5 {
        e = pre - 1;
        let mut adj = 0i32;
        if e != 0 {
            if e < 0 {
                adj = (-e) % 3;
                if adj != 0 {
                    adj = 3 - adj;
                }
            } else {
                adj = e % 3;
            }
            e -= adj;
        }
    }
    if e != 0 {
        string.push('E');
        if e < 0 {
            string.push('-');
            e = -e;
        } else {
            string.push('+');
        }
        if e < 1000 {
            let bcd = &BIN2BCD8[e as usize];
            let start = 3 - bcd[3] as usize;
            for i in start..3 {
                string.push((b'0' + bcd[i]) as char);
            }
        }
    }
    string
}
/// Convert decDouble to unsigned 32-bit integer
pub fn decDoubleToUInt32(df: &DecDouble, set: &mut DecContext, round: Rounding) -> u32 {
    decToInt32(df, set, round, 0, 1)
}
/// Rotate digits of a decDouble
pub fn decDoubleRotate<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan_internal() || dfr.is_nan_internal() {
        return decNaNs(result, dfl, Some(dfr), set);
    }
    let dfr_hi = dfr.get_hi();
    if !((dfr_hi & 0x63fc0000) == 0x22380000 || (dfr_hi & 0x7bfc0000) == 0x6a380000) {
        return decInvalid(result, set);
    }
    let digits = decDoubleDigits(dfr);
    if digits > 2 {
        return decInvalid(result, set);
    }
    let mut rotate = DPD2BIN[(dfr.get_lo() & 0x3ff) as usize] as i32;
    if rotate > DECDOUBLE_PMAX as i32 {
        return decInvalid(result, set);
    }
    if dfl.is_inf_internal() {
        return decInfinity(result, dfl);
    }
    if rotate == 0 || rotate == DECDOUBLE_PMAX as i32 {
        return decCanonical(result, dfl);
    }
    if dfr.is_negative_internal() {
        rotate = -rotate;
    }
    if rotate.abs() > DECDOUBLE_PMAX as i32 / 2 {
        if rotate < 0 {
            rotate = DECDOUBLE_PMAX as i32 + rotate;
        } else {
            rotate -= DECDOUBLE_PMAX as i32;
        }
    }
    let mut buf = [0u8; 32];
    extract_bcd(dfl, &mut buf[..DECDOUBLE_PMAX as usize]);
    let mut rotated = [0u8; 16];
    for i in 0..DECDOUBLE_PMAX as usize {
        let src_idx = if rotate >= 0 {
            (i + rotate as usize) % DECDOUBLE_PMAX as usize
        } else {
            (i + DECDOUBLE_PMAX as usize - (-rotate) as usize) % DECDOUBLE_PMAX as usize
        };
        rotated[i] = buf[src_idx];
    }
    let mut num = BcdNum {
        msd: rotated.to_vec(),
        msd_idx: 0,
        lsd_idx: DECDOUBLE_PMAX as usize - 1,
        sign: dfl.get_hi() & DECFLOAT_SIGN,
        exponent: get_exponent(dfl),
    };
    let savestat = set.status;
    decFinalize(result, &mut num, set);
    set.status = savestat;
    result
}
/// Integer division of two decDoubles
pub fn decDoubleDivideInteger<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    decDivide(result, dfl, dfr, set, 0x20000000)
}
/// Add zero to a decDouble (canonicalize positive zero)
pub fn decDoublePlus<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if df.is_nan_internal() {
        return decNaNs(result, df, None, set);
    }
    decCanonical(result, df);
    if df.is_zero_internal() {
        result.bytes[DECDOUBLE_BYTES - 1] &= !0x80;
    }
    result
}
/// Compare total magnitude of two decDoubles
pub fn decDoubleCompareTotalMag<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
) -> &'a mut DecDouble {
    let mut a = *dfl;
    let mut b = *dfr;
    if dfl.is_negative_internal() {
        decDoubleCopyAbs(&mut a, dfl);
    }
    if dfr.is_negative_internal() {
        decDoubleCopyAbs(&mut b, dfr);
    }
    decDoubleCompareTotal(result, &a, &b)
}
/// Check if decDouble is negative (and not zero or NaN)
pub fn decDoubleIsNegative(df: &DecDouble) -> u32 {
    if df.is_negative_internal() && !df.is_zero_internal() && !df.is_nan_internal() {
        1
    } else {
        0
    }
}
fn extract_bcd(df: &DecDouble, bcd: &mut [u8]) {
    let sourhi = df.get_hi();
    let sourlo = df.get_lo();
    bcd[0] = DECCOMBMSD[(sourhi >> 26) as usize];
    let dpd1 = ((sourhi >> 8) & 0x3ff) as usize;
    let dpd2 = (((sourhi << 2) | (sourlo >> 30)) & 0x3ff) as usize;
    let dpd3 = ((sourlo >> 20) & 0x3ff) as usize;
    let dpd4 = ((sourlo >> 10) & 0x3ff) as usize;
    let dpd5 = (sourlo & 0x3ff) as usize;
    bcd[1..4].copy_from_slice(&DPD2BCD8[dpd1][0..3]);
    bcd[4..7].copy_from_slice(&DPD2BCD8[dpd2][0..3]);
    bcd[7..10].copy_from_slice(&DPD2BCD8[dpd3][0..3]);
    bcd[10..13].copy_from_slice(&DPD2BCD8[dpd4][0..3]);
    bcd[13..16].copy_from_slice(&DPD2BCD8[dpd5][0..3]);
}
const DECFLOAT_OP_DIVIDE: u32 = 0x80000000;
const DECFLOAT_OP_REMAINDER: u32 = 0x40000000;
const DECFLOAT_OP_REMNEAR: u32 = 0x20000000;
const DECFLOAT_OP_DIVIDEINT: u32 = 0x10000000;
const DEC_DIVISION_IMPOSSIBLE: u32 = 0x00000004;
const DEC_DIVISION_UNDEFINED: u32 = 0x00000008;
const BILLION: u32 = 1_000_000_000;
static DPD2BINK: [u32; 1024] = {
    let mut table = [0u32; 1024];
    let mut i = 0;
    while i < 1024 {
        table[i] = ((i % 1000) * 1000) as u32;
        i += 1;
    }
    table
};
static DPD2BINM: [u32; 1024] = {
    let mut table = [0u32; 1024];
    let mut i = 0;
    while i < 1024 {
        table[i] = ((i % 10) * 1000000) as u32;
        i += 1;
    }
    table
};
static DECPOWERS: [u32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
/// Helper to check if a DecDouble is infinity
#[inline]
fn is_infinity(df: &DecDouble) -> bool {
    (df.hi() & 0x7c000000) == 0x78000000
}
/// Check if a DecDouble is infinite
pub fn decDoubleIsInfinite(df: &DecDouble) -> u32 {
    if (df.hi() & 0x7c000000) == 0x78000000 { 1 } else { 0 }
}
/// Check if a DecDouble is subnormal
pub fn decDoubleIsSubnormal(df: &DecDouble) -> u32 {
    if (df.hi() & 0x78000000) == 0x78000000 {
        return 0;
    }
    if decDoubleIsNormal(df) != 0 {
        return 0;
    }
    if is_zero(df) {
        return 0;
    }
    1
}
/// Convert from packed BCD with checked validation
pub fn decDoubleFromPackedChecked<'a>(
    df: &'a mut DecDouble,
    exp: i32,
    packed: &[u8],
) -> Option<&'a mut DecDouble> {
    let mut bcdar = [0u8; DECDOUBLE_PMAX as usize + 2];
    let pack_len = (DECDOUBLE_PMAX as usize + 2) / 2;
    if packed.len() < pack_len {
        return None;
    }
    let mut sig = 0u32;
    let mut op = 0usize;
    for ip in 0..pack_len {
        bcdar[op] = packed[ip] >> 4;
        if bcdar[op] > 9 {
            return None;
        }
        op += 1;
        bcdar[op] = packed[ip] & 0x0f;
        if bcdar[op] > 9 && ip < pack_len - 1 {
            return None;
        }
        op += 1;
    }
    op -= 1;
    if bcdar[op] <= 9 {
        return None;
    }
    if bcdar[op] == 0x0D || bcdar[op] == 0x0B {
        sig = DECFLOAT_SIGN;
    }
    if bcdar[0] != 0 {
        return None;
    }
    if exp == DECFLOAT_QNAN as i32 || exp == DECFLOAT_SNAN as i32 {
        if bcdar[1] != 0 {
            return None;
        }
    } else if exp == DECFLOAT_INF as i32 {
        for i in 0..DECDOUBLE_PMAX as usize {
            if bcdar[i + 1] != 0 {
                return None;
            }
        }
    } else {
        if exp > DECDOUBLE_EMAX - DECDOUBLE_PMAX + 1 {
            return None;
        }
        if exp < DECDOUBLE_EMIN - DECDOUBLE_PMAX + 1 {
            return None;
        }
    }
    decDoubleFromBCD(df, exp, &bcdar[1..], sig as i32);
    Some(df)
}
/// Divide two DecDouble values
pub fn decDoubleDivide<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    decDivide(result, dfl, dfr, set, DECFLOAT_OP_DIVIDE)
}
/// Next toward another value
pub fn decDoubleNextToward<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if is_nan(dfl) || is_nan(dfr) {
        return decNaNs(result, dfl, Some(dfr), set);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    if comp == 0 {
        return decDoubleCopySign(result, dfl, dfr);
    }
    let saveround = set.round;
    let savestat = set.status;
    let deltatop: u32;
    if comp < 0 {
        if is_infinity(dfl) && (dfl.hi() & DECFLOAT_SIGN) != 0 {
            result.set_hi(0x77fcff3f | DECFLOAT_SIGN);
            result.set_lo(0xcff3fcff);
            return result;
        }
        set.round = Rounding::Ceiling;
        deltatop = 0;
    } else {
        if is_infinity(dfl) && (dfl.hi() & DECFLOAT_SIGN) == 0 {
            result.set_hi(0x77fcff3f);
            result.set_lo(0xcff3fcff);
            return result;
        }
        set.round = Rounding::Floor;
        deltatop = DECFLOAT_SIGN;
    }
    let mut delta = DecDouble::new();
    decDoubleZero(&mut delta);
    delta.dd_set_word((DECDOUBLE_BYTES / 4) - 1, 1);
    delta.dd_set_word(0, deltatop);
    let mut pointone = DecDouble::new();
    decDoubleFromString(&mut pointone, "1E-1", set);
    decDoubleFMA(result, &delta, &pointone, dfl, set);
    if decDoubleIsNormal(result) != 0 {
        set.status = savestat;
    }
    set.round = saveround;
    result
}
/// Create a DecDouble from an i32
pub fn decDoubleFromInt32<'a>(result: &'a mut DecDouble, n: i32) -> &'a mut DecDouble {
    let mut u = n as u32;
    result.set_hi(0x22380000);
    if n < 0 {
        u = (!u).wrapping_add(1);
        let hi = result.hi() | DECFLOAT_SIGN;
        result.set_hi(hi);
    }
    let mut encode = BIN2DPD[(u % 1000) as usize];
    u /= 1000;
    encode |= BIN2DPD[(u % 1000) as usize] << 10;
    u /= 1000;
    encode |= BIN2DPD[(u % 1000) as usize] << 20;
    u /= 1000;
    encode |= u << 30;
    result.dd_set_word((DECDOUBLE_BYTES / 4) - 1, encode);
    result
}
fn dec_double_to_string_internal<'a>(df: &DecDouble, buf: &'a mut [u8]) -> &'a str {
    let sourhi = df.get_word_hi();
    let sourlo = df.get_word_lo();
    let mut idx = 0;
    if (sourhi & 0x78000000) == 0x78000000 {
        if (sourhi & 0x7c000000) == 0x7c000000 {
            if (sourhi & 0x80000000) != 0 {
                buf[idx] = b'-';
                idx += 1;
            }
            if (sourhi & 0x02000000) != 0 {
                buf[idx..idx + 4].copy_from_slice(b"sNaN");
                idx += 4;
            } else {
                buf[idx..idx + 3].copy_from_slice(b"NaN");
                idx += 3;
            }
        } else {
            if (sourhi & 0x80000000) != 0 {
                buf[idx..idx + 9].copy_from_slice(b"-Infinity");
                idx += 9;
            } else {
                buf[idx..idx + 8].copy_from_slice(b"Infinity");
                idx += 8;
            }
        }
        buf[idx] = 0;
        return std::str::from_utf8(&buf[..idx]).unwrap_or("0");
    }
    if (sourhi & 0x80000000) != 0 {
        buf[idx] = b'-';
        idx += 1;
    }
    if df.is_zero_internal() {
        buf[idx] = b'0';
        idx += 1;
        buf[idx] = 0;
        return std::str::from_utf8(&buf[..idx]).unwrap_or("0");
    }
    let comb = (sourhi >> 26) as usize;
    let msd = DECCOMBMSD[comb];
    let d1 = ((sourhi >> 8) & 0x3ff) as usize;
    let d2 = (((sourhi << 2) | (sourlo >> 30)) & 0x3ff) as usize;
    let d3 = ((sourlo >> 20) & 0x3ff) as usize;
    let d4 = ((sourlo >> 10) & 0x3ff) as usize;
    let d5 = (sourlo & 0x3ff) as usize;
    let mut coeff = [0u8; 16];
    coeff[0] = msd;
    coeff[1] = DPD2BCD8[d1][1];
    coeff[2] = DPD2BCD8[d1][2];
    coeff[3] = DPD2BCD8[d1][3];
    coeff[4] = DPD2BCD8[d2][1];
    coeff[5] = DPD2BCD8[d2][2];
    coeff[6] = DPD2BCD8[d2][3];
    coeff[7] = DPD2BCD8[d3][1];
    coeff[8] = DPD2BCD8[d3][2];
    coeff[9] = DPD2BCD8[d3][3];
    coeff[10] = DPD2BCD8[d4][1];
    coeff[11] = DPD2BCD8[d4][2];
    coeff[12] = DPD2BCD8[d4][3];
    coeff[13] = DPD2BCD8[d5][1];
    coeff[14] = DPD2BCD8[d5][2];
    coeff[15] = DPD2BCD8[d5][3];
    let mut first_nonzero = 0;
    while first_nonzero < 16 && coeff[first_nonzero] == 0 {
        first_nonzero += 1;
    }
    if first_nonzero >= 16 {
        buf[idx] = b'0';
        idx += 1;
    } else {
        for i in first_nonzero..16 {
            buf[idx] = b'0' + coeff[i];
            idx += 1;
        }
    }
    buf[idx] = 0;
    std::str::from_utf8(&buf[..idx]).unwrap_or("0")
}
/// Compare two DecDoubles numerically
pub fn decDoubleCompare<'a>(
    result: &'a mut DecDouble,
    dfl: &DecDouble,
    dfr: &DecDouble,
    set: &mut DecContext,
) -> &'a mut DecDouble {
    if dfl.is_nan_internal() || dfr.is_nan_internal() {
        return decNaNs(result, dfl, Some(dfr), set);
    }
    let comp = decNumCompare(dfl, dfr, 0);
    decDoubleZero(result);
    if comp == 0 {
        return result;
    }
    result.bytes[7] = 0x01;
    if comp < 0 {
        result.bytes[0] |= 0x80;
    }
    result
}
/// Check if the DecDouble is positive (not negative, not zero, not NaN)
pub fn decDoubleIsPositive(df: &DecDouble) -> u32 {
    let is_signed = df.is_signed_internal();
    let is_zero = df.is_zero_internal();
    let is_nan = df.is_nan_internal();
    if !is_signed && !is_zero && !is_nan { 1 } else { 0 }
}
/// Get the exponent of a DecDouble
pub fn decDoubleGetExponent(df: &DecDouble) -> i32 {
    let sourhi = df.get_word_hi();
    if (sourhi & 0x78000000) == 0x78000000 {
        return (sourhi & 0x7e000000) as i32;
    }
    df.get_biased_exponent() - DECDOUBLE_BIAS
}
/// Extract coefficient as BCD array
fn extract_coefficient(df: &DecDouble) -> [u8; 16] {
    let sourhi = df.get_word_hi();
    let sourlo = df.get_word_lo();
    let comb = (sourhi >> 26) as usize;
    let msd = DECCOMBMSD[comb];
    let d1 = ((sourhi >> 8) & 0x3ff) as usize;
    let d2 = (((sourhi << 2) | (sourlo >> 30)) & 0x3ff) as usize;
    let d3 = ((sourlo >> 20) & 0x3ff) as usize;
    let d4 = ((sourlo >> 10) & 0x3ff) as usize;
    let d5 = (sourlo & 0x3ff) as usize;
    let mut coeff = [0u8; 16];
    coeff[0] = msd;
    coeff[1] = DPD2BCD8[d1][1];
    coeff[2] = DPD2BCD8[d1][2];
    coeff[3] = DPD2BCD8[d1][3];
    coeff[4] = DPD2BCD8[d2][1];
    coeff[5] = DPD2BCD8[d2][2];
    coeff[6] = DPD2BCD8[d2][3];
    coeff[7] = DPD2BCD8[d3][1];
    coeff[8] = DPD2BCD8[d3][2];
    coeff[9] = DPD2BCD8[d3][3];
    coeff[10] = DPD2BCD8[d4][1];
    coeff[11] = DPD2BCD8[d4][2];
    coeff[12] = DPD2BCD8[d4][3];
    coeff[13] = DPD2BCD8[d5][1];
    coeff[14] = DPD2BCD8[d5][2];
    coeff[15] = DPD2BCD8[d5][3];
    coeff
}
/// Check if DecDouble is a signaling NaN
pub fn decDoubleIsSignaling(df: &DecDouble) -> u32 {
    if (df.get_word_hi() & 0x7e000000) == 0x7e000000 { 1 } else { 0 }
}
/// Display a DecDouble for debugging
pub fn decDoubleShow(df: &DecDouble, tag: &str) {
    let mut hexbuf = String::with_capacity(24);
    for i in 0..8 {
        hexbuf.push_str(&format!("{:02x}", df.bytes[i]));
        if (i + 1) % 4 == 0 && i < 7 {
            hexbuf.push(' ');
        }
    }
    let mut buf = [0u8; 43];
    let string_val = dec_double_to_string_internal(df, &mut buf);
    println!(">{tag}> {hexbuf} [big-endian]  {string_val}");
}
/// Return the radix (always 10 for decimal)
pub fn decDoubleRadix(_df: &DecDouble) -> u32 {
    10
}
/// Convert to integral value with specified rounding
pub fn decDoubleToIntegralValue<'a>(
    result: &'a mut DecDouble,
    df: &DecDouble,
    set: &mut DecContext,
    round: Rounding,
) -> &'a mut DecDouble {
    decToIntegral(result, df, set, round, false)
}
impl fmt::Debug for DecDouble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DecDouble({:?})", self.bytes)
    }
}
impl DecContext {
    pub fn new_double() -> Self {
        Self::default()
    }
    pub fn new_quad() -> Self {
        DecContext {
            digits: DECQUAD_PMAX as i32,
            emax: 6144,
            emin: -6143,
            round: Rounding::HalfEven,
            traps: 0,
            status: 0,
            clamp: 0,
        }
    }
}
impl DecDouble {
    /// Create a new zero DecDouble
    pub fn new() -> Self {
        DecDouble { bytes: [0; 8] }
    }
    /// Get word at index (big-endian style) - internal version for decdouble
    #[inline]
    fn dd_get_word(&self, idx: usize) -> u32 {
        let word_idx = (8 / 4) - 1 - idx;
        let start = word_idx * 4;
        u32::from_le_bytes([
            self.bytes[start],
            self.bytes[start + 1],
            self.bytes[start + 2],
            self.bytes[start + 3],
        ])
    }
    /// Set word at index (big-endian style) - internal version for decdouble
    #[inline]
    fn dd_set_word(&mut self, idx: usize, value: u32) {
        let word_idx = (8 / 4) - 1 - idx;
        let start = word_idx * 4;
        let bytes = value.to_le_bytes();
        self.bytes[start..start + 4].copy_from_slice(&bytes);
    }
    /// Check if this is a special value (NaN or Infinity)
    #[inline]
    fn is_special(&self) -> bool {
        (self.dd_get_word(0) & DECSPECIAL_MASK) == DECSPECIAL_VALUE
    }
    /// Check if this is a NaN (quiet or signaling)
    #[inline]
    fn is_nan_internal(&self) -> bool {
        (self.dd_get_word(0) & DECNAN_MASK) == DECNAN_VALUE
    }
    /// Check if this is a signaling NaN
    #[inline]
    fn is_snan(&self) -> bool {
        (self.dd_get_word(0) & DECSNAN_MASK) == DECSNAN_VALUE
    }
    /// Check if this is infinity
    #[inline]
    fn is_infinity(&self) -> bool {
        (self.dd_get_word(0) & DECINF_MASK) == DECINF_VALUE
    }
    /// Check if this is zero
    #[inline]
    fn is_zero_internal(&self) -> bool {
        self.dd_get_word(1) == 0 && (self.dd_get_word(0) & DECZERO_LO_MASK) == 0
            && (self.dd_get_word(0) & DECZERO_SPECIAL_MASK) != DECZERO_SPECIAL_VALUE
    }
    /// Check if this is negative
    #[inline]
    fn is_negative_internal(&self) -> bool {
        (self.dd_get_word(0) & DECSIGN_MASK) != 0
    }
    /// Create a new zero DecDouble
    pub fn zero() -> Self {
        DecDouble { bytes: [0; 8] }
    }
    /// Get the high word (big-endian layout)
    #[inline]
    fn get_word_hi(&self) -> u32 {
        u32::from_be_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]])
    }
    /// Get the low word (big-endian layout)
    #[inline]
    fn get_word_lo(&self) -> u32 {
        u32::from_be_bytes([self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7]])
    }
    /// Set the high word
    #[inline]
    fn set_word_hi(&mut self, value: u32) {
        let bytes = value.to_be_bytes();
        self.bytes[0..4].copy_from_slice(&bytes);
    }
    /// Set the low word
    #[inline]
    fn set_word_lo(&mut self, value: u32) {
        let bytes = value.to_be_bytes();
        self.bytes[4..8].copy_from_slice(&bytes);
    }
    /// Check if this is a NaN
    #[inline]
    pub fn is_nan(&self) -> bool {
        (self.get_word_hi() & DECFLOAT_NAN_MASK) == DECFLOAT_NAN_VALUE
    }
    /// Check if this is infinity
    #[inline]
    pub fn is_infinite(&self) -> bool {
        (self.get_word_hi() & DECFLOAT_INF_MASK) == DECFLOAT_INF_VALUE
    }
    /// Check if this is a signaling NaN
    #[inline]
    pub fn is_signaling(&self) -> bool {
        (self.get_word_hi() & DECFLOAT_SNAN_MASK) == DECFLOAT_SNAN_VALUE
    }
    /// Check if the sign bit is set
    #[inline]
    pub fn is_signed(&self) -> bool {
        (self.get_word_hi() & DECFLOAT_SIGN) != 0
    }
    /// Check if value is a valid logical operand
    #[inline]
    fn is_logical(&self) -> bool {
        let hi = self.get_word_hi();
        let lo = self.get_word_lo();
        (hi & DECFLOAT_LOG_MASK_HI) == DECFLOAT_LOG_VALUE_HI
            && (hi & !DECFLOAT_LOG_CHECK_HI) == 0 && (lo & !DECFLOAT_LOG_CHECK_LO) == 0
    }
    fn is_zero(&self) -> bool {
        let hi = self.get_word_hi();
        let lo = self.get_word_lo();
        if (hi & 0x78000000) == 0x78000000 {
            return false;
        }
        let msd = DECCOMBMSD[(hi >> 26) as usize];
        if msd != 0 {
            return false;
        }
        (hi & 0x0003ffff) == 0 && lo == 0
    }
    /// Check if value is quiet NaN
    #[inline]
    pub fn is_qnan(&self) -> bool {
        (self.dd_get_word(0) & MASK_SNAN) == MASK_QNAN
    }
    /// Check if value is negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        (self.dd_get_word(0) & MASK_SIGN) != 0
    }
    /// Get the sign bit
    #[inline]
    pub fn get_sign(&self) -> u32 {
        self.dd_get_word(0) & MASK_SIGN
    }
    /// Get biased exponent
    #[inline]
    pub fn get_biased_exp(&self) -> i32 {
        let w0 = self.dd_get_word(0);
        let comb = (w0 >> 26) as usize;
        DECCOMBEXP[comb] + ((w0 & 0x03ffffff) >> (32 - 6 - 8)) as i32
    }
    /// Get unbiased exponent
    #[inline]
    pub fn get_exponent(&self) -> i32 {
        self.get_biased_exp() - DECDOUBLE_BIAS
    }
    /// Convert to string (internal helper)
    fn to_string_internal<'a>(&self, buf: &'a mut [u8]) -> &'a str {
        if self.is_nan() {
            if self.is_snan() {
                return if self.is_negative() { "-sNaN" } else { "sNaN" };
            }
            return if self.is_negative() { "-NaN" } else { "NaN" };
        }
        if self.is_infinite() {
            return if self.is_negative() { "-Infinity" } else { "Infinity" };
        }
        let mut bcd = [0u8; 16];
        self.get_coefficient_bcd(&mut bcd);
        let mut first = 0;
        while first < 15 && bcd[first] == 0 {
            first += 1;
        }
        let exp = self.get_exponent();
        let digits = 16 - first;
        let mut pos = 0;
        if self.is_negative() {
            buf[pos] = b'-';
            pos += 1;
        }
        if exp >= 0 && exp < 16 {
            for i in first..16 {
                buf[pos] = b'0' + bcd[i];
                pos += 1;
            }
            for _ in 0..exp {
                buf[pos] = b'0';
                pos += 1;
            }
        } else if exp < 0 && exp.abs() < digits as i32 {
            let point_pos = digits as i32 + exp;
            for (i, &d) in bcd[first..16].iter().enumerate() {
                if i as i32 == point_pos {
                    buf[pos] = b'.';
                    pos += 1;
                }
                buf[pos] = b'0' + d;
                pos += 1;
            }
        } else {
            buf[pos] = b'0' + bcd[first];
            pos += 1;
            if first < 15 {
                buf[pos] = b'.';
                pos += 1;
                for i in (first + 1)..16 {
                    buf[pos] = b'0' + bcd[i];
                    pos += 1;
                }
            }
            buf[pos] = b'E';
            pos += 1;
            let adj_exp = exp + (15 - first as i32);
            if adj_exp < 0 {
                buf[pos] = b'-';
                pos += 1;
            } else {
                buf[pos] = b'+';
                pos += 1;
            }
            let abs_exp = adj_exp.abs();
            if abs_exp >= 100 {
                buf[pos] = b'0' + (abs_exp / 100) as u8;
                pos += 1;
            }
            if abs_exp >= 10 {
                buf[pos] = b'0' + ((abs_exp / 10) % 10) as u8;
                pos += 1;
            }
            buf[pos] = b'0' + (abs_exp % 10) as u8;
            pos += 1;
        }
        std::str::from_utf8(&buf[..pos]).unwrap_or("?")
    }
    /// Extract coefficient as BCD array
    fn get_coefficient_bcd(&self, bcd: &mut [u8; 16]) {
        let w0 = self.dd_get_word(0);
        let w1 = self.dd_get_word(1);
        bcd[0] = DECCOMBMSD[(w0 >> 26) as usize];
        let declet0 = ((w0 >> 8) & 0x3ff) as usize;
        let declet1 = (((w0 << 2) | (w1 >> 30)) & 0x3ff) as usize;
        let declet2 = ((w1 >> 20) & 0x3ff) as usize;
        let declet3 = ((w1 >> 10) & 0x3ff) as usize;
        let declet4 = (w1 & 0x3ff) as usize;
        bcd[1..4].copy_from_slice(&DPD2BCD8[declet0][..3]);
        bcd[4..7].copy_from_slice(&DPD2BCD8[declet1][..3]);
        bcd[7..10].copy_from_slice(&DPD2BCD8[declet2][..3]);
        bcd[10..13].copy_from_slice(&DPD2BCD8[declet3][..3]);
        bcd[13..16].copy_from_slice(&DPD2BCD8[declet4][..3]);
    }
    /// Get high word
    #[inline]
    fn hi(&self) -> u32 {
        self.dd_get_word(0)
    }
    /// Get low word
    #[inline]
    fn lo(&self) -> u32 {
        self.dd_get_word(1)
    }
    /// Set high word
    #[inline]
    fn set_hi(&mut self, value: u32) {
        self.dd_set_word(0, value);
    }
    /// Set low word
    #[inline]
    fn set_lo(&mut self, value: u32) {
        self.dd_set_word(1, value);
    }
    /// Get byte at index (accounting for endianness)
    #[inline]
    fn get_byte(&self, idx: usize) -> u8 {
        unsafe { self.bytes[8 - 1 - idx] }
    }
    /// Set byte at index (accounting for endianness)
    #[inline]
    fn set_byte(&mut self, idx: usize, value: u8) {
        unsafe {
            self.bytes[8 - 1 - idx] = value;
        }
    }
    /// Get the high 32-bit word (big-endian word 0)
    #[inline]
    fn get_hi(&self) -> u32 {
        u32::from_le_bytes([self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7]])
    }
    /// Get the low 32-bit word (big-endian word 1)
    #[inline]
    fn get_lo(&self) -> u32 {
        u32::from_le_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]])
    }
    /// Check if this is infinity
    #[inline]
    fn is_inf_internal(&self) -> bool {
        (self.get_hi() & 0x7c000000) == DECFLOAT_INF as u32
    }
    /// Check if this is infinity
    #[inline]
    fn is_infinity_internal(&self) -> bool {
        (self.get_word_hi() & 0x7c000000) == 0x78000000
    }
    /// Check if this is signed (negative)
    #[inline]
    fn is_signed_internal(&self) -> bool {
        (self.get_word_hi() & 0x80000000) != 0
    }
    /// Get the biased exponent
    fn get_biased_exponent(&self) -> i32 {
        let sourhi = self.get_word_hi();
        let comb = (sourhi >> 26) as usize;
        let exp_cont = ((sourhi & 0x03ffffff) >> (32 - 6 - 8)) as i32;
        (DECCOMBEXP[comb] as i32 * 256) + exp_cont
    }
}
