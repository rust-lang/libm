/* sf_tan.c -- float version of s_tan.c.
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
use super::{k_tanf, rem_pio2f};
use crate::math::consts::*;

#[inline]
pub fn tanf(x: f32) -> f32 {
    let mut ix = x.to_bits();
    ix &= UF_ABS;

    /* |x| ~< pi/4 */
    if ix <= 0x_3f49_0fda {
        k_tanf(x, 0., 1)
    } else if ix >= UF_INF {
        /* tan(Inf or NaN) is NaN */
        f32::NAN /* NaN */
    /* argument reduction needed */
    } else {
        let (n, y0, y1) = rem_pio2f(x);
        k_tanf(y0, y1, 1 - ((n & 1) << 1)) /*   1 -- n even
                                           -1 -- n odd */
    }
}
