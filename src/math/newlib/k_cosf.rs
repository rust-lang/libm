/* kf_cos.c -- float version of k_cos.c
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

const ONE: f32 = 1.0000000000e+00; /* 0x3f800000 */
const C1: f32 = 4.1666667908e-02; /* 0x3d2aaaab */
const C2: f32 = -1.3888889225e-03; /* 0xbab60b61 */
const C3: f32 = 2.4801587642e-05; /* 0x37d00d01 */
const C4: f32 = -2.7557314297e-07; /* 0xb493f27c */
const C5: f32 = 2.0875723372e-09; /* 0x310f74f6 */
const C6: f32 = -1.1359647598e-11; /* 0xad47d74e */

#[inline]
pub fn k_cosf(x: f32, y: f32) -> f32 {
    let mut ix = x.to_bits();
    ix &= 0x7fffffff; /* ix = |x|'s high word*/
    if ix < 0x32000000 {
        /* if x < 2**27 */
        if (x as i32) == 0 {
            /* generate inexact */
            return ONE;
        }
    }
    let z = x * x;
    let r = z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6)))));
    if ix < 0x3e99999a {
        /* if |x| < 0.3 */
        ONE - (0.5 * z - (z * r - x * y))
    } else {
        let qx = if ix > 0x3f480000 {
            /* x > 0.78125 */
            0.28125
        } else {
            f32::from_bits((ix - 0x01000000) as u32) /* x/4 */
        };
        let hz = 0.5 * z - qx;
        let a = ONE - qx;
        a - (hz - (z * r - x * y))
    }
}
