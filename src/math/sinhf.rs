use super::expm1f;
use super::k_expo2f;
use math::consts::*;

#[inline]
pub fn sinhf(x: f32) -> f32 {
    let mut h = 0.5_f32;
    let mut ix = x.to_bits();
    if (ix >> 31) != 0 {
        h = -h;
    }
    /* |x| */
    ix &= UF_ABS;
    let absx = f32::from_bits(ix);
    let w = ix;

    /* |x| < log(FLT_MAX) */
    if w < 0x_42b1_7217 {
        let t = expm1f(absx);
        if w < UF_1 {
            if w < (UF_1 - (12 << 23)) {
                return x;
            }
            return h * (2. * t - t * t / (t + 1.));
        }
        return h * (t + t / (t + 1.));
    }

    /* |x| > logf(FLT_MAX) or nan */
    2. * h * k_expo2f(absx)
}
