/* origin: FreeBSD /usr/src/lib/msun/src/s_cbrtf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Debugged and optimized by Bruce D. Evans.
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
/* cbrtf(x)
 * Return cube root of x
 */

use core::f32;
use math::consts::*;

const B1: u32 = 709_958_130; /* B1 = (127-127.0/3-0.03306235651)*2**23 */
const B2: u32 = 642_849_266; /* B2 = (127-127.0/3-24/3-0.03306235651)*2**23 */

#[inline]
pub fn cbrtf(x: f32) -> f32 {
    let x1p24 = f32::from_bits(0x_4b80_0000); // 0x1p24f === 2 ^ 24

    let mut r: f64;
    let mut t: f64;
    let mut ui: u32 = x.to_bits();
    let mut hx: u32 = ui & UF_ABS;

    if hx >= UF_INF {
        /* cbrt(NaN,INF) is itself */
        return x + x;
    }

    /* rough cbrt to 5 bits */
    if hx < UF_MIN {
        /* zero or subnormal? */
        if hx == 0 {
            return x; /* cbrt(+-0) is itself */
        }
        ui = (x * x1p24).to_bits();
        hx = ui & UF_ABS;
        hx = hx / 3 + B2;
    } else {
        hx = hx / 3 + B1;
    }
    ui &= UF_SIGN;
    ui |= hx;

    /*
     * First step Newton iteration (solving t*t-x/t == 0) to 16 bits.  In
     * double precision so that its terms can be arranged for efficiency
     * without causing overflow or underflow.
     */
    t = f32::from_bits(ui) as f64;
    r = t * t * t;
    t = t * (x as f64 + x as f64 + r) / (x as f64 + r + r);

    /*
     * Second step Newton iteration to 47 bits.  In double precision for
     * efficiency and accuracy.
     */
    r = t * t * t;
    t = t * (x as f64 + x as f64 + r) / (x as f64 + r + r);

    /* rounding to 24 bits is perfect in round-to-nearest mode */
    t as f32
}
