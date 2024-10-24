pub fn fabs(x: f64) -> f64 {
    unsafe { ::core::intrinsics::fabsf64(x) }
}
