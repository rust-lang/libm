/* origin: FreeBSD /usr/src/lib/msun/src/e_expf.c */
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

use core::f32;

use super::scalbnf;
use crate::math::consts::*;

const HALF: [f32; 2] = [0.5, -0.5];
const LN2_HI: f32 = 6.931_457_519_5_e-01; /* 0x_3f31_7200 */
const LN2_LO: f32 = 1.428_606_765_3_e-06; /* 0x_35bf_be8e */
const INV_LN2: f32 = 1.442_695_021_6; /* 0x_3fb8_aa3b */
/*
 * Domain [-0.345_68, 0.345_68], range ~[-4.278_e-9, 4.447_e-9]:
 * |x*(exp(x)+1)/(exp(x)-1) - p(x)| < 2**-27.74
 */
const P1: f32 = 1.666_662_544_e-1; /*  0x_aa_aa8f.0p-26 */
const P2: f32 = -2.766_733_290_6_e-3; /* -0x_b5_5215.0p-32 */

/// Exponential, base *e* (f32)
///
/// Calculate the exponential of `x`, that is, *e* raised to the power `x`
/// (where *e* is the base of the natural system of logarithms, approximately 2.71828).
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn expf(mut x: f32) -> f32 {
    let x1p127 = f32::from_bits(0x_7f00_0000); // 0x1p127f === 2 ^ 127
    let x1p_126 = f32::MIN_POSITIVE; // 0x1p-126f === 2 ^ -126  /*original 0x1p-149f    ??????????? */
    let mut hx = x.to_bits();
    let sign = (hx >> 31) as i32; /* sign bit of x */
    let signb: bool = sign != 0;
    hx &= UF_ABS; /* high word of |x| */

    /* special cases */
    if hx >= 0x_42ae_ac50 {
        /* if |x| >= -87.336_55 or NaN */
        if hx > UF_INF {
            /* NaN */
            return x;
        }
        if (hx >= 0x_42b1_7218) && (!signb) {
            /* x >= 88.722_839 */
            /* overflow */
            x *= x1p127;
            return x;
        }
        if signb {
            /* underflow */
            force_eval!(-x1p_126 / x);
            if hx >= 0x_42cf_f1b5 {
                /* x <= -103.972_084 */
                return 0.;
            }
        }
    }

    /* argument reduction */
    let k: i32;
    let hi: f32;
    let lo: f32;
    if hx > 0x_3eb1_7218 {
        /* if |x| > 0.5 ln2 */
        if hx > 0x_3f85_1592 {
            /* if |x| > 1.5 ln2 */
            k = (INV_LN2 * x + HALF[sign as usize]) as i32;
        } else {
            k = 1 - sign - sign;
        }
        let kf = k as f32;
        hi = x - kf * LN2_HI; /* k*ln2hi is exact here */
        lo = kf * LN2_LO;
        x = hi - lo;
    } else if hx > 0x_3900_0000 {
        /* |x| > 2**-14 */
        k = 0;
        hi = x;
        lo = 0.;
    } else {
        /* raise inexact */
        force_eval!(x1p127 + x);
        return 1. + x;
    }

    /* x is now in primary range */
    let xx = x * x;
    let c = x - xx * (P1 + xx * P2);
    let y = 1. + (x * c / (2. - c) - lo + hi);
    if k == 0 {
        y
    } else {
        scalbnf(y, k)
    }
}
