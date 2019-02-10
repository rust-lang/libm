#![allow(unused_unsafe)]
/* kf_rem_pio2.c -- float version of k_rem_pio2.c
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

use super::floorf;
use math::scalbnf;

#[allow(dead_code)]
pub enum Precision {
    Zero,
    One,
    Two,
}

const PI_O2: [f32; 11] = [
    1.570_312_500_0,      /* 0x_3fc9_0000 */
    4.577_636_718_8_e-04, /* 0x_39f0_0000 */
    2.598_762_512_2_e-05, /* 0x_37da_0000 */
    7.543_712_854_4_e-08, /* 0x_33a2_0000 */
    6.002_665_031_7_e-11, /* 0x_2e84_0000 */
    7.389_644_451_9_e-13, /* 0x_2b50_0000 */
    5.384_581_669_4_e-15, /* 0x_27c2_0000 */
    5.637_851_296_9_e-18, /* 0x_22d0_0000 */
    8.300_922_883_1_e-20, /* 0x_1fc4_0000 */
    3.275_635_225_7_e-22, /* 0x_1bc6_0000 */
    6.333_101_564_9_e-25, /* 0x_1744_0000 */
];

const ZERO: f32 = 0.;
const ONE: f32 = 1.;
const TWO8: f32 = 2.560_000_000_0_e+02; /* 0x_4380_0000 */
const TWON8: f32 = 3.906_250_000_0_e-03; /* 0x_3b80_0000 */

#[allow(clippy::cyclomatic_complexity)]
#[inline]
pub fn k_rem_pio2f(x: &[f32], e0: i32, prec: Precision, ipio2: &'static [u8]) -> (i32, f32, f32) {
    let nx = x.len();

    let mut f = [0f32; 20];
    let mut fq = [0f32; 20];
    let mut q = [0f32; 20];
    let mut iq = [0i32; 20];
    let mut z: f32;
    let mut fw: f32;
    let mut ih: i32;
    let mut n: i32;

    /* initialize jk*/
    let jk = match prec {
        Precision::Zero => 4,
        Precision::One => 7,
        Precision::Two => 9,
    };
    let jp = jk;

    /* determine jx,jv,q0, note that 3>q0 */
    let jx = nx - 1;
    let mut jv = (e0 - 3) / 8;
    if jv < 0 {
        jv = 0;
    }
    let mut q0 = e0 - 8 * (jv + 1);
    let jv = jv as usize;

    /* set up f[0] to f[jx+jk] where f[jx+jk] = ipio2[jv+jk] */
    let mut j = (jv - jx) as i32;
    let m = jx + jk;
    for i in 0..=m {
        i!(f, i, =, if j < 0 {
            ZERO
        } else {
            i!(ipio2, j as usize) as f32
        });
        j += 1;
    }

    /* compute q[0],q[1],...q[jk] */
    for i in 0..=jk {
        fw = 0.;
        for j in 0..=jx {
            fw += i!(x, j) * i!(f, jx + i - j);
            i!(q, i, =, fw);
        }
    }

    let mut jz = jk;
    'recompute: loop {
        /* distill q[] into iq[] reversingly */
        let mut i = 0i32;
        z = i!(q, jz);
        for j in (1..=jz).rev() {
            fw = (TWON8 * z) as i32 as f32;
            i!(iq, i as usize, =, (z - TWO8 * fw) as i32);
            z = i!(q, j - 1) + fw;
            i += 1;
        }

        /* compute n */
        z = scalbnf(z, q0 as i32); /* actual value of z */
        z -= 8. * floorf(z * 0.125); /* trim off integer >= 8 */
        n = z as i32;
        z -= n as f32;
        ih = 0;
        if q0 > 0 {
            /* need iq[jz-1] to determine n */
            i = i!(iq, jz - 1) >> (8 - q0);
            n += i;
            i!(iq, jz - 1, -=, i << (8 - q0));
            ih = i!(iq, jz - 1) >> (7 - q0);
        } else if q0 == 0 {
            ih = i!(iq, jz - 1) >> 8;
        } else if z >= 0.5 {
            ih = 2;
        }

        if ih > 0 {
            /* q > 0.5 */
            n += 1;
            let mut carry = 0i32;
            for i in 0..jz {
                /* compute 1-q */
                j = i!(iq, i);
                if carry == 0 {
                    if j != 0 {
                        carry = 1;
                        i!(iq, i, =, 0x100 - j);
                    }
                } else {
                    i!(iq, i, =, 0xff - j);
                }
            }
            if q0 > 0 {
                /* rare case: chance is 1 in 12 */
                match q0 {
                    1 => {
                        i!(iq, jz - 1, &=, 0x7f);
                    }
                    2 => {
                        i!(iq, jz - 1, &=, 0x3f);
                    }
                    _ => {}
                }
            }
            if ih == 2 {
                z = ONE - z;
                if carry != 0 {
                    z -= scalbnf(ONE, q0);
                }
            }
        }

        /* check if recomputation is needed */
        if z == ZERO {
            j = 0;
            for i in (jk..jz).rev() {
                j |= i!(iq, i);
            }
            if j == 0 {
                /* need recomputation */
                let mut k = 1;
                while i!(iq, jk - k) == 0 {
                    k += 1; /* k = no. of terms needed */
                }

                for i in (jz + 1)..=(jz + k) {
                    /* add q[jz+1] to q[jz+k] */
                    i!(f, jx + i, =, i!(ipio2, jv + i) as f32);
                    fw = 0.;
                    for j in 0..=jx {
                        fw += i!(x, j) * i!(f, jx + i - j);
                    }
                    i!(q, i, =, fw);
                }
                jz += k;
                continue 'recompute;
            }
        }
        break;
    }

    /* chop off zero terms */
    if z == 0. {
        jz -= 1;
        q0 -= 8;
        while i!(iq, jz) == 0 {
            jz -= 1;
            q0 -= 8;
        }
    } else {
        /* break z into 8-bit if necessary */
        z = scalbnf(z, -q0);
        if z >= TWO8 {
            fw = (TWON8 * z) as i32 as f32;
            i!(iq, jz, =, (z - TWO8 * fw) as i32);
            jz += 1;
            q0 += 8;
            i!(iq, jz, =, fw as i32);
        } else {
            i!(iq, jz, =, z as i32);
        }
    }

    /* convert integer "bit" chunk to floating-point value */
    fw = scalbnf(ONE, q0);
    for i in (0..=jz).rev() {
        i!(q, i, =, fw * (i!(iq, i) as f32));
        fw *= TWON8;
    }

    /* compute PIo2[0,...,jp]*q[jz,...,0] */
    for i in (0..=jz).rev() {
        fw = 0.;
        let mut k = 0;
        while (k <= jp) && (k <= jz - i) {
            fw += i!(PI_O2, k) * i!(q, i + k);
            k += 1;
        }
        i!(fq, jz - i, =, fw);
    }

    /* compress fq[] */
    match prec {
        Precision::Zero => {
            fw = 0.;
            for i in (0..=jz).rev() {
                fw += i!(fq, i);
            }
            (n & 7, if ih == 0 { fw } else { -fw }, 0.)
        }
        Precision::One | Precision::Two => {
            fw = 0.;
            for i in (0..=jz).rev() {
                fw += i!(fq, i);
            }
            let y0 = if ih == 0 { fw } else { -fw };
            fw = i!(fq, 0) - fw;
            for i in 1..=jz {
                fw += i!(fq, i);
            }
            (n & 7, y0, if ih == 0 { fw } else { -fw })
        }
    }
}
