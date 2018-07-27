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
use math::fabsf;

const ONE: f32 = 1.0000000000e+00; /* 0x3f800000 */
const PIO4: f32 = 7.8539812565e-01; /* 0x3f490fda */
const PIO4_LO: f32 = 3.7748947079e-08; /* 0x33222168 */
const T: [f32; 13] = [
    3.3333334327e-01,  /* 0x3eaaaaab */
    1.3333334029e-01,  /* 0x3e088889 */
    5.3968254477e-02,  /* 0x3d5d0dd1 */
    2.1869488060e-02,  /* 0x3cb327a4 */
    8.8632395491e-03,  /* 0x3c11371f */
    3.5920790397e-03,  /* 0x3b6b6916 */
    1.4562094584e-03,  /* 0x3abede48 */
    5.8804126456e-04,  /* 0x3a1a26c8 */
    2.4646313977e-04,  /* 0x398137b9 */
    7.8179444245e-05,  /* 0x38a3f445 */
    7.1407252108e-05,  /* 0x3895c07a */
    -1.8558637748e-05, /* 0xb79bae5f */
    2.5907305826e-05,  /* 0x37d95384 */
];

#[inline]
pub fn k_tanf(mut x: f32, mut y: f32, iy: i32) -> f32 {
    let mut z: f32;
    let mut w: f32;
    let hx = x.to_bits() as i32;
    let ix = hx & 0x7fffffff; /* high word of |x| */
    if ix < 0x31800000 {
        /* x < 2**-28 */

        if (x as i32) == 0 {
            /* generate inexact */
            return if (ix | (iy + 1)) == 0 {
                ONE / fabsf(x)
            } else {
                if iy == 1 {
                    x
                } else {
                    -ONE / x
                }
            };
        }
    }
    if ix >= 0x3f2ca140 {
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
    if ix >= 0x3f2ca140 {
        v = iy as f32;
        return ((1 - ((hx >> 30) & 2)) as f32) * (v - 2. * (x - (w * w / (w + v) - r)));
    }
    return if iy == 1 {
        w
    } else {
        /* if allow error up to 2 ulp, 
               simply return -1.0/(x+r) here */
        /*  compute -1.0/(x+r) accurately */
        let mut i = w.to_bits() as i32;
        z = f32::from_bits(i as u32 & 0xfffff000 );
        v = r - (z - x); /* z+v = r+x */
        let a = -1. / w; /* a = -1.0/w */
        i = a.to_bits() as i32;
        let t = f32::from_bits(i as u32 & 0xfffff000);
        s = 1. + t * z;
        t + a * (s + t * v)
    };
}
