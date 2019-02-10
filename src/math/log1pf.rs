/* origin: FreeBSD /usr/src/lib/msun/src/s_log1pf.c */
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

const LN2_HI: f32 = 6.931_381_225_6_e-01; /* 0x_3f31_7180 */
const LN2_LO: f32 = 9.058_000_614_5_e-06; /* 0x_3717_f7d1 */
/* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95e-11, 4.97e-11]). */
const LG1: f32 = 0.666_666_626_93; /* 0xaaaaaa.0p-24 */
const LG2: f32 = 0.400_009_721_52; /* 0xccce13.0p-25 */
const LG3: f32 = 0.284_987_866_88; /* 0x91e9ee.0p-25 */
const LG4: f32 = 0.242_790_788_41; /* 0xf89e26.0p-26 */

#[inline]
pub fn log1pf(x: f32) -> f32 {
    let mut ui: u32 = x.to_bits();
    let hfsq: f32;
    let mut f: f32 = 0.;
    let mut c: f32 = 0.;
    let s: f32;
    let z: f32;
    let r: f32;
    let w: f32;
    let t1: f32;
    let t2: f32;
    let dk: f32;
    let ix: u32;
    let mut iu: u32;
    let mut k: i32;

    ix = ui;
    k = 1;
    if ix < 0x_3ed4_13d0 || (ix >> 31) > 0 {
        /* 1+x < sqrt(2)+  */
        if ix >= 0x_bf80_0000 {
            /* x <= -1.0 */
            if x == -1. {
                return f32::INFINITY; /* log1p(-1)=+inf */
            }
            return f32::NAN; /* log1p(x<-1)=NaN */
        }
        if ix << 1 < 0x_3380_0000 << 1 {
            /* |x| < 2**-24 */
            /* underflow if subnormal */
            if (ix & UF_INF) == 0 {
                force_eval!(x * x);
            }
            return x;
        }
        if ix <= 0x_be95_f619 {
            /* sqrt(2)/2- <= 1+x < sqrt(2)+ */
            k = 0;
            c = 0.;
            f = x;
        }
    } else if ix >= UF_INF {
        return x;
    }
    if k > 0 {
        ui = (1. + x).to_bits();
        iu = ui;
        iu += UF_1 - 0x_3f35_04f3;
        k = (iu >> 23) as i32 - 0x7f;
        /* correction term ~ log(1+x)-log(u), avoid underflow in c/u */
        if k < 25 {
            c = if k >= 2 {
                1. - (f32::from_bits(ui) - x)
            } else {
                x - (f32::from_bits(ui) - 1.)
            };
            c /= f32::from_bits(ui);
        } else {
            c = 0.;
        }
        /* reduce u into [sqrt(2)/2, sqrt(2)] */
        iu = (iu & 0x_007f_ffff) + 0x_3f35_04f3;
        ui = iu;
        f = f32::from_bits(ui) - 1.;
    }
    s = f / (2.0 + f);
    z = s * s;
    w = z * z;
    t1 = w * (LG2 + w * LG4);
    t2 = z * (LG1 + w * LG3);
    r = t2 + t1;
    hfsq = 0.5 * f * f;
    dk = k as f32;
    s * (hfsq + r) + (dk * LN2_LO + c) - hfsq + f + dk * LN2_HI
}
