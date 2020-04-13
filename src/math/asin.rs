/* origin: FreeBSD /usr/src/lib/msun/src/e_asin.c */

use super::{fabs, get_high_word, get_low_word, sqrt, with_set_low_word};

const PIO2_HI: f64 = 1.57079632679489655800e+00; /* 0x3FF921FB, 0x54442D18 */
const PIO2_LO: f64 = 6.12323399573676603587e-17; /* 0x3C91A626, 0x33145C07 */
/* coefficients for R(x^2) */
const P_S0: f64 = 1.66666666666666657415e-01; /* 0x3FC55555, 0x55555555 */
const P_S1: f64 = -3.25565818622400915405e-01; /* 0xBFD4D612, 0x03EB6F7D */
const P_S2: f64 = 2.01212532134862925881e-01; /* 0x3FC9C155, 0x0E884455 */
const P_S3: f64 = -4.00555345006794114027e-02; /* 0xBFA48228, 0xB5688F3B */
const P_S4: f64 = 7.91534994289814532176e-04; /* 0x3F49EFE0, 0x7501B288 */
const P_S5: f64 = 3.47933107596021167570e-05; /* 0x3F023DE1, 0x0DFDF709 */
const Q_S1: f64 = -2.40339491173441421878e+00; /* 0xC0033A27, 0x1C8A2D4B */
const Q_S2: f64 = 2.02094576023350569471e+00; /* 0x40002AE5, 0x9C598AC8 */
const Q_S3: f64 = -6.88283971605453293030e-01; /* 0xBFE6066C, 0x1B8D0159 */
const Q_S4: f64 = 7.70381505559019352791e-02; /* 0x3FB3B8C5, 0xB12E9282 */

fn comp_r(z: f64) -> f64 {
    let p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5)))));
    let q = 1.0 + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4)));
    p / q
}

/// Arcsine (f64)
///
/// Computes the inverse sine (arc sine) of the argument `x`.
/// Arguments to asin must be in the range -1 to 1.
/// Returns values in radians, in the range of -pi/2 to pi/2.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn asin(mut x: f64) -> f64 {
    let z: f64;
    let r: f64;
    let s: f64;
    let hx: u32;
    let ix: u32;

    hx = get_high_word(x);
    ix = hx & 0x7fffffff;
    /* |x| >= 1 or nan */
    if ix >= 0x3ff00000 {
        let lx: u32;
        lx = get_low_word(x);
        if ((ix - 0x3ff00000) | lx) == 0 {
            /* asin(1) = +-pi/2 with inexact */
            return x * PIO2_HI + f64::from_bits(0x3870000000000000);
        } else {
            return 0.0 / (x - x);
        }
    }
    /* |x| < 0.5 */
    if ix < 0x3fe00000 {
        /* if 0x1p-1022 <= |x| < 0x1p-26, avoid raising underflow */
        if ix < 0x3e500000 && ix >= 0x00100000 {
            return x;
        } else {
            return x + x * comp_r(x * x);
        }
    }
    /* 1 > |x| >= 0.5 */
    z = (1.0 - fabs(x)) * 0.5;
    s = sqrt(z);
    r = comp_r(z);
    if ix >= 0x3fef3333 {
        /* if |x| > 0.975 */
        x = PIO2_HI - (2. * (s + s * r) - PIO2_LO);
    } else {
        let f: f64;
        let c: f64;
        /* f+c = sqrt(z) */
        f = with_set_low_word(s, 0);
        c = (z - f * f) / (s + f);
        x = 0.5 * PIO2_HI - (2.0 * s * r - (PIO2_LO - 2.0 * c) - (0.5 * PIO2_HI - 2.0 * f));
    }
    if hx >> 31 != 0 {
        -x
    } else {
        x
    }
}
