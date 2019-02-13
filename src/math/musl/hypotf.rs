use core::f32;

use crate::math::sqrtf;

/// Distance from origin (f64)
///
/// Calculates the Euclidean distance `sqrt(x*x + y*y)` between the origin (0,0)
/// and a point represented by the Cartesian coordinates (`x`,`y`).
#[inline]
pub fn hypotf(mut x: f32, mut y: f32) -> f32 {
    let x1p90 = f32::from_bits(0x_6c80_0000); // 0x1p90f === 2 ^ 90
    let x1p_90 = f32::from_bits(0x_1280_0000); // 0x1p-90f === 2 ^ -90

    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let uti;
    let mut z: f32;

    uxi &= -1i32 as u32 >> 1;
    uyi &= -1i32 as u32 >> 1;
    if uxi < uyi {
        uti = uxi;
        uxi = uyi;
        uyi = uti;
    }

    x = f32::from_bits(uxi);
    y = f32::from_bits(uyi);
    if uyi == 0xff << 23 {
        return y;
    }
    if uxi >= 0xff << 23 || uyi == 0 || uxi - uyi >= 25 << 23 {
        return x + y;
    }

    z = 1.;
    if uxi >= (0x7f + 60) << 23 {
        z = x1p90;
        x *= x1p_90;
        y *= x1p_90;
    } else if uyi < (0x7f - 60) << 23 {
        z = x1p_90;
        x *= x1p90;
        y *= x1p90;
    }
    z * sqrtf((x as f64 * x as f64 + y as f64 * y as f64) as f32)
}
