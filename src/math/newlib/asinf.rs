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

use math::fabsf;
use math::sqrtf;

const ONE: f32 = 1.0000000000e+00; /* 0x3F800000 */
const HUGE: f32 = 1.000e+30;
const PIO2_HI: f32 = 1.57079637050628662109375;
const PIO2_LO: f32 = -4.37113900018624283e-8;
const PIO4_HI: f32 = 0.785398185253143310546875;
/* coefficient for R(x^2) */
const P_S0: f32 = 1.6666667163e-01; /* 0x3e2aaaab */
const P_S1: f32 = -3.2556581497e-01; /* 0xbea6b090 */
const P_S2: f32 = 2.0121252537e-01; /* 0x3e4e0aa8 */
const P_S3: f32 = -4.0055535734e-02; /* 0xbd241146 */
const P_S4: f32 = 7.9153501429e-04; /* 0x3a4f7f04 */
const P_S5: f32 = 3.4793309169e-05; /* 0x3811ef08 */
const Q_S1: f32 = -2.4033949375e+00; /* 0xc019d139 */
const Q_S2: f32 = 2.0209457874e+00; /* 0x4001572d */
const Q_S3: f32 = -6.8828397989e-01; /* 0xbf303361 */
const Q_S4: f32 = 7.7038154006e-02; /* 0x3d9dc62e */

#[inline]
pub fn asinf(x: f32) -> f32 {
    let mut w: f32;
    let mut t: f32;
    let mut p: f32;
    let mut q: f32;
    let hx = x.to_bits() as i32;
    let ix = hx & 0x7fffffff;

    if ix == 0x3f800000 {
        /* asin(1)=+-pi/2 with inexact */
        return x * PIO2_HI + x * PIO2_LO;
    } else if ix > 0x3f800000 {
        /* |x|>= 1 */
        return (x - x) / (x - x); /* asin(|x|>1) is NaN */
    } else if ix < 0x3f000000 {
        /* |x|<0.5 */
        if ix < 0x32000000 {
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
    t = if ix >= 0x3F79999A {
        /* if |x| > 0.975 */
        w = p / q;
        PIO2_HI - (2. * (s + s * w) - PIO2_LO)
    } else {
        w = s;
        let iw = w.to_bits() as i32;
        w = f32::from_bits((iw as u32) & 0xfffff000);
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
