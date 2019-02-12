use super::log1pf;
use crate::math::consts::*;

/* atanh(x) = log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2 ~= x + x^3/3 + o(x^5) */
pub fn atanhf(mut x: f32) -> f32 {
    let mut u = x.to_bits();
    let sign = (u >> 31) != 0;

    /* |x| */
    u &= UF_ABS;
    x = f32::from_bits(u);

    if u < 0x_3f80_0000 - (1 << 23) {
        if u < 0x_3f80_0000 - (32 << 23) {
            /* handle underflow */
            if u < (1 << 23) {
                force_eval!((x * x) as f32);
            }
        } else {
            /* |x| < 0.5, up to 1.7ulp error */
            x = 0.5 * log1pf(2. * x + 2. * x * x / (1. - x));
        }
    } else {
        /* avoid overflow */
        x = 0.5 * log1pf(2. * (x / (1. - x)));
    }

    if sign {
        -x
    } else {
        x
    }
}
