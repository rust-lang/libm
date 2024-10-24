pub fn sqrtf(x: f32) -> f32 {
    return if x < 0.0 {
        ::core::f32::NAN
    } else {
        unsafe { ::core::intrinsics::sqrtf32(x) }
    };
}
