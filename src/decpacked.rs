//! Module: decpacked
//!
//! Contains 2 transpiled functions:
//! - decPackedFromNumber:5472262157976987822:./src/decNumber/decPacked.c
//! - decPackedToNumber:377027131558053559:./src/decNumber/decPacked.c
use crate::types::*;
/// Powers of 10 for BCD arithmetic (0-2 for 3-digit groups)
const DECPOWERS: [u32; 3] = [1, 10, 100];
/// Maximum exponent value
const DEC_MAX_EMAX: i32 = 999999999;
/// Minimum exponent value
const DEC_MIN_EMIN: i32 = -999999999;
/// decNumber flags
const DECNEG: u8 = 0x80;
const DECINF: u8 = 0x40;
const DECNAN: u8 = 0x20;
const DECSNAN: u8 = 0x10;
const DECSPECIAL: u8 = DECINF | DECNAN | DECSNAN;
/// Convert a packed BCD byte array to a DecNumber
///
/// # Arguments
/// * `bcd` - The packed BCD byte array
/// * `length` - Length of the BCD array in bytes
/// * `scale` - Pointer to scale value (negative of exponent)
/// * `dn` - The DecNumber to populate
///
/// # Returns
/// * `Some(&mut DecNumber)` on success
/// * `None` if the BCD array is invalid
pub fn decPackedToNumber<'a>(
    bcd: &[u8],
    length: i32,
    scale: &i32,
    dn: &'a mut DecNumber,
) -> Option<&'a mut DecNumber> {
    if length <= 0 || bcd.len() < length as usize {
        return None;
    }
    let length = length as usize;
    dn.zero();
    let last_idx = length - 1;
    let nib = bcd[last_idx] & 0x0F;
    if nib == 0x0D || nib == 0x0B {
        dn.bits = DECNEG;
    } else if nib <= 9 {
        return None;
    }
    let mut first_idx = 0;
    while first_idx < length && bcd[first_idx] == 0 {
        first_idx += 1;
    }
    let mut digits: i32;
    if first_idx >= length {
        digits = 0;
    } else {
        digits = ((last_idx - first_idx) * 2 + 1) as i32;
        if (bcd[first_idx] & 0xF0) == 0 {
            digits -= 1;
        }
    }
    if digits != 0 {
        dn.digits = digits;
    }
    dn.exponent = -*scale;
    if *scale >= 0 {
        if (dn.digits - *scale - 1) < DEC_MIN_EMIN {
            dn.zero();
            return None;
        }
    } else {
        if *scale < DEC_MIN_EMIN || (dn.digits - *scale - 1) > DEC_MAX_EMAX {
            dn.zero();
            return None;
        }
    }
    if digits == 0 {
        return Some(dn);
    }
    let units_needed = ((digits + 2) / 3) as usize;
    dn.lsu.clear();
    dn.lsu.resize(units_needed, 0);
    let mut up_idx = 0;
    let mut cut: u32 = 0;
    let mut remaining = digits;
    let mut pos = last_idx;
    loop {
        let nib = ((bcd[pos] & 0xF0) >> 4) as u32;
        if nib > 9 {
            dn.zero();
            return None;
        }
        if cut == 0 {
            dn.lsu[up_idx] = nib as u16;
        } else {
            dn.lsu[up_idx] = (dn.lsu[up_idx] as u32 + nib * DECPOWERS[cut as usize])
                as u16;
        }
        remaining -= 1;
        if remaining == 0 {
            break;
        }
        cut += 1;
        if cut == 3 {
            up_idx += 1;
            cut = 0;
        }
        if pos == 0 {
            break;
        }
        pos -= 1;
        let nib = (bcd[pos] & 0x0F) as u32;
        if nib > 9 {
            dn.zero();
            return None;
        }
        if cut == 0 {
            dn.lsu[up_idx] = nib as u16;
        } else {
            dn.lsu[up_idx] = (dn.lsu[up_idx] as u32 + nib * DECPOWERS[cut as usize])
                as u16;
        }
        remaining -= 1;
        if remaining == 0 {
            break;
        }
        cut += 1;
        if cut == 3 {
            up_idx += 1;
            cut = 0;
        }
    }
    Some(dn)
}
/// Convert a DecNumber to a packed BCD byte array
///
/// # Arguments
/// * `bcd` - The output buffer for packed BCD bytes
/// * `length` - Length of the output buffer in bytes
/// * `scale` - Output scale value (negative of exponent)
/// * `dn` - The DecNumber to convert
///
/// # Returns
/// * `Some(&mut [u8])` slice of the BCD output on success
/// * `None` if the number cannot fit or is a special value
pub fn decPackedFromNumber<'a>(
    bcd: &'a mut [u8],
    length: i32,
    scale: &mut i32,
    dn: &DecNumber,
) -> Option<&'a mut [u8]> {
    let length = length as usize;
    if bcd.len() < length {
        return None;
    }
    if dn.digits > (length * 2 - 1) as i32 || dn.is_special() {
        return None;
    }
    let sign_nibble: u8 = if dn.is_negative() { 0x0D } else { 0x0C };
    *scale = -dn.exponent;
    for byte in bcd[..length].iter_mut() {
        *byte = 0;
    }
    let mut out_idx = length - 1;
    let mut indigs = dn.digits;
    let mut cut: u32 = 3;
    let mut up_idx = 0;
    let mut u: u32 = if !dn.lsu.is_empty() { dn.lsu[0] as u32 } else { 0 };
    let mut obyte = sign_nibble;
    loop {
        if indigs > 0 {
            if cut == 0 {
                up_idx += 1;
                u = if up_idx < dn.lsu.len() { dn.lsu[up_idx] as u32 } else { 0 };
                cut = 3;
            }
            let temp = (u.wrapping_mul(6554)) >> 16;
            let nib = u - ((temp << 1) + (temp << 3));
            u = temp;
            obyte |= (nib as u8) << 4;
            indigs -= 1;
            cut -= 1;
        }
        bcd[out_idx] = obyte;
        obyte = 0;
        if out_idx == 0 {
            break;
        }
        if indigs > 0 {
            if cut == 0 {
                up_idx += 1;
                u = if up_idx < dn.lsu.len() { dn.lsu[up_idx] as u32 } else { 0 };
                cut = 3;
            }
            let temp = (u.wrapping_mul(6554)) >> 16;
            obyte = (u - ((temp << 1) + (temp << 3))) as u8;
            u = temp;
            indigs -= 1;
            cut -= 1;
        }
        out_idx -= 1;
    }
    Some(&mut bcd[..length])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dec_number_zero() {
        let mut dn = DecNumber::new();
        dn.digits = 5;
        dn.exponent = 10;
        dn.bits = DECNEG;
        dn.zero();
        assert_eq!(dn.digits, 1);
        assert_eq!(dn.exponent, 0);
        assert_eq!(dn.bits, 0);
        assert_eq!(dn.lsu, vec![0]);
    }
    #[test]
    fn test_packed_roundtrip() {
        let mut dn = DecNumber::new();
        dn.digits = 3;
        dn.exponent = -2;
        dn.bits = 0;
        dn.lsu = vec![123];
        let mut bcd = [0u8; 4];
        let mut scale = 0i32;
        let result = decPackedFromNumber(&mut bcd, 4, &mut scale, &dn);
        assert!(result.is_some());
        assert_eq!(scale, 2);
        let mut dn2 = DecNumber::new();
        let result2 = decPackedToNumber(&bcd, 4, &scale, &mut dn2);
        assert!(result2.is_some());
        assert_eq!(dn2.digits, dn.digits);
        assert_eq!(dn2.exponent, dn.exponent);
    }
}
impl DecNumber {
    /// Create a new DecNumber initialized to zero
    pub fn new() -> Self {
        Self {
            digits: 1,
            exponent: 0,
            bits: 0,
            lsu: vec![0],
        }
    }
    /// Reset the number to zero
    pub fn zero(&mut self) {
        self.digits = 1;
        self.exponent = 0;
        self.bits = 0;
        self.lsu.clear();
        self.lsu.push(0);
    }
    /// Check if the number is negative
    pub fn is_negative(&self) -> bool {
        (self.bits & DECNEG) != 0
    }
    /// Check if the number is a special value (Inf, NaN, sNaN)
    pub fn is_special(&self) -> bool {
        (self.bits & DECSPECIAL) != 0
    }
}
impl Default for DecNumber {
    fn default() -> Self {
        Self::new()
    }
}
