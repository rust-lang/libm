/* origin: FreeBSD /usr/src/lib/msun/src/e_powf.c */
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

use super::{fabsf, scalbnf, sqrtf};
use crate::math::consts::*;
use core::f32;

pub const IF_1: i32 = UF_1 as i32;
pub const IF_INF: i32 = UF_INF as i32;
pub const IF_MIN: i32 = UF_MIN as i32;
pub const IF_ABS: i32 = UF_ABS as i32;

const BP: [f32; 2] = [1., 1.5];
const DP_H: [f32; 2] = [0., 5.849_609_38_e-01]; /* 0x_3f15_c000 */
const DP_L: [f32; 2] = [0., 1.563_220_85_e-06]; /* 0x_35d1_cfdc */
const TWO24: f32 = 16_777_216.; /* 0x_4b80_0000 */
const HUGE: f32 = 1_e30;
const TINY: f32 = 1_e-30;
const L1: f32 = 6.000_000_238_4_e-01; /* 0x_3f19_999a */
const L2: f32 = 4.285_714_328_3_e-01; /* 0x_3edb_6db7 */
const L3: f32 = 3.333_333_432_7_e-01; /* 0x_3eaa_aaab */
const L4: f32 = 2.727_281_153_2_e-01; /* 0x_3e8b_a305 */
const L5: f32 = 2.306_607_514_6_e-01; /* 0x_3e6c_3255 */
const L6: f32 = 2.069_750_130_2_e-01; /* 0x_3e53_f142 */
const P1: f32 = 1.666_666_716_3_e-01; /* 0x_3e2a_aaab */
const P2: f32 = -2.777_777_845_0_e-03; /* 0x_bb36_0b61 */
const P3: f32 = 6.613_755_977_e-05; /* 0x_388a_b355 */
const P4: f32 = -1.653_390_199_9_e-06; /* 0x_b5dd_ea0e */
const P5: f32 = 4.138_136_944_2_e-08; /* 0x_3331_bb4c */
const LG2: f32 = 6.931_471_824_6_e-01; /* 0x_3f31_7218 */
const LG2_H: f32 = 6.931_457_52_e-01; /* 0x_3f31_7200 */
const LG2_L: f32 = 1.428_606_54_e-06; /* 0x_35bf_be8c */
const OVT: f32 = 4.299_566_569_4e-08; /* -(128-log2(ovfl+.5ulp)) */
const CP: f32 = 9.617_967_009_5_e-01; /* 0x_3f76_384f =2/(3ln2) */
const CP_H: f32 = 9.619_140_625_e-01; /* 0x_3f76_4000 =12b cp */
const CP_L: f32 = -1.173_685_740_2_e-04; /* 0x_b8f6_23c6 =tail of cp_h */
const IVLN2: f32 = 1.442_695_021_6;
const IVLN2_H: f32 = 1.442_687_988_3;
const IVLN2_L: f32 = 7.052_607_543_3_e-06;

#[inline]
fn high_low(x: f32) -> (f32, f32) {
    let high = f32::from_bits(x.to_bits() & 0x_ffff_f000);
    (high, x - high)
}

#[allow(clippy::cyclomatic_complexity)]
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn powf(x: f32, y: f32) -> f32 {
    let mut z: f32;
    let mut ax: f32;
    let mut sn: f32;
    let mut t: f32;
    let mut n: i32;

    let hx = x.to_bits() as i32;
    let hy = y.to_bits() as i32;

    let mut ix = hx & IF_ABS;
    let iy = hy & IF_ABS;

    /* x**0 = 1, even if x is NaN */
    if iy == 0 {
        return 1.;
    }

    /* 1**y = 1, even if y is NaN */
    if hx == IF_1 {
        return 1.;
    }

    /* NaN if either arg is NaN */
    if ix > IF_INF || iy > IF_INF {
        return x + y;
    }

    /* determine if y is an odd int when x < 0
     * yisint = 0       ... y is not an integer
     * yisint = 1       ... y is an odd int
     * yisint = 2       ... y is an even int
     */
    let mut yisint = 0;
    if hx < 0 {
        if iy >= 0x_4b80_0000 {
            yisint = 2; /* even integer y */
        } else if iy >= IF_1 {
            let k = (iy >> 23) - 0x7f; /* exponent */
            let j = iy >> (23 - k);
            if (j << (23 - k)) == iy {
                yisint = 2 - (j & 1);
            }
        }
    }

    /* special value of y */
    if iy == IF_INF {
        /* y is +-inf */
        return if ix == IF_1 {
            /* (-1)**+-inf is 1 */
            1.
        } else if ix > IF_1 {
            /* (|x|>1)**+-inf = inf,0 */
            if hy >= 0 {
                y
            } else {
                0.
            }
        } else {
            /* (|x|<1)**+-inf = 0,inf */
            if hy >= 0 {
                0.
            } else {
                -y
            }
        };
    }
    if iy == IF_1 {
        /* y is +-1 */
        return if hy >= 0 { x } else { 1. / x };
    }

    if hy == 0x_4000_0000 {
        /* y is 2 */
        return x * x;
    }

    if (hy == 0x_3f00_0000) && (hx >= 0) {
        /* y is  0.5 */
        /* x >= +0 */
        return sqrtf(x);
    }

    ax = fabsf(x);
    /* special value of x */
    if ix == 0x_7f80_0000 || ix == 0 || ix == 0x_3f80_0000 {
        /* x is +-0,+-inf,+-1 */
        z = if hy < 0 {
            /* z = (1/|x|) */
            1. / ax
        } else {
            ax
        };

        if hx < 0 {
            if ((ix - 0x_3f80_0000) | yisint) == 0 {
                z = (z - z) / (z - z); /* (-1)**non-int is NaN */
            } else if yisint == 1 {
                z = -z; /* (x<0)**odd = -(|x|**odd) */
            }
        }
        return z;
    }

    sn = 1.; /* sign of result */
    if hx < 0 {
        if yisint == 0 {
            /* (x<0)**(non-int) is NaN */
            return (x - x) / (x - x);
        }

        if yisint == 1 {
            /* (x<0)**(odd int) */
            sn = -1.;
        }
    }

    /* |y| is HUGE */
    let (t1, t2) = if iy > 0x_4d00_0000 {
        /* if |y| > 2**27 */
        /* over/underflow if x is not close to one */
        if ix < 0x_3f7f_fff8 {
            return if hy < 0 {
                sn * HUGE * HUGE
            } else {
                sn * TINY * TINY
            };
        }

        if ix > 0x_3f80_0007 {
            return if hy > 0 {
                sn * HUGE * HUGE
            } else {
                sn * TINY * TINY
            };
        }

        /* now |1-x| is TINY <= 2**-20, suffice to compute
        log(x) by x-x^2/2+x^3/3-x^4/4 */
        t = ax - 1.; /* t has 20 trailing zeros */
        let w = (t * t) * (0.5 - t * (0.333_333_333_333 - t * 0.25));
        let u = IVLN2_H * t; /* IVLN2_H has 16 sig. bits */
        let v = t * IVLN2_L - w * IVLN2;
        high_low(u + v)
    } else {
        /* take care subnormal number */
        n = if ix < IF_MIN {
            ax *= TWO24;
            ix = ax.to_bits() as i32;
            -24
        } else {
            0
        };
        n += ((ix) >> 23) - 0x7f;
        let j = ix & 0x_007f_ffff;
        /* determine interval */
        ix = j | IF_1; /* normalize ix */
        let k = if j <= 0x_001c_c471 {
            /* |x|<sqrt(3/2) */
            0_usize
        } else if j < 0x_005d_b3d7 {
            /* |x|<sqrt(3)   */
            1
        } else {
            n += 1;
            ix -= IF_MIN;
            0
        };
        ax = f32::from_bits(ix as u32);

        /* compute s = s_h+s_l = (x-1)/(x+1) or (x-1.5)/(x+1.5) */
        let u = ax - BP[k]; /* bp[0]=1.0, bp[1]=1.5 */
        let v = 1. / (ax + BP[k]);
        let s = u * v;
        let s_h = f32::from_bits(s.to_bits() & 0x_ffff_f000);
        /* t_h=ax+bp[k] High */
        let is = (((ix as u32 >> 1) & 0x_ffff_f000) | 0x_2000_0000) as i32;
        let t_h = f32::from_bits(is as u32 + 0x_0040_0000 + ((k as u32) << 21));
        let t_l = ax - (t_h - BP[k]);
        let s_l = v * ((u - s_h * t_h) - s_h * t_l);
        /* compute log(ax) */
        let s2 = s * s;
        let mut r = s2 * s2 * (L1 + s2 * (L2 + s2 * (L3 + s2 * (L4 + s2 * (L5 + s2 * L6)))));
        r += s_l * (s_h + s);
        let s2 = s_h * s_h;
        let (t_h, t_l) = high_low(3. + s2 + r);
        /* u+v = s*(1+...) */
        let u = s_h * t_h;
        let v = s_l * t_h + t_l * s;
        /* 2/(3log2)*(s+...) */
        let (p_h, p_l) = high_low(u + v);
        let z_h = CP_H * p_h; /* cp_h+cp_l = 2/(3*log2) */
        let z_l = CP_L * p_h + p_l * CP + DP_L[k];
        /* log2(ax) = (s+..)*2/(3*log2) = n + dp_h + z_h + z_l */
        t = n as f32;
        let t1 = f32::from_bits((z_h + z_l + DP_H[k] + t).to_bits() & 0x_ffff_f000);
        (t1, z_l - (t1 - t - DP_H[k] - z_h))
    };

    /* split up y into y1+y2 and compute (y1+y2)*(t1+t2) */
    let (y1, y2) = high_low(y);
    let p_l = y2 * t1 + y * t2;
    let mut p_h = y1 * t1;
    z = p_l + p_h;
    let j = z.to_bits() as i32;
    if j > 0x_4300_0000 {
        /* if z > 128 */
        return sn * HUGE * HUGE; /* overflow */
    } else if j == 0x_4300_0000 {
        /* if z == 128 */
        if p_l + OVT > z - p_h {
            return sn * HUGE * HUGE; /* overflow */
        }
    } else if (j & IF_ABS) > 0x_4316_0000 {
        /* z < -150 */
        // FIXME: check should be  (uint32_t)j > 0x_c316_0000
        return sn * TINY * TINY; /* underflow */
    } else if j as u32 == 0x_c316_0000 && p_l <= z - p_h {
        /* z == -150 */
        return sn * TINY * TINY; /* underflow */
    }

    /*
     * compute 2**(p_h+p_l)
     */
    let i = j & IF_ABS;
    let mut k = (i >> 23) - 0x7f;
    n = 0;
    if i > 0x_3f00_0000 {
        /* if |z| > 0.5, set n = [z+0.5] */
        n = j + (IF_MIN >> (k + 1));
        k = ((n & IF_ABS) >> 23) - 0x7f; /* new k for n */
        t = f32::from_bits(n as u32 & !(0x_007f_ffff >> k));
        n = ((n & 0x_007f_ffff) | IF_MIN) >> (23 - k);
        if j < 0 {
            n = -n;
        }
        p_h -= t;
    }
    t = f32::from_bits((p_l + p_h).to_bits() & 0x_ffff_8000);
    let u = t * LG2_H;
    let v = (p_l - (t - p_h)) * LG2 + t * LG2_L;
    z = u + v;
    let w = v - (z - u);
    t = z * z;
    let t1 = z - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))));
    let r = (z * t1) / (t1 - 2.) - (w + z * w);
    z = 1. - (r - z);
    let j = (z.to_bits() as i32) + (n << 23);
    z = if (j >> 23) <= 0 {
        /* subnormal output */
        scalbnf(z, n)
    } else {
        f32::from_bits(j as u32)
    };
    sn * z
}
