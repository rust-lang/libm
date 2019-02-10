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

use math::consts::*;

const ONE: f32 = 1.; /* 0x_3f80_0000 */
const C1: f32 = 4.166_666_790_8_e-02; /* 0x_3d2a_aaab */
const C2: f32 = -1.388_888_922_5_e-03; /* 0x_bab6_0b61 */
const C3: f32 = 2.480_158_764_2_e-05; /* 0x_37d0_0d01 */
const C4: f32 = -2.755_731_429_7_e-07; /* 0x_b493_f27c */
const C5: f32 = 2.087_572_337_2_e-09; /* 0x_310f_74f6 */
const C6: f32 = -1.135_964_759_8_e-11; /* 0x_ad47_d74e */

#[inline]
pub fn k_cosf(x: f32, y: f32) -> f32 {
    let mut ix = x.to_bits();
    ix &= UF_ABS; /* ix = |x|'s high word*/
    if ix < 0x_3200_0000 {
        /* if x < 2**27 */
        if (x as i32) == 0 {
            /* generate inexact */
            return ONE;
        }
    }
    let z = x * x;
    let r = z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6)))));
    if ix < 0x_3e99_999a {
        /* if |x| < 0.3 */
        ONE - (0.5 * z - (z * r - x * y))
    } else {
        let qx = if ix > 0x_3f48_0000 {
            /* x > 0.78125 */
            0.28125
        } else {
            f32::from_bits((ix - 0x_0100_0000) as u32) /* x/4 */
        };
        let hz = 0.5 * z - qx;
        let a = ONE - qx;
        a - (hz - (z * r - x * y))
    }
}
