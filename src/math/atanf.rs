/* origin: FreeBSD /usr/src/lib/msun/src/s_atanf.c */
/*
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

use super::fabsf;
use math::consts::*;

const ATAN_HI: [f32; 4] = [
    4.636_476_039_9_e-01, /* atan(0.5)hi 0x_3eed_6338 */
    7.853_981_256_5_e-01, /* atan(1.0)hi 0x_3f49_0fda */
    9.827_936_887_7_e-01, /* atan(1.5)hi 0x_3f7b_985e */
    1.570_796_251_3, /* atan(inf)hi 0x_3fc9_0fda */
];

const ATAN_LO: [f32; 4] = [
    5.012_158_244_e-09,   /* atan(0.5)lo 0x_31ac_3769 */
    3.774_894_707_9_e-08, /* atan(1.0)lo 0x_3322_2168 */
    3.447_321_717_e-08,   /* atan(1.5)lo 0x_3314_0fb4 */
    7.549_789_415_9_e-08, /* atan(inf)lo 0x_33a2_2168 */
];

const A_T: [f32; 5] = [
    3.333_332_836_6_e-01,
    -1.999_915_838_2_e-01,
    1.425_363_570_5_e-01,
    -1.064_801_737_7_e-01,
    6.168_760_731_8_e-02,
];

#[inline]
pub fn atanf(mut x: f32) -> f32 {
    let x1p_120 = f32::from_bits(0x_0380_0000); // 0x1p-120 === 2 ^ (-120)

    let z: f32;

    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= UF_ABS;

    if ix >= 0x_4c80_0000 {
        /* if |x| >= 2**26 */
        if x.is_nan() {
            return x;
        }
        z = ATAN_HI[3] + x1p_120;
        return if sign { -z } else { z };
    }
    let id = if ix < 0x_3ee0_0000 {
        /* |x| < 0.4375 */
        if ix < 0x_3980_0000 {
            /* |x| < 2**-12 */
            if ix < UF_MIN {
                /* raise underflow for subnormal x */
                force_eval!(x * x);
            }
            return x;
        }
        -1
    } else {
        x = fabsf(x);
        if ix < 0x_3f98_0000 {
            /* |x| < 1.1875 */
            if ix < 0x_3f30_0000 {
                /*  7/16 <= |x| < 11/16 */
                x = (2. * x - 1.) / (2. + x);
                0
            } else {
                /* 11/16 <= |x| < 19/16 */
                x = (x - 1.) / (x + 1.);
                1
            }
        } else if ix < 0x_401c_0000 {
            /* |x| < 2.4375 */
            x = (x - 1.5) / (1. + 1.5 * x);
            2
        } else {
            /* 2.4375 <= |x| < 2**26 */
            x = -1. / x;
            3
        }
    };
    /* end of argument reduction */
    z = x * x;
    let w = z * z;
    /* break sum from i=0 to 10 aT[i]z**(i+1) into odd and even poly */
    let s1 = z * (A_T[0] + w * (A_T[2] + w * A_T[4]));
    let s2 = w * (A_T[1] + w * A_T[3]);
    if id < 0 {
        return x - x * (s1 + s2);
    }
    let id = id as usize;
    let z = ATAN_HI[id] - ((x * (s1 + s2) - ATAN_LO[id]) - x);
    if sign {
        -z
    } else {
        z
    }
}
