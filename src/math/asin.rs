/* origin: FreeBSD /usr/src/lib/msun/src/e_asin.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* asin(x)
 * Method :
 *      Since  asin(x) = x + x^3/6 + x^5*3/40 + x^7*15/336 + ...
 *      we approximate asin(x) on [0,0.5] by
 *              asin(x) = x + x*x^2*R(x^2)
 *      where
 *              R(x^2) is a rational approximation of (asin(x)-x)/x^3
 *      and its remez error is bounded by
 *              |(asin(x)-x)/x^3 - R(x^2)| < 2^(-58.75)
 *
 *      For x in [0.5,1]
 *              asin(x) = pi/2-2*asin(sqrt((1-x)/2))
 *      Let y = (1-x), z = y/2, s := sqrt(z), and pio2_hi+pio2_lo=pi/2;
 *      then for x>0.98
 *              asin(x) = pi/2 - 2*(s+s*z*R(z))
 *                      = pio2_hi - (2*(s+s*z*R(z)) - pio2_lo)
 *      For x<=0.98, let pio4_hi = pio2_hi/2, then
 *              f = hi part of s;
 *              c = sqrt(z) - f = (z-f*f)/(s+f)         ...f+c=sqrt(z)
 *      and
 *              asin(x) = pi/2 - 2*(s+s*z*R(z))
 *                      = pio4_hi+(pio4-2s)-(2s*z*R(z)-pio2_lo)
 *                      = pio4_hi+(pio4-2f)-(2s*z*R(z)-(pio2_lo+2c))
 *
 * Special cases:
 *      if x is NaN, return x itself;
 *      if |x|>1, return NaN with invalid signal.
 *
 */

use super::consts::*;
use super::{fabs, get_high_word, get_low_word, sqrt, with_set_low_word};
use core::f64;

const PIO2_HI: f64 = f64::consts::FRAC_PI_2; /* 0x_3FF9_21FB, 0x_5444_2D18 */
const PIO2_LO: f64 = 6.123_233_995_736_766_035_87_e-17; /* 0x_3C91_A626, 0x_3314_5C07 */
/* coefficients for R(x^2) */
const P_S0: f64 = 1.666_666_666_666_666_574_15_e-01; /* 0x_3FC5_5555, 0x_5555_5555 */
const P_S1: f64 = -3.255_658_186_224_009_154_05_e-01; /* 0x_BFD4_D612, 0x_03EB_6F7D */
const P_S2: f64 = 2.012_125_321_348_629_258_81_e-01; /* 0x_3FC9_C155, 0x_0E88_4455 */
const P_S3: f64 = -4.005_553_450_067_941_140_27_e-02; /* 0x_BFA4_8228, 0x_B568_8F3B */
const P_S4: f64 = 7.915_349_942_898_145_321_76_e-04; /* 0x_3F49_EFE0, 0x_7501_B288 */
const P_S5: f64 = 3.479_331_075_960_211_675_7_e-05; /* 0x_3F02_3DE1, 0x_0DFD_F709 */
const Q_S1: f64 = -2.403_394_911_734_414_218_78; /* 0x_C003_3A27, 0x_1C8A_2D4B */
const Q_S2: f64 = 2.020_945_760_233_505_694_71; /* 0x_4000_2AE5, 0x_9C59_8AC8 */
const Q_S3: f64 = -6.882_839_716_054_532_930_3_e-01; /* 0x_BFE6_066C, 0x_1B8D_0159 */
const Q_S4: f64 = 7.703_815_055_590_193_527_91_e-02; /* 0x_3FB3_B8C5, 0x_B12E_9282 */

#[inline]
fn comp_r(z: f64) -> f64 {
    let p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
    let q = 1. + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
    p / q
}

/// Arcsine (f64)
///
/// Computes the inverse sine (arc sine) of the argument `x`.
/// Arguments to asin must be in the range -1 to 1.
/// Returns values in radians, in the range of -pi/2 to pi/2.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn asin(mut x: f64) -> f64 {
    let z: f64;
    let r: f64;
    let s: f64;
    let hx: u32;
    let ix: u32;

    hx = get_high_word(x);
    ix = hx & UF_ABS;
    /* |x| >= 1 or nan */
    if ix >= 0x_3ff0_0000 {
        let lx: u32;
        lx = get_low_word(x);
        if ((ix - 0x_3ff0_0000) | lx) == 0 {
            /* asin(1) = +-pi/2 with inexact */
            return x * PIO2_HI + f64::from_bits(0x_3870_0000_0000_0000);
        } else {
            return 0.0 / (x - x);
        }
    }
    /* |x| < 0.5 */
    if ix < 0x_3fe0_0000 {
        /* if 0x1p-1022 <= |x| < 0x1p-26, avoid raising underflow */
        if ix < 0x_3e50_0000 && ix >= 0x_0010_0000 {
            return x;
        } else {
            return x + x * comp_r(x * x);
        }
    }
    /* 1 > |x| >= 0.5 */
    z = (1. - fabs(x)) * 0.5;
    s = sqrt(z);
    r = comp_r(z);
    if ix >= 0x_3fef_3333 {
        /* if |x| > 0.975 */
        x = PIO2_HI - (2. * (s + s * r) - PIO2_LO);
    } else {
        let f: f64;
        let c: f64;
        /* f+c = sqrt(z) */
        f = with_set_low_word(s, 0);
        c = (z - f * f) / (s + f);
        x = 0.5 * PIO2_HI - (2. * s * r - (PIO2_LO - 2. * c) - (0.5 * PIO2_HI - 2. * f));
    }
    if hx >> 31 != 0 {
        -x
    } else {
        x
    }
}
