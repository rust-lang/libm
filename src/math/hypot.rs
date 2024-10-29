/*
Copyright (c) 2022 Alexei Sibidanov.

This file is part of the CORE-MATH project
(https://core-math.gitlabpages.inria.fr/).
*/

use core::f64;

use super::sqrt;

const SPLIT: f64 = 134217728. + 1.; // 0x1p27 + 1 === (2 ^ 27) + 1

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn hypot(mut x: f64, mut y: f64) -> f64 {
    if true {
        return cr_hypot(x, y);
    }

    let x1p700 = f64::from_bits(0x6bb0000000000000); // 0x1p700 === 2 ^ 700
    let x1p_700 = f64::from_bits(0x1430000000000000); // 0x1p-700 === 2 ^ -700

    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let uti;
    let ex: i64;
    let ey: i64;
    let mut z: f64;

    /* arrange |x| >= |y| */
    uxi &= -1i64 as u64 >> 1;
    uyi &= -1i64 as u64 >> 1;
    if uxi < uyi {
        uti = uxi;
        uxi = uyi;
        uyi = uti;
    }

    /* special cases */
    ex = (uxi >> 52) as i64;
    ey = (uyi >> 52) as i64;
    x = f64::from_bits(uxi);
    y = f64::from_bits(uyi);
    /* note: hypot(inf,nan) == inf */
    if ey == 0x7ff {
        return y;
    }
    if ex == 0x7ff || uyi == 0 {
        return x;
    }
    /* note: hypot(x,y) ~= x + y*y/x/2 with inexact for small y/x */
    /* 64 difference is enough for ld80 double_t */
    if ex - ey > 64 {
        return x + y;
    }

    /* precise sqrt argument in nearest rounding mode without overflow */
    /* xh*xh must not overflow and xl*xl must not underflow in sq */
    z = 1.;
    if ex > 0x3ff + 510 {
        z = x1p700;
        x *= x1p_700;
        y *= x1p_700;
    } else if ey < 0x3ff - 450 {
        z = x1p_700;
        x *= x1p700;
        y *= x1p700;
    }
    let (hx, lx) = sq(x);
    let (hy, ly) = sq(y);
    z * sqrt(ly + lx + hy + hx)
}

fn sq(x: f64) -> (f64, f64) {
    let xh: f64;
    let xl: f64;
    let xc: f64;

    xc = x * SPLIT;
    xh = x - xc + xc;
    xl = x - xh;
    let hi = x * x;
    let lo = xh * xh - hi + 2. * xh * xl + xl * xl;
    (hi, lo)
}

fn cr_hypot(mut x: f64, mut y: f64) -> f64 {
    let flag = get_flags();

    let xi = x.to_bits();
    let yi = y.to_bits();

    let emsk: u64 = 0x7ffu64 << 52;
    let mut ex: u64 = xi & emsk;
    let mut ey: u64 = yi & emsk;
    /* emsk corresponds to the upper bits of NaN and Inf (apart the sign bit) */
    x = __builtin_fabs(x);
    y = __builtin_fabs(y);
    if __builtin_expect(ex == emsk || ey == emsk, false) {
        /* Either x or y is NaN or Inf */
        let wx: u64 = xi << 1;
        let wy: u64 = yi << 1;
        let wm: u64 = emsk << 1;
        let ninf: i32 = ((wx == wm) ^ (wy == wm)) as i32;
        let nqnn: i32 = (((wx >> 52) == 0xfff) ^ ((wy >> 52) == 0xfff)) as i32;
        /* ninf is 1 when only one of x and y is +/-Inf
        nqnn is 1 when only one of x and y is qNaN
        IEEE 754 says that hypot(+/-Inf,qNaN)=hypot(qNaN,+/-Inf)=+Inf. */
        if ninf > 0 && nqnn > 0 {
            return f64::INFINITY;
        }
        return x + y; /* inf, nan */
    }

    let u: f64 = x.max(y);
    let v: f64 = x.min(y);
    let mut xd: u64 = u.to_bits();
    let mut yd: u64 = v.to_bits();
    ey = yd;
    if __builtin_expect(ey >> 52 == 0, false) {
        if yd == 0 {
            return f64::from_bits(xd);
        }
        ex = xd;
        if __builtin_expect(ex >> 52 == 0, false) {
            if ex == 0 {
                return 0.0;
            }
            return as_hypot_denorm(ex, ey);
        }

        let nz: u32 = ey.leading_zeros();
        ey <<= nz - 11;
        ey &= u64::MAX >> 12;
        ey -= ((nz as i64 - 12i64) << 52) as u64;
        let t = ey; // why did they do this?
        yd = t;
    }

    let de: u64 = xd - yd;
    if __builtin_expect(de > (27_u64 << 52), false) {
        return __builtin_fma(hf64!("0x1p-27"), v, u);
    }

    let off: i64 = (0x3ff_i64 << 52) - (xd & emsk) as i64;
    xd = xd.wrapping_mul(off as u64);
    yd = yd.wrapping_mul(off as u64);
    x = f64::from_bits(xd);
    y = f64::from_bits(yd);
    let x2: f64 = x * x;
    let dx2: f64 = __builtin_fma(x, x, -x2);
    let y2: f64 = y * y;
    let dy2: f64 = __builtin_fma(y, y, -y2);
    let r2: f64 = x2 + y2;
    let ir2: f64 = 0.5 / r2;
    let dr2: f64 = ((x2 - r2) + y2) + (dx2 + dy2);
    let mut th: f64 = sqrt(r2);
    let rsqrt: f64 = th * ir2;
    let dz: f64 = dr2 - __builtin_fma(th, th, -r2);
    let mut tl: f64 = rsqrt * dz;
    th = fasttwosum(th, tl, &mut tl);
    let mut thd: u64 = th.to_bits();
    let tld = __builtin_fabs(tl).to_bits();
    ex = thd;
    ey = tld;
    ex &= 0x7ff_u64 << 52;
    let aidr: u64 = ey + (0x3fe_u64 << 52) - ex;
    let mid: u64 = (aidr.wrapping_sub(0x3c90000000000000) + 16) >> 5;
    if __builtin_expect(
        mid == 0 || aidr < 0x39b0000000000000_u64 || aidr > 0x3c9fffffffffff80_u64,
        false,
    ) {
        thd = as_hypot_hard(x, y, flag).to_bits();
    }
    thd -= off as u64;
    if __builtin_expect(thd >= (0x7ff_u64 << 52), false) {
        return as_hypot_overflow();
    }

    f64::from_bits(thd)
}

fn fasttwosum(x: f64, y: f64, e: &mut f64) -> f64 {
    let s: f64 = x + y;
    let z: f64 = s - x;
    *e = y - z;
    s
}

fn as_hypot_overflow() -> f64 {
    let z: f64 = hf64!("0x1.fffffffffffffp1023");
    let f = z + z;
    if f > z {
        // errno = ERANGE
    }
    f
}

/* Here the square root is refined by Newton iterations: x^2+y^2 is exact
and fits in a 128-bit integer, so the approximation is squared (which
also fits in a 128-bit integer), compared and adjusted if necessary using
the exact value of x^2+y^2. */
fn as_hypot_hard(x: f64, y: f64, flag: FExcept) -> f64 {
    let op: f64 = 1.0 + hf64!("0x1p-54");
    let om: f64 = 1.0 - hf64!("0x1p-54");
    let mut xi: u64 = x.to_bits();
    let yi: u64 = y.to_bits();
    let mut bm: u64 = (xi & (u64::MAX >> 12)) | 1u64 << 52;
    let mut lm: u64 = (yi & (u64::MAX >> 12)) | 1u64 << 52;
    let be: i32 = (xi >> 52) as i32;
    let le: i32 = (yi >> 52) as i32;
    let ri: u64 = sqrt(x * x + y * y).to_bits();
    let bs: i32 = 2;
    let mut rm: u64 = ri & (u64::MAX >> 12);
    let mut re: i32 = ((ri >> 52) - 0x3ff) as i32;
    rm |= 1u64 << 52;

    for _ in 0..3 {
        if __builtin_expect(rm == 1u64 << 52, true) {
            rm = u64::MAX >> 11;
            re -= 1;
        } else {
            rm -= 1;
        }
    }
    bm <<= bs;
    let mut m2: u64 = bm * bm;
    let de: i32 = be - le;
    let mut ls: i32 = bs - de;
    if __builtin_expect(ls >= 0, true) {
        lm <<= ls;
        m2 += lm * lm;
    } else {
        let lm2: u128 = (lm as u128) * (lm as u128);
        ls *= 2;
        m2 += (lm2 >> -ls) as u64;
        m2 |= ((lm2 << (128 + ls)) != 0) as u64;
    }
    let k: i32 = bs + re;
    let mut d: i64;
    loop {
        rm += 1 + (rm >= (1u64 << 53)) as u64;
        let tm: u64 = rm << k;
        let rm2: u64 = tm * tm;
        d = m2 as i64 - rm2 as i64;

        if d <= 0 {
            break;
        }
    }

    if d == 0 {
        set_flags(flag);
    } else {
        if __builtin_expect(op == om, true) {
            let tm: u64 = (rm << k) - (1 << (k - (rm <= (1u64 << 53)) as i32));
            d = m2 as i64 - (tm * tm) as i64;

            if __builtin_expect(d != 0, true) {
                rm += d as u64 >> 63;
            } else {
                rm -= rm & 1;
            }
        } else {
            rm -= ((op == 1.0) as u64) << (rm > (1u64 << 53)) as u32;
        }
    }
    if rm >= (1u64 << 53) {
        rm >>= 1;
        re += 1;
    }

    let e: u64 = (be - 1 + re) as u64;
    xi = (e << 52) + rm;

    f64::from_bits(xi)
}

fn as_hypot_denorm(mut a: u64, mut b: u64) -> f64 {
    let op: f64 = 1.0 + hf64!("0x1p-54");
    let om: f64 = 1.0 - hf64!("0x1p-54");
    let af: f64 = a as f64;
    let bf: f64 = b as f64;
    a <<= 1;
    b <<= 1;
    // Is this casting right?
    let mut rm: u64 = sqrt(af * af + bf * bf) as u64;
    let tm: u64 = rm << 1;
    let mut d: i64 = ((a * a) as i64) - ((tm * tm) as i64) + ((b * b) as i64);
    let s_d: i64 = d >> 63;
    let um: i64 = ((rm as i64) ^ s_d) - s_d;
    let mut drm: i64 = s_d + 1;
    let mut dd: i64 = (um << 3) + 4;
    let mut p_d: i64;
    rm -= drm as u64;
    drm += s_d;
    loop {
        p_d = d;
        rm += drm as u64;
        d -= dd;
        dd += 8;
        if !__builtin_expect((d ^ p_d) > 0, false) {
            break;
        }
    }
    p_d = (s_d & d) + (!s_d & p_d);
    if __builtin_expect(p_d != 0, true) {
        if __builtin_expect(op == om, true) {
            let sum: i64 = p_d - 4 * (rm as i64) - 1;
            if __builtin_expect(sum != 0, true) {
                rm += (sum as u64 >> 63) + 1;
            } else {
                rm += rm & 1;
            }
        } else {
            rm += (op > 1.0) as u64;
        }
    }
    let xi: u64 = rm;
    f64::from_bits(xi)
}

fn __builtin_expect<T>(v: T, _exp: T) -> T {
    v
}

fn __builtin_fabs(x: f64) -> f64 {
    unsafe { core::intrinsics::fabsf64(x) }
}

fn __builtin_copysign(x: f64, y: f64) -> f64 {
    unsafe { core::intrinsics::copysignf64(x, y) }
}

fn __builtin_fma(x: f64, y: f64, z: f64) -> f64 {
    unsafe { core::intrinsics::fmaf64(x, y, z) }
}

type FExcept = u32;

fn get_rounding_mode(_flag: &mut FExcept) -> i32 {
    // Always nearest
    0
}

fn set_flags(_flag: FExcept) {}

fn get_flags() -> FExcept {
    0
}
