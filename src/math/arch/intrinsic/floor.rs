pub fn floor(x: f64) -> f64 {
    unsafe { ::core::intrinsics::floorf64(x) }
}
