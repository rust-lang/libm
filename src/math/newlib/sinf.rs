use core::f32;
use super::{k_cosf, k_sinf, rem_pio2f};

#[inline]
pub fn sinf(x: f32) -> f32 {
    let mut ix = x.to_bits();
    ix &= 0x_7fff_ffff;

    /* |x| ~< pi/4 */
    if ix <= 0x_3f49_0fd8 {
        k_sinf(x, 0., false)
    } else if ix >= 0x_7f80_0000 {
        /* sin(Inf or NaN) is NaN */
        f32::NAN
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
