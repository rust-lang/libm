/* origin: FreeBSD /usr/src/lib/msun/src/e_acos.c */
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
/* acos(x)
 * Method :
 *      acos(x)  = pi/2 - asin(x)
 *      acos(-x) = pi/2 + asin(x)
 * For |x|<=0.5
 *      acos(x) = pi/2 - (x + x*x^2*R(x^2))     (see asin.c)
 * For x>0.5
 *      acos(x) = pi/2 - (pi/2 - 2asin(sqrt((1-x)/2)))
 *              = 2asin(sqrt((1-x)/2))
 *              = 2s + 2s*z*R(z)        ...z=(1-x)/2, s=sqrt(z)
 *              = 2f + (2c + 2s*z*R(z))
 *     where f=hi part of s, and c = (z-f*f)/(s+f) is the correction term
 *     for f so that f+c ~ sqrt(z).
 * For x<-0.5
 *      acos(x) = pi - 2asin(sqrt((1-|x|)/2))
 *              = pi - 0.5*(s+s*z*R(z)), where z=(1-|x|)/2,s=sqrt(z)
 *
 * Special cases:
 *      if x is NaN, return x itself;
 *      if |x|>1, return NaN with invalid signal.
 *
 * Function needed: sqrt
 */

use core::f64;
use super::sqrt;

const PIO2_HI: f64 = 1.570_796_326_794_896_558; /* 0x_3FF9_21FB, 0x_5444_2D18 */
const PIO2_LO: f64 = 6.123_233_995_736_766_035_87_e-17; /* 0x_3C91_A626, 0x_3314_5C07 */
const PS0: f64 = 1.666_666_666_666_666_574_15_e-01; /* 0x_3FC5_5555, 0x_5555_5555 */
const PS1: f64 = -3.255_658_186_224_009_154_05_e-01; /* 0x_BFD4_D612, 0x_03EB_6F7D */
const PS2: f64 = 2.012_125_321_348_629_258_81_e-01; /* 0x_3FC9_C155, 0x_0E88_4455 */
const PS3: f64 = -4.005_553_450_067_941_140_27_e-02; /* 0x_BFA4_8228, 0x_B568_8F3B */
const PS4: f64 = 7.915_349_942_898_145_321_76_e-04; /* 0x_3F49_EFE0, 0x_7501_B288 */
const PS5: f64 = 3.479_331_075_960_211_675_70_e-05; /* 0x_3F02_3DE1, 0x_0DFD_F709 */
const QS1: f64 = -2.403_394_911_734_414_218_78; /* 0x_C003_3A27, 0x_1C8A_2D4B */
const QS2: f64 = 2.020_945_760_233_505_694_71; /* 0x_4000_2AE5, 0x_9C59_8AC8 */
const QS3: f64 = -6.882_839_716_054_532_930_30_e-01; /* 0x_BFE6_066C, 0x_1B8D_0159 */
const QS4: f64 = 7.703_815_055_590_193_527_91_e-02; /* 0x_3FB3_B8C5, 0x_B12E_9282 */

#[inline]
fn r(z: f64) -> f64 {
    let p: f64 = z * (PS0 + z * (PS1 + z * (PS2 + z * (PS3 + z * (PS4 + z * PS5)))));
    let q: f64 = 1.0 + z * (QS1 + z * (QS2 + z * (QS3 + z * QS4)));
    p / q
}

#[inline]
pub fn acos(x: f64) -> f64 {
    let x1p_120f = f64::from_bits(0x_3870_0000_0000_0000); // 0x1p-120 === 2 ^ -120
    let z: f64;
    let w: f64;
    let s: f64;
    let c: f64;
    let df: f64;
    let hx: u32;
    let ix: u32;

    hx = (x.to_bits() >> 32) as u32;
    ix = hx & 0x_7fff_ffff;
    /* |x| >= 1 or nan */
    if ix >= 0x_3ff0_0000 {
        let lx: u32 = x.to_bits() as u32;

        if ((ix - 0x_3ff0_0000) | lx) == 0 {
            /* acos(1)=0, acos(-1)=pi */
            if (hx >> 31) != 0 {
                return 2. * PIO2_HI + x1p_120f;
            }
            return 0.;
        }
        return f64::NAN;
    }
    /* |x| < 0.5 */
    if ix < 0x_3fe0_0000 {
        if ix <= 0x_3c60_0000 {
            /* |x| < 2**-57 */
            return PIO2_HI + x1p_120f;
        }
        return PIO2_HI - (x - (PIO2_LO - x * r(x * x)));
    }
    /* x < -0.5 */
    if (hx >> 31) != 0 {
        z = (1.0 + x) * 0.5;
        s = sqrt(z);
        w = r(z) * s - PIO2_LO;
        return 2. * (PIO2_HI - (s + w));
    }
    /* x > 0.5 */
    z = (1.0 - x) * 0.5;
    s = sqrt(z);
    // Set the low 4 bytes to zero
    df = f64::from_bits(s.to_bits() & 0xff_ff_ff_ff_00_00_00_00);

    c = (z - df * df) / (s + df);
    w = r(z) * s + c;
    2. * (df + w)
}
