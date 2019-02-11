/* origin: FreeBSD /usr/src/lib/msun/src/e_log2.c */
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
 * Return the base 2 logarithm of x.  See log.c for most comments.
 *
 * Reduce x to 2^k (1+f) and calculate r = log(1+f) - f + f*f/2
 * as in log.c, then combine and scale in extra precision:
 *    log2(x) = (f - f*f/2 + r)/log(2) + k
 */

use core::f64;
#[cfg(all(target_os = "cuda", not(feature = "stable")))]
use super::cuda_intrinsics;

const IVLN2HI: f64 = 1.442_695_040_721_446_275_71; /* 0x_3ff7_1547, 0x_6520_0000 */
const IVLN2LO: f64 = 1.675_171_316_488_651_183_53_e-10; /* 0x_3de7_05fc, 0x_2eef_a200 */
const LG1: f64 = 6.666_666_666_666_735_13_e-01; /* 3FE55555 55555593 */
const LG2: f64 = 3.999_999_999_940_941_908_e-01; /* 3FD99999 9997FA04 */
const LG3: f64 = 2.857_142_874_366_239_149_e-01; /* 3FD24924 94229359 */
const LG4: f64 = 2.222_219_843_214_978_396_e-01; /* 3FCC71C5 1D8E78AF */
const LG5: f64 = 1.818_357_216_161_805_012_e-01; /* 3FC74664 96CB03DE */
const LG6: f64 = 1.531_383_769_920_937_332_e-01; /* 3FC39A09 D078C69F */
const LG7: f64 = 1.479_819_860_511_658_591_e-01; /* 3FC2F112 DF3E5244 */

#[inline]
pub fn log2(mut x: f64) -> f64 {
    llvm_intrinsically_optimized! {
        #[cfg(target_os = "cuda")] {
            return unsafe { cuda_intrinsics::lg2_approx(x) }
        }
    }

    let x1p54 = f64::from_bits(0x_4350_0000_0000_0000); // 0x1p54 === 2 ^ 54

    let mut ui: u64 = x.to_bits();
    let hfsq: f64;
    let f: f64;
    let s: f64;
    let z: f64;
    let r: f64;
    let mut w: f64;
    let t1: f64;
    let t2: f64;
    let y: f64;
    let mut hi: f64;
    let lo: f64;
    let mut val_hi: f64;
    let mut val_lo: f64;
    let mut hx: u32;
    let mut k: i32;

    hx = (ui >> 32) as u32;
    k = 0;
    if hx < 0x_0010_0000 || (hx >> 31) > 0 {
        if ui << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
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

    f = x - 1.;
    hfsq = 0.5 * f * f;
    s = f / (2. + f);
    z = s * s;
    w = z * z;
    t1 = w * (LG2 + w * (LG4 + w * LG6));
    t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    r = t2 + t1;

    /* hi+lo = f - hfsq + s*(hfsq+R) ~ log(1+f) */
    hi = f - hfsq;
    ui = hi.to_bits();
    ui &= (-1i64 as u64) << 32;
    hi = f64::from_bits(ui);
    lo = f - hi - hfsq + s * (hfsq + r);

    val_hi = hi * IVLN2HI;
    val_lo = (lo + hi) * IVLN2LO + lo * IVLN2HI;

    /* spadd(val_hi, val_lo, y), except for not using double_t: */
    y = k.into();
    w = y + val_hi;
    val_lo += (y - w) + val_hi;
    val_hi = w;

    val_lo + val_hi
}
