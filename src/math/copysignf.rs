use super::consts::*;

/// Sign of Y, magnitude of X (f32)
///
/// Constructs a number with the magnitude (absolute value) of its
/// first argument, `x`, and the sign of its second argument, `y`.
pub fn copysignf(x: f32, y: f32) -> f32 {
    let mut ux = x.to_bits();
    let uy = y.to_bits();
    ux &= UF_ABS;
    ux |= uy & UF_SIGN;
    f32::from_bits(ux)
}
