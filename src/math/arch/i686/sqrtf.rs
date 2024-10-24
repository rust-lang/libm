pub fn sqrtf(x: f32) -> f32 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    unsafe {
        let m = _mm_set_ss(x);
        let m_sqrt = _mm_sqrt_ss(m);
        _mm_cvtss_f32(m_sqrt)
    }
}
