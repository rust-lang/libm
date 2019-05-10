/* origin: FreeBSD /usr/src/lib/msun/src/s_log1p.c */
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
/* double log1p(double x)
 * Return the natural logarithm of 1+x.
 *
 * Method :
 *   1. Argument Reduction: find k and f such that
 *                      1+x = 2^k * (1+f),
 *         where  sqrt(2)/2 < 1+f < sqrt(2) .
 *
 *      Note. If k=0, then f=x is exact. However, if k!=0, then f
 *      may not be representable exactly. In that case, a correction
 *      term is need. Let u=1+x rounded. Let c = (1+x)-u, then
 *      log(1+x) - log(u) ~ c/u. Thus, we proceed to compute log(u),
 *      and add back the correction term c/u.
 *      (Note: when x > 2**53, one can simply return log(x))
 *
 *   2. Approximation of log(1+f): See log.c
 *
 *   3. Finally, log1p(x) = k*ln2 + log(1+f) + c/u. See log.c
 *
 * Special cases:
 *      log1p(x) is NaN with signal if x < -1 (including -INF) ;
 *      log1p(+INF) is +INF; log1p(-1) is -INF with signal;
 *      log1p(NaN) is that NaN with no signal.
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
 *
 * Note: Assuming log() return accurate answer, the following
 *       algorithm can be used to compute log1p(x) to within a few ULP:
 *
 *              u = 1+x;
 *              if(u==1.0) return x ; else
 *                         return log(u)*(x/(u-1.0));
 *
 *       See HP-15C Advanced Functions Handbook, p.193.
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

/// Log of 1 + X (f64)
///
/// Calculates the natural logarithm of `1+x`.
/// You can use `log1p` rather than `log(1+x)` for greater precision when `x` is very small.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log1p(x: f64) -> f64 {
    let mut ui: u64 = x.to_bits();
    let mut f: f64 = 0.;
    let mut c: f64 = 0.;

    let hx = (ui >> 32) as u32;
    let mut k = 1_i32;
    if hx < 0x_3fda_827a || (hx >> 31) > 0 {
        /* 1+x < sqrt(2)+ */
        if hx >= 0x_bff0_0000 {
            /* x <= -1. */
            if x == -1. {
                return f64::NEG_INFINITY; /* log1p(-1) = -inf */
            }
            return (x - x) / 0.0; /* log1p(x<-1) = NaN */
        }
        if (hx << 1) < (0x_3ca0_0000 << 1) {
            /* |x| < 2**-53 */
            /* underflow if subnormal */
            if (hx & 0x_7ff0_0000) == 0 {
                force_eval!(x as f32);
            }
            return x;
        }
        if hx <= 0x_bfd2_bec4 {
            /* sqrt(2)/2- <= 1+x < sqrt(2)+ */
            k = 0;
            c = 0.;
            f = x;
        }
    } else if hx >= 0x_7ff0_0000 {
        return x;
    }
    if k > 0 {
        ui = (1. + x).to_bits();
        let mut hu = (ui >> 32) as u32;
        hu += 0x_3ff0_0000 - 0x_3fe6_a09e;
        k = (hu >> 20) as i32 - 0x3ff;
        /* correction term ~ log(1+x)-log(u), avoid underflow in c/u */
        if k < 54 {
            c = if k >= 2 {
                1. - (f64::from_bits(ui) - x)
            } else {
                x - (f64::from_bits(ui) - 1.)
            };
            c /= f64::from_bits(ui);
        } else {
            c = 0.;
        }
        /* reduce u into [sqrt(2)/2, sqrt(2)] */
        hu = (hu & 0x_000f_ffff) + 0x_3fe6_a09e;
        ui = (hu as u64) << 32 | (ui & 0x_ffff_ffff);
        f = f64::from_bits(ui) - 1.;
    }
    let hfsq = 0.5 * f * f;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * (LG4 + w * LG6));
    let t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r = t2 + t1;
    let dk = k as f64;
    s * (hfsq + r) + (dk * LN2_LO + c) - hfsq + f + dk * LN2_HI
}
