/* ef_acos.c -- float version of e_acos.c.
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
const ONE: f32 = 1.; /* 0x_3F80_0000 */
const PI: f32 = 3.141_592_502_6; /* 0x_4049_0fda */
const PIO2_HI: f32 = 1.570_796_251_3; /* 0x_3fc9_0fda */
const PIO2_LO: f32 = 7.549_789_415_9_e-08; /* 0x_33a2_2168 */

const P_S0: f32 = 1.666_666_716_3_e-01; /* 0x_3e2a_aaab */
const P_S1: f32 = -3.255_658_149_7_e-01; /* 0x_bea6_b090 */
const P_S2: f32 = 2.012_125_253_7_e-01; /* 0x_3e4e_0aa8 */
const P_S3: f32 = -4.005_553_573_4_e-02; /* 0x_bd24_1146 */
const P_S4: f32 = 7.915_350_142_9_e-04; /* 0x_3a4f_7f04 */
const P_S5: f32 = 3.479_330_916_9_e-05; /* 0x_3811_ef08 */
const Q_S1: f32 = -2.403_394_937_5; /* 0x_c019_d139 */
const Q_S2: f32 = 2.020_945_787_4; /* 0x_4001_572d */
const Q_S3: f32 = -6.882_839_798_9_e-01; /* 0x_bf30_3361 */
const Q_S4: f32 = 7.703_815_400_6_e-02; /* 0x_3d9d_c62e */

#[inline]
pub fn acosf(x: f32) -> f32 {
    let hx = x.to_bits() as i32;
    let ix = hx & UF_ABS;

    if ix == UF_1 {
        /* |x|==1 */
        if (hx > 0) {
            return 0.; /* acos(1) = 0  */
        } else {
            return PI + 2. * PIO2_LO; /* acos(-1)= pi */
        }
    } else if ix > UF_1 {
        /* |x| >= 1 */
        return (x - x) / (x - x); /* acos(|x|>1) is NaN */
    }
    let z: f32;
    let w: f32;
    let r: f32;
    let p: f32;
    let q: f32;
    let s: f32;
    if ix < 0x_3f00_0000 {
        /* |x| < 0.5 */
        if (ix <= 0x_2300_0000) {
            return PIO2_HI + PIO2_LO; /*if|x|<2**-57*/
        }
        z = x * x;
        p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
        q = ONE + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
        r = p / q;
        PIO2_HI - (x - (PIO2_LO - x * r))
    } else if hx < 0 {
        /* x < -0.5 */
        z = (ONE + x) * 0.5;
        p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
        q = ONE + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
        s = sqrtf(z);
        r = p / q;
        w = r * s - PIO2_LO;
        PI - 2. * (s + w)
    } else {
        /* x > 0.5 */
        z = (ONE - x) * 0.5;
        s = sqrtf(z);
        let idf = s.to_bits() as i32;
        let df = f32::from_bits((idf & 0x_ffff_f000) as u32);
        c = (z - df * df) / (s + df);
        p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
        q = ONE + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
        r = p / q;
        w = r * s + c;
        2. * (df + w)
    }
}
