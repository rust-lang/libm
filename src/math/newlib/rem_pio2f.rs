/* ef_rem_pio2.c -- float version of e_rem_pio2.c
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
 *
 */

use core::f32;
use super::k_rem_pio2f;
use super::Precision;
use math::fabsf;

/* Table of constants for 2/pi, 396 Hex digits (476 decimal) of 2/pi */
const TWO_OVER_PI: [u8; 198] = [
    0xA2, 0xF9, 0x83, 0x6E, 0x4E, 0x44, 0x15, 0x29, 0xFC, 0x27, 0x57, 0xD1, 0xF5, 0x34, 0xDD, 0xC0,
    0xDB, 0x62, 0x95, 0x99, 0x3C, 0x43, 0x90, 0x41, 0xFE, 0x51, 0x63, 0xAB, 0xDE, 0xBB, 0xC5, 0x61,
    0xB7, 0x24, 0x6E, 0x3A, 0x42, 0x4D, 0xD2, 0xE0, 0x06, 0x49, 0x2E, 0xEA, 0x09, 0xD1, 0x92, 0x1C,
    0xFE, 0x1D, 0xEB, 0x1C, 0xB1, 0x29, 0xA7, 0x3E, 0xE8, 0x82, 0x35, 0xF5, 0x2E, 0xBB, 0x44, 0x84,
    0xE9, 0x9C, 0x70, 0x26, 0xB4, 0x5F, 0x7E, 0x41, 0x39, 0x91, 0xD6, 0x39, 0x83, 0x53, 0x39, 0xF4,
    0x9C, 0x84, 0x5F, 0x8B, 0xBD, 0xF9, 0x28, 0x3B, 0x1F, 0xF8, 0x97, 0xFF, 0xDE, 0x05, 0x98, 0x0F,
    0xEF, 0x2F, 0x11, 0x8B, 0x5A, 0x0A, 0x6D, 0x1F, 0x6D, 0x36, 0x7E, 0xCF, 0x27, 0xCB, 0x09, 0xB7,
    0x4F, 0x46, 0x3F, 0x66, 0x9E, 0x5F, 0xEA, 0x2D, 0x75, 0x27, 0xBA, 0xC7, 0xEB, 0xE5, 0xF1, 0x7B,
    0x3D, 0x07, 0x39, 0xF7, 0x8A, 0x52, 0x92, 0xEA, 0x6B, 0xFB, 0x5F, 0xB1, 0x1F, 0x8D, 0x5D, 0x08,
    0x56, 0x03, 0x30, 0x46, 0xFC, 0x7B, 0x6B, 0xAB, 0xF0, 0xCF, 0xBC, 0x20, 0x9A, 0xF4, 0x36, 0x1D,
    0xA9, 0xE3, 0x91, 0x61, 0x5E, 0xE6, 0x1B, 0x08, 0x65, 0x99, 0x85, 0x5F, 0x14, 0xA0, 0x68, 0x40,
    0x8D, 0xFF, 0xD8, 0x80, 0x4D, 0x73, 0x27, 0x31, 0x06, 0x06, 0x15, 0x56, 0xCA, 0x73, 0xA8, 0xC9,
    0x60, 0xE2, 0x7B, 0xC0, 0x8C, 0x6B,
];

/* This array is like the one in e_rem_pio2.c, but the numbers are
single precision and the last 8 bits are forced to 0.  */
const NPIO2_HW: [u32; 32] = [
    0x_3fc9_0f00,
    0x_4049_0f00,
    0x_4096_cb00,
    0x_40c9_0f00,
    0x_40fb_5300,
    0x_4116_cb00,
    0x_412f_ed00,
    0x_4149_0f00,
    0x_4162_3100,
    0x_417b_5300,
    0x_418a_3a00,
    0x_4196_cb00,
    0x_41a3_5c00,
    0x_41af_ed00,
    0x_41bc_7e00,
    0x_41c9_0f00,
    0x_41d5_a000,
    0x_41e2_3100,
    0x_41ee_c200,
    0x_41fb_5300,
    0x_4203_f200,
    0x_420a_3a00,
    0x_4210_8300,
    0x_4216_cb00,
    0x_421d_1400,
    0x_4223_5c00,
    0x_4229_a500,
    0x_422f_ed00,
    0x_4236_3600,
    0x_423c_7e00,
    0x_4242_c700,
    0x_4249_0f00,
];

/*
 * invpio2:  24 bits of 2/pi
 * pio2_1:   first  17 bit of pi/2
 * pio2_1t:  pi/2 - pio2_1
 * pio2_2:   second 17 bit of pi/2
 * pio2_2t:  pi/2 - (pio2_1+pio2_2)
 * pio2_3:   third  17 bit of pi/2
 * pio2_3t:  pi/2 - (pio2_1+pio2_2+pio2_3)
 */
const ZERO: f32 = 0.; /* 0x_0000_0000 */
const HALF: f32 = 5_e-01; /* 0x_3f00_0000 */
const TWO8: f32 = 2.560_000_000_0_e+02; /* 0x_4380_0000 */
const INV_PIO2: f32 = 6.366_198_062_9_e-01; /* 0x_3f22_f984 */
const PIO2_1: f32 = 1.570_785_522_5; /* 0x_3fc9_0f80 */
const PIO2_1T: f32 = 1.080_433_412_4_e-05; /* 0x_3735_4443 */
const PIO2_2: f32 = 1.080_427_318_8_e-05; /* 0x_3735_4400 */
const PIO2_2T: f32 = 6.077_099_934_4_e-11; /* 0x_2e85_a308 */
const PIO2_3: f32 = 6.077_094_383_3_e-11; /* 0x_2e85_a300 */
const PIO2_3T: f32 = 6.123_234_262_9_e-17; /* 0x_248d_3132 */

const UF_INF: u32 = 0x_7f80_0000;
//const UF_1_PI_4: u32 = 0x_3f49_0fdb;
const UF_3_PI_4: u32 = 0x_4016_cbe4;

/// Return the remainder of x rem pi/2 in y[0]+y[1]
#[inline]
pub fn rem_pio2f(x: f32) -> (i32, f32, f32) {
    let mut y0: f32;
    let mut z: f32;

    let hx = x.to_bits();
    let ix = hx & 0x_7fff_ffff;
    let sign = (hx >> 31) != 0;
    if ix <= 0x_3f49_0fd8 {
        /* |x| ~<= pi/4 , no need for reduction */
        return (0, x, 0.);
    }
    if ix < UF_3_PI_4 {
        /* |x| < 3pi/4, special case with n=+-1 */
        return if !sign {
            z = x - PIO2_1;
            if (ix & 0x_ffff_fff0) != 0x_3fc9_0fd0 {
                /* 24+24 bit pi OK */
                y0 = z - PIO2_1T;
                (1, y0, (z - y0) - PIO2_1T)
            } else {
                /* near pi/2, use 24+24+24 bit pi */
                z -= PIO2_2;
                y0 = z - PIO2_2T;
                (1, y0, (z - y0) - PIO2_2T)
            }
        } else {
            /* negative x */
            z = x + PIO2_1;
            if (ix & 0x_ffff_fff0) != 0x_3fc9_0fd0 {
                /* 24+24 bit pi OK */
                y0 = z + PIO2_1T;
                (-1, y0, (z - y0) + PIO2_1T)
            } else {
                /* near pi/2, use 24+24+24 bit pi */
                z += PIO2_2;
                y0 = z + PIO2_2T;
                (-1, y0, (z - y0) + PIO2_2T)
            }
        };
    }
    if ix <= 0x_4349_0f80 {
        /* |x| ~<= 2^7*(pi/2), medium size */
        let t = fabsf(x);
        let n = (t * INV_PIO2 + HALF) as i32;
        let nf = n as f32;
        let mut r = t - nf * PIO2_1;
        let mut w = nf * PIO2_1T; /* 1st round good to 40 bit */
        if (n < 32) && (ix & 0x_ffff_ff00 != NPIO2_HW[(n - 1) as usize]) {
            y0 = r - w; /* quick check no cancellation */
        } else {
            let j = (ix as i32) >> 23;
            y0 = r - w;
            let high = y0.to_bits();
            let i = j - ((high >> 23) & 0xff) as i32;
            if i > 8 {
                /* 2nd iteration needed, good to 57 */
                let t = r;
                w = nf * PIO2_2;
                r = t - w;
                w = nf * PIO2_2T - ((t - r) - w);
                y0 = r - w;
                let high = y0.to_bits();
                let i = j - ((high >> 23) & 0xff) as i32;
                if i > 25 {
                    /* 3rd iteration need, 74 bits acc */
                    let t = r; /* will cover all possible cases */
                    w = nf * PIO2_3;
                    r = t - w;
                    w = nf * PIO2_3T - ((t - r) - w);
                    y0 = r - w;
                }
            }
        }
        let y1 = (r - y0) - w;
        return if sign { (-n, -y0, -y1) } else { (n, y0, y1) };
    }
    /*
     * all other (large) arguments
     */
    if ix >= UF_INF {
        y0 = f32::NAN;
        return (0, y0, y0);
    }
    /* set z = scalbn(|x|,ilogb(x)-7) */
    let ix = ix as i32;
    let e0 = (ix >> 23) - 134; /* e0 = ilogb(z)-7; */
    z = f32::from_bits((ix - (e0 << 23)) as u32);
    let mut tx = [0f32; 3];

    for txi in tx.iter_mut().take(2) {
        *txi = z as i32 as f32;
        z = (z - *txi) * TWO8;
    }
    tx[2] = z;
    let mut nx = 3;
    while tx[nx - 1] == ZERO {
        /* skip zero term */
        nx -= 1;
    }
    let (n, y0, y1) = k_rem_pio2f(&tx[..nx], e0, Precision::Two, &TWO_OVER_PI);
    if sign {
        (-n, -y0, -y1)
    } else {
        (n, y0, y1)
    }
}
