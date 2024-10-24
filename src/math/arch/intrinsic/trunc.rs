pub fn trunc(x: f64) -> f64 {
    unsafe { ::core::intrinsics::truncf64(x) }
}
