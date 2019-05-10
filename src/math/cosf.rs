/* origin: FreeBSD /usr/src/lib/msun/src/s_cosf.c */
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
use core::f64::consts::FRAC_PI_2;

/* Small multiples of pi/2 rounded to double precision. */
const C1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x_3FF9_21FB, 0x_5444_2D18 */
const C2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x_4009_21FB, 0x_5444_2D18 */
const C3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x_4012_D97C, 0x_7F33_21D2 */
const C4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x_4019_21FB, 0x_5444_2D18 */

const UF_1_PI_4: u32 = 0x_3f49_0fdb;
const UF_3_PI_4: u32 = 0x_4016_cbe4;
const UF_5_PI_4: u32 = 0x_407b_53d1;
const UF_7_PI_4: u32 = 0x_40af_eddf;
const UF_9_PI_4: u32 = 0x_40e2_31d6;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn cosf(x: f32) -> f32 {
    let x64 = x as f64;

    let x1p120 = f32::from_bits(0x_7b80_0000); // 0x1p120f === 2 ^ 120

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= UF_ABS;

    if ix < UF_1_PI_4 {
        /* |x| ~<= pi/4 */
        if ix < 0x_3980_0000 {
            /* |x| < 2**-12 */
            /* raise inexact if x != 0 */
            force_eval!(x + x1p120);
            1.
        } else {
            k_cosf(x64)
        }
    } else if ix <= UF_5_PI_4 {
        /* |x| ~<= 5*pi/4 */
        if ix >= UF_3_PI_4 {
            /* |x|  ~> 3*pi/4 */
            -k_cosf(if sign { x64 + C2_PIO2 } else { x64 - C2_PIO2 })
        } else if sign {
            k_sinf(x64 + C1_PIO2)
        } else {
            k_sinf(C1_PIO2 - x64)
        }
    } else if ix < UF_9_PI_4 {
        /* |x| ~<= 9*pi/4 */
        if ix > UF_7_PI_4 {
            /* |x| ~> 7*pi/4 */
            k_cosf(if sign { x64 + C4_PIO2 } else { x64 - C4_PIO2 })
        } else if sign {
            k_sinf(-x64 - C3_PIO2)
        } else {
            k_sinf(x64 - C3_PIO2)
        }
    } else if ix >= UF_INF {
        /* cos(Inf or NaN) is NaN */
        x - x
    } else {
        /* general argument reduction needed */
        let (n, y) = rem_pio2f(x);
        match n & 3 {
            0 => k_cosf(y),
            1 => k_sinf(-y),
            2 => -k_cosf(y),
            _ => k_sinf(y),
        }
    }
}
