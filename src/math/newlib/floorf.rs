/* sf_floor.c -- float version of s_floor.c.
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

const HUGE: f32 = 1.0e30;

/// Return x rounded toward -inf to integral value
///
/// Method:
/// Bit twiddling.
/// Exception:
/// Inexact flag raised if x not equal to floorf(x).
#[inline]
pub fn floorf(x: f32) -> f32 {
    let mut i0 = x.to_bits() as i32;
    let ix = (i0 & 0x7fffffff) as u32;
    let j0 = ((ix >> 23) - 0x7f) as i32;
    if j0 < 23 {
        if j0 < 0 {
            /* raise inexact if x != 0 */
            if HUGE + x > 0. {
                /* return 0*sign(x) if |x|<1 */
                if i0 >= 0 {
                    i0 = 0;
                } else if !(ix == 0) {
                    i0 = 0xbf800000 as i32;
                }
            }
        } else {
            let i = ((0x007fffff) >> j0) as u32;
            if (i0 & (i as i32)) == 0 {
                /* x is integral */
                return x;
            }
            if HUGE + x > 0. {
                /* raise inexact flag */
                if i0 < 0 {
                    i0 += (0x00800000) >> j0;
                }
                i0 &= (!i) as i32;
            }
        }
    } else {
        return if !(ix < 0x7f800000) {
            /* inf or NaN */
            x + x
        } else {
            x /* x is integral */
        };
    }
    f32::from_bits(i0 as u32)
}
