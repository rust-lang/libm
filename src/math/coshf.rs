use super::expf;
use super::expm1f;
use super::k_expo2f;
use math::consts::*;

#[inline]
pub fn coshf(mut x: f32) -> f32 {
    let x1p120 = f32::from_bits(0x_7b80_0000); // 0x1p120f === 2 ^ 120

    /* |x| */
    let mut ix = x.to_bits();
    ix &= UF_ABS;
    x = f32::from_bits(ix);
    let w = ix;

    /* |x| < log(2) */
    if w < 0x_3f31_7217 {
        if w < (UF_1 - (12 << 23)) {
            force_eval!(x + x1p120);
            return 1.;
        }
        let t = expm1f(x);
        return 1. + t * t / (2. * (1. + t));
    }

    /* |x| < log(FLT_MAX) */
    if w < 0x_42b1_7217 {
        let t = expf(x);
        return 0.5 * (t + 1. / t);
    }

    /* |x| > log(FLT_MAX) or nan */
    k_expo2f(x)
}
