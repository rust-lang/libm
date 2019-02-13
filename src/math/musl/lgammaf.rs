/* origin: FreeBSD /usr/src/lib/msun/src/e_lgammaf_r.c */
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

use super::{floorf::floorf, k_cosf, k_sinf, logf};
use crate::math::consts::*;
use core::f32;

const PI: f32 = 3.141_592_741; /* 0x_4049_0fdb */
const A0: f32 = 7.721_566_408_9_e-02; /* 0x_3d9e_233f */
const A1: f32 = 3.224_670_290_9_e-01; /* 0x_3ea5_1a66 */
const A2: f32 = 6.735_230_237_2_e-02; /* 0x_3d89_f001 */
const A3: f32 = 2.058_080_770_1_e-02; /* 0x_3ca8_9915 */
const A4: f32 = 7.385_550_998_2_e-03; /* 0x_3bf2_027e */
const A5: f32 = 2.890_513_744_2_e-03; /* 0x_3b3d_6ec6 */
const A6: f32 = 1.192_707_684_8_e-03; /* 0x_3a9c_54a1 */
const A7: f32 = 5.100_697_744_6_e-04; /* 0x_3a05_b634 */
const A8: f32 = 2.208_627_847_7_e-04; /* 0x_3967_9767 */
const A9: f32 = 1.080_115_689_5_e-04; /* 0x_38e2_8445 */
const A10: f32 = 2.521_445_640_0_e-05; /* 0x_37d3_83a2 */
const A11: f32 = 4.486_409_670_8_e-05; /* 0x_383c_2c75 */
const TC: f32 = 1.461_632_132_5; /* 0x_3fbb_16c3 */
const TF: f32 = -1.214_862_838_4_e-01; /* 0x_bdf8_cdcd */
/* TT = -(tail of TF) */
const TT: f32 = 6.697_100_651_8_e-09; /* 0x_31e6_1c52 */
const T0: f32 = 4.838_361_144_1_e-01; /* 0x_3ef7_b95e */
const T1: f32 = -1.475_877_165_8_e-01; /* 0x_be17_213c */
const T2: f32 = 6.462_494_283_9_e-02; /* 0x_3d84_5a15 */
const T3: f32 = -3.278_854_116_8_e-02; /* 0x_bd06_4d47 */
const T4: f32 = 1.797_067_560_3_e-02; /* 0x_3c93_373d */
const T5: f32 = -1.031_422_428_8_e-02; /* 0x_bc28_fcfe */
const T6: f32 = 6.100_538_652_4_e-03; /* 0x_3bc7_e707 */
const T7: f32 = -3.684_520_255_8_e-03; /* 0x_bb71_77fe */
const T8: f32 = 2.259_647_706_5_e-03; /* 0x_3b14_1699 */
const T9: f32 = -1.403_464_702_9_e-03; /* 0x_bab7_f476 */
const T10: f32 = 8.810_818_544_6_e-04; /* 0x_3a66_f867 */
const T11: f32 = -5.385_953_118_1_e-04; /* 0x_ba0d_3085 */
const T12: f32 = 3.156_320_599_4_e-04; /* 0x_39a5_7b6b */
const T13: f32 = -3.127_541_567_7_e-04; /* 0x_b9a3_f927 */
const T14: f32 = 3.355_291_846_7_e-04; /* 0x_39af_e9f7 */
const U0: f32 = -7.721_566_408_9_e-02; /* 0x_bd9e_233f */
const U1: f32 = 6.328_270_435_3_e-01; /* 0x_3f22_00f4 */
const U2: f32 = 1.454_922_556_9; /* 0x_3fba_3ae7 */
const U3: f32 = 9.777_175_188_1_e-01; /* 0x_3f7a_4bb2 */
const U4: f32 = 2.289_637_327_2_e-01; /* 0x_3e6a_7578 */
const U5: f32 = 1.338_109_187_8_e-02; /* 0x_3c5b_3c5e */
const V1: f32 = 2.455_977_916_7; /* 0x_401d_2ebe */
const V2: f32 = 2.128_489_732_7; /* 0x_4008_392d */
const V3: f32 = 7.692_851_424_2_e-01; /* 0x_3f44_efdf */
const V4: f32 = 1.042_226_478_5_e-01; /* 0x_3dd5_72af */
const V5: f32 = 3.217_092_482_4_e-03; /* 0x_3b52_d5db */
const S0: f32 = -7.721_566_408_9_e-02; /* 0x_bd9e_233f */
const S1: f32 = 2.149_824_202_1_e-01; /* 0x_3e5c_245a */
const S2: f32 = 3.257_787_823_7_e-01; /* 0x_3ea6_cc7a */
const S3: f32 = 1.463_504_731_7_e-01; /* 0x_3e15_dce6 */
const S4: f32 = 2.664_227_038_6_e-02; /* 0x_3cda_40e4 */
const S5: f32 = 1.840_284_559_9_e-03; /* 0x_3af1_35b4 */
const S6: f32 = 3.194_753_298_9_e-05; /* 0x_3805_ff67 */
const R1: f32 = 1.392_005_324_4; /* 0x_3fb2_2d3b */
const R2: f32 = 7.219_355_702_4_e-01; /* 0x_3f38_d0c5 */
const R3: f32 = 1.719_338_595_9_e-01; /* 0x_3e30_0f6e */
const R4: f32 = 1.864_591_985_9_e-02; /* 0x_3c98_bf54 */
const R5: f32 = 7.779_424_777_3_e-04; /* 0x_3a4b_eed6 */
const R6: f32 = 7.326_684_226_4_e-06; /* 0x_36f5_d7bd */
const W0: f32 = 4.189_385_473_7_e-01; /* 0x_3ed6_7f1d */
const W1: f32 = 8.333_333_581_7_e-02; /* 0x_3daa_aaab */
const W2: f32 = -2.777_777_845_0_e-03; /* 0x_bb36_0b61 */
const W3: f32 = 7.936_505_717_2_e-04; /* 0x_3a50_0cfd */
const W4: f32 = -5.951_875_355_1_e-04; /* 0x_ba1c_065c */
const W5: f32 = 8.363_398_956_1_e-04; /* 0x_3a5b_3dd2 */
const W6: f32 = -1.630_929_298_7_e-03; /* 0x_bad5_c4e8 */

/* sin(PI*x) assuming x > 2^-100, if sin(PI*x)==0 the sign is arbitrary */
fn sin_pi(mut x: f32) -> f32 {
    let mut y: f64;
    let mut n: isize;

    /* spurious inexact if odd int */
    x = 2. * (x * 0.5 - floorf(x * 0.5)); /* x mod 2. */

    n = (x * 4.) as isize;
    n = (n + 1) / 2;
    y = (x as f64) - (n as f64) * 0.5;
    y *= 3.141_592_653_589_793_238_46;
    match n {
        1 => k_cosf(y),
        2 => k_sinf(-y),
        3 => -k_cosf(y),
        0 | _ => k_sinf(y),
    }
}

pub fn lgammaf(x: f32) -> f32 {
    lgammaf_r(x).0
}

pub fn lgammaf_r(mut x: f32) -> (f32, isize) {
    let u = x.to_bits();
    let mut t: f32;
    let y: f32;
    let mut z: f32;
    let nadj: f32;
    let p: f32;
    let p1: f32;
    let p2: f32;
    let p3: f32;
    let q: f32;
    let mut r: f32;
    let w: f32;
    let ix: u32;
    let i: isize;
    let sign: bool;
    let mut signgam: isize;

    /* purge off +-inf, NaN, +-0, tiny and negative arguments */
    signgam = 1;
    sign = (u >> 31) != 0;
    ix = u & UF_ABS;
    if ix >= UF_INF {
        return (x * x, signgam);
    }
    if ix < 0x_3500_0000 {
        /* |x| < 2**-21, return -log(|x|) */
        if sign {
            signgam = -1;
            x = -x;
        }
        return (-logf(x), signgam);
    }
    if sign {
        x = -x;
        t = sin_pi(x);
        if t == 0. {
            /* -integer */
            return (f32::INFINITY, signgam);
        }
        if t > 0. {
            signgam = -1;
        } else {
            t = -t;
        }
        nadj = logf(PI / (t * x));
    } else {
        nadj = 0.;
    }

    /* purge off 1 and 2 */
    if ix == 0x_3f80_0000 || ix == 0x_4000_0000 {
        r = 0.;
    }
    /* for x < 2. */
    else if ix < 0x_4000_0000 {
        if ix <= 0x_3f66_6666 {
            /* lgamma(x) = lgamma(x+1)-log(x) */
            r = -logf(x);
            if ix >= 0x_3f3b_4a20 {
                y = 1. - x;
                i = 0;
            } else if ix >= 0x_3e6d_3308 {
                y = x - (TC - 1.);
                i = 1;
            } else {
                y = x;
                i = 2;
            }
        } else {
            r = 0.;
            if ix >= 0x_3fdd_a618 {
                /* [1.7316,2] */
                y = 2. - x;
                i = 0;
            } else if ix >= 0x_3f9d_a620 {
                /* [1.23,1.73] */
                y = x - TC;
                i = 1;
            } else {
                y = x - 1.;
                i = 2;
            }
        }
        match i {
            0 => {
                z = y * y;
                p1 = A0 + z * (A2 + z * (A4 + z * (A6 + z * (A8 + z * A10))));
                p2 = z * (A1 + z * (A3 + z * (A5 + z * (A7 + z * (A9 + z * A11)))));
                p = y * p1 + p2;
                r += p - 0.5 * y;
            }
            1 => {
                z = y * y;
                w = z * y;
                p1 = T0 + w * (T3 + w * (T6 + w * (T9 + w * T12))); /* parallel comp */
                p2 = T1 + w * (T4 + w * (T7 + w * (T10 + w * T13)));
                p3 = T2 + w * (T5 + w * (T8 + w * (T11 + w * T14)));
                p = z * p1 - (TT - w * (p2 + y * p3));
                r += TF + p;
            }
            2 => {
                p1 = y * (U0 + y * (U1 + y * (U2 + y * (U3 + y * (U4 + y * U5)))));
                p2 = 1. + y * (V1 + y * (V2 + y * (V3 + y * (V4 + y * V5))));
                r += -0.5 * y + p1 / p2;
            }
            #[cfg(feature = "checked")]
            _ => unreachable!(),
            #[cfg(not(feature = "checked"))]
            _ => {}
        }
    } else if ix < 0x_4100_0000 {
        /* x < 8. */
        i = x as isize;
        y = x - (i as f32);
        p = y * (S0 + y * (S1 + y * (S2 + y * (S3 + y * (S4 + y * (S5 + y * S6))))));
        q = 1. + y * (R1 + y * (R2 + y * (R3 + y * (R4 + y * (R5 + y * R6)))));
        r = 0.5 * y + p / q;
        z = 1.; /* lgamma(1+s) = log(s) + lgamma(s) */
        // TODO: In C, this was implemented using switch jumps with fallthrough.
        // Does this implementation have performance problems?
        if i >= 7 {
            z *= y + 6.;
        }
        if i >= 6 {
            z *= y + 5.;
        }
        if i >= 5 {
            z *= y + 4.;
        }
        if i >= 4 {
            z *= y + 3.;
        }
        if i >= 3 {
            z *= y + 2.;
            r += logf(z);
        }
    } else if ix < 0x_5c80_0000 {
        /* 8. <= x < 2**58 */
        t = logf(x);
        z = 1. / x;
        y = z * z;
        w = W0 + z * (W1 + y * (W2 + y * (W3 + y * (W4 + y * (W5 + y * W6)))));
        r = (x - 0.5) * (t - 1.) + w;
    } else {
        /* 2**58 <= x <= inf */
        r = x * (logf(x) - 1.);
    }
    if sign {
        r = nadj - r;
    }
    (r, signgam)
}
