/* origin: FreeBSD /usr/src/lib/msun/src/e_log.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* log(x)
 * Return the logarithm of x
 *
 * Method :
 *   1. Argument Reduction: find k and f such that
 *                      x = 2^k * (1+f),
 *         where  sqrt(2)/2 < 1+f < sqrt(2) .
 *
 *   2. Approximation of log(1+f).
 *      Let s = f/(2+f) ; based on log(1+f) = log(1+s) - log(1-s)
 *               = 2s + 2/3 s**3 + 2/5 s**5 + .....,
 *               = 2s + s*R
 *      We use a special Remez algorithm on [0,0.1716] to generate
 *      a polynomial of degree 14 to approximate R The maximum error
 *      of this polynomial approximation is bounded by 2**-58.45. In
 *      other words,
 *                      2      4      6      8      10      12      14
 *          R(z) ~ Lg1*s +Lg2*s +Lg3*s +Lg4*s +Lg5*s  +Lg6*s  +Lg7*s
 *      (the values of Lg1 to Lg7 are listed in the program)
 *      and
 *          |      2          14          |     -58.45
 *          | Lg1*s +...+Lg7*s    -  R(z) | <= 2
 *          |                             |
 *      Note that 2s = f - s*f = f - hfsq + s*hfsq, where hfsq = f*f/2.
 *      In order to guarantee error in log below 1ulp, we compute log
 *      by
 *              log(1+f) = f - s*(f - R)        (if f is not too large)
 *              log(1+f) = f - (hfsq - s*(hfsq+R)).     (better accuracy)
 *
 *      3. Finally,  log(x) = k*ln2 + log(1+f).
 *                          = k*ln2_hi+(f-(hfsq-(s*(hfsq+R)+k*ln2_lo)))
 *         Here ln2 is split into two floating point number:
 *                      ln2_hi + ln2_lo,
 *         where n*ln2_hi is always exact for |n| < 2000.
 *
 * Special cases:
 *      log(x) is NaN with signal if x < 0 (including -INF) ;
 *      log(+INF) is +INF; log(0) is -INF with signal;
 *      log(NaN) is that NaN with no signal.
 *
 * Accuracy:
 *      according to an error analysis, the error is always less than
 *      1 ulp (unit in the last place).
 *
 * Constants:
 * The hexadecimal values are the intended ones for the following
 * constants. The decimal values may be used, provided that the
 * compiler will convert from decimal to binary accurately enough
 * to produce the hexadecimal values shown.
 */

use core::f64;

const LN2_HI: f64 = 6.931_471_803_691_238_164_9_e-01; /* 3fe62e42 fee00000 */
const LN2_LO: f64 = 1.908_214_929_270_587_700_02_e-10; /* 3dea39ef 35793c76 */
const LG1: f64 = 6.666_666_666_666_735_13_e-01; /* 3FE55555 55555593 */
const LG2: f64 = 3.999_999_999_940_941_908_e-01; /* 3FD99999 9997FA04 */
const LG3: f64 = 2.857_142_874_366_239_149_e-01; /* 3FD24924 94229359 */
const LG4: f64 = 2.222_219_843_214_978_396_e-01; /* 3FCC71C5 1D8E78AF */
const LG5: f64 = 1.818_357_216_161_805_012_e-01; /* 3FC74664 96CB03DE */
const LG6: f64 = 1.531_383_769_920_937_332_e-01; /* 3FC39A09 D078C69F */
const LG7: f64 = 1.479_819_860_511_658_591_e-01; /* 3FC2F112 DF3E5244 */

#[inline]
pub fn log(mut x: f64) -> f64 {
    let x1p54 = f64::from_bits(0x_4350_0000_0000_0000); // 0x1p54 === 2 ^ 54

    let mut ui = x.to_bits();
    let mut hx: u32 = (ui >> 32) as u32;
    let mut k: i32 = 0;

    if (hx < 0x_0010_0000) || ((hx >> 31) != 0) {
        /* x < 2**-126  */
        if ui << 1 == 0 {
            return f64::NEG_INFINITY; /* log(+-0)=-inf */
        }
        if hx >> 31 != 0 {
            return f64::NAN; /* log(-#) = NaN */
        }
        /* subnormal number, scale x up */
        k -= 54;
        x *= x1p54;
        ui = x.to_bits();
        hx = (ui >> 32) as u32;
    } else if hx >= 0x_7ff0_0000 {
        return x;
    } else if hx == 0x_3ff0_0000 && ui << 32 == 0 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    hx += 0x_3ff0_0000 - 0x_3fe6_a09e;
    k += ((hx >> 20) as i32) - 0x3ff;
    hx = (hx & 0x_000f_ffff) + 0x_3fe6_a09e;
    ui = ((hx as u64) << 32) | (ui & 0x_ffff_ffff);
    x = f64::from_bits(ui);

    let f = x - 1.;
    let hfsq = 0.5 * f * f;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * (LG4 + w * LG6));
    let t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r = t2 + t1;
    let dk = k as f64;
    s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI
}
