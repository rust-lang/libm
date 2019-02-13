/* origin: FreeBSD /usr/src/lib/msun/src/s_sinf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Optimized by Bruce D. Evans.
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

use super::{k_cosf, k_sinf, rem_pio2f};
use crate::math::consts::*;
use core::f32;

/* Small multiples of pi/2 rounded to double precision. */
const PI_2: f32 = f32::consts::FRAC_PI_2;
const S1PIO2: f32 = 1. * PI_2; /* 0x_3FF9_21FB, 0x_5444_2D18 */
const S2PIO2: f32 = 2. * PI_2; /* 0x_4009_21FB, 0x_5444_2D18 */
const S3PIO2: f32 = 3. * PI_2; /* 0x_4012_D97C, 0x_7F33_21D2 */
const S4PIO2: f32 = 4. * PI_2; /* 0x_4019_21FB, 0x_5444_2D18 */

pub fn sincosf(x: f32) -> (f32, f32) {
    let s: f32;
    let c: f32;
    let mut ix: u32;
    let sign: bool;

    ix = x.to_bits();
    sign = (ix >> 31) != 0;
    ix &= UF_ABS;

    /* |x| ~<= pi/4 */
    if ix <= 0x_3f49_0fda {
        /* |x| < 2**-12 */
        if ix < 0x_3980_0000 {
            /* raise inexact if x!=0 and underflow if subnormal */

            let x1p120 = f32::from_bits(0x_7b80_0000); // 0x1p120 == 2^120
            if ix < 0x_0010_0000 {
                force_eval!(x / x1p120);
            } else {
                force_eval!(x + x1p120);
            }
            return (x, 1.);
        }
        return (k_sinf(x as f64), k_cosf(x as f64));
    }

    /* |x| ~<= 5*pi/4 */
    if ix <= 0x_407b_53d1 {
        if ix <= 0x_4016_cbe3 {
            /* |x| ~<= 3pi/4 */
            if sign {
                s = -k_cosf((x + S1PIO2) as f64);
                c = k_sinf((x + S1PIO2) as f64);
            } else {
                s = k_cosf((S1PIO2 - x) as f64);
                c = k_sinf((S1PIO2 - x) as f64);
            }
        }
        /* -sin(x+c) is not correct if x+c could be 0: -0 vs +0 */
        else if sign {
            s = k_sinf((x + S2PIO2) as f64);
            c = k_cosf((x + S2PIO2) as f64);
        } else {
            s = k_sinf((x - S2PIO2) as f64);
            c = k_cosf((x - S2PIO2) as f64);
        }

        return (s, c);
    }

    /* |x| ~<= 9*pi/4 */
    if ix <= 0x_40e2_31d5 {
        if ix <= 0x_40af_eddf {
            /* |x| ~<= 7*pi/4 */
            if sign {
                s = k_cosf((x + S3PIO2) as f64);
                c = -k_sinf((x + S3PIO2) as f64);
            } else {
                s = -k_cosf((x - S3PIO2) as f64);
                c = k_sinf((x - S3PIO2) as f64);
            }
        } else if sign {
            s = k_cosf((x + S4PIO2) as f64);
            c = k_sinf((x + S4PIO2) as f64);
        } else {
            s = k_cosf((x - S4PIO2) as f64);
            c = k_sinf((x - S4PIO2) as f64);
        }

        return (s, c);
    }

    /* sin(Inf or NaN) is NaN */
    if ix >= UF_INF {
        return (f32::NAN, f32::NAN);
    }

    /* general argument reduction needed */
    let (n, y) = rem_pio2f(x);
    s = k_sinf(y);
    c = k_cosf(y);
    match n & 3 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        3 => (-c, s),
        #[cfg(feature = "checked")]
        _ => unreachable!(),
        #[cfg(not(feature = "checked"))]
        _ => (0., 1.),
    }
}
