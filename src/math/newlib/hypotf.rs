/* ef_hypot.c -- float version of e_hypot.c.
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

use math::consts::*;
use math::sqrtf;

#[inline]
pub fn hypotf(x: f32, y: f32) -> f32 {
    let mut ha = (x.to_bits() as i32) & IF_ABS;
    let mut hb = (y.to_bits() as i32) & IF_ABS;
    if hb > ha {
        core::mem::swap(&mut ha, &mut hb);
    }
    let mut a = f32::from_bits(ha as u32); /* a <- |a| */
    let mut b = f32::from_bits(hb as u32); /* b <- |b| */
    if (ha - hb) > 0x_0f00_0000 {
        /* x/y > 2**30 */
        return a + b;
    }
    let mut k = 0i32;
    if ha > 0x_5880_0000 {
        /* a>2**50 */
        if ha >= IF_INF {
            /* Inf or NaN */
            return if ha == IF_INF {
                a
            } else if hb == IF_INF {
                b
            } else {
                a + b /* for sNaN */
            };
        }
        /* scale a and b by 2**-68 */
        ha -= 0x_2200_0000;
        hb -= 0x_2200_0000;
        k += 68;
        a = f32::from_bits(ha as u32);
        b = f32::from_bits(hb as u32);
    }
    if hb < 0x_2680_0000 {
        /* b < 2**-50 */
        if hb == 0 {
            return a;
        } else if hb < IF_MIN {
            let t1 = f32::from_bits(0x_7e80_0000); /* t1=2^126 */
            b *= t1;
            a *= t1;
            k -= 126;
        } else {
            /* scale a and b by 2^68 */
            ha += 0x_2200_0000; /* a *= 2^68 */
            hb += 0x_2200_0000; /* b *= 2^68 */
            k -= 68;
            a = f32::from_bits(ha as u32);
            b = f32::from_bits(hb as u32);
        }
    }
    /* medium size a and b */
    let w = a - b;
    let w = if w > b {
        let t1 = f32::from_bits((ha as u32) & 0x_ffff_f000);
        let t2 = a - t1;
        sqrtf(t1 * t1 - (b * (-b) - t2 * (a + t1)))
    } else {
        a += a;
        let y1 = f32::from_bits((hb as u32) & 0x_ffff_f000);
        let y2 = b - y1;
        let t1 = f32::from_bits((ha as u32) + UF_MIN);
        let t2 = a - t1;
        sqrtf(t1 * y1 - (w * (-w) - (t1 * y2 + t2 * b)))
    };
    if k != 0 {
        w * f32::from_bits(UF_1 + ((k as u32) << 23))
    } else {
        w
    }
}
