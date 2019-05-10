/* origin: FreeBSD /usr/src/lib/msun/src/e_acosf.c */
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
use super::sqrtf::sqrtf;
use core::f32;

const PIO2_HI: f32 = 1.570_796_251_3_e+00; /* 0x_3fc9_0fda */
const PIO2_LO: f32 = 7.549_789_415_9_e-08; /* 0x_33a2_2168 */
const P_S0: f32 = 1.666_658_669_7_e-01;
const P_S1: f32 = -4.274_342_209_1_e-02;
const P_S2: f32 = -8.656_363_003_0_e-03;
const Q_S1: f32 = -7.066_296_339_0_e-01;

const UF_0_5: u32 = 0x_3f00_0000;

#[inline]
fn r(z: f32) -> f32 {
    let p = z * (P_S0 + z * (P_S1 + z * P_S2));
    let q = 1. + z * Q_S1;
    p / q
}

/// Arccosine (f32)
///
/// Computes the inverse cosine (arc cosine) of the input value.
/// Arguments must be in the range -1 to 1.
/// Returns values in radians, in the range of 0 to pi.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn acosf(x: f32) -> f32 {
    let x1p_120 = f32::from_bits(0x_0380_0000); // 0x1p-120 === 2 ^ (-120)

    let z: f32;
    let w: f32;
    let s: f32;

    let mut hx = x.to_bits();
    let sign = (hx >> 31) != 0;
    let ix = hx & UF_ABS;
    /* |x| >= 1 or nan */
    if ix >= UF_1 {
        if ix == UF_1 {
            if sign {
                return 2. * PIO2_HI + x1p_120;
            }
            return 0.;
        }
        return 0. / (x - x);
    }
    /* |x| < 0.5 */
    if ix < UF_0_5 {
        if ix <= 0x_3280_0000 {
            /* |x| < 2**-26 */
            return PIO2_HI + x1p_120;
        }
        return PIO2_HI - (x - (PIO2_LO - x * r(x * x)));
    }
    /* x < -0.5 */
    if sign {
        z = (1. + x) * 0.5;
        s = sqrtf(z);
        w = r(z) * s - PIO2_LO;
        return 2. * (PIO2_HI - (s + w));
    }
    /* x > 0.5 */
    z = (1. - x) * 0.5;
    s = sqrtf(z);
    hx = s.to_bits();
    let df = f32::from_bits(hx & 0x_ffff_f000);
    let c = (z - df * df) / (s + df);
    w = r(z) * s + c;
    2. * (df + w)
}
