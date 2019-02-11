// origin: FreeBSD /usr/src/lib/msun/src/e_rem_pio2.c
//
// ====================================================
// Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
//
// Developed at SunPro, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this
// software is freely granted, provided that this notice
// is preserved.
// ====================================================
//
// Optimized by Bruce D. Evans. */

use core::f64;
use super::rem_pio2_large;
use crate::math::consts::*;

// #if FLT_EVAL_METHOD==0 || FLT_EVAL_METHOD==1
// #define EPS DBL_EPSILON
const EPS: f64 = 2.220_446_049_250_313_1_e-16;
// #elif FLT_EVAL_METHOD==2
// #define EPS LDBL_EPSILON
// #endif

// TODO: Support FLT_EVAL_METHOD?

const TO_INT: f64 = 1.5 / EPS;
/// 53 bits of 2/pi
const INV_PIO2: f64 = 6.366_197_723_675_813_824_33_e-01; /* 0x_3FE4_5F30, 0x_6DC9_C883 */
/// first 33 bits of pi/2
const PIO2_1: f64 = 1.570_796_326_734_125_614_17; /* 0x_3FF9_21FB, 0x_5440_0000 */
/// pi/2 - PIO2_1
const PIO2_1T: f64 = 6.077_100_506_506_192_249_32_e-11; /* 0x_3DD0_B461, 0x_1A62_6331 */
/// second 33 bits of pi/2
const PIO2_2: f64 = 6.077_100_506_303_965_976_6_e-11; /* 0x_3DD0_B461, 0x_1A60_0000 */
/// pi/2 - (PIO2_1+PIO2_2)
const PIO2_2T: f64 = 2.022_266_248_795_950_631_54_e-21; /* 0x_3BA3_198A, 0x_2E03_7073 */
/// third 33 bits of pi/2
const PIO2_3: f64 = 2.022_266_248_711_166_455_8_e-21; /* 0x_3BA3_198A, 0x_2E00_0000 */
/// pi/2 - (PIO2_1+PIO2_2+PIO2_3)
const PIO2_3T: f64 = 8.478_427_660_368_899_569_97_e-32; /* 0x_397B_839A, 0x_2520_49C1 */

// return the remainder of x rem pi/2 in y[0]+y[1]
// use rem_pio2_large() for large x
//
// caller must handle the case when reduction is not needed: |x| ~<= pi/4 */
#[inline]
pub fn rem_pio2(x: f64) -> (i32, f64, f64) {
    let x1p24 = f64::from_bits(0x_4170_0000_0000_0000);

    let sign = (f64::to_bits(x) >> 63) as i32;
    let ix = (f64::to_bits(x) >> 32) as u32 & UF_ABS;

    #[inline]
    fn medium(x: f64, ix: u32) -> (i32, f64, f64) {
        /* rint(x/(pi/2)), Assume round-to-nearest. */
        let f_n = x as f64 * INV_PIO2 + TO_INT - TO_INT;
        let n = f_n as i32;
        let mut r = x - f_n * PIO2_1;
        let mut w = f_n * PIO2_1T; /* 1st round, good to 85 bits */
        let mut y0 = r - w;
        let ui = f64::to_bits(y0);
        let ey = (ui >> 52) as i32 & 0x7ff;
        let ex = (ix >> 20) as i32;
        if ex - ey > 16 {
            /* 2nd round, good to 118 bits */
            let t = r;
            w = f_n * PIO2_2;
            r = t - w;
            w = f_n * PIO2_2T - ((t - r) - w);
            y0 = r - w;
            let ey = (f64::to_bits(y0) >> 52) as i32 & 0x7ff;
            if ex - ey > 49 {
                /* 3rd round, good to 151 bits, covers all cases */
                let t = r;
                w = f_n * PIO2_3;
                r = t - w;
                w = f_n * PIO2_3T - ((t - r) - w);
                y0 = r - w;
            }
        }
        let y1 = (r - y0) - w;
        (n, y0, y1)
    }

    if ix <= 0x_400f_6a7a {
        /* |x| ~<= 5pi/4 */
        if (ix & 0xfffff) == 0x_0009_21fb {
            /* |x| ~= pi/2 or 2pi/2 */
            medium(x, ix); /* cancellation -- use medium case */
        }
        if ix <= 0x_4002_d97c {
            /* |x| ~<= 3pi/4 */
            if sign == 0 {
                let z = x - PIO2_1; /* one round good to 85 bits */
                let y0 = z - PIO2_1T;
                let y1 = (z - y0) - PIO2_1T;
                return (1, y0, y1);
            } else {
                let z = x + PIO2_1;
                let y0 = z + PIO2_1T;
                let y1 = (z - y0) + PIO2_1T;
                return (-1, y0, y1);
            }
        } else if sign == 0 {
            let z = x - 2. * PIO2_1;
            let y0 = z - 2. * PIO2_1T;
            let y1 = (z - y0) - 2. * PIO2_1T;
            return (2, y0, y1);
        } else {
            let z = x + 2. * PIO2_1;
            let y0 = z + 2. * PIO2_1T;
            let y1 = (z - y0) + 2. * PIO2_1T;
            return (-2, y0, y1);
        }
    }
    if ix <= 0x_401c_463b {
        /* |x| ~<= 9pi/4 */
        if ix <= 0x_4015_fdbc {
            /* |x| ~<= 7pi/4 */
            if ix == 0x_4012_d97c {
                /* |x| ~= 3pi/2 */
                return medium(x, ix);
            }
            if sign == 0 {
                let z = x - 3. * PIO2_1;
                let y0 = z - 3. * PIO2_1T;
                let y1 = (z - y0) - 3. * PIO2_1T;
                return (3, y0, y1);
            } else {
                let z = x + 3. * PIO2_1;
                let y0 = z + 3. * PIO2_1T;
                let y1 = (z - y0) + 3. * PIO2_1T;
                return (-3, y0, y1);
            }
        } else {
            if ix == 0x_4019_21fb {
                /* |x| ~= 4pi/2 */
                return medium(x, ix);
            }
            if sign == 0 {
                let z = x - 4. * PIO2_1;
                let y0 = z - 4. * PIO2_1T;
                let y1 = (z - y0) - 4. * PIO2_1T;
                return (4, y0, y1);
            } else {
                let z = x + 4. * PIO2_1;
                let y0 = z + 4. * PIO2_1T;
                let y1 = (z - y0) + 4. * PIO2_1T;
                return (-4, y0, y1);
            }
        }
    }
    if ix < 0x_4139_21fb {
        /* |x| ~< 2^20*(pi/2), medium size */
        return medium(x, ix);
    }
    /*
     * all other (large) arguments
     */
    if ix >= 0x_7ff0_0000 {
        /* x is inf or NaN */
        let y0 = f64::NAN;
        let y1 = y0;
        return (0, y0, y1);
    }
    /* set z = scalbn(|x|,-ilogb(x)+23) */
    let mut ui = f64::to_bits(x);
    ui &= (!1) >> 12;
    ui |= (0x3ff + 23) << 52;
    let mut z = f64::from_bits(ui);
    let mut tx = [0.; 3];

    for txi in tx.iter_mut().take(2) {
        *txi = z as i32 as f64;
        z = (z - *txi) * x1p24;
    }
    tx[2] = z;
    /* skip zero terms, first term is non-zero */
    let mut i = 2;
    while i != 0 && tx[i] == 0. {
        i -= 1;
    }
    let (n, ty) = rem_pio2_large(&tx[..=i], ((ix >> 20) - (0x3ff + 23)) as i32, 1);
    if sign != 0 {
        return (-n, -ty[0], -ty[1]);
    }
    (n, ty[0], ty[1])
}
