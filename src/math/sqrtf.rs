/* ef_sqrtf.c -- float version of e_sqrt.c.
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

const ONE: f32 = 1.0;
const TINY: f32 = 1.0e-30;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn sqrtf(x: f32) -> f32 {
    use super::fdlibm::{FLT_UWORD_IS_FINITE, FLT_UWORD_IS_SUBNORMAL, FLT_UWORD_IS_ZERO};
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.sqrt` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return if x < 0.0 {
                core::f32::NAN
            } else {
                unsafe { core::intrinsics::sqrtf32(x) }
            }
        }
    }

    let mut z: f32;

    let mut r: u32;
    let hx: u32;

    let mut ix: i32;
    let mut s: i32;
    let mut q: i32;
    let mut m: i32;
    let mut t: i32;
    let mut i: i32;

    ix = x.to_bits() as i32;
    hx = ix as u32 & 0x7fff_ffff;

    /* take care of Inf and NaN */
    if !FLT_UWORD_IS_FINITE(hx) {
        return x * x + x; /* sqrt(NaN)=NaN, sqrt(+inf)=+inf, sqrt(-inf)=sNaN */
    }

    /* take care of zero and -ves */
    if FLT_UWORD_IS_ZERO(hx) {
        return x; /* sqrt(+-0) = +-0 */
    }
    if ix < 0 {
        return (x - x) / (x - x); /* sqrt(-ve) = sNaN */
    }

    /* normalize x */
    m = ix >> 23;
    if FLT_UWORD_IS_SUBNORMAL(hx) {
        /* subnormal x */
        i = 0;
        while ix & 0x0080_0000 == 0 {
            ix <<= 1;
            i += 1;
        }
        m -= i - 1;
    }
    m -= 127; /* unbias exponent */
    ix = (ix & 0x007f_ffff) | 0x0080_0000;
    /* odd m, double x to make it even */
    if m & 1 == 1 {
        ix += ix;
    }
    m >>= 1; /* m = [m/2] */

    /* generate sqrt(x) bit by bit */
    ix += ix;
    q = 0;
    s = 0; /* q = sqrt(x) */
    r = 0x0100_0000; /* r = moving bit from right to left */

    while r != 0 {
        t = s + r as i32;
        if t <= ix {
            s = t + r as i32;
            ix -= t;
            q += r as i32;
        }
        ix += ix;
        r >>= 1;
    }

    /* use floating add to find out rounding direction */
    if ix != 0 {
        z = ONE - TINY; /* trigger inexact flag */
        if z >= ONE {
            z = ONE + TINY;
            if z > ONE {
                q += 2;
            } else {
                q += q & 1;
            }
        }
    }
    ix = (q >> 1) + 0x3f00_0000;
    ix += m << 23;
    f32::from_bits(ix as u32)
}
