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
/* pow(x,y) return x**y
 *
 *                    n
 * Method:  Let x =  2   * (1+f)
 *      1. Compute and return log2(x) in two pieces:
 *              log2(x) = w1 + w2,
 *         where w1 has 53-24 = 29 bit trailing zeros.
 *      2. Perform y*log2(x) = n+y' by simulating muti-precision
 *         arithmetic, where |y'|<=0.5.
 *      3. Return x**y = 2**n*exp(y'*log2)
 *
 * Special cases:
 *      1.  (anything) ** 0  is 1
 *      2.  1 ** (anything)  is 1
 *      3.  (anything except 1) ** NAN is NAN
 *      4.  NAN ** (anything except 0) is NAN
 *      5.  +-(|x| > 1) **  +INF is +INF
 *      6.  +-(|x| > 1) **  -INF is +0
 *      7.  +-(|x| < 1) **  +INF is +0
 *      8.  +-(|x| < 1) **  -INF is +INF
 *      9.  -1          ** +-INF is 1
 *      10. +0 ** (+anything except 0, NAN)               is +0
 *      11. -0 ** (+anything except 0, NAN, odd integer)  is +0
 *      12. +0 ** (-anything except 0, NAN)               is +INF, raise divbyzero
 *      13. -0 ** (-anything except 0, NAN, odd integer)  is +INF, raise divbyzero
 *      14. -0 ** (+odd integer) is -0
 *      15. -0 ** (-odd integer) is -INF, raise divbyzero
 *      16. +INF ** (+anything except 0,NAN) is +INF
 *      17. +INF ** (-anything except 0,NAN) is +0
 *      18. -INF ** (+odd integer) is -INF
 *      19. -INF ** (anything) = -0 ** (-anything), (anything except odd integer)
 *      20. (anything) ** 1 is (anything)
 *      21. (anything) ** -1 is 1/(anything)
 *      22. (-anything) ** (integer) is (-1)**(integer)*(+anything**integer)
 *      23. (-anything except 0 and inf) ** (non-integer) is NAN
 *
 * Accuracy:
 *      pow(x,y) returns x**y nearly rounded. In particular
 *                      pow(integer,integer)
 *      always returns the correct integer provided it is
 *      representable.
 *
 * Constants :
 * The hexadecimal values are the intended ones for the following
 * constants. The decimal values may be used, provided that the
 * compiler will convert from decimal to binary accurately enough
 * to produce the hexadecimal values shown.
 */

use super::{fabs, scalbn, sqrt, with_set_low_word, with_set_high_word};

fn i32_to_u32(x: i32) -> u32 {
    unsafe { ::core::mem::transmute(x) }
}

fn u32_to_i32(x: u32) -> i32 {
    unsafe { ::core::mem::transmute(x) }
}

const BP: [f64; 2] = [1.0, 1.5];
const DP_H: [f64; 2] = [0.0, 5.84962487220764160156e-01]; /* 0x3FE2B803, 0x40000000 */
const DP_L: [f64; 2] = [0.0, 1.35003920212974897128e-08]; /* 0x3E4CFDEB, 0x43CFD006 */
const TWO53: f64 = 9007199254740992.0; /* 0x43400000, 0x00000000 */
const HUGE: f64 = 1.0e300;
const TINY: f64 = 1.0e-300;
/* poly coefs for (3/2)*(log(x)-2s-2/3*s**3 */
const L1: f64 =  5.99999999999994648725e-01; /* 0x3FE33333, 0x33333303 */
const L2: f64 =  4.28571428578550184252e-01; /* 0x3FDB6DB6, 0xDB6FABFF */
const L3: f64 =  3.33333329818377432918e-01; /* 0x3FD55555, 0x518F264D */
const L4: f64 =  2.72728123808534006489e-01; /* 0x3FD17460, 0xA91D4101 */
const L5: f64 =  2.30660745775561754067e-01; /* 0x3FCD864A, 0x93C9DB65 */
const L6: f64 =  2.06975017800338417784e-01; /* 0x3FCA7E28, 0x4A454EEF */
const P1: f64 =  1.66666666666666019037e-01; /* 0x3FC55555, 0x5555553E */
const P2: f64 = -2.77777777770155933842e-03; /* 0xBF66C16C, 0x16BEBD93 */
const P3: f64 =  6.61375632143793436117e-05; /* 0x3F11566A, 0xAF25DE2C */
const P4: f64 = -1.65339022054652515390e-06; /* 0xBEBBBD41, 0xC5D26BF1 */
const P5: f64 =  4.13813679705723846039e-08; /* 0x3E663769, 0x72BEA4D0 */
const LG2: f64     =  6.93147180559945286227e-01; /* 0x3FE62E42, 0xFEFA39EF */
const LG2_H: f64   =  6.93147182464599609375e-01; /* 0x3FE62E43, 0x00000000 */
const LG2_L: f64   = -1.90465429995776804525e-09; /* 0xBE205C61, 0x0CA86C39 */
const OVT:     f64 =  8.0085662595372944372e-017; /* -(1024-log2(ovfl+.5ulp)) */
const CP:      f64 =  9.61796693925975554329e-01; /* 0x3FEEC709, 0xDC3A03FD =2/(3ln2) */
const CP_H:    f64 =  9.61796700954437255859e-01; /* 0x3FEEC709, 0xE0000000 =(float)cp */
const CP_L:    f64 = -7.02846165095275826516e-09; /* 0xBE3E2FE0, 0x145B01F5 =tail of cp_h*/
const IVLN2:   f64 =  1.44269504088896338700e+00; /* 0x3FF71547, 0x652B82FE =1/ln2 */
const IVLN2_H: f64 =  1.44269502162933349609e+00; /* 0x3FF71547, 0x60000000 =24b 1/ln2*/
const IVLN2_L: f64 =  1.92596299112661746887e-08; /* 0x3E54AE0B, 0xF85DDF44 =1/ln2 tail*/

#[inline]
pub fn pow(x: f64, mut y: f64) -> f64 {
    let mut z: f64;
    let mut ax: f64;
    let z_h: f64;
    let z_l: f64;
    let mut p_h: f64;
    let mut p_l: f64;
    let mut y1: f64;
    let mut t1: f64;
    let t2: f64;
    let mut r: f64;
    let mut s: f64;
    let mut t: f64;
    let mut u: f64;
    let mut v: f64;
    let mut w: f64;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut yisint: i32;
    let mut n: i32;
    let hx: i32;
    let hy: i32;
    let mut ix: i32;
    let iy: i32;
    let lx: u32;
    let ly: u32;

    hx = (x.to_bits() >> 32) as i32;
    lx = x.to_bits() as u32;
    hy = (y.to_bits() >> 32) as i32;
    ly = y.to_bits() as u32;

    ix = hx & 0x7fffffff;
    iy = hy & 0x7fffffff;

    /* x**0 = 1, even if x is NaN */
    if (i32_to_u32(iy)|ly) == 0 {
        return 1.0;
    }
    /* 1**y = 1, even if y is NaN */
    if hx == 0x3ff00000 && lx == 0 {
        return 1.0;
    }
    /* NaN if either arg is NaN */
    if ix > 0x7ff00000 || (ix == 0x7ff00000 && lx != 0) ||
       iy > 0x7ff00000 || (iy == 0x7ff00000 && ly != 0) {
        return x + y;
    }

    /* determine if y is an odd int when x < 0
     * yisint = 0       ... y is not an integer
     * yisint = 1       ... y is an odd int
     * yisint = 2       ... y is an even int
     */
    yisint = 0;
    if hx < 0 {
        if iy >= 0x43400000 {
            yisint = 2; /* even integer y */
        } else if iy >= 0x3ff00000 {
            k = (iy>>20) - 0x3ff;  /* exponent */
            debug_assert!(k >= 0);
            debug_assert!(k < 53);
            if k > 20 {
                let j = ly>>i32_to_u32(52-k);
                if (j<<(52-k)) == ly {
                    yisint = 2 - u32_to_i32(j&1)
                };
            } else if ly == 0 {
                let j = i32_to_u32(iy>>(20-k));
                if u32_to_i32(j<<(20-k)) == iy {
                    yisint = 2 - u32_to_i32(j&1);
                }
            }
        }
    }

    /* special value of y */
    if ly == 0 {
        if iy == 0x7ff00000 {  /* y is +-inf */
            if (i32_to_u32(ix-0x3ff00000)|lx) == 0 {  /* (-1)**+-inf is 1 */
                return 1.0;
            } else if ix >= 0x3ff00000 { /* (|x|>1)**+-inf = inf,0 */
                return if hy >= 0 { y } else { 0.0 };
            } else {                     /* (|x|<1)**+-inf = 0,inf */
                return if hy >= 0 { 0.0 } else { -y };
            }
        }
        if iy == 0x3ff00000 {    /* y is +-1 */
            if hy >= 0 {
                return x;
            }
            y = 1./x;
/*
#if FLT_EVAL_METHOD!=0
            {
                union {double f; uint64_t i;} u = {y};
                uint64_t i = u.i & -1ULL/2;
                if (i>>52 == 0 && (i&(i-1)))
                    FORCE_EVAL((float)y);
            }
#endif
*/
            return y;
        }
        if hy == 0x40000000 {  /* y is 2 */
            return x*x;
        }
        if hy == 0x3fe00000 {  /* y is 0.5 */
            if hx >= 0 {       /* x >= +0 */
                return sqrt(x);
            }
        }
    }

    ax = fabs(x);
    /* special value of x */
    if lx == 0 {
        if ix == 0x7ff00000 || ix == 0 || ix == 0x3ff00000 { /* x is +-0,+-inf,+-1 */
            z = ax;
            if hy < 0 {  /* z = (1/|x|) */
                z = 1.0/z;
            }
            if hx < 0 {
                if ((ix-0x3ff00000)|yisint) == 0 {
                    z = (z-z)/(z-z); /* (-1)**non-int is NaN */
                } else if yisint == 1 {
                    z = -z;          /* (x<0)**odd = -(|x|**odd) */
                }
            }
            return z;
        }
    }

    s = 1.0; /* sign of result */
    if hx < 0 {
        if yisint == 0 { /* (x<0)**(non-int) is NaN */
            return (x-x)/(x-x);
        }
        if yisint == 1 { /* (x<0)**(odd int) */
            s = -1.0;
        }
    }

    /* |y| is huge */
    if iy > 0x41e00000 { /* if |y| > 2**31 */
        if iy > 0x43f00000 {  /* if |y| > 2**64, must o/uflow */
            if ix <= 0x3fefffff {
                return if hy < 0 { HUGE*HUGE } else { TINY*TINY };
            }
            if ix >= 0x3ff00000 {
                return if hy > 0 { HUGE*HUGE } else { TINY*TINY };
            }
        }
        /* over/underflow if x is not close to one */
        if ix < 0x3fefffff {
            return if hy < 0 { s*HUGE*HUGE } else { s*TINY*TINY };
        }
        if ix > 0x3ff00000 {
            return if hy > 0 { s*HUGE*HUGE } else { s*TINY*TINY };
        }
        /* now |1-x| is tiny <= 2**-20, suffice to compute
           log(x) by x-x^2/2+x^3/3-x^4/4 */
        t = ax - 1.0;       /* t has 20 trailing zeros */
        w = (t*t)*(0.5 - t*(0.3333333333333333333333-t*0.25));
        u = IVLN2_H*t;      /* ivln2_h has 21 sig. bits */
        v = t*IVLN2_L - w*IVLN2;
        t1 = u + v;
        t1 = with_set_low_word(t1, 0);
        t2 = v - (t1-u);
    } else {
        let ss: f64;
        let mut s2: f64;
        let mut s_h: f64;
        let s_l: f64;
        let mut t_h: f64;
        let mut t_l: f64;
        n = 0;
        /* take care subnormal number */
        if ix < 0x00100000 {
            ax *= TWO53;
            n -= 53;
            ix = (f64::to_bits(ax) >> 32) as i32;
        }
        n += ((ix)>>20) - 0x3ff;
        j = ix & 0x000fffff;
        /* determine interval */
        ix = j | 0x3ff00000;    /* normalize ix */
        if j <= 0x3988E {       /* |x|<sqrt(3/2) */
            k = 0;
        } else if j < 0xBB67A { /* |x|<sqrt(3)   */
            k = 1;
        } else {
            k = 0;
            n += 1;
            ix -= 0x00100000;
        }
        ax = with_set_high_word(ax, i32_to_u32(ix));

        /* compute ss = s_h+s_l = (x-1)/(x+1) or (x-1.5)/(x+1.5) */
        u = ax - BP[k as usize];        /* bp[0]=1.0, bp[1]=1.5 */
        v = 1.0/(ax+BP[k as usize]);
        ss = u*v;
        s_h = ss;
        s_h = with_set_low_word(s_h, 0);
        /* t_h=ax+bp[k] High */
        t_h = 0.0;
        t_h = with_set_high_word(t_h, i32_to_u32(((ix>>1)|0x20000000) + 0x00080000 + (k<<18)));
        t_l = ax - (t_h-BP[k as usize]);
        s_l = v*((u-s_h*t_h)-s_h*t_l);
        /* compute log(ax) */
        s2 = ss*ss;
        r = s2*s2*(L1+s2*(L2+s2*(L3+s2*(L4+s2*(L5+s2*L6)))));
        r += s_l*(s_h+ss);
        s2 = s_h*s_h;
        t_h = 3.0 + s2 + r;
        t_h = with_set_low_word(t_h, 0);
        t_l = r - ((t_h-3.0)-s2);
        /* u+v = ss*(1+...) */
        u = s_h*t_h;
        v = s_l*t_h + t_l*ss;
        /* 2/(3log2)*(ss+...) */
        p_h = u + v;
        p_h = with_set_low_word(p_h, 0);
        p_l = v - (p_h-u);
        z_h = CP_H*p_h;        /* cp_h+cp_l = 2/(3*log2) */
        z_l = CP_L*p_h+p_l*CP + DP_L[k as usize];
        /* log2(ax) = (ss+..)*2/(3*log2) = n + DP_H + z_h + z_l */
        t = n as f64;
        t1 = ((z_h + z_l) + DP_H[k as usize]) + t;
        t1 = with_set_low_word(t1, 0);
        t2 = z_l - (((t1 - t) - DP_H[k as usize]) - z_h);
    }

    /* split up y into y1+y2 and compute (y1+y2)*(t1+t2) */
    y1 = y;
    y1 = with_set_low_word(y1, 0);
    p_l = (y-y1)*t1 + y*t2;
    p_h = y1*t1;
    z = p_l + p_h;
    j = (z.to_bits() >> 32) as i32;
    i = z.to_bits() as i32;
    if j >= 0x40900000 {                      /* z >= 1024 */
        if ((j-0x40900000)|i) != 0 {          /* if z > 1024 */
            return s*HUGE*HUGE;         /* overflow */
        }
        if p_l + OVT > z - p_h {
            return s*HUGE*HUGE;         /* overflow */
        }
    } else if (j&0x7fffffff) >= 0x4090cc00 {  /* z <= -1075 */  // FIXME: instead of abs(j) use unsigned j
        if (j-u32_to_i32(0xc090cc00))|i != 0 {          /* z < -1075 */
            return s*TINY*TINY;         /* underflow */
        }
        if p_l <= z - p_h {
            return s*TINY*TINY;         /* underflow */
        }
    }
    /*
     * compute 2**(p_h+p_l)
     */
    i = j & 0x7fffffff;
    k = (i>>20) - 0x3ff;
    n = 0;
    if i > 0x3fe00000 {  /* if |z| > 0.5, set n = [z+0.5] */
        n = j + (0x00100000>>(k+1));
        k = ((n&0x7fffffff)>>20) - 0x3ff;  /* new k for n */
        t = 0.0;
        t = with_set_low_word(t, i32_to_u32(n & !(0x000fffff>>k)));
        n = ((n&0x000fffff)|0x00100000)>>(20-k);
        if j < 0 {
            n = -n;
        }
        p_h -= t;
    }
    t = p_l + p_h;
    t = with_set_low_word(t, 0);
    u = t*LG2_H;
    v = (p_l-(t-p_h))*LG2 + t*LG2_L;
    z = u + v;
    w = v - (z-u);
    t = z*z;
    t1 = z - t*(P1+t*(P2+t*(P3+t*(P4+t*P5))));
    r = (z*t1)/(t1-2.0) - (w + z*w);
    z = 1.0 - (r-z);
    j = (f64::to_bits(z) >> 32) as i32;
    j += n<<20;
    if (j>>20) <= 0 { /* subnormal output */
        z = scalbn(z,n);
    } else {
        z = with_set_high_word(z, i32_to_u32(j));
    }
    return s*z;
}
