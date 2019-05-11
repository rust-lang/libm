/* origin: FreeBSD /usr/src/lib/msun/src/e_logf.c */
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

use super::consts::*;
use core::f32;

const LN2_HI: f32 = 6.931_381_225_6_e-01; /* 0x_3f31_7180 */
const LN2_LO: f32 = 9.058_000_614_5_e-06; /* 0x_3717_f7d1 */
/* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95_e-11, 4.97_e-11]). */
const LG1: f32 = 0.666_666_626_93; /*  0x_aa_aaaa.0p-24*/
const LG2: f32 = 0.400_009_721_52; /*  0x_cc_ce13.0p-25 */
const LG3: f32 = 0.284_987_866_88; /*  0x_91_e9ee.0p-25 */
const LG4: f32 = 0.242_790_788_41; /*  0x_f8_9e26.0p-26 */

/// Natural logarithm (f32)
///
/// Returns the natural logarithm of `x`, that is, its logarithm base *e*
/// (where *e* is the base of the natural system of logarithms, 2.71828…).
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn logf(mut x: f32) -> f32 {
    let x1p25 = f32::from_bits(0x_4c00_0000); // 0x1p25f === 2 ^ 25

    let mut ix = x.to_bits();
    let mut k = 0_i32;

    if (ix < UF_MIN) || ((ix & UF_SIGN) != 0) {
        /* x < 2**-126  */
        if ix << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if (ix & UF_SIGN) != 0 {
            return (x - x) / 0.; /* log(-#) = NaN */
        }
        /* subnormal number, scale up x */
        k -= 25;
        x *= x1p25;
        ix = x.to_bits();
    } else if ix >= UF_INF {
        return x;
    } else if ix == UF_1 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    ix += UF_1 - 0x_3f35_04f3;
    k += ((ix >> 23) as i32) - 0x7f;
    ix = (ix & 0x_007f_ffff) + 0x_3f35_04f3;
    x = f32::from_bits(ix);

    let f = x - 1.;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * LG4);
    let t2 = z * (LG1 + w * LG3);
    let r = t2 + t1;
    let hfsq = 0.5 * f * f;
    let dk = k as f32;
    s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI
}
