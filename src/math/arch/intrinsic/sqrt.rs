pub fn sqrt(x: f64) -> f64 {
    return if x < 0.0 {
        f64::NAN
    } else {
        unsafe { ::core::intrinsics::sqrtf64(x) }
    };
}
