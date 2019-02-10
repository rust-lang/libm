/* origin: FreeBSD /usr/src/lib/msun/src/s_sin.c */
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

use core::f64;
use super::{get_high_word, k_cos, k_sin, rem_pio2};
use math::consts::*;

pub fn sincos(x: f64) -> (f64, f64)
{
    let s: f64;
    let c: f64;
    let mut ix: u32;

    ix = get_high_word(x);
    ix &= UF_ABS;

    /* |x| ~< pi/4 */
    if ix <= 0x_3fe9_21fb {
        /* if |x| < 2**-27 * sqrt(2) */
        if ix < 0x_3e46_a09e {
            /* raise inexact if x!=0 and underflow if subnormal */
            let x1p120 = f64::from_bits(0x_4770_0000_0000_0000); // 0x1p120 == 2^120
            if ix < 0x_0010_0000 {
                force_eval!(x/x1p120);
            } else {
                force_eval!(x+x1p120);
            }
            return (x, 1.0);
        }
        return (k_sin(x, 0.0, 0), k_cos(x, 0.0));
    }

    /* sincos(Inf or NaN) is NaN */
    if ix >= 0x_7ff0_0000 {
        return (f64::NAN, f64::NAN);
    }

    /* argument reduction needed */
    let (n, y0, y1) = rem_pio2(x);
    s = k_sin(y0, y1, 1);
    c = k_cos(y0, y1);
    match n&3 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        3 => (-c, s),
        #[cfg(feature = "checked")]
        _ => unreachable!(),
        #[cfg(not(feature = "checked"))]
        _ => (0.0, 1.0),
    }
}
