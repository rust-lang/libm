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
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn floorf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.floor` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { core::intrinsics::floorf32(x) }
        }
    }

    use super::fdlibm::{FLT_UWORD_IS_FINITE, FLT_UWORD_IS_ZERO};

    let mut i0: u32 = x.to_bits();
    let ix: u32 = i0 & 0x7fff_ffff;
    let j0: i32 = (ix >> 23) as i32 - 0x7f;
    let sign: bool = (i0 >> 31) != 0;

    if j0 < 23 {
        if j0 < 0 {
            /* raise inexact if x != 0 */
            if HUGE + x > 0.0 {
                /* return 0*sign(x) if |x|<1 */
                if !sign {
                    i0 = 0;
                } else if !FLT_UWORD_IS_ZERO(ix) {
                    i0 = 0xbf80_0000;
                }
            }
        } else {
            let i = 0x007f_ffff >> j0;
            if i0 & i == 0 {
                return x; /* x is integral */
            }
            if HUGE + x > 0.0 {
                /* raise inexact flag */
                if sign {
                    i0 += 0x0080_0000 >> j0;
                }
                i0 &= !i;
            }
        }
    } else {
        if !FLT_UWORD_IS_FINITE(ix) {
            return x + x; /* inf or NaN */
        } else {
            return x; /* x is integral */
        }
    }
    f32::from_bits(i0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_overflow() {
        assert_eq!(super::floorf(0.5), 0.0);
    }
}
