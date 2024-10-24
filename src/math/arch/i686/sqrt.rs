pub fn sqrt(x: f64) -> f64 {
    // Note: This path is unlikely since LLVM will usually have already
    // optimized sqrt calls into hardware instructions if sse2 is available,
    // but if someone does end up here they'll appreciate the speed increase.
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    unsafe {
        let m = _mm_set_sd(x);
        let m_sqrt = _mm_sqrt_pd(m);
        _mm_cvtsd_f64(m_sqrt)
    }
}
