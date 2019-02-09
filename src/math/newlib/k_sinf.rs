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

const HALF: f32 = 5.0000000000e-01; /* 0x3f000000 */
const S1: f32 = -1.6666667163e-01; /* 0xbe2aaaab */
const S2: f32 = 8.3333337680e-03; /* 0x3c088889 */
const S3: f32 = -1.9841270114e-04; /* 0xb9500d01 */
const S4: f32 = 2.7557314297e-06; /* 0x3638ef1b */
const S5: f32 = -2.5050759689e-08; /* 0xb2d72f34 */
const S6: f32 = 1.5896910177e-10; /* 0x2f2ec9d3 */

#[inline]
pub fn k_sinf(x: f32, y: f32, iy: bool) -> f32 {
    let mut ix = x.to_bits();
    ix &= 0x7fffffff; /* high word of x */
    if ix < 0x32000000 {
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
