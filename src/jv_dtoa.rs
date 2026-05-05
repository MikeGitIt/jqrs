//! Module: jv_dtoa
//!
//! Contains 29 transpiled functions:
//! - s2b:132932997636521357:./src/jv_dtoa.c
//! - multadd:16567485181704114086:./src/jv_dtoa.c
//! - quorem:16694708742473904562:./src/jv_dtoa.c
//! - rv_alloc:9735679727817127075:./src/jv_dtoa.c
//! - bigcomp:4875740215565229139:./src/jv_dtoa.c
//! - ratio:11354645766940460308:./src/jv_dtoa.c
//! - d2b:5454373160071685796:./src/jv_dtoa.c
//! - lo0bits:16515587328683898529:./src/jv_dtoa.c
//! - match:13078071829155337316:./src/jv_dtoa.c
//! - nrv_alloc:6091107240451854204:./src/jv_dtoa.c
//! - hi0bits:1136903388659687301:./src/jv_dtoa.c
//! - cmp:1025865832362955574:./src/jv_dtoa.c
//! - jvp_freedtoa:18313461047633628000:./src/jv_dtoa.c
//! - i2b:17953321342922772131:./src/jv_dtoa.c
//! - b2d:13835837350939575248:./src/jv_dtoa.c
//! - ulp:1603443971198272412:./src/jv_dtoa.c
//! - lshift:12164825207742108847:./src/jv_dtoa.c
//! - jvp_dtoa:15213502113113472361:./src/jv_dtoa.c
//! - pow5mult:6921053584236641966:./src/jv_dtoa.c
//! - diff:4704090884256761742:./src/jv_dtoa.c
//! - jvp_dtoa_context_free:15178529885114304823:./src/jv_dtoa.c
//! - Bfree:4255777292531617407:./src/jv_dtoa.c
//! - Balloc:3878904353389285359:./src/jv_dtoa.c
//! - jvp_dtoa_fmt:15783940960572990723:./src/jv_dtoa.c
//! - mult:18313832063071140095:./src/jv_dtoa.c
//! - sulp:4407052374332842624:./src/jv_dtoa.c
//! - jvp_strtod:7784730279421285525:./src/jv_dtoa.c
//! - jvp_dtoa_context_init:13204674458996851445:./src/jv_dtoa.c
//! - dshift:5041820441007521248:./src/jv_dtoa.c

use std::mem;
use crate::types::{BCinfo, Jv, ULong, dtoa_context};

/// Big integer for arbitrary precision arithmetic in dtoa
#[derive(Debug, Clone)]
pub struct Bigint {
    pub k: i32,
    pub maxwds: i32,
    pub sign: i32,
    pub wds: i32,
    pub x: Vec<ULong>,
    pub next: Option<Box<Bigint>>,
}

/// Context for dtoa operations with freelist for Bigint allocation
#[derive(Debug)]
pub struct DtoaContext {
    pub freelist: [Option<Box<Bigint>>; 8],
    pub p5s: Option<Box<Bigint>>,
}

/// Allocate a Bigint with capacity for 2^k words
pub fn Balloc(ctx: &mut DtoaContext, k: i32) -> Box<Bigint> {
    if k <= 7 {
        if let Some(mut rv) = ctx.freelist[k as usize].take() {
            ctx.freelist[k as usize] = rv.next.take();
            rv.sign = 0;
            rv.wds = 0;
            return rv;
        }
    }
    let x = 1i32 << k;
    let mut rv = Box::new(Bigint {
        k,
        maxwds: x,
        sign: 0,
        wds: 0,
        x: vec![0; x as usize],
        next: None,
    });
    rv.sign = 0;
    rv.wds = 0;
    rv
}
/// Free a Bigint, returning it to the freelist if possible
pub fn Bfree(ctx: &mut DtoaContext, v: Option<Box<Bigint>>) {
    if let Some(mut bigint) = v {
        if bigint.k > 7 {
            drop(bigint);
        } else {
            let k = bigint.k as usize;
            bigint.next = ctx.freelist[k].take();
            ctx.freelist[k] = Some(bigint);
        }
    }
}
/// Count leading zero bits in x
pub fn hi0bits(_ctx: &mut DtoaContext, x: ULong) -> i32 {
    let mut x = x;
    let mut k = 0i32;
    if (x & 0xffff0000) == 0 {
        k = 16;
        x <<= 16;
    }
    if (x & 0xff000000) == 0 {
        k += 8;
        x <<= 8;
    }
    if (x & 0xf0000000) == 0 {
        k += 4;
        x <<= 4;
    }
    if (x & 0xc0000000) == 0 {
        k += 2;
        x <<= 2;
    }
    if (x & 0x80000000) == 0 {
        k += 1;
        if (x & 0x40000000) == 0 {
            return 32;
        }
    }
    k
}
/// Count trailing zero bits, shifting y right
pub fn lo0bits(_ctx: &mut DtoaContext, y: &mut ULong) -> i32 {
    let mut x = *y;
    let mut k = 0i32;
    if (x & 7) != 0 {
        if (x & 1) != 0 {
            return 0;
        }
        if (x & 2) != 0 {
            *y = x >> 1;
            return 1;
        }
        *y = x >> 2;
        return 2;
    }
    if (x & 0xffff) == 0 {
        k = 16;
        x >>= 16;
    }
    if (x & 0xff) == 0 {
        k += 8;
        x >>= 8;
    }
    if (x & 0xf) == 0 {
        k += 4;
        x >>= 4;
    }
    if (x & 0x3) == 0 {
        k += 2;
        x >>= 2;
    }
    if (x & 1) == 0 {
        k += 1;
        x >>= 1;
        if x == 0 {
            return 32;
        }
    }
    *y = x;
    k
}
/// Convert a Bigint to double, returning exponent in e
pub fn b2d(ctx: &mut DtoaContext, a: &Bigint, e: &mut i32) -> f64 {
    let mut d = U::new();
    let xa0 = 0usize;
    let mut xa = a.wds as usize;
    if xa == 0 {
        *e = 0;
        return 0.0;
    }
    xa -= 1;
    let y = a.x[xa];
    let k = hi0bits(ctx, y);
    *e = 32 - k;
    if k < 11 {
        d.set_l1(0x3ff00000 | (y >> (11 - k)));
        let w = if xa > xa0 {
            xa -= 1;
            a.x[xa]
        } else {
            0
        };
        d.set_l0((y << (32 - 11 + k)) | (w >> (11 - k)));
    } else {
        let z = if xa > xa0 {
            xa -= 1;
            a.x[xa]
        } else {
            0
        };
        let k = k - 11;
        if k != 0 {
            d.set_l1(0x3ff00000 | (y << k) | (z >> (32 - k)));
            let y2 = if xa > xa0 {
                xa -= 1;
                a.x[xa]
            } else {
                0
            };
            d.set_l0((z << k) | (y2 >> (32 - k)));
        } else {
            d.set_l1(0x3ff00000 | y);
            d.set_l0(z);
        }
    }
    d.d()
}
/// Convert double to Bigint
pub fn d2b(
    ctx: &mut DtoaContext,
    d: &mut U,
    e: &mut i32,
    bits: &mut i32,
) -> Box<Bigint> {
    let mut b = Balloc(ctx, 1);
    let mut z = d.l1() & 0xfffff;
    let l1 = d.l1() & 0x7fffffff;
    d.set_l1(l1);
    let de = (d.l1() >> 20) as i32;
    if de != 0 {
        z |= 0x100000;
    }
    let mut y = d.l0();
    let i;
    if y != 0 {
        let k = lo0bits(ctx, &mut y);
        if k != 0 {
            b.x[0] = y | (z << (32 - k));
            z >>= k;
        } else {
            b.x[0] = y;
        }
        if z != 0 {
            b.x[1] = z;
            i = 2;
        } else {
            i = 1;
        }
        b.wds = i;
        if de != 0 {
            *e = de - 1023 - 52 + k;
            *bits = 53 - k;
        } else {
            *e = de - 1023 - 52 + 1 + k;
            *bits = 32 * i - hi0bits(ctx, b.x[(i - 1) as usize]);
        }
    } else {
        let k = lo0bits(ctx, &mut z);
        b.x[0] = z;
        i = 1;
        b.wds = 1;
        let k = k + 32;
        if de != 0 {
            *e = de - 1023 - 52 + k;
            *bits = 53 - k;
        } else {
            *e = de - 1023 - 52 + 1 + k;
            *bits = 32 * i - hi0bits(ctx, b.x[(i - 1) as usize]);
        }
    }
    b
}
/// Multiply Bigint by small integer and add
pub fn multadd(ctx: &mut DtoaContext, b: Box<Bigint>, m: i32, a: i32) -> Box<Bigint> {
    let wds = b.wds as usize;
    let mut carry = a as u64;
    let m = m as u64;
    let mut b = b;
    for i in 0..wds {
        let y = (b.x[i] as u64) * m + carry;
        carry = y >> 32;
        b.x[i] = (y & 0xffffffff) as ULong;
    }
    if carry != 0 {
        if wds >= b.maxwds as usize {
            let mut b1 = Balloc(ctx, b.k + 1);
            for i in 0..wds {
                b1.x[i] = b.x[i];
            }
            Bfree(ctx, Some(b));
            b = b1;
        }
        b.x[wds] = carry as ULong;
        b.wds = (wds + 1) as i32;
    }
    b
}
/// Multiply two Bigints
pub fn mult(ctx: &mut DtoaContext, a: &Bigint, b: &Bigint) -> Box<Bigint> {
    let (a, b) = if a.wds < b.wds { (b, a) } else { (a, b) };
    let mut k = a.k;
    let wa = a.wds as usize;
    let wb = b.wds as usize;
    let wc = wa + wb;
    if wc > a.maxwds as usize {
        k += 1;
    }
    let mut c = Balloc(ctx, k);
    for i in 0..wc {
        c.x[i] = 0;
    }
    for j in 0..wb {
        let y = b.x[j];
        if y != 0 {
            let mut carry: u64 = 0;
            for i in 0..wa {
                let z = (a.x[i] as u64) * (y as u64) + (c.x[i + j] as u64) + carry;
                carry = z >> 32;
                c.x[i + j] = (z & 0xffffffff) as ULong;
            }
            c.x[wa + j] = carry as ULong;
        }
    }
    let mut wc = wc;
    while wc > 0 && c.x[wc - 1] == 0 {
        wc -= 1;
    }
    c.wds = wc as i32;
    c
}
/// Compare two Bigints
pub fn cmp(_ctx: &mut DtoaContext, a: &Bigint, b: &Bigint) -> i32 {
    let mut i = a.wds;
    let mut j = b.wds;
    if i != j {
        return if i - j > 0 { 1 } else { -1 };
    }
    i -= 1;
    while i >= 0 {
        let ai = a.x[i as usize];
        let bi = b.x[i as usize];
        if ai != bi {
            return if ai > bi { 1 } else { -1 };
        }
        i -= 1;
    }
    0
}
/// Subtract Bigint b from a
pub fn diff(c: &mut DtoaContext, a: &Bigint, b: &Bigint) -> Box<Bigint> {
    let cmp_result = cmp(c, a, b);
    if cmp_result == 0 {
        let mut result = Balloc(c, 0);
        result.wds = 1;
        result.x[0] = 0;
        return result;
    }
    let (a, b, sign) = if cmp_result < 0 { (b, a, 1) } else { (a, b, 0) };
    let wa = a.wds as usize;
    let wb = b.wds as usize;
    let mut result_k = 0i32;
    while (1 << result_k) < wa {
        result_k += 1;
    }
    let mut result = Balloc(c, result_k);
    result.sign = sign;
    let mut borrow: i64 = 0;
    for i in 0..wb {
        let y = (a.x[i] as i64) - (b.x[i] as i64) - borrow;
        if y < 0 {
            result.x[i] = (y + 0x100000000) as ULong;
            borrow = 1;
        } else {
            result.x[i] = y as ULong;
            borrow = 0;
        }
    }
    for i in wb..wa {
        let y = (a.x[i] as i64) - borrow;
        if y < 0 {
            result.x[i] = (y + 0x100000000) as ULong;
            borrow = 1;
        } else {
            result.x[i] = y as ULong;
            borrow = 0;
        }
    }
    result.wds = wa as i32;
    while result.wds > 0 && result.x[result.wds as usize - 1] == 0 {
        result.wds -= 1;
    }
    if result.wds == 0 {
        result.wds = 1;
    }
    result
}
/// Convert integer to Bigint
pub fn i2b(ctx: &mut DtoaContext, i: i32) -> Box<Bigint> {
    let mut b = Balloc(ctx, 1);
    b.x[0] = i as ULong;
    b.wds = 1;
    b
}
/// Convert string to Bigint
pub fn s2b(
    ctx: &mut DtoaContext,
    s: &[u8],
    nd0: i32,
    nd: i32,
    y9: ULong,
    dplen: i32,
) -> Box<Bigint> {
    let x = (nd + 8) / 9;
    let mut k = 0;
    let mut y = 1;
    while x > y {
        y <<= 1;
        k += 1;
    }
    let mut b = Balloc(ctx, k);
    b.x[0] = y9;
    b.wds = 1;
    let mut i = 9i32;
    let mut s_idx = 0usize;
    if 9 < nd0 {
        s_idx += 9;
        while i < nd0 {
            let digit = (s[s_idx] - b'0') as i32;
            b = multadd(ctx, b, 10, digit);
            s_idx += 1;
            i += 1;
        }
        s_idx += dplen as usize;
    } else {
        s_idx += (dplen + 9) as usize;
    }
    while i < nd {
        let digit = (s[s_idx] - b'0') as i32;
        b = multadd(ctx, b, 10, digit);
        s_idx += 1;
        i += 1;
    }
    b
}
/// Left shift a Bigint by k bits
pub fn lshift(ctx: &mut DtoaContext, b: Box<Bigint>, k: i32) -> Box<Bigint> {
    let n = k >> 5;
    let mut k1 = b.k;
    let mut n1 = n + b.wds + 1;
    let mut i = b.maxwds;
    while n1 > i {
        i <<= 1;
        k1 += 1;
    }
    let mut b1 = Balloc(ctx, k1);
    for i in 0..(n as usize) {
        b1.x[i] = 0;
    }
    let k = k & 0x1f;
    let mut x1_idx = n as usize;
    if k != 0 {
        let k1 = 32 - k;
        let mut z: ULong = 0;
        for i in 0..(b.wds as usize) {
            b1.x[x1_idx] = (b.x[i] << k) | z;
            z = b.x[i] >> k1;
            x1_idx += 1;
        }
        if z != 0 {
            b1.x[x1_idx] = z;
            n1 = (x1_idx + 1) as i32;
        } else {
            n1 = x1_idx as i32;
        }
    } else {
        for i in 0..(b.wds as usize) {
            b1.x[x1_idx] = b.x[i];
            x1_idx += 1;
        }
        n1 = x1_idx as i32;
    }
    b1.wds = n1 - 1;
    if b1.wds == 0 {
        b1.wds = 1;
    }
    Bfree(ctx, Some(b));
    b1
}
/// Compute quotient of b/S and update b to remainder
pub fn quorem(ctx: &mut DtoaContext, b: &mut Bigint, s: &Bigint) -> i32 {
    let n = s.wds as usize;
    if b.wds < s.wds {
        return 0;
    }
    if b.wds > s.wds || b.x[n - 1] >= s.x[n - 1] {} else {
        return 0;
    }
    let mut q: ULong;
    if n == 1 {
        q = b.x[0] / s.x[0];
        let r = b.x[0] % s.x[0];
        b.x[0] = r;
        if r == 0 {
            b.wds = 1;
        }
        return q as i32;
    }
    let mut borrow: i64 = 0;
    q = (b.x[n - 1] as u64 / s.x[n - 1] as u64) as ULong;
    if q == 0 {
        q = 1;
    }
    for i in 0..n {
        let prod = (s.x[i] as u64) * (q as u64);
        let diff = (b.x[i] as i64) - (prod as i64 & 0xffffffff) - borrow;
        b.x[i] = diff as ULong;
        borrow = (prod >> 32) as i64 - (diff >> 32);
    }
    while borrow != 0 || cmp(ctx, b, s) >= 0 {
        if borrow < 0 {
            q -= 1;
            let mut carry: u64 = 0;
            for i in 0..n {
                let sum = (b.x[i] as u64) + (s.x[i] as u64) + carry;
                b.x[i] = sum as ULong;
                carry = sum >> 32;
            }
            borrow += carry as i64;
        } else {
            break;
        }
    }
    let mut wds = n;
    while wds > 1 && b.x[wds - 1] == 0 {
        wds -= 1;
    }
    b.wds = wds as i32;
    q as i32
}
/// Compute ratio of two Bigints as a double
pub fn ratio(ctx: &mut DtoaContext, a: &Bigint, b: &Bigint) -> f64 {
    let mut da: f64;
    let mut db: f64;
    let mut ea: i32 = 0;
    let mut eb: i32 = 0;
    da = b2d(ctx, a, &mut ea);
    db = b2d(ctx, b, &mut eb);
    let k = ea - eb + 32 * (a.wds - b.wds);
    if k > 0 {
        let mut u = U::from_double(da);
        let l1 = u.l1() + ((k as ULong) << 20);
        u.set_l1(l1);
        da = u.d();
    } else if k < 0 {
        let k = -k;
        let mut u = U::from_double(db);
        let l1 = u.l1() + ((k as ULong) << 20);
        u.set_l1(l1);
        db = u.d();
    }
    da / db
}
/// Compute shift amount for normalization
pub fn dshift(_ctx: &mut DtoaContext, b: &Bigint, p2: i32) -> i32 {
    let mut rv = hi0bits(_ctx, b.x[(b.wds - 1) as usize]) - 4;
    if p2 > 0 {
        rv -= p2;
    }
    if rv < 0 {
        rv += 32;
    }
    rv
}
/// Unit in last place
pub fn ulp(_ctx: &mut DtoaContext, x: &U) -> f64 {
    let mut u = U::new();
    let l = (x.l1() & 0x7ff00000) as i64 - (52 << 20);
    if l > 0 {
        u.set_l1(l as ULong);
        u.set_l0(0);
    } else {
        let l = -l >> 20;
        if l < 20 {
            u.set_l1(0);
            u.set_l0(0x80000 >> l);
        } else {
            u.set_l1(0);
            u.set_l0(1 << (52 - l));
        }
    }
    u.d()
}
/// Compute smallest unit in last place
pub fn sulp(_c: &mut DtoaContext, x: &U, bc: &BCinfo) -> f64 {
    let mut u = U { d: 0.0 };
    unsafe {
        let exp = ((x.L[1] & EXP_MASK) >> EXP_SHIFT) as i32;
        if bc.scale != 0 && exp + (bc.scale as i32) <= 0 {
            u.L[1] = (P + 2) as ULong * EXP_MSK1;
            u.L[0] = 0;
        } else {
            u.L[1] = (exp - (P - 1) as i32 - bc.scale) as ULong * EXP_MSK1;
            u.L[0] = 0;
        }
        u.d
    }
}
/// Big comparison for edge cases
pub fn bigcomp(c: &mut DtoaContext, rv: &mut U, s0: &str, bc: &mut BCinfo) {
    let dsign = bc.dsign;
    let nd = bc.nd;
    let nd0 = bc.nd0;
    let p5 = nd + bc.e0 - 1;
    let mut speccase = false;
    let (mut b, mut p2, bbits) = unsafe {
        if rv.d == 0.0 {
            let b = i2b(c, 1);
            rv.L[1] = ((P + 2) as ULong) << 20;
            (b, EMIN - P + 1, 1)
        } else {
            let mut e = 0i32;
            let mut bits = 0i32;
            let b = d2b(c, rv, &mut e, &mut bits);
            (b, e, bits)
        }
    };
    p2 -= bc.scale;
    let mut i = P - bbits;
    let j = P - EMIN - 1 + p2;
    if i > j {
        i = j;
    }
    b = lshift(c, b, i + 1);
    b.x[0] |= 1;
    let p2 = p2 - p5 - i;
    let mut d = i2b(c, 1);
    let mut p5_val = p5;
    let mut neg_p5 = 0i32;
    if p5_val > 0 {
        d = pow5mult(c, d, p5_val);
    } else if p5_val < 0 {
        neg_p5 = -p5_val;
        b = pow5mult(c, b, neg_p5);
    }
    let (mut b2, mut d2) = if p2 > 0 { (p2, 0) } else { (0, -p2) };
    let shift = dshift(c, &d, d2);
    b2 += shift;
    d2 += shift;
    if b2 > 0 {
        b = lshift(c, b, b2);
    }
    if d2 > 0 {
        d = lshift(c, d, d2);
    }
    let mut dig = quorem(c, &mut b, &d);
    if dig == 0 {
        b = multadd(c, b, 10, 0);
        dig = quorem(c, &mut b, &d);
    }
    let s0_bytes = s0.as_bytes();
    let mut dd = 0i32;
    for idx in 0..nd0 as usize {
        if idx >= s0_bytes.len() {
            break;
        }
        dd = (s0_bytes[idx] as i32) - ('0' as i32) - dig;
        if dd != 0 {
            break;
        }
        if b.x[0] == 0 && b.wds == 1 {
            if (idx + 1) < nd as usize {
                dd = 1;
            }
            break;
        }
        b = multadd(c, b, 10, 0);
        dig = quorem(c, &mut b, &d);
    }
    unsafe {
        if speccase {
            if dd <= 0 {
                rv.d = 0.0;
            }
        } else if dd < 0 {
            if dsign == 0 {
                rv.d -= sulp(c, rv, bc);
            }
        } else if dd > 0 {
            if dsign != 0 {
                rv.d += sulp(c, rv, bc);
            }
        }
    }
}
/// Multiply by power of 5
pub fn pow5mult(ctx: &mut DtoaContext, b: Box<Bigint>, mut k: i32) -> Box<Bigint> {
    static P05: [i32; 3] = [5, 25, 125];
    let mut b = b;
    if let Some(i) = (k & 3).checked_sub(1) {
        if (i as usize) < P05.len() {
            b = multadd(ctx, b, P05[i as usize], 0);
        }
    }
    k >>= 2;
    if k == 0 {
        return b;
    }
    if ctx.p5s.is_none() {
        ctx.p5s = Some(i2b(ctx, 625));
    }
    loop {
        if (k & 1) != 0 {
            let p5 = ctx.p5s.as_ref().unwrap().clone();
            b = mult(ctx, &b, &p5);
        }
        k >>= 1;
        if k == 0 {
            break;
        }
        let p5 = ctx.p5s.take().unwrap();
        let p5_squared = mult(ctx, &p5, &p5);
        Bfree(ctx, Some(p5));
        ctx.p5s = Some(p5_squared);
    }
    b
}
/// Main dtoa function - converts double to string
pub fn jvp_dtoa(
    c: &mut DtoaContext,
    dd: f64,
    mode: i32,
    ndigits: i32,
    decpt: &mut i32,
    sign: &mut i32,
) -> String {
    let mut u = U { d: dd };
    unsafe {
        if (u.L[1] & SIGN_BIT) != 0 {
            *sign = 1;
            u.L[1] &= !SIGN_BIT;
        } else {
            *sign = 0;
        }
        if (u.L[1] & EXP_MASK) == EXP_MASK {
            *decpt = 9999;
            if u.L[0] == 0 && (u.L[1] & BNDRY_MASK) == 0 {
                return "Infinity".to_string();
            }
            return "NaN".to_string();
        }
        if u.d == 0.0 {
            *decpt = 1;
            return "0".to_string();
        }
        let mut be = 0i32;
        let mut bbits = 0i32;
        let b = d2b(c, &mut u, &mut be, &mut bbits);
        let i_exp = ((u.L[1] >> EXP_SHIFT) & ((EXP_MASK >> EXP_SHIFT) as ULong)) as i32;
        let (i, denorm) = if i_exp != 0 {
            (i_exp - BIAS, false)
        } else {
            (bbits + be + (BIAS + P - 2), true)
        };
        let mut d2 = U { d: u.d };
        d2.L[1] &= BNDRY_MASK;
        d2.L[1] |= (BIAS as ULong) << EXP_SHIFT;
        let ds = (d2.d - 1.5) * 0.289529654602168 + 0.1760912590558 + (i as f64) * LOG2P;
        let mut k = ds as i32;
        if ds < 0.0 && ds != k as f64 {
            k -= 1;
        }
        let mode = if mode < 0 || mode > 9 { 0 } else { mode };
        let mut ndigits = ndigits;
        let (leftright, ilim, ilim1) = match mode {
            0 | 1 => {
                ndigits = 0;
                (true, -1, -1)
            }
            2 | 4 => {
                let n = if ndigits <= 0 { 1 } else { ndigits };
                (mode == 4, n, n)
            }
            3 | 5 => {
                let n = ndigits + k + 1;
                (mode == 5, n, n - 1)
            }
            _ => (true, -1, -1),
        };
        let mut result = String::new();
        if be >= 0 && k <= 14 {
            let ds = TENS[k as usize];
            let mut val = u.d;
            for _ in 0..18 {
                let l = (val / ds) as i32;
                val -= (l as f64) * ds;
                result.push((b'0' + l as u8) as char);
                if val == 0.0 {
                    break;
                }
                val *= 10.0;
            }
        } else {
            let formatted = format!("{:.15e}", u.d);
            let parts: Vec<&str> = formatted.split('e').collect();
            if parts.len() == 2 {
                let mantissa = parts[0].replace(".", "").replace("-", "");
                let exp: i32 = parts[1].parse().unwrap_or(0);
                let mantissa = mantissa.trim_start_matches('0');
                if mantissa.is_empty() {
                    result.push('0');
                    k = 0;
                } else {
                    result.push_str(mantissa);
                    k = exp;
                }
            }
        }
        while result.ends_with('0') && result.len() > 1 {
            result.pop();
        }
        *decpt = k + 1;
        drop(b);
        result
    }
}
/// Format a double to string with proper formatting
pub fn jvp_dtoa_fmt(c: &mut DtoaContext, x: f64) -> String {
    let debug = std::env::var("DEBUG_DTOA").is_ok();
    if debug { eprintln!("DEBUG jvp_dtoa_fmt: x={} bits={:#x}", x, x.to_bits()); }
    if x.is_finite() && x.fract() == 0.0 && x.abs() < 1e21 {
        return format!("{:.0}", x);
    }
    let mut decpt = 0i32;
    let mut sign = 0i32;
    let s0 = jvp_dtoa(c, x, 0, 0, &mut decpt, &mut sign);
    if debug { eprintln!("DEBUG jvp_dtoa_fmt: s0={:?} decpt={} sign={}", s0, decpt, sign); }
    let mut result = String::new();
    if sign != 0 {
        result.push('-');
    }
    if decpt == 9999 {
        result.push_str(&s0);
        return result;
    }
    let se_len = s0.len() as i32;
    if decpt <= -4 || decpt > se_len + 15 {
        let mut chars = s0.chars();
        if let Some(first) = chars.next() {
            result.push(first);
        }
        let rest: String = chars.collect();
        if !rest.is_empty() {
            result.push('.');
            result.push_str(&rest);
        }
        result.push('e');
        let exp = decpt - 1;
        if exp < 0 {
            result.push('-');
            result.push_str(&format!("{}", - exp));
        } else {
            result.push('+');
            result.push_str(&format!("{}", exp));
        }
    } else if decpt <= 0 {
        result.push('0');
        result.push('.');
        for _ in 0..(-decpt) {
            result.push('0');
        }
        result.push_str(&s0);
    } else {
        let chars: Vec<char> = s0.chars().collect();
        let decpt_usize = decpt as usize;
        for (i, &ch) in chars.iter().enumerate() {
            result.push(ch);
            if i + 1 == decpt_usize && i + 1 < chars.len() {
                result.push('.');
            }
        }
        if decpt_usize > chars.len() {
            for _ in 0..(decpt_usize - chars.len()) {
                result.push('0');
            }
        }
    }
    jvp_freedtoa(c, s0);
    result
}
/// Free dtoa result string (no-op in Rust since strings are managed)
pub fn jvp_freedtoa(_ctx: &mut DtoaContext, _s: String) {}
/// Initialize dtoa context
pub fn jvp_dtoa_context_init(ctx: &mut DtoaContext) {
    for slot in ctx.freelist.iter_mut() {
        *slot = None;
    }
    ctx.p5s = None;
}
/// Free dtoa context resources
pub fn jvp_dtoa_context_free(c: &mut DtoaContext) {
    while let Some(mut p5) = c.p5s.take() {
        c.p5s = p5.next.take();
    }
    for k in 0..c.freelist.len() {
        while let Some(mut v) = c.freelist[k].take() {
            c.freelist[k] = v.next.take();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hi0bits() {
        let mut ctx = DtoaContext::new();
        assert_eq!(hi0bits(& mut ctx, 0x80000000), 0);
        assert_eq!(hi0bits(& mut ctx, 0x40000000), 1);
        assert_eq!(hi0bits(& mut ctx, 0x00010000), 15);
        assert_eq!(hi0bits(& mut ctx, 0x00000001), 31);
    }
    #[test]
    fn test_balloc_bfree() {
        let mut ctx = DtoaContext::new();
        let b = Balloc(&mut ctx, 2);
        assert_eq!(b.k, 2);
        assert_eq!(b.maxwds, 4);
        Bfree(&mut ctx, Some(b));
        let b2 = Balloc(&mut ctx, 2);
        assert_eq!(b2.k, 2);
    }
    #[test]
    fn test_i2b() {
        let mut ctx = DtoaContext::new();
        let b = i2b(&mut ctx, 42);
        assert_eq!(b.wds, 1);
        assert_eq!(b.x[0], 42);
    }
    #[test]
    fn test_multadd() {
        let mut ctx = DtoaContext::new();
        let b = i2b(&mut ctx, 10);
        let b = multadd(&mut ctx, b, 10, 5);
        assert_eq!(b.x[0], 105);
    }
    #[test]
    fn test_dtoa_basic() {
        let mut ctx = DtoaContext::new();
        let mut decpt = 0;
        let mut sign = 0;
        let result = jvp_dtoa(&mut ctx, 123.456, 0, 0, &mut decpt, &mut sign);
        assert_eq!(sign, 0);
        assert!(! result.is_empty());
    }
    #[test]
    fn test_dtoa_fmt() {
        let mut ctx = DtoaContext::new();
        let result = jvp_dtoa_fmt(&mut ctx, 0.0);
        assert!(result.contains('0'));
    }
}
/// Union for accessing double as both float and integer parts
#[derive(Clone, Copy)]
#[repr(C)]
pub union U {
    pub d: f64,
    pub L: [ULong; 2],
}
const EXP_SHIFT: u32 = 20;
const EXP_SHIFT1: u32 = 20;
const EXP_MSK1: ULong = 0x100000;
const EXP_MASK: ULong = 0x7ff00000;
const BIAS: i32 = 1023;
const P: i32 = 53;
const EMIN: i32 = -1022;
const BNDRY_MASK: ULong = 0xfffff;
const SIGN_BIT: ULong = 0x80000000;
const LOG2P: f64 = 0.301029995663981;
static TENS: [f64; 23] = [
    1e0,
    1e1,
    1e2,
    1e3,
    1e4,
    1e5,
    1e6,
    1e7,
    1e8,
    1e9,
    1e10,
    1e11,
    1e12,
    1e13,
    1e14,
    1e15,
    1e16,
    1e17,
    1e18,
    1e19,
    1e20,
    1e21,
    1e22,
];
static BIGTENS: [f64; 5] = [1e16, 1e32, 1e64, 1e128, 1e256];
/// Allocate string buffer
pub fn rv_alloc(c: &mut DtoaContext, i: i32) -> Vec<u8> {
    let j = mem::size_of::<u32>() as i32;
    let mut k = 0i32;
    let base_size = mem::size_of::<Bigint>() as i32 - mem::size_of::<u32>() as i32
        - mem::size_of::<i32>() as i32;
    let mut current_j = j;
    while base_size + current_j <= i {
        k += 1;
        current_j <<= 1;
    }
    let size = (i + 1) as usize;
    vec![0u8; size]
}
/// Allocate and copy string
pub fn nrv_alloc(c: &mut DtoaContext, s: &str, n: i32) -> (Vec<u8>, usize) {
    let mut rv = rv_alloc(c, n);
    let bytes = s.as_bytes();
    let len = bytes.len().min(n as usize);
    rv[..len].copy_from_slice(&bytes[..len]);
    (rv, len)
}
/// Check if jv is valid (placeholder for compatibility)
pub fn jv_is_valid(_x: &Jv) -> i32 {
    1
}
/// Parse string to double
pub fn jvp_strtod(c: &mut DtoaContext, s00: &str) -> (f64, usize) {
    let bytes = s00.as_bytes();
    let mut pos = 0usize;
    let mut sign = false;
    let mut nz0 = false;
    let mut nz = 0i32;
    let mut nz1 = 0i32;
    let mut rv = U { d: 0.0 };
    let mut bc = BCinfo {
        dp0: 0,
        dp1: 0,
        dplen: 0,
        dsign: 0,
        e0: 0,
        inexact: 0,
        nd: 0,
        nd0: 0,
        rounding: 0,
        scale: 0,
        uflchk: 0,
    };
    while pos < bytes.len() {
        match bytes[pos] {
            b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r' | b' ' => pos += 1,
            b'-' => {
                sign = true;
                pos += 1;
                break;
            }
            b'+' => {
                pos += 1;
                break;
            }
            0 => return (0.0, 0),
            _ => break,
        }
    }
    if pos >= bytes.len() {
        return (0.0, 0);
    }
    if bytes[pos] == b'i' || bytes[pos] == b'I' {
        pos += 1;
        if match_str(c, bytes, &mut pos, "nf") {
            pos -= 1;
            if !match_str(c, bytes, &mut pos, "inity") {
                pos += 1;
            }
            unsafe {
                rv.L[1] = 0x7ff00000;
                rv.L[0] = 0;
            }
            let result = unsafe { if sign { -rv.d } else { rv.d } };
            return (result, pos);
        }
    }
    if bytes[pos] == b'n' || bytes[pos] == b'N' {
        pos += 1;
        if match_str(c, bytes, &mut pos, "an") {
            unsafe {
                rv.L[1] = 0x7ff80000;
                rv.L[0] = 0;
            }
            return (unsafe { rv.d }, pos);
        }
    }
    if pos < bytes.len() && bytes[pos] == b'0' {
        nz0 = true;
        while pos < bytes.len() && bytes[pos] == b'0' {
            pos += 1;
        }
        if pos >= bytes.len() {
            return (0.0, pos);
        }
    }
    let s0_start = pos;
    let mut y = 0u64;
    let mut z = 0u64;
    let mut nd = 0i32;
    let mut nf = 0i32;
    while pos < bytes.len() && bytes[pos] >= b'0' && bytes[pos] <= b'9' {
        if nd < 9 {
            y = 10 * y + (bytes[pos] - b'0') as u64;
        } else if nd < 16 {
            z = 10 * z + (bytes[pos] - b'0') as u64;
        }
        nd += 1;
        pos += 1;
    }
    let nd0 = nd;
    bc.dp0 = (pos - s0_start) as i32;
    bc.dp1 = bc.dp0;
    if pos < bytes.len() && bytes[pos] == b'.' {
        pos += 1;
        bc.dp1 = (pos - s0_start) as i32;
        bc.dplen = bc.dp1 - bc.dp0;
        if nd == 0 {
            while pos < bytes.len() && bytes[pos] == b'0' {
                nz += 1;
                pos += 1;
            }
        }
        while pos < bytes.len() && bytes[pos] >= b'0' && bytes[pos] <= b'9' {
            nz += 1;
            let c = bytes[pos] - b'0';
            if c != 0 {
                nf += nz;
                for _ in 1..nz {
                    if nd < 9 {
                        y *= 10;
                    } else if nd < 16 {
                        z *= 10;
                    }
                    nd += 1;
                }
                if nd < 9 {
                    y = 10 * y + c as u64;
                } else if nd < 16 {
                    z = 10 * z + c as u64;
                }
                nd += 1;
                nz = 0;
                nz1 = 0;
            }
            pos += 1;
        }
    }
    let mut e = 0i32;
    if pos < bytes.len() && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        if nd == 0 && nz == 0 && !nz0 {
            return (0.0, 0);
        }
        pos += 1;
        let mut esign = false;
        if pos < bytes.len() {
            if bytes[pos] == b'-' {
                esign = true;
                pos += 1;
            } else if bytes[pos] == b'+' {
                pos += 1;
            }
        }
        if pos < bytes.len() && bytes[pos] >= b'0' && bytes[pos] <= b'9' {
            while pos < bytes.len() && bytes[pos] == b'0' {
                pos += 1;
            }
            if pos < bytes.len() && bytes[pos] >= b'0' && bytes[pos] <= b'9' {
                let mut l = (bytes[pos] - b'0') as i64;
                pos += 1;
                while pos < bytes.len() && bytes[pos] >= b'0' && bytes[pos] <= b'9' {
                    l = 10 * l + (bytes[pos] - b'0') as i64;
                    pos += 1;
                }
                e = if l > 19999 { 19999 } else { l as i32 };
                if esign {
                    e = -e;
                }
            }
        }
    }
    if nd == 0 {
        if !nz0 && nz == 0 {
            return (0.0, 0);
        }
        return (0.0, pos);
    }
    e -= nf;
    bc.e0 = e;
    bc.nd0 = nd0;
    let k = nd.min(16);
    unsafe {
        rv.d = y as f64;
        if k > 9 {
            rv.d = TENS[(k - 9) as usize] * rv.d + z as f64;
        }
        if nd <= 15 {
            if e == 0 {
                let result = if sign { -rv.d } else { rv.d };
                return (result, pos);
            }
            if e > 0 && e <= 22 {
                rv.d *= TENS[e as usize];
                let result = if sign { -rv.d } else { rv.d };
                return (result, pos);
            }
            if e < 0 && e >= -22 {
                rv.d /= TENS[(-e) as usize];
                let result = if sign { -rv.d } else { rv.d };
                return (result, pos);
            }
        }
    }
    match s00[..pos].parse::<f64>() {
        Ok(v) => (if sign { -v.abs() } else { v.abs() }, pos),
        Err(_) => (0.0, 0),
    }
}
/// Match string prefix (case insensitive)
pub fn match_str(_c: &mut DtoaContext, s: &[u8], pos: &mut usize, t: &str) -> bool {
    let t_bytes = t.as_bytes();
    let mut i = 0;
    while i < t_bytes.len() && *pos + i < s.len() {
        let sc = s[*pos + i];
        let tc = t_bytes[i];
        let sc_lower = if sc >= b'A' && sc <= b'Z' { sc + 32 } else { sc };
        let tc_lower = if tc >= b'A' && tc <= b'Z' { tc + 32 } else { tc };
        if sc_lower != tc_lower {
            return false;
        }
        i += 1;
    }
    if i == t_bytes.len() {
        *pos += i;
        true
    } else {
        false
    }
}
// Note: TENS and BIGTENS are already defined as static earlier in this file
const TINYTENS: [f64; 5] = [1e-16, 1e-32, 1e-64, 1e-128, 1e-256];
const P05: [i32; 3] = [5, 25, 125];
/// Helper to get p5s at a certain level
fn get_p5s_at_level(c: &DtoaContext, level: i32) -> Bigint {
    let mut current = c.p5s.as_ref().unwrap();
    for _ in 0..level {
        current = current.next.as_ref().unwrap();
    }
    current.as_ref().clone()
}
/// Ensure p5s chain has enough levels
fn ensure_p5s_level(c: &mut DtoaContext, level: i32) {
    for _ in 0..level {
        // Check current chain depth
        let current_depth = {
            let mut depth = 0;
            let mut current = c.p5s.as_ref();
            while let Some(node) = current {
                depth += 1;
                current = node.next.as_ref();
            }
            depth
        };

        if current_depth <= level {
            // Find the last node and get its clone
            let last_clone = {
                let mut current = c.p5s.as_ref().unwrap();
                while current.next.is_some() {
                    current = current.next.as_ref().unwrap();
                }
                current.as_ref().clone()
            };

            // Compute the squared value
            let p51 = mult(c, &last_clone, &last_clone);

            // Find the last node again and attach
            let mut current = c.p5s.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(p51);
        }
    }
}
/// Matches a case-insensitive string pattern.
///
/// Advances the string pointer `sp` past the matched pattern `t` if successful.
/// The comparison is case-insensitive (converts uppercase to lowercase).
///
/// # Arguments
/// * `_c` - The dtoa context (unused in this function but kept for API compatibility)
/// * `sp` - A mutable reference to the current position in the source string
/// * `t` - The pattern to match against
///
/// # Returns
/// * `true` if the pattern matches, `false` otherwise
///
/// When returning `true`, `sp` is advanced past the matched portion.
pub fn match_pattern(_c: &mut DtoaContext, sp: &mut &str, t: &str) -> bool {
    let s_bytes = sp.as_bytes();
    let t_bytes = t.as_bytes();
    if s_bytes.is_empty() {
        return t_bytes.is_empty();
    }
    let mut s_idx = 0;
    for &d in t_bytes {
        s_idx += 1;
        if s_idx >= s_bytes.len() {
            return false;
        }
        let mut c = s_bytes[s_idx];
        if c >= b'A' && c <= b'Z' {
            c += b'a' - b'A';
        }
        if c != d {
            return false;
        }
    }
    if s_idx + 1 < s_bytes.len() {
        *sp = std::str::from_utf8(&s_bytes[s_idx + 1..]).unwrap_or("");
    } else {
        *sp = "";
    }
    true
}
/// C-compatible version of match that works with raw pointers.
/// This is provided for compatibility with code that uses the original C-style API.
///
/// # Safety
/// This function is unsafe because it operates on raw pointers.
/// The caller must ensure that:
/// - `sp` points to a valid mutable pointer to a valid C string
/// - `t` points to a valid null-terminated C string
/// - The memory regions don't overlap inappropriately
#[allow(dead_code)]
pub unsafe fn match_raw(_c: *mut dtoa_context, sp: *mut *const i8, t: *const i8) -> i32 {
    if sp.is_null() || (*sp).is_null() || t.is_null() {
        return 0;
    }
    let mut s = *sp;
    let mut t_ptr = t;
    loop {
        let d = *t_ptr;
        if d == 0 {
            break;
        }
        t_ptr = t_ptr.add(1);
        s = s.add(1);
        let mut c = *s as u8;
        if c >= b'A' && c <= b'Z' {
            c += b'a' - b'A';
        }
        if c as i8 != d {
            return 0;
        }
    }
    *sp = s.add(1);
    1
}
impl Default for U {
    fn default() -> Self {
        Self::new()
    }
}
impl Bigint {
    pub fn new(k: i32) -> Self {
        let maxwds = 1 << k;
        Bigint {
            k,
            maxwds,
            sign: 0,
            wds: 0,
            x: vec![0; maxwds as usize],
            next: None,
        }
    }
}
impl U {
    pub fn new() -> Self {
        U { d: 0.0 }
    }
    pub fn from_double(d: f64) -> Self {
        U { d }
    }
    pub fn d(&self) -> f64 {
        unsafe { self.d }
    }
    pub fn set_d(&mut self, d: f64) {
        self.d = d;
    }
    /// Get the low 32 bits (L[0])
    pub fn l0(&self) -> ULong {
        unsafe { self.L[0] }
    }
    /// Get the high 32 bits (L[1])
    pub fn l1(&self) -> ULong {
        unsafe { self.L[1] }
    }
    /// Set the low 32 bits (L[0])
    pub fn set_l0(&mut self, v: ULong) {
        unsafe { self.L[0] = v; }
    }
    /// Set the high 32 bits (L[1])
    pub fn set_l1(&mut self, v: ULong) {
        unsafe { self.L[1] = v; }
    }
}
impl DtoaContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DtoaContext {
    fn default() -> Self {
        DtoaContext {
            freelist: [None, None, None, None, None, None, None, None],
            p5s: None,
        }
    }
}
impl Default for Bigint {
    fn default() -> Self {
        Bigint {
            k: 0,
            maxwds: 0,
            sign: 0,
            wds: 0,
            x: Vec::new(),
            next: None,
        }
    }
}
