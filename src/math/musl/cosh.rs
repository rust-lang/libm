use super::exp;
use super::expm1;
use super::k_expo2;
use crate::math::consts::*;

/// Hyperbolic cosine (f64)
///
/// Computes the hyperbolic cosine of the argument x.
/// Is defined as `(exp(x) + exp(-x))/2`
/// Angles are specified in radians.
#[inline]
pub fn cosh(mut x: f64) -> f64 {
    /* |x| */
    let mut ix = x.to_bits();
    ix &= UD_ABS;
    x = f64::from_bits(ix);
    let w = ix >> 32;

    /* |x| < log(2) */
    if w < 0x_3fe6_2e42 {
        if w < 0x_3ff0_0000 - (26 << 20) {
            let x1p120 = f64::from_bits(0x_4770_0000_0000_0000);
            force_eval!(x + x1p120);
            return 1.;
        }
        let t = expm1(x); // exponential minus 1
        return 1. + t * t / (2. * (1. + t));
    }

    /* |x| < log(DBL_MAX) */
    if w < 0x_4086_2e42 {
        let t = exp(x);
        /* note: if x>log(0x1p26) then the 1/t is not needed */
        return 0.5 * (t + 1. / t);
    }

    /* |x| > log(DBL_MAX) or nan */
    k_expo2(x)
}
