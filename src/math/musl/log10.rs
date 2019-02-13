/* origin: FreeBSD /usr/src/lib/msun/src/e_log10.c */
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
/*
 * Return the base 10 logarithm of x.  See log.c for most comments.
 *
 * Reduce x to 2^k (1+f) and calculate r = log(1+f) - f + f*f/2
 * as in log.c, then combine and scale in extra precision:
 *    log10(x) = (f - f*f/2 + r)/log(10) + k*log10(2)
 */

use core::f64;

const IVLN10HI: f64 = 4.342_944_818_781_688_809_39_e-01; /* 0x_3fdb_cb7b, 0x_1520_0000 */
const IVLN10LO: f64 = 2.508_294_671_164_527_522_98_e-11; /* 0x_3dbb_9438, 0x_ca9a_add5 */
const LOG10_2HI: f64 = 3.010_299_956_636_117_713_06_e-01; /* 0x_3FD3_4413, 0x_509F_6000 */
const LOG10_2LO: f64 = 3.694_239_077_158_930_786_16_e-13; /* 0x_3D59_FEF3, 0x_11F1_2B36 */
const LG1: f64 = 6.666_666_666_666_735_13_e-01; /* 3FE55555 55555593 */
const LG2: f64 = 3.999_999_999_940_941_908_e-01; /* 3FD99999 9997FA04 */
const LG3: f64 = 2.857_142_874_366_239_149_e-01; /* 3FD24924 94229359 */
const LG4: f64 = 2.222_219_843_214_978_396_e-01; /* 3FCC71C5 1D8E78AF */
const LG5: f64 = 1.818_357_216_161_805_012_e-01; /* 3FC74664 96CB03DE */
const LG6: f64 = 1.531_383_769_920_937_332_e-01; /* 3FC39A09 D078C69F */
const LG7: f64 = 1.479_819_860_511_658_591_e-01; /* 3FC2F112 DF3E5244 */

/// Base 10 logarithm (f64)
///
/// Returns the base 10 logarithm of `x`. It is implemented as `log(x)/log(10)`.
#[inline]
pub fn log10(mut x: f64) -> f64 {
    let x1p54 = f64::from_bits(0x_4350_0000_0000_0000); // 0x1p54 === 2 ^ 54

    let mut ui: u64 = x.to_bits();

    let mut hx = (ui >> 32) as u32;
    let mut k = 0;
    if hx < 0x_0010_0000 || (hx >> 31) > 0 {
        if ui << 1 == 0 {
            return f64::NEG_INFINITY; /* log(+-0)=-inf */
        }
        if (hx >> 31) > 0 {
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
    k += (hx >> 20) as i32 - 0x3ff;
    hx = (hx & 0x_000f_ffff) + 0x_3fe6_a09e;
    ui = (hx as u64) << 32 | (ui & 0x_ffff_ffff);
    x = f64::from_bits(ui);

    let f = x - 1.;
    let hfsq = 0.5 * f * f;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * (LG4 + w * LG6));
    let t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r = t2 + t1;

    /* See log2.c for details. */
    /* hi+lo = f - hfsq + s*(hfsq+R) ~ log(1+f) */
    let hi = f - hfsq;
    ui = hi.to_bits();
    ui &= (-1_i64 as u64) << 32;
    let hi = f64::from_bits(ui);
    let lo = f - hi - hfsq + s * (hfsq + r);

    /* val_hi+val_lo ~ log10(1+f) + k*log10(2) */
    let mut val_hi = hi * IVLN10HI;
    let dk = k as f64;
    let y = dk * LOG10_2HI;
    let mut val_lo = dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI;

    /*
     * Extra precision in for adding y is not strictly needed
     * since there is no very large cancellation near x = sqrt(2) or
     * x = 1/sqrt(2), but we do it anyway since it costs little on CPUs
     * with some parallelism and it reduces the error for many args.
     */
    let w = y + val_hi;
    val_lo += (y - w) + val_hi;
    val_hi = w;

    val_lo + val_hi
}
