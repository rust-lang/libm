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

use math::consts::*;
const ONE: f32 = 1.0000000000e+00; /* 0x3F800000 */
const PI: f32 = 3.1415925026e+00; /* 0x40490fda */
const PIO2_HI: f32 = 1.5707962513e+00; /* 0x3fc90fda */
const PIO2_LO: f32 = 7.5497894159e-08; /* 0x33a22168 */

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
pub fn acosf(x: f32) -> f32 {
    let hx = x.to_bits() as i32;
    let ix = hx & 0x7fffffff;

    if ix == UF_1 {
        /* |x|==1 */
        if (hx > 0) {
            return 0.0; /* acos(1) = 0  */
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
    if ix < 0x3f000000 {
        /* |x| < 0.5 */
        if (ix <= 0x23000000) {
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
        let df = f32::from_bits((idf & 0xfffff000) as u32);
        c = (z - df * df) / (s + df);
        p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
        q = ONE + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
        r = p / q;
        w = r * s + c;
        2. * (df + w)
    }
}
