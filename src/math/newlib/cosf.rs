/* sf_cos.c -- float version of s_cos.c.
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

use core::f32;
use super::{k_cosf, k_sinf, rem_pio2f};
use crate::math::consts::*;

#[inline]
pub fn cosf(x: f32) -> f32 {
    let mut ix = x.to_bits();
    ix &= UF_ABS;

    /* |x| ~< pi/4 */
    if ix <= 0x_3f49_0fd8 {
        k_cosf(x, 0.)
    } else if ix >= UF_INF {
        /* cos(Inf or NaN) is NaN */
        f32::NAN
    } else {
        /* argument reduction needed */
        let (n, y0, y1) = rem_pio2f(x);
        match n & 3 {
            0 => k_cosf(y0, y1),
            1 => -k_sinf(y0, y1, true),
            2 => -k_cosf(y0, y1),
            _ => k_sinf(y0, y1, true),
        }
    }
}
