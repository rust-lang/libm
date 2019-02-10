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

const HALF: f32 = 5.000_000_000_0_e-01; /* 0x_3f00_0000 */
const S1: f32 = -1.666_666_716_3_e-01; /* 0x_be2a_aaab */
const S2: f32 = 8.333_333_768_0_e-03; /* 0x_3c08_8889 */
const S3: f32 = -1.984_127_011_4_e-04; /* 0x_b950_0d01 */
const S4: f32 = 2.755_731_429_7_e-06; /* 0x_3638_ef1b */
const S5: f32 = -2.505_075_968_9_e-08; /* 0x_b2d7_2f34 */
const S6: f32 = 1.589_691_017_7_e-10; /* 0x_2f2e_c9d3 */

#[inline]
pub fn k_sinf(x: f32, y: f32, iy: bool) -> f32 {
    let mut ix = x.to_bits();
    ix &= 0x_7fff_ffff; /* high word of x */
    if ix < 0x_3200_0000 {
        /* |x| < 2**-27 */
        if (x as i32) == 0 {
            /* generate inexact */
            return x;
        }
    }
    let z = x * x;
    let v = z * x;
    let r = S2 + z * (S3 + z * (S4 + z * (S5 + z * S6)));
    if !iy {
        x + v * (S1 + z * r)
    } else {
        x - ((z * (HALF * y - v * r) - y) - v * S1)
    }
}
