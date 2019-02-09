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

use core::f64::consts::FRAC_PI_2;

/* Small multiples of pi/2 rounded to double precision. */
const C1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x3FF921FB, 0x54442D18 */
const C2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x400921FB, 0x54442D18 */
const C3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x4012D97C, 0x7F3321D2 */
const C4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x401921FB, 0x54442D18 */

const UF_INF: u32 = 0x7f800000;
const UF_1_PI_4: u32 = 0x3f490fdb;
const UF_3_PI_4: u32 = 0x4016cbe4;
const UF_5_PI_4: u32 = 0x407b53d1;
const UF_7_PI_4: u32 = 0x40afeddf;
const UF_9_PI_4: u32 = 0x40e231d6;

#[inline]
pub fn cosf(x: f32) -> f32 {
    let x64 = x as f64;

    let x1p120 = f32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    if ix < UF_1_PI_4 {
        /* |x| ~<= pi/4 */
        if ix < 0x39800000 {
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
        } else {
            if sign {
                k_sinf(x64 + C1_PIO2)
            } else {
                k_sinf(C1_PIO2 - x64)
            }
        }
    } else if ix < UF_9_PI_4 {
        /* |x| ~<= 9*pi/4 */
        if ix > UF_7_PI_4 {
            /* |x| ~> 7*pi/4 */
            k_cosf(if sign { x64 + C4_PIO2 } else { x64 - C4_PIO2 })
        } else {
            if sign {
                k_sinf(-x64 - C3_PIO2)
            } else {
                k_sinf(x64 - C3_PIO2)
            }
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
