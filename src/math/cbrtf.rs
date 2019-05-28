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

use super::fdlibm::{FLT_UWORD_IS_FINITE, FLT_UWORD_IS_SUBNORMAL, FLT_UWORD_IS_ZERO};

const B1: u32 = 709_958_130; /* B1 = (84+2/3-0.03306235651)*2**23 */
const B2: u32 = 642_849_266; /* B2 = (76+2/3-0.03306235651)*2**23 */

const C: f32 = 5.428_571_701_0e-01; /* 19/35        = 0x3f0a_f8b0 */
const D: f32 = -7.053_061_127_7e-01; /* -864/1225   = 0xbf34_8ef1 */
const E: f32 = 1.414_285_659_8e+00; /* 99/70        = 0x3fb5_0750 */
const F: f32 = 1.607_142_806_1e+00; /* 45/28        = 0x3fcd_b6db */
const G: f32 = 3.571_428_656_6e-01; /* 5/14         = 0x3eb6_db6e */

/// Cube root (f32)
///
/// Computes the cube root of the argument.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn cbrtf(x: f32) -> f32 {
    let hx: u32 = x.to_bits() & 0x7fff_ffff; /* |x| */
    let sign: u32 = x.to_bits() & 0x8000_0000; /* sign = sign(x) */

    if !FLT_UWORD_IS_FINITE(hx) {
        return x + x; /* cbrt(NaN,INF) is itself */
    }
    if FLT_UWORD_IS_ZERO(hx) {
        return x; /* cbrt(0) is itself */
    }

    let x = f32::from_bits(hx); /* x <- |x| */
    /* rough cbrt to 5 bits */
    let mut t: f32 = if FLT_UWORD_IS_SUBNORMAL(hx) {
        /* subnormal number */
        let t: f32 = f32::from_bits(0x4b80_0000);
        let high: u32 = (x * t).to_bits(); /* x * (2 ** 24)*/
        f32::from_bits((high / 3).wrapping_add(B2))
    } else {
        f32::from_bits((hx / 3).wrapping_add(B1))
    };

    /* new cbrt to 23 bits */
    let r: f32 = t * t / x;
    let s: f32 = C + r * t;
    t *= G + F / (s + E + D / s);

    /* restore the sign bit */
    f32::from_bits(t.to_bits() | sign)
}
