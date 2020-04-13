/* origin: FreeBSD /usr/src/lib/msun/src/e_jn.c */

use super::{cos, fabs, get_high_word, get_low_word, j0, j1, log, sin, sqrt, y0, y1};

const INVSQRTPI: f64 = 5.64189583547756279280e-01; /* 0x3FE20DD7, 0x50429B6D */

pub fn jn(n: i32, mut x: f64) -> f64 {
    let mut ix: u32;
    let lx: u32;
    let nm1: i32;
    let mut i: i32;
    let mut sign: bool;
    let mut a: f64;
    let mut b: f64;
    let mut temp: f64;

    ix = get_high_word(x);
    lx = get_low_word(x);
    sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    // -lx == !lx + 1
    if (ix | (lx | ((!lx).wrapping_add(1))) >> 31) > 0x7ff00000 {
        /* nan */
        return x;
    }

    /* J(-n,x) = (-1)^n * J(n, x), J(n, -x) = (-1)^n * J(n, x)
     * Thus, J(-n,x) = J(n,-x)
     */
    /* nm1 = |n|-1 is used instead of |n| to handle n==INT_MIN */
    if n == 0 {
        return j0(x);
    }
    if n < 0 {
        nm1 = -(n + 1);
        x = -x;
        sign = !sign;
    } else {
        nm1 = n - 1;
    }
    if nm1 == 0 {
        return j1(x);
    }

    sign &= (n & 1) != 0; /* even n: 0, odd n: signbit(x) */
    x = fabs(x);
    if (ix | lx) == 0 || ix == 0x7ff00000 {
        /* if x is 0 or inf */
        b = 0.0;
    } else if (nm1 as f64) < x {
        /* Safe to use J(n+1,x)=2n/x *J(n,x)-J(n-1,x) */
        if ix >= 0x52d00000 {
            /* x > 2**302 */
            temp = match nm1 & 3 {
                0 => -cos(x) + sin(x),
                1 => -cos(x) - sin(x),
                2 => cos(x) - sin(x),
                3 | _ => cos(x) + sin(x),
            };
            b = INVSQRTPI * temp / sqrt(x);
        } else {
            a = j0(x);
            b = j1(x);
            i = 0;
            while i < nm1 {
                i += 1;
                temp = b;
                b = b * (2.0 * (i as f64) / x) - a; /* avoid underflow */
                a = temp;
            }
        }
    } else {
        if ix < 0x3e100000 {
            /* x < 2**-29 */
            /* x is tiny, return the first Taylor expansion of J(n,x)
             * J(n,x) = 1/n!*(x/2)^n  - ...
             */
            if nm1 > 32 {
                /* underflow */
                b = 0.0;
            } else {
                temp = x * 0.5;
                b = temp;
                a = 1.0;
                i = 2;
                while i <= nm1 + 1 {
                    a *= i as f64; /* a = n! */
                    b *= temp; /* b = (x/2)^n */
                    i += 1;
                }
                b = b / a;
            }
        } else {
            /* use backward recurrence to determine k */
            let mut t: f64;
            let mut q0: f64;
            let mut q1: f64;
            let mut w: f64;
            let h: f64;
            let mut z: f64;
            let mut tmp: f64;
            let nf: f64;

            let mut k: i32;

            nf = (nm1 as f64) + 1.0;
            w = 2.0 * nf / x;
            h = 2.0 / x;
            z = w + h;
            q0 = w;
            q1 = w * z - 1.0;
            k = 1;
            while q1 < 1.0e9 {
                k += 1;
                z += h;
                tmp = z * q1 - q0;
                q0 = q1;
                q1 = tmp;
            }
            t = 0.0;
            i = k;
            while i >= 0 {
                t = 1.0 / (2.0 * ((i as f64) + nf) / x - t);
                i -= 1;
            }
            a = t;
            b = 1.0;
            tmp = nf * log(fabs(w));
            if tmp < 7.09782712893383973096e+02 {
                i = nm1;
                while i > 0 {
                    temp = b;
                    b = b * (2.0 * (i as f64)) / x - a;
                    a = temp;
                    i -= 1;
                }
            } else {
                i = nm1;
                while i > 0 {
                    temp = b;
                    b = b * (2.0 * (i as f64)) / x - a;
                    a = temp;
                    /* scale b to avoid spurious overflow */
                    let x1p500 = f64::from_bits(0x5f30000000000000); // 0x1p500 == 2^500
                    if b > x1p500 {
                        a /= b;
                        t /= b;
                        b = 1.0;
                    }
                    i -= 1;
                }
            }
            z = j0(x);
            w = j1(x);
            if fabs(z) >= fabs(w) {
                b = t * z / b;
            } else {
                b = t * w / a;
            }
        }
    }

    if sign {
        -b
    } else {
        b
    }
}

pub fn yn(n: i32, x: f64) -> f64 {
    let mut ix: u32;
    let lx: u32;
    let mut ib: u32;
    let nm1: i32;
    let mut sign: bool;
    let mut i: i32;
    let mut a: f64;
    let mut b: f64;
    let mut temp: f64;

    ix = get_high_word(x);
    lx = get_low_word(x);
    sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;

    // -lx == !lx + 1
    if (ix | (lx | ((!lx).wrapping_add(1))) >> 31) > 0x7ff00000 {
        /* nan */
        return x;
    }
    if sign && (ix | lx) != 0 {
        /* x < 0 */
        return 0.0 / 0.0;
    }
    if ix == 0x7ff00000 {
        return 0.0;
    }

    if n == 0 {
        return y0(x);
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
            return -y1(x);
        } else {
            return y1(x);
        }
    }

    if ix >= 0x52d00000 {
        /* x > 2**302 */
        temp = match nm1 & 3 {
            0 => -sin(x) - cos(x),
            1 => -sin(x) + cos(x),
            2 => sin(x) + cos(x),
            3 | _ => sin(x) - cos(x),
        };
        b = INVSQRTPI * temp / sqrt(x);
    } else {
        a = y0(x);
        b = y1(x);
        /* quit if b is -inf */
        ib = get_high_word(b);
        i = 0;
        while i < nm1 && ib != 0xfff00000 {
            i += 1;
            temp = b;
            b = (2.0 * (i as f64) / x) * b - a;
            ib = get_high_word(b);
            a = temp;
        }
    }

    if sign {
        -b
    } else {
        b
    }
}
