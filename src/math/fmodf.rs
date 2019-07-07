/* ef_fmod.c -- float version of e_fmod.c.
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

const ONE: f32 = 1.0f32;
const ZERO: [f32; 2] = [0.0f32, -0.0];

/// Return `x mod y` in exact arithmetic.
///
/// Method: shift and subtract
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    use super::fdlibm::{
        FLT_UWORD_IS_FINITE, FLT_UWORD_IS_NAN, FLT_UWORD_IS_SUBNORMAL, FLT_UWORD_IS_ZERO,
    };
    const U32_BITS: u8 = 32;
    let mut n: i32;
    let mut hx: u32;
    let mut hy: u32;
    let mut hz: i32;
    let mut ix: i32;
    let mut iy: i32;
    let sx: u32;
    let mut i: i32;

    hx = x.to_bits();
    sx = hx & 0x8000_0000; /* sign of x */
    hx ^= sx; /* |x| */
    hy = y.to_bits() & 0x7fff_ffff; /* |y| */

    /* purge off exception values */
    if FLT_UWORD_IS_ZERO(hy) || !FLT_UWORD_IS_FINITE(hx) || FLT_UWORD_IS_NAN(hy) {
        return (x * y) / (x * y);
    }
    if hx < hy {
        return x; /* |x|<|y| return x */
    }
    if hx == hy {
        return ZERO[sx as usize >> (U32_BITS - 1)]; /* |x|=|y| return x*0 */
    }

    /* Note: y cannot be zero if we reach here. */

    /* determine ix = ilogb(x) */
    if FLT_UWORD_IS_SUBNORMAL(hx) {
        /* subnormal x */
        ix = -126;
        i = (hx << 8) as i32;
        while i > 0 {
            ix -= 1;
            i <<= 1;
        }
    } else {
        ix = (hx >> 23) as i32 - 127;
    }

    /* determine iy = ilogb(y) */
    if FLT_UWORD_IS_SUBNORMAL(hy) {
        /* subnormal y */
        iy = -126;
        i = (hy << 8) as i32;
        while i >= 0 {
            iy -= 1;
            i <<= 1;
        }
    } else {
        iy = (hy >> 23) as i32 - 127;
    }

    /* set up {hx,lx}, {hy,ly} and align y to x */
    if ix >= -126 {
        hx = 0x0080_0000 | (0x007f_ffff & hx);
    } else {
        /* subnormal x, shift x to normal */
        n = -126 - ix;
        hx <<= n;
    }
    if iy >= -126 {
        hy = 0x0080_0000 | (0x007f_ffff & hy);
    } else {
        /* subnormal y, shift y to normal */
        n = -126 - iy;
        hy <<= n;
    }

    /* fix point fmod */
    n = ix - iy;
    while n != 0 {
        hz = hx.wrapping_sub(hy) as i32;
        if hz < 0 {
            hx += hx;
        } else {
            if hz == 0 {
                /* return sign(x)*0 */
                return ZERO[sx as usize >> (U32_BITS - 1)];
            }
            hx = (hz + hz) as u32;
        }
        n -= 1;
    }
    hz = hx.wrapping_sub(hy) as i32;
    if hz >= 0 {
        hx = hz as u32;
    }

    /* convert back to floating value and restore the sign */
    if hx == 0 {
        /* return sign(x)*0 */
        return ZERO[sx as usize >> (U32_BITS - 1)];
    }
    while hx < 0x0080_0000 {
        /* normalize x */
        hx += hx;
        iy -= 1;
    }
    if iy >= -126 {
        /* normalize output */
        hx = (hx.wrapping_sub(0x0080_0000)) | ((iy + 127) << 23) as u32;
        f32::from_bits(hx | sx)
    } else {
        /* subnormal output */
        /* If denormals are not supported, this code will generate a zero representation.  */
        n = -126 - iy;
        hx >>= n;
        /* create necessary signal */
        f32::from_bits(hx | sx) * ONE
    }
}
