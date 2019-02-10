/* sf_cbrt.c -- float version of s_cbrt.c.
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
 *
 */

use math::consts::*;

const B1: u32 = 709_958_130; /* B1 = (84+2/3-0.03306235651)*2**23 */
const B2: u32 = 642_849_266; /* B2 = (76+2/3-0.03306235651)*2**23 */

const C: f32 = 5.428_571_701_0_e-01; /* 19/35     = 0x_3f0a_f8b0 */
const D: f32 = -7.053_061_127_7_e-01; /* -864/1225 = 0x_bf34_8ef1 */
const E: f32 = 1.414_285_659_8; /* 99/70     = 0x_3fb5_0750 */
const F: f32 = 1.607_142_806_1; /* 45/28     = 0x_3fcd_b6db */
const G: f32 = 3.571_428_656_6_e-01; /* 5/14      = 0x_3eb6_db6e */

/// Return cube root of x
#[inline]
pub fn cbrtf(mut x: f32) -> f32 {
    let mut hx = x.to_bits() as i32;
    let sign = (hx as u32) & 0x_8000_0000; /* sign= sign(x) */
    hx ^= sign as i32;
    if hx >= IF_INF {
        return x + x; /* cbrt(NaN,INF) is itself */
    }
    if hx == 0 {
        return x; /* cbrt(0) is itself */
    }
    x = f32::from_bits(hx as u32); /* x <- |x| */
    /* rough cbrt to 5 bits */
    let mut t = if hx < 0x_0080_0000
    /* subnormal number */
    {
        /* set 2**24 */
        let high = (f32::from_bits(0x_4b80_0000) * x).to_bits();
        f32::from_bits(high as u32 / 3 + B2)
    } else {
        f32::from_bits(hx as u32 / 3 + B1)
    };

    /* new cbrt to 23 bits */
    let r = t * t / x;
    let s = C + r * t;
    t *= G + F / (s + E + D / s);

    /* retore the sign bit */
    f32::from_bits(t.to_bits() | sign)
}
