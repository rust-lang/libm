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

use math::floorf;
use math::scalbnf;

#[allow(dead_code)]
pub enum Precision {
  Zero,
  One,
  Two,
}

/* In the float version, the input parameter x contains 8 bit
   integers, not 24 bit integers.  113 bit precision is not supported.  */
//const INIT_JK: [u8; 3] = [4, 7, 9]; /* initial value for jk */

const PI_O2: [f32; 11] = [
    1.5703125000e+00, /* 0x3fc90000 */
    4.5776367188e-04, /* 0x39f00000 */
    2.5987625122e-05, /* 0x37da0000 */
    7.5437128544e-08, /* 0x33a20000 */
    6.0026650317e-11, /* 0x2e840000 */
    7.3896444519e-13, /* 0x2b500000 */
    5.3845816694e-15, /* 0x27c20000 */
    5.6378512969e-18, /* 0x22d00000 */
    8.3009228831e-20, /* 0x1fc40000 */
    3.2756352257e-22, /* 0x1bc60000 */
    6.3331015649e-25, /* 0x17440000 */
];

const ZERO: f32 = 0.;
const ONE: f32 = 1.;
const TWO8: f32 = 2.5600000000e+02; /* 0x43800000 */
const TWON8: f32 = 3.9062500000e-03; /* 0x3b800000 */

#[inline]
pub fn k_rem_pio2f(
    x: &[f32],
    e0: i32,
    prec: Precision,
    ipio2: &'static [u8],
//) -> (i32, [f32; 3]) {
) -> (i32, f32, f32) {
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
    //let jk = INIT_JK[prec as usize];
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
        f[i] = if j < 0 { ZERO } else { ipio2[j as usize] as f32 };
        j += 1;
    }

    /* compute q[0],q[1],...q[jk] */
    for i in 0..=jk {
        fw = 0.;
        for j in 0..=jx {
            fw += x[j] * f[jx + i - j];
            q[i] = fw;
        }
    }

    let mut jz = jk;
    'recompute: loop {
        /* distill q[] into iq[] reversingly */
        let mut i = 0i32;
        z = q[jz];
        for j in (1..=jz).rev() {
            fw = (TWON8 * z) as i32 as f32;
            iq[i as usize] = (z - TWO8 * fw) as i32;
            z = q[j - 1] + fw;
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
            i = iq[jz - 1] >> (8 - q0);
            n += i;
            iq[jz - 1] -= i << (8 - q0);
            ih = iq[jz - 1] >> (7 - q0);
        } else if q0 == 0 {
            ih = iq[jz - 1] >> 8;
        } else if z >= 0.5 {
            ih = 2;
        }

        if ih > 0 {
            /* q > 0.5 */
            n += 1;
            let mut carry = 0i32;
            for i in 0..jz {
                /* compute 1-q */
                j = iq[i];
                if carry == 0 {
                    if j != 0 {
                        carry = 1;
                        iq[i] = 0x100 - j;
                    }
                } else {
                    iq[i] = 0xff - j;
                }
            }
            if q0 > 0 {
                /* rare case: chance is 1 in 12 */
                match q0 {
                    1 => iq[jz - 1] &= 0x7f,
                    2 => iq[jz - 1] &= 0x3f,
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
            for i in (jk..=jz - 1).rev() {
                j |= iq[i];
            }
            if j == 0 {
                /* need recomputation */
                let mut k = 1;
                while iq[jk - k] == 0 {
                    k += 1; /* k = no. of terms needed */
                }

                for i in (jz + 1)..=(jz + k) {
                    /* add q[jz+1] to q[jz+k] */
                    f[jx + i] = ipio2[jv + i] as f32;
                    fw = 0.;
                    for j in 0..=jx {
                        fw += x[j] * f[jx + i - j];
                    }
                    q[i] = fw;
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
        while iq[jz] == 0 {
            jz -= 1;
            q0 -= 8;
        }
    } else {
        /* break z into 8-bit if necessary */
        z = scalbnf(z, -q0);
        if z >= TWO8 {
            fw = (TWON8 * z) as i32 as f32;
            iq[jz] = (z - TWO8 * fw) as i32;
            jz += 1;
            q0 += 8;
            iq[jz] = fw as i32;
        } else {
            iq[jz] = z as i32;
        }
    }

    /* convert integer "bit" chunk to floating-point value */
    fw = scalbnf(ONE, q0);
    for i in (0..=jz).rev() {
        q[i] = fw * (iq[i] as f32);
        fw *= TWON8;
    }

    /* compute PIo2[0,...,jp]*q[jz,...,0] */
    for i in (0..=jz).rev() {
        fw = 0.;
        let mut k = 0;
        while (k <= jp) && (k <= jz - i) {
            fw += PI_O2[k] * q[i + k];
            k += 1;
        }
        fq[jz - i] = fw;
    }

    /* compress fq[] into y[] */
    //let mut y = [0f32; 3];
    match prec {
        Precision::Zero => {
            fw = 0.;
            for i in (0..=jz).rev() {
                fw += fq[i];
            }
            //y[0] = if ih == 0 { fw } else { -fw };
            (n & 7, if ih == 0 { fw } else { -fw }, 0.)
        },
        Precision::One | Precision::Two => {
            fw = 0.;
            for i in (0..=jz).rev() {
                fw += fq[i];
            }
            let y0 = if ih == 0 { fw } else { -fw };
            fw = fq[0] - fw;
            for i in 1..=jz {
                fw += fq[i];
            }
            //y[1] = if ih == 0 { fw } else { -fw };
            (n & 7, y0, if ih == 0 { fw } else { -fw })
        },
        /*3 => {
            /* painful */
            for i in (1..=jz).rev() {
                fw = fq[i - 1] + fq[i];
                fq[i] += fq[i - 1] - fw;
                fq[i - 1] = fw;
            }
            for i in (2..=jz).rev() {
                fw = fq[i - 1] + fq[i];
                fq[i] += fq[i - 1] - fw;
                fq[i - 1] = fw;
            }
            fw = 0.;
            for i in (2..=jz).rev() {
                fw += fq[i];
            }
            if ih == 0 {
                y[0] = fq[0];
                y[1] = fq[1];
                y[2] = fw;
            } else {
                y[0] = -fq[0];
                y[1] = -fq[1];
                y[2] = -fw;
            }
        },*/
        //_ => unreachable!(),
    }
    //(n & 7, y)
}
