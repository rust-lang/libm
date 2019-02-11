use crate::math::consts::*;

pub fn copysignf(x: f32, y: f32) -> f32 {
    let mut ux = x.to_bits();
    let uy = y.to_bits();
    ux &= UF_ABS;
    ux |= uy & UF_SIGN;
    f32::from_bits(ux)
}
