/* origin: FreeBSD /usr/src/lib/msun/src/s_sinf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Optimized by Bruce D. Evans.
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

use super::{k_cosf, k_sinf, rem_pio2f};

use core::f64::consts::FRAC_PI_2;

/* Small multiples of pi/2 rounded to double precision. */
const S1_PIO2: f64 = 1. * FRAC_PI_2; /* 0x_3FF9_21FB, 0x_5444_2D18 */
const S2_PIO2: f64 = 2. * FRAC_PI_2; /* 0x_4009_21FB, 0x_5444_2D18 */
const S3_PIO2: f64 = 3. * FRAC_PI_2; /* 0x_4012_D97C, 0x_7F33_21D2 */
const S4_PIO2: f64 = 4. * FRAC_PI_2; /* 0x_4019_21FB, 0x_5444_2D18 */

const UF_INF: u32 = 0x_7f80_0000;
const UF_1_PI_4: u32 = 0x_3f49_0fdb;
const UF_3_PI_4: u32 = 0x_4016_cbe4;
const UF_5_PI_4: u32 = 0x_407b_53d1;
const UF_7_PI_4: u32 = 0x_40af_eddf;
const UF_9_PI_4: u32 = 0x_40e2_31d6;

#[inline]
pub fn sinf(x: f32) -> f32 {
    let x64 = x as f64;

    let x1p120 = f32::from_bits(0x_7b80_0000); // 0x1p120f === 2 ^ 120

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x_7fff_ffff;

    if ix < UF_1_PI_4 {
        /* |x| ~<= pi/4 */
        if ix < 0x_3980_0000 {
            /* |x| < 2**-12 */
            /* raise inexact if x!=0 and underflow if subnormal */
            force_eval!(if ix < 0x_0080_0000 {
                x / x1p120
            } else {
                x + x1p120
            });
            return x;
        }
        return k_sinf(x64);
    }
    if ix <= UF_5_PI_4 {
        /* |x| ~<= 5*pi/4 */
        if ix < UF_3_PI_4 {
            /* |x| ~<= 3pi/4 */
            if sign {
                return -k_cosf(x64 + S1_PIO2);
            } else {
                return k_cosf(x64 - S1_PIO2);
            }
        }
        return k_sinf(if sign {
            -(x64 + S2_PIO2)
        } else {
            -(x64 - S2_PIO2)
        });
    }
    if ix < UF_9_PI_4 {
        /* |x| ~<= 9*pi/4 */
        if ix <= UF_7_PI_4 {
            /* |x| ~<= 7*pi/4 */
            if sign {
                return k_cosf(x64 + S3_PIO2);
            } else {
                return -k_cosf(x64 - S3_PIO2);
            }
        }
        return k_sinf(if sign { x64 + S4_PIO2 } else { x64 - S4_PIO2 });
    }

    /* sin(Inf or NaN) is NaN */
    if ix >= UF_INF {
        return x - x;
    }

    /* general argument reduction needed */
    let (n, y) = rem_pio2f(x);
    match n & 3 {
        0 => k_sinf(y),
        1 => k_cosf(y),
        2 => k_sinf(-y),
        _ => -k_cosf(y),
    }
}
