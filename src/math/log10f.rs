/* origin: FreeBSD /usr/src/lib/msun/src/e_log10f.c */
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
/*
 * See comments in log10.c.
 */

use crate::math::consts::*;
use core::f32;

const IVLN10HI: f32 = 4.343_261_718_8_e-01; /* 0x_3ede_6000 */
const IVLN10LO: f32 = -3.168_997_136_5_e-05; /* 0x_b804_ead9 */
const LOG10_2HI: f32 = 3.010_292_053_2_e-01; /* 0x_3e9a_2080 */
const LOG10_2LO: f32 = 7.903_415_166_8_e-07; /* 0x_3554_27db */
/* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95_e-11, 4.97_e-11]). */
const LG1: f32 = 0.666_666_626_93; /* 0x_aa_aaaa.0p-24 */
const LG2: f32 = 0.400_009_721_52; /* 0x_cc_ce13.0p-25 */
const LG3: f32 = 0.284_987_866_88; /* 0x_91_e9ee.0p-25 */
const LG4: f32 = 0.242_790_788_41; /* 0x_f8_9e26.0p-26 */

/// Base 10 logarithm (f32)
///
/// Returns the base 10 logarithm of `x`. It is implemented as `log(x)/log(10)`.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log10f(mut x: f32) -> f32 {
    let x1p25f = f32::from_bits(0x_4c00_0000); // 0x1p25f === 2 ^ 25

    let mut ui: u32 = x.to_bits();

    let mut ix = ui;
    let mut k = 0;
    if ix < UF_MIN || (ix >> 31) > 0 {
        /* x < 2**-126  */
        if ix << 1 == 0 {
            return f32::NEG_INFINITY; /* log(+-0)=-inf */
        }
        if (ix >> 31) > 0 {
            return f32::NAN; /* log(-#) = NaN */
        }
        /* subnormal number, scale up x */
        k -= 25;
        x *= x1p25f;
        ui = x.to_bits();
        ix = ui;
    } else if ix >= UF_INF {
        return x;
    } else if ix == UF_1 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    ix += UF_1 - 0x_3f35_04f3;
    k += (ix >> 23) as i32 - 0x7f;
    ix = (ix & 0x_007f_ffff) + 0x_3f35_04f3;
    ui = ix;
    x = f32::from_bits(ui);

    let f = x - 1.;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * LG4);
    let t2 = z * (LG1 + w * LG3);
    let r = t2 + t1;
    let hfsq = 0.5 * f * f;

    let hi = f - hfsq;
    ui = hi.to_bits();
    ui &= 0x_ffff_f000;
    let hi = f32::from_bits(ui);
    let lo = f - hi - hfsq + s * (hfsq + r);
    let dk = k as f32;
    dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI + hi * IVLN10HI + dk * LOG10_2HI
}
