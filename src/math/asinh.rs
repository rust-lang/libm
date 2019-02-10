use super::{log, log1p, sqrt};

const LN2: f64 = 0.693_147_180_559_945_309_417_232_121_458_176_568; /* 0x_3fe6_2e42,  0x_fefa_39ef*/

/* asinh(x) = sign(x)*log(|x|+sqrt(x*x+1)) ~= x - x^3/6 + o(x^5) */
pub fn asinh(mut x: f64) -> f64 {
    let mut u = x.to_bits();
    let e = ((u >> 52) as usize) & 0x7ff;
    let sign = (u >> 63) != 0;

    /* |x| */
    u &= (!0) >> 1;
    x = f64::from_bits(u);

    if e >= (0x3ff + 26) {
        /* |x| >= 0x1p26 or inf or nan */
        x = log(x) + LN2;
    } else if e >= (0x3ff + 1) {
        /* |x| >= 2 */
        x = log(2. * x + 1. / (sqrt(x * x + 1.0) + x));
    } else if e >= (0x3ff - 26) {
        /* |x| >= 0x1p-26, up to 1.6ulp error in [0.125,0.5] */
        x = log1p(x + x * x / (sqrt(x * x + 1.0) + 1.0));
    } else {
        /* |x| < 0x1p-26, raise inexact if x != 0 */
        let x1p120 = f64::from_bits(0x_4770_0000_0000_0000);
        force_eval!(x + x1p120);
    }

    if sign {
        -x
    } else {
        x
    }
}
