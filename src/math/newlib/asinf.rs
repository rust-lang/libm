/* ef_asin.c -- float version of e_asin.c.
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
use math::consts::*;
use math::fabsf;
use math::sqrtf;

const ONE: f32 = 1.; /* 0x_3F80_0000 */
const HUGE: f32 = 1_e+30;
const PIO2_HI: f32 = 1.570_796_370_506_286_621_093_75;
const PIO2_LO: f32 = -4.371_139_000_186_242_83_e-8;
const PIO4_HI: f32 = 0.785_398_185_253_143_310_546_875;
/* coefficient for R(x^2) */
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
pub fn asinf(x: f32) -> f32 {
    let mut w: f32;
    let mut t: f32;
    let mut p: f32;
    let mut q: f32;
    let hx = x.to_bits() as i32;
    let ix = (hx as u32) & 0x_7fff_ffff;

    if ix == UF_1 {
        /* asin(1)=+-pi/2 with inexact */
        return x * PIO2_HI + x * PIO2_LO;
    } else if ix > UF_1 {
        /* |x|>= 1 */
        return f32::NAN; /* asin(|x|>1) is NaN */
    } else if ix < 0x_3f00_0000 {
        /* |x|<0.5 */
        if ix < 0x_3200_0000 {
            /* if |x| < 2**-27 */
            if HUGE + x > ONE {
                return x;
            } /* return x with inexact if x!=0*/
        } else {
            t = x * x;
            p = t * (P_S0 + t * (P_S1 + t * (P_S2 + t * (P_S3 + t * (P_S4 + t * P_S5)))));
            q = ONE + t * (Q_S1 + t * (Q_S2 + t * (Q_S3 + t * Q_S4)));
            w = p / q;
            return x + x * w;
        }
    }
    /* 1> |x|>= 0.5 */
    w = ONE - fabsf(x);
    t = w * 0.5;
    p = t * (P_S0 + t * (P_S1 + t * (P_S2 + t * (P_S3 + t * (P_S4 + t * P_S5)))));
    q = ONE + t * (Q_S1 + t * (Q_S2 + t * (Q_S3 + t * Q_S4)));
    let s = sqrtf(t);
    t = if ix >= 0x_3f79_999a {
        /* if |x| > 0.975 */
        w = p / q;
        PIO2_HI - (2. * (s + s * w) - PIO2_LO)
    } else {
        w = s;
        let iw = w.to_bits() as i32;
        w = f32::from_bits((iw as u32) & 0x_ffff_f000);
        let c = (t - w * w) / (s + w);
        let r = p / q;
        p = 2. * s * r - (PIO2_LO - 2. * c);
        q = PIO4_HI - 2. * w;
        PIO4_HI - (p - q)
    };
    if hx > 0 {
        t
    } else {
        -t
    }
}
