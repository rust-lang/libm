/* kf_tan.c -- float version of k_tan.c
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

use crate::math::fabsf;
use crate::math::consts::*;

const ONE: f32 = 1.; /* 0x_3f80_0000 */
const PIO4: f32 = 7.853_981_256_5_e-01; /* 0x_3f49_0fda */
const PIO4_LO: f32 = 3.774_894_707_9_e-08; /* 0x_3322_2168 */
const T: [f32; 13] = [
    3.333_333_432_7_e-01,  /* 0x_3eaa_aaab */
    1.333_333_402_9_e-01,  /* 0x_3e08_8889 */
    5.396_825_447_7_e-02,  /* 0x_3d5d_0dd1 */
    2.186_948_806_0_e-02,  /* 0x_3cb3_27a4 */
    8.863_239_549_1_e-03,  /* 0x_3c11_371f */
    3.592_079_039_7_e-03,  /* 0x_3b6b_6916 */
    1.456_209_458_4_e-03,  /* 0x_3abe_de48 */
    5.880_412_645_6_e-04,  /* 0x_3a1a_26c8 */
    2.464_631_397_7_e-04,  /* 0x_3981_37b9 */
    7.817_944_424_5_e-05,  /* 0x_38a3_f445 */
    7.140_725_210_8_e-05,  /* 0x_3895_c07a */
    -1.855_863_774_8_e-05, /* 0x_b79b_ae5f */
    2.590_730_582_6_e-05,  /* 0x_37d9_5384 */
];

#[inline]
pub fn k_tanf(mut x: f32, mut y: f32, iy: i32) -> f32 {
    let mut z: f32;
    let mut w: f32;
    let hx = x.to_bits() as i32;
    let ix = hx & IF_ABS; /* high word of |x| */
    if ix < 0x_3180_0000 {
        /* x < 2**-28 */

        if (x as i32) == 0 {
            /* generate inexact */
            return if (ix | (iy + 1)) == 0 {
                ONE / fabsf(x)
            } else if iy == 1 {
                x
            } else {
                -ONE / x
            };
        }
    }
    if ix >= 0x_3f2c_a140 {
        /* |x|>=0.6744 */
        if hx < 0 {
            x = -x;
            y = -y;
        }
        z = PIO4 - x;
        w = PIO4_LO - y;
        x = z + w;
        y = 0.;
    }
    z = x * x;
    w = z * z;
    /* Break x^5*(T[1]+x^2*T[2]+...) into
     *      x^5(T[1]+x^4*T[3]+...+x^20*T[11]) +
     *      x^5(x^2*(T[2]+x^4*T[4]+...+x^22*[T12]))
     */
    let mut r = T[1] + w * (T[3] + w * (T[5] + w * (T[7] + w * (T[9] + w * T[11]))));
    let mut v = z * (T[2] + w * (T[4] + w * (T[6] + w * (T[8] + w * (T[10] + w * T[12])))));
    let mut s = z * x;
    r = y + z * (s * (r + v) + y);
    r += T[0] * s;
    w = x + r;
    if ix >= 0x_3f2c_a140 {
        v = iy as f32;
        return ((1 - ((hx >> 30) & 2)) as f32) * (v - 2. * (x - (w * w / (w + v) - r)));
    }
    if iy == 1 {
        w
    } else {
        /* if allow error up to 2 ulp,
        simply return -1./(x+r) here */
        /*  compute -1./(x+r) accurately */
        let mut i = w.to_bits() as i32;
        z = f32::from_bits(i as u32 & 0x_ffff_f000);
        v = r - (z - x); /* z+v = r+x */
        let a = -1. / w; /* a = -1./w */
        i = a.to_bits() as i32;
        let t = f32::from_bits(i as u32 & 0x_ffff_f000);
        s = 1. + t * z;
        t + a * (s + t * v)
    }
}
