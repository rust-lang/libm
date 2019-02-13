/* origin: FreeBSD /usr/src/lib/msun/src/e_jnf.c */
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

use super::{fabsf, j0f, j1f, logf, y0f, y1f};
use crate::math::consts::*;
use core::f32;

pub fn jnf(n: isize, mut x: f32) -> f32 {
    let mut ix: u32;
    let mut nm1: isize;
    let mut sign: bool;
    let mut i: isize;
    let mut a: f32;
    let mut b: f32;
    let mut temp: f32;

    ix = x.to_bits();
    sign = (ix >> 31) != 0;
    ix &= UF_ABS;
    if ix > UF_INF {
        /* nan */
        return x;
    }

    /* J(-n,x) = J(n,-x), use |n|-1 to avoid overflow in -n */
    if n == 0 {
        return j0f(x);
    }
    if n < 0 {
        nm1 = -(n + 1);
        x = -x;
        sign = !sign;
    } else {
        nm1 = n - 1;
    }
    if nm1 == 0 {
        return j1f(x);
    }

    sign &= (n & 1) != 0; /* even n: 0, odd n: signbit(x) */
    x = fabsf(x);
    if ix == 0 || ix == UF_INF {
        /* if x is 0 or inf */
        b = 0.;
    } else if (nm1 as f32) < x {
        /* Safe to use J(n+1,x)=2n/x *J(n,x)-J(n-1,x) */
        a = j0f(x);
        b = j1f(x);
        i = 0;
        while i < nm1 {
            i += 1;
            temp = b;
            b = b * (2. * (i as f32) / x) - a;
            a = temp;
        }
    } else if ix < 0x_3580_0000 {
        /* x < 2**-20 */
        /* x is tiny, return the first Taylor expansion of J(n,x)
         * J(n,x) = 1/n!*(x/2)^n  - ...
         */
        if nm1 > 8 {
            /* underflow */
            nm1 = 8;
        }
        temp = 0.5 * x;
        b = temp;
        a = 1.;
        i = 2;
        while i <= nm1 + 1 {
            a *= i as f32; /* a = n! */
            b *= temp; /* b = (x/2)^n */
            i += 1;
        }
        b /= a;
    } else {
        /* use backward recurrence */
        /*                      x      x^2      x^2
         *  J(n,x)/J(n-1,x) =  ----   ------   ------   .....
         *                      2n  - 2(n+1) - 2(n+2)
         *
         *                      1      1        1
         *  (for large x)   =  ----  ------   ------   .....
         *                      2n   2(n+1)   2(n+2)
         *                      -- - ------ - ------ -
         *                       x     x         x
         *
         * Let w = 2n/x and h=2/x, then the above quotient
         * is equal to the continued fraction:
         *                  1
         *      = -----------------------
         *                     1
         *         w - -----------------
         *                        1
         *              w+h - ---------
         *                     w+2h - ...
         *
         * To determine how many terms needed, let
         * Q(0) = w, Q(1) = w(w+h) - 1,
         * Q(k) = (w+k*h)*Q(k-1) - Q(k-2),
         * When Q(k) > 1e4      good for single
         * When Q(k) > 1e9      good for double
         * When Q(k) > 1e17     good for quadruple
         */
        /* determine k */
        let mut t: f32;
        let mut q0: f32;
        let mut q1: f32;
        let mut w: f32;
        let h: f32;
        let mut z: f32;
        let mut tmp: f32;
        let nf: f32;
        let mut k: isize;

        nf = (nm1 as f32) + 1.;
        w = 2. * (nf as f32) / x;
        h = 2. / x;
        z = w + h;
        q0 = w;
        q1 = w * z - 1.;
        k = 1;
        while q1 < 1e4 {
            k += 1;
            z += h;
            tmp = z * q1 - q0;
            q0 = q1;
            q1 = tmp;
        }
        t = 0.;
        i = k;
        while i >= 0 {
            t = 1. / (2. * ((i as f32) + nf) / x - t);
            i -= 1;
        }
        a = t;
        b = 1.;
        /*  estimate log((2/x)^n*n!) = n*log(2/x)+n*ln(n)
         *  Hence, if n*(log(2n/x)) > ...
         *  single 8.872_283_935_5_e+01
         *  double 7.097_827_128_933_839_730_96_e+02
         *  long double 1.135_652_340_629_414_394949_193_107_797_076_50061_70_e+04
         *  then recurrent value may overflow and the result is
         *  likely underflow to zero
         */
        tmp = nf * logf(fabsf(w));
        if tmp < 88.721_679_688 {
            i = nm1;
            while i > 0 {
                temp = b;
                b = 2. * (i as f32) * b / x - a;
                a = temp;
                i -= 1;
            }
        } else {
            i = nm1;
            while i > 0 {
                temp = b;
                b = 2. * (i as f32) * b / x - a;
                a = temp;
                /* scale b to avoid spurious overflow */
                let x1p60 = f32::from_bits(0x_5d80_0000); // 0x1p60 == 2^60
                if b > x1p60 {
                    a /= b;
                    t /= b;
                    b = 1.;
                }
                i -= 1;
            }
        }
        z = j0f(x);
        w = j1f(x);
        if fabsf(z) >= fabsf(w) {
            b = t * z / b;
        } else {
            b = t * w / a;
        }
    }

    if sign {
        -b
    } else {
        b
    }
}

pub fn ynf(n: isize, x: f32) -> f32 {
    let mut ix: u32;
    let mut ib: u32;
    let nm1: isize;
    let mut sign: bool;
    let mut i: isize;
    let mut a: f32;
    let mut b: f32;
    let mut temp: f32;

    ix = x.to_bits();
    sign = (ix >> 31) != 0;
    ix &= UF_ABS;
    if ix > UF_INF {
        /* nan */
        return x;
    }
    if sign && ix != 0 {
        /* x < 0 */
        return f32::NAN;
    }
    if ix == UF_INF {
        return 0.;
    }

    if n == 0 {
        return y0f(x);
    }
    if n < 0 {
        nm1 = -(n + 1);
        sign = (n & 1) != 0;
    } else {
        nm1 = n - 1;
        sign = false;
    }
    if nm1 == 0 {
        if sign {
            return -y1f(x);
        } else {
            return y1f(x);
        }
    }

    a = y0f(x);
    b = y1f(x);
    /* quit if b is -inf */
    ib = b.to_bits();
    i = 0;
    while i < nm1 && ib != 0x_ff80_0000 {
        i += 1;
        temp = b;
        b = (2.0 * (i as f32) / x) * b - a;
        ib = b.to_bits();
        a = temp;
    }

    if sign {
        -b
    } else {
        b
    }
}
