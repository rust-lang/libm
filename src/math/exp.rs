/* origin: FreeBSD /usr/src/lib/msun/src/e_exp.c */
/*
 * ====================================================
 * Copyright (C) 2004 by Sun Microsystems, Inc. All rights reserved.
 *
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* exp(x)
 * Returns the exponential of x.
 *
 * Method
 *   1. Argument reduction:
 *      Reduce x to an r so that |r| <= 0.5*ln2 ~ 0.34658.
 *      Given x, find r and integer k such that
 *
 *               x = k*ln2 + r,  |r| <= 0.5*ln2.
 *
 *      Here r will be represented as r = hi-lo for better
 *      accuracy.
 *
 *   2. Approximation of exp(r) by a special rational function on
 *      the interval [0,0.34658]:
 *      Write
 *          R(r**2) = r*(exp(r)+1)/(exp(r)-1) = 2 + r*r/6 - r**4/360 + ...
 *      We use a special Remez algorithm on [0,0.34658] to generate
 *      a polynomial of degree 5 to approximate R. The maximum error
 *      of this polynomial approximation is bounded by 2**-59. In
 *      other words,
 *          R(z) ~ 2.0 + P1*z + P2*z**2 + P3*z**3 + P4*z**4 + P5*z**5
 *      (where z=r*r, and the values of P1 to P5 are listed below)
 *      and
 *          |                  5          |     -59
 *          | 2.0+P1*z+...+P5*z   -  R(z) | <= 2
 *          |                             |
 *      The computation of exp(r) thus becomes
 *                              2*r
 *              exp(r) = 1 + ----------
 *                            R(r) - r
 *                                 r*c(r)
 *                     = 1 + r + ----------- (for better accuracy)
 *                                2 - c(r)
 *      where
 *                              2       4             10
 *              c(r) = r - (P1*r  + P2*r  + ... + P5*r   ).
 *
 *   3. Scale back to obtain exp(x):
 *      From step 1, we have
 *         exp(x) = 2^k * exp(r)
 *
 * Special cases:
 *      exp(INF) is INF, exp(NaN) is NaN;
 *      exp(-INF) is 0, and
 *      for finite argument, only exp(0)=1 is exact.
 *
 * Accuracy:
 *      according to an error analysis, the error is always less than
 *      1 ulp (unit in the last place).
 *
 * Misc. info.
 *      For IEEE double
 *          if x >  709.782_712_893_383_973_096 then exp(x) overflows
 *          if x < -745.133_219_101_941_108_420 then exp(x) underflows
 */

use super::scalbn;

const HALF: [f64; 2] = [0.5, -0.5];
const LN2HI: f64 = 6.931_471_803_691_238_164_90_e-01; /* 0x_3fe6_2e42, 0x_fee0_0000 */
const LN2LO: f64 = 1.908_214_929_270_587_700_02_e-10; /* 0x_3dea_39ef, 0x_3579_3c76 */
const INVLN2: f64 = 1.442_695_040_888_963_387; /* 0x_3ff7_1547, 0x_652b_82fe */
const P1: f64 = 1.666_666_666_666_660_190_37_e-01; /* 0x_3FC5_5555, 0x_5555_553E */
const P2: f64 = -2.777_777_777_701_559_338_42_e-03; /* 0x_BF66_C16C, 0x_16BE_BD93 */
const P3: f64 = 6.613_756_321_437_934_361_17_e-05; /* 0x_3F11_566A, 0x_AF25_DE2C */
const P4: f64 = -1.653_390_220_546_525_153_90_e-06; /* 0x_BEBB_BD41, 0x_C5D2_6BF1 */
const P5: f64 = 4.138_136_797_057_238_460_39_e-08; /* 0x_3E66_3769, 0x_72BE_A4D0 */

#[inline]
pub fn exp(mut x: f64) -> f64 {
    let x1p1023 = f64::from_bits(0x_7fe0_0000_0000_0000); // 0x1p1023 === 2 ^ 1023
    let x1p_149 = f64::from_bits(0x_36a0_0000_0000_0000); // 0x1p-149 === 2 ^ -149

    let hi: f64;
    let lo: f64;
    let c: f64;
    let xx: f64;
    let y: f64;
    let k: i32;
    let sign: i32;
    let mut hx: u32;

    hx = (x.to_bits() >> 32) as u32;
    sign = (hx >> 31) as i32;
    hx &= 0x_7fff_ffff; /* high word of |x| */

    /* special cases */
    if hx >= 0x_4086_232b {
        /* if |x| >= 708.39... */
        if x.is_nan() {
            return x;
        }
        if x > 709.782_712_893_383_973_096 {
            /* overflow if x!=inf */
            x *= x1p1023;
            return x;
        }
        if x < -708.396_418_532_264_106_22 {
            /* underflow if x!=-inf */
            force_eval!((-x1p_149 / x) as f32);
            if x < -745.133_219_101_941_108_42 {
                return 0.;
            }
        }
    }

    /* argument reduction */
    if hx > 0x_3fd6_2e42 {
        /* if |x| > 0.5 ln2 */
        if hx >= 0x_3ff0_a2b2 {
            /* if |x| >= 1.5 ln2 */
            k = (INVLN2 * x + HALF[sign as usize]) as i32;
        } else {
            k = 1 - sign - sign;
        }
        hi = x - k as f64 * LN2HI; /* k*ln2hi is exact here */
        lo = k as f64 * LN2LO;
        x = hi - lo;
    } else if hx > 0x_3e30_0000 {
        /* if |x| > 2**-28 */
        k = 0;
        hi = x;
        lo = 0.;
    } else {
        /* inexact if x!=0 */
        force_eval!(x1p1023 + x);
        return 1. + x;
    }

    /* x is now in primary range */
    xx = x * x;
    c = x - xx * (P1 + xx * (P2 + xx * (P3 + xx * (P4 + xx * P5))));
    y = 1. + (x * c / (2. - c) - lo + hi);
    if k == 0 {
        y
    } else {
        scalbn(y, k)
    }
}
