

use super::{k_cosf, k_sinf, rem_pio2f};

#[inline]
pub fn sinf(x: f32) -> f32 {
    let z = 0f32;
    let mut ix = x.to_bits() as i32;

    /* |x| ~< pi/4 */
    ix &= 0x7fffffff;
    if ix <= 0x3f490fd8 {
        k_sinf(x, z, false)
    } else if !(ix<0x7f800000) {
        /* sin(Inf or NaN) is NaN */
        x - x
    } else {
        /* argument reduction needed */
        let (n, y0, y1) = rem_pio2f(x);
        match n & 3 {
            0 => k_sinf(y0, y1, true),
            1 => k_cosf(y0, y1),
            2 => -k_sinf(y0, y1, true),
            _ => -k_cosf(y0, y1),
        }
    }
}
