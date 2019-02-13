/* origin: FreeBSD /usr/src/lib/msun/src/s_expm1f.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

use crate::math::consts::*;

const O_THRESHOLD: f32 = 8.872_167_968_8_e+01; /* 0x_42b1_7180 */
const LN2_HI: f32 = 6.931_381_225_6_e-01; /* 0x_3f31_7180 */
const LN2_LO: f32 = 9.058_000_614_5_e-06; /* 0x_3717_f7d1 */
const INV_LN2: f32 = 1.442_695_021_6; /* 0x_3fb8_aa3b */
/*
 * Domain [-0.345_68, 0.345_68], range ~[-6.694_e-10, 6.696_e-10]:
 * |6 / x * (1 + 2 * (1 / (exp(x) - 1) - 1 / x)) - q(x)| < 2**-30.04
 * Scaled coefficients: Qn_here = 2**n * Qn_for_q (see s_expm1.c):
 */
const Q1: f32 = -3.333_321_213_7_e-2; /* -0x_88_8868.0p-28 */
const Q2: f32 = 1.580_717_042_1_e-3; /*  0x_cf_3010.0p-33 */

/// Exponential, base *e*, of x-1 (f32)
///
/// Calculates the exponential of `x` and subtract 1, that is, *e* raised
/// to the power `x` minus 1 (where *e* is the base of the natural
/// system of logarithms, approximately 2.71828).
/// The result is accurate even for small values of `x`,
/// where using `exp(x)-1` would lose many significant digits.
#[inline]
pub fn expm1f(mut x: f32) -> f32 {
    let x1p127 = f32::from_bits(0x_7f00_0000); // 0x1p127f === 2 ^ 127

    let mut hx = x.to_bits();
    let sign = (hx >> 31) != 0;
    hx &= UF_ABS;

    /* filter out huge and non-finite argument */
    if hx >= 0x_4195_b844 {
        /* if |x|>=27*ln2 */
        if hx > UF_INF {
            /* NaN */
            return x;
        }
        if sign {
            return -1.;
        }
        if x > O_THRESHOLD {
            x *= x1p127;
            return x;
        }
    }

    let k: i32;
    let hi: f32;
    let lo: f32;
    let mut c = 0_f32;
    /* argument reduction */
    if hx > 0x_3eb1_7218 {
        /* if  |x| > 0.5 ln2 */
        if hx < 0x_3f85_1592 {
            /* and |x| < 1.5 ln2 */
            if !sign {
                hi = x - LN2_HI;
                lo = LN2_LO;
                k = 1;
            } else {
                hi = x + LN2_HI;
                lo = -LN2_LO;
                k = -1;
            }
        } else {
            k = (INV_LN2 * x + (if sign { -0.5 } else { 0.5 })) as i32;
            let t = k as f32;
            hi = x - t * LN2_HI; /* t*ln2_hi is exact here */
            lo = t * LN2_LO;
        }
        x = hi - lo;
        c = (hi - x) - lo;
    } else if hx < 0x_3300_0000 {
        /* when |x|<2**-25, return x */
        if hx < UF_MIN {
            force_eval!(x * x);
        }
        return x;
    } else {
        k = 0;
    }

    /* x is now in primary range */
    let hfx = 0.5 * x;
    let hxs = x * hfx;
    let r1 = 1. + hxs * (Q1 + hxs * Q2);
    let t = 3. - r1 * hfx;
    let mut e = hxs * ((r1 - t) / (6. - x * t));
    if k == 0 {
        /* c is 0 */
        return x - (x * e - hxs);
    }
    e = x * (e - c) - c;
    e -= hxs;
    /* exp(x) ~ 2^k (x_reduced - e + 1) */
    if k == -1 {
        return 0.5 * (x - e) - 0.5;
    }
    if k == 1 {
        if x < -0.25 {
            return -2. * (e - (x + 0.5));
        }
        return 1. + 2. * (x - e);
    }
    let twopk = f32::from_bits(((0x7f + k) << 23) as u32); /* 2^k */
    if (k < 0) || (k > 56) {
        /* suffice to return exp(x)-1 */
        let mut y = x - e + 1.;
        if k == 128 {
            y *= 2. * x1p127;
        } else {
            y *= twopk;
        }
        return y - 1.;
    }
    let uf = f32::from_bits(((0x7f - k) << 23) as u32); /* 2^-k */
    if k < 23 {
        (x - e + (1. - uf)) * twopk
    } else {
        (x - (e + uf) + 1.) * twopk
    }
}
