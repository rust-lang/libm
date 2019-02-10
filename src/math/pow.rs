/* origin: FreeBSD /usr/src/lib/msun/src/e_pow.c */
/*
 * ====================================================
 * Copyright (C) 2004 by Sun Microsystems, Inc. All rights reserved.
 *
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

// pow(x,y) return x**y
//
//                    n
// Method:  Let x =  2   * (1+f)
//      1. Compute and return log2(x) in two pieces:
//              log2(x) = w1 + w2,
//         where w1 has 53-24 = 29 bit trailing zeros.
//      2. Perform y*log2(x) = n+y' by simulating muti-precision
//         arithmetic, where |y'|<=0.5.
//      3. Return x**y = 2**n*exp(y'*log2)
//
// Special cases:
//      1.  (anything) ** 0  is 1
//      2.  1 ** (anything)  is 1
//      3.  (anything except 1) ** NAN is NAN
//      4.  NAN ** (anything except 0) is NAN
//      5.  +-(|x| > 1) **  +INF is +INF
//      6.  +-(|x| > 1) **  -INF is +0
//      7.  +-(|x| < 1) **  +INF is +0
//      8.  +-(|x| < 1) **  -INF is +INF
//      9.  -1          ** +-INF is 1
//      10. +0 ** (+anything except 0, NAN)               is +0
//      11. -0 ** (+anything except 0, NAN, odd integer)  is +0
//      12. +0 ** (-anything except 0, NAN)               is +INF, raise divbyzero
//      13. -0 ** (-anything except 0, NAN, odd integer)  is +INF, raise divbyzero
//      14. -0 ** (+odd integer) is -0
//      15. -0 ** (-odd integer) is -INF, raise divbyzero
//      16. +INF ** (+anything except 0,NAN) is +INF
//      17. +INF ** (-anything except 0,NAN) is +0
//      18. -INF ** (+odd integer) is -INF
//      19. -INF ** (anything) = -0 ** (-anything), (anything except odd integer)
//      20. (anything) ** 1 is (anything)
//      21. (anything) ** -1 is 1/(anything)
//      22. (-anything) ** (integer) is (-1)**(integer)*(+anything**integer)
//      23. (-anything except 0 and inf) ** (non-integer) is NAN
//
// Accuracy:
//      pow(x,y) returns x**y nearly rounded. In particular
//                      pow(integer,integer)
//      always returns the correct integer provided it is
//      representable.
//
// Constants :
// The hexadecimal values are the intended ones for the following
// constants. The decimal values may be used, provided that the
// compiler will convert from decimal to binary accurately enough
// to produce the hexadecimal values shown.
//

use core::f64;
use super::{fabs, get_high_word, scalbn, sqrt, with_set_high_word, with_set_low_word};
use math::consts::*;

const BP: [f64; 2] = [1., 1.5];
const DP_H: [f64; 2] = [0., 5.849_624_872_207_641_601_56_e-01]; /* 0x_3fe2_b803_4000_0000 */
const DP_L: [f64; 2] = [0., 1.350_039_202_129_748_971_28_e-08]; /* 0x_3E4C_FDEB, 0x_43CF_D006 */
const TWO53: f64 = 9_007_199_254_740_992.; /* 0x_4340_0000_0000_0000 */
const HUGE: f64 = 1_e300;
const TINY: f64 = 1_e-300;

// poly coefs for (3/2)*(log(x)-2s-2/3*s**3:
const L1: f64 = 5.999_999_999_999_946_487_25_e-01; /* 0x_3fe3_3333_3333_3303 */
const L2: f64 = 4.285_714_285_785_501_842_52_e-01; /* 0x_3fdb_6db6_db6f_abff */
const L3: f64 = 3.333_333_298_183_774_329_18_e-01; /* 0x_3fd5_5555_518f_264d */
const L4: f64 = 2.727_281_238_085_340_064_89_e-01; /* 0x_3fd1_7460_a91d_4101 */
const L5: f64 = 2.306_607_457_755_617_540_67_e-01; /* 0x_3fcd_864a_93c9_db65 */
const L6: f64 = 2.069_750_178_003_384_177_84_e-01; /* 0x_3fca_7e28_4a45_4eef */
const P1: f64 = 1.666_666_666_666_660_190_37_e-01; /* 0x_3fc5_5555_5555_553e */
const P2: f64 = -2.777_777_777_701_559_338_42_e-03; /* 0x_bf66_c16c_16be_bd93 */
const P3: f64 = 6.613_756_321_437_934_361_17_e-05; /* 0x_3f11_566a_af25_de2c */
const P4: f64 = -1.653_390_220_546_525_153_9_e-06; /* 0x_bebb_bd41_c5d2_6bf1 */
const P5: f64 = 4.138_136_797_057_238_460_39_e-08; /* 0x_3e66_3769_72be_a4d0 */
const LG2: f64 = 6.931_471_805_599_452_862_27_e-01; /* 0x_3fe6_2e42_fefa_39ef */
const LG2_H: f64 = 6.931_471_824_645_996_093_75_e-01; /* 0x_3fe6_2e43_0000_0000 */
const LG2_L: f64 = -1.904_654_299_957_768_045_25_e-09; /* 0x_be20_5c61_0ca8_6c39 */
const OVT: f64 = 8.008_566_259_537_294_437_2_e-017; /* -(1024-log2(ovfl+.5ulp)) */
const CP: f64 = 9.617_966_939_259_755_543_29_e-01; /* 0x_3fee_c709_dc3a_03fd =2/(3ln2) */
const CP_H: f64 = 9.617_967_009_544_372_558_59_e-01; /* 0x_3fee_c709_e000_0000 =(float)cp */
const CP_L: f64 = -7.028_461_650_952_758_265_16_e-09; /* 0x_be3e_2fe0_145b_01f5 =tail of cp_h*/
const IVLN2: f64 = f64::consts::LOG2_E; /* 0x_3ff7_1547_652b_82fe =1/ln2 */
const IVLN2_H: f64 = 1.442_695_021_629_333_496_09; /* 0x_3ff7_1547_6000_0000 =24b 1/ln2*/
const IVLN2_L: f64 = 1.925_962_991_126_617_468_87_e-08; /* 0x_3e54_ae0b_f85d_df44 =1/ln2 tail*/

#[allow(clippy::cyclomatic_complexity)]
#[inline]
pub fn pow(x: f64, y: f64) -> f64 {
    let t1: f64;
    let t2: f64;

    let (hx, lx): (i32, u32) = ((x.to_bits() >> 32) as i32, x.to_bits() as u32);
    let (hy, ly): (i32, u32) = ((y.to_bits() >> 32) as i32, y.to_bits() as u32);

    let mut ix: i32 = (hx & IF_ABS) as i32;
    let iy: i32 = (hy & IF_ABS) as i32;

    /* x**0 = 1, even if x is NaN */
    if ((iy as u32) | ly) == 0 {
        return 1.0;
    }

    /* 1**y = 1, even if y is NaN */
    if hx == 0x_3ff0_0000 && lx == 0 {
        return 1.0;
    }

    /* NaN if either arg is NaN */
    if ix > 0x_7ff0_0000
        || (ix == 0x_7ff0_0000 && lx != 0)
        || iy > 0x_7ff0_0000
        || (iy == 0x_7ff0_0000 && ly != 0)
    {
        return x + y;
    }

    /* determine if y is an odd int when x < 0
     * yisint = 0       ... y is not an integer
     * yisint = 1       ... y is an odd int
     * yisint = 2       ... y is an even int
     */
    let mut yisint: i32 = 0;
    let mut k: i32;
    let mut j: i32;
    if hx < 0 {
        if iy >= 0x_4340_0000 {
            yisint = 2; /* even integer y */
        } else if iy >= 0x_3ff0_0000 {
            k = (iy >> 20) - 0x3ff; /* exponent */

            if k > 20 {
                j = (ly >> (52 - k)) as i32;

                if (j << (52 - k)) == (ly as i32) {
                    yisint = 2 - (j & 1);
                }
            } else if ly == 0 {
                j = iy >> (20 - k);

                if (j << (20 - k)) == iy {
                    yisint = 2 - (j & 1);
                }
            }
        }
    }

    if ly == 0 {
        /* special value of y */
        if iy == 0x_7ff0_0000 {
            /* y is +-inf */

            return if ((ix - 0x_3ff0_0000) | (lx as i32)) == 0 {
                /* (-1)**+-inf is 1 */
                1.0
            } else if ix >= 0x_3ff0_0000 {
                /* (|x|>1)**+-inf = inf,0 */
                if hy >= 0 {
                    y
                } else {
                    0.0
                }
            } else {
                /* (|x|<1)**+-inf = 0,inf */
                if hy >= 0 {
                    0.0
                } else {
                    -y
                }
            };
        }

        if iy == 0x_3ff0_0000 {
            /* y is +-1 */
            return if hy >= 0 { x } else { 1.0 / x };
        }

        if hy == 0x_4000_0000 {
            /* y is 2 */
            return x * x;
        }

        if hy == 0x_3fe0_0000 {
            /* y is 0.5 */
            if hx >= 0 {
                /* x >= +0 */
                return sqrt(x);
            }
        }
    }

    let mut ax: f64 = fabs(x);
    if lx == 0 {
        /* special value of x */
        if ix == 0x_7ff0_0000 || ix == 0 || ix == 0x_3ff0_0000 {
            /* x is +-0,+-inf,+-1 */
            let mut z: f64 = ax;

            if hy < 0 {
                /* z = (1/|x|) */
                z = 1.0 / z;
            }

            if hx < 0 {
                if ((ix - 0x_3ff0_0000) | yisint) == 0 {
                    z = f64::NAN; /* (-1)**non-int is NaN */
                } else if yisint == 1 {
                    z = -z; /* (x<0)**odd = -(|x|**odd) */
                }
            }

            return z;
        }
    }

    let mut s: f64 = 1.0; /* sign of result */
    if hx < 0 {
        if yisint == 0 {
            /* (x<0)**(non-int) is NaN */
            return f64::NAN;
        }

        if yisint == 1 {
            /* (x<0)**(odd int) */
            s = -1.;
        }
    }

    /* |y| is HUGE */
    if iy > 0x_41e0_0000 {
        /* if |y| > 2**31 */
        if iy > 0x_43f0_0000 {
            /* if |y| > 2**64, must o/uflow */
            if ix <= 0x_3fef_ffff {
                return if hy < 0 { HUGE * HUGE } else { TINY * TINY };
            }

            if ix >= 0x_3ff0_0000 {
                return if hy > 0 { HUGE * HUGE } else { TINY * TINY };
            }
        }

        /* over/underflow if x is not close to one */
        if ix < 0x_3fef_ffff {
            return if hy < 0 {
                s * HUGE * HUGE
            } else {
                s * TINY * TINY
            };
        }
        if ix > 0x_3ff0_0000 {
            return if hy > 0 {
                s * HUGE * HUGE
            } else {
                s * TINY * TINY
            };
        }

        /* now |1-x| is TINY <= 2**-20, suffice to compute
        log(x) by x-x^2/2+x^3/3-x^4/4 */
        let t: f64 = ax - 1.0; /* t has 20 trailing zeros */
        let w: f64 = (t * t) * (0.5 - t * (0.333_333_333_333_333_333_333_3 - t * 0.25));
        let u: f64 = IVLN2_H * t; /* ivln2_h has 21 sig. bits */
        let v: f64 = t * IVLN2_L - w * IVLN2;
        t1 = with_set_low_word(u + v, 0);
        t2 = v - (t1 - u);
    } else {
        // double ss,s2,s_h,s_l,t_h,t_l;
        let mut n: i32 = 0;

        if ix < 0x_0010_0000 {
            /* take care subnormal number */
            ax *= TWO53;
            n -= 53;
            ix = get_high_word(ax) as i32;
        }

        n += (ix >> 20) - 0x3ff;
        j = ix & 0x_000f_ffff;

        /* determine interval */
        let k: i32;
        ix = j | 0x_3ff0_0000; /* normalize ix */
        if j <= 0x3988E {
            /* |x|<sqrt(3/2) */
            k = 0;
        } else if j < 0xBB67A {
            /* |x|<sqrt(3)   */
            k = 1;
        } else {
            k = 0;
            n += 1;
            ix -= 0x_0010_0000;
        }
        ax = with_set_high_word(ax, ix as u32);

        /* compute ss = s_h+s_l = (x-1)/(x+1) or (x-1.5)/(x+1.5) */
        let u: f64 = ax - BP[k as usize]; /* bp[0]=1.0, bp[1]=1.5 */
        let v: f64 = 1.0 / (ax + BP[k as usize]);
        let ss: f64 = u * v;
        let s_h = with_set_low_word(ss, 0);

        /* t_h=ax+bp[k] High */
        let t_h: f64 = with_set_high_word(
            0.0,
            ((ix as u32 >> 1) | 0x_2000_0000) + 0x_0008_0000 + ((k as u32) << 18),
        );
        let t_l: f64 = ax - (t_h - BP[k as usize]);
        let s_l: f64 = v * ((u - s_h * t_h) - s_h * t_l);

        /* compute log(ax) */
        let s2: f64 = ss * ss;
        let mut r: f64 = s2 * s2 * (L1 + s2 * (L2 + s2 * (L3 + s2 * (L4 + s2 * (L5 + s2 * L6)))));
        r += s_l * (s_h + ss);
        let s2: f64 = s_h * s_h;
        let t_h: f64 = with_set_low_word(3.0 + s2 + r, 0);
        let t_l: f64 = r - ((t_h - 3.0) - s2);

        /* u+v = ss*(1+...) */
        let u: f64 = s_h * t_h;
        let v: f64 = s_l * t_h + t_l * ss;

        /* 2/(3log2)*(ss+...) */
        let p_h: f64 = with_set_low_word(u + v, 0);
        let p_l = v - (p_h - u);
        let z_h: f64 = CP_H * p_h; /* cp_h+cp_l = 2/(3*log2) */
        let z_l: f64 = CP_L * p_h + p_l * CP + DP_L[k as usize];

        /* log2(ax) = (ss+..)*2/(3*log2) = n + dp_h + z_h + z_l */
        let t: f64 = n as f64;
        t1 = with_set_low_word(((z_h + z_l) + DP_H[k as usize]) + t, 0);
        t2 = z_l - (((t1 - t) - DP_H[k as usize]) - z_h);
    }

    /* split up y into y1+y2 and compute (y1+y2)*(t1+t2) */
    let y1: f64 = with_set_low_word(y, 0);
    let p_l: f64 = (y - y1) * t1 + y * t2;
    let mut p_h: f64 = y1 * t1;
    let z: f64 = p_l + p_h;
    let mut j: i32 = (z.to_bits() >> 32) as i32;
    let i: i32 = z.to_bits() as i32;
    // let (j, i): (i32, i32) = ((z.to_bits() >> 32) as i32, z.to_bits() as i32);

    if j >= 0x_4090_0000 {
        /* z >= 1024 */
        if (j - 0x_4090_0000) | i != 0 {
            /* if z > 1024 */
            return s * HUGE * HUGE; /* overflow */
        }

        if p_l + OVT > z - p_h {
            return s * HUGE * HUGE; /* overflow */
        }
    } else if (j & IF_ABS) >= 0x_4090_cc00 {
        /* z <= -1075 */
        // FIXME: instead of abs(j) use unsigned j

        if (((j as u32) - 0x_c090_cc00) | (i as u32)) != 0 {
            /* z < -1075 */
            return s * TINY * TINY; /* underflow */
        }

        if p_l <= z - p_h {
            return s * TINY * TINY; /* underflow */
        }
    }

    /* compute 2**(p_h+p_l) */
    let i: i32 = j & (UF_ABS as i32);
    k = (i >> 20) - 0x3ff;
    let mut n: i32 = 0;

    if i > 0x_3fe0_0000 {
        /* if |z| > 0.5, set n = [z+0.5] */
        n = j + (0x_0010_0000 >> (k + 1));
        k = ((n & IF_ABS) >> 20) - 0x3ff; /* new k for n */
        let t: f64 = with_set_high_word(0.0, (n & !(0x_000f_ffff >> k)) as u32);
        n = ((n & 0x_000f_ffff) | 0x_0010_0000) >> (20 - k);
        if j < 0 {
            n = -n;
        }
        p_h -= t;
    }

    let t: f64 = with_set_low_word(p_l + p_h, 0);
    let u: f64 = t * LG2_H;
    let v: f64 = (p_l - (t - p_h)) * LG2 + t * LG2_L;
    let mut z: f64 = u + v;
    let w: f64 = v - (z - u);
    let t: f64 = z * z;
    let t1: f64 = z - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))));
    let r: f64 = (z * t1) / (t1 - 2.0) - (w + z * w);
    z = 1.0 - (r - z);
    j = get_high_word(z) as i32;
    j += n << 20;

    if (j >> 20) <= 0 {
        /* subnormal output */
        z = scalbn(z, n);
    } else {
        z = with_set_high_word(z, j as u32);
    }

    s * z
}
