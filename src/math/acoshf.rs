use super::{log1pf, logf, sqrtf};
use math::consts::*;

const LN2: f32 = 0.693_147_180_559_945_309_417_232_121_458_176_568;

/* acosh(x) = log(x + sqrt(x*x-1)) */
pub fn acoshf(x: f32) -> f32 {
    let u = x.to_bits();
    let a = u & UF_ABS;

    if a < 0x_3f80_0000 + (1 << 23) {
        /* |x| < 2, invalid if x < 1 or nan */
        /* up to 2ulp error in [1,1.125] */
        return log1pf(x - 1.0 + sqrtf((x - 1.0) * (x - 1.0) + 2.0 * (x - 1.0)));
    }
    if a < 0x_3f80_0000 + (12 << 23) {
        /* |x| < 0x1p12 */
        return logf(2.0 * x - 1.0 / (x + sqrtf(x * x - 1.0)));
    }
    /* x >= 0x1p12 */
    logf(x) + LN2
}
