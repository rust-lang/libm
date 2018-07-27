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

use super::{k_tanf, rem_pio2f};

#[inline]
pub fn tanf(x: f32) -> f32 {
    let z = 0f32;

    let mut ix = x.to_bits() as i32;

    /* |x| ~< pi/4 */
    ix &= 0x7fffffff;
    if ix <= 0x3f490fda {
        k_tanf(x, z, 1)
    } else if !(ix < 0x7f800000) {
        /* tan(Inf or NaN) is NaN */
        x - x /* NaN */
    /* argument reduction needed */
    } else {
        let (n, y0, y1) = rem_pio2f(x);
        k_tanf(y0, y1, 1 - ((n & 1) << 1)) /*   1 -- n even
                                  -1 -- n odd */
    }
}
