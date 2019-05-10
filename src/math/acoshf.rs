use super::consts::*;
use super::{log1pf, logf, sqrtf};

const LN2: f32 = 0.693_147_180_559_945_309_417_232_121_458_176_568;

/// Inverse hyperbolic cosine (f32)
///
/// Calculates the inverse hyperbolic cosine of `x`.
/// Is defined as `log(x + sqrt(x*x-1))`.
/// `x` must be a number greater than or equal to 1.
pub fn acoshf(x: f32) -> f32 {
    let u = x.to_bits();
    let a = u & UF_ABS;

    if a < 0x_3f80_0000 + (1 << 23) {
        /* |x| < 2, invalid if x < 1 or nan */
        /* up to 2ulp error in [1,1.125] */
        log1pf(x - 1. + sqrtf((x - 1.) * (x - 1.) + 2. * (x - 1.)))
    } else if a < 0x_3f80_0000 + (12 << 23) {
        /* |x| < 0x1p12 */
        logf(2. * x - 1. / (x + sqrtf(x * x - 1.)))
    } else {
        /* x >= 0x1p12 */
        logf(x) + LN2
    }
}
