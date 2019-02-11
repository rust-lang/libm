extern {
    #[link_name = "llvm.nvvm.fabs.d"]
    pub fn abs(v: f64) -> f64;
    #[link_name = "llvm.nvvm.fabs.f"]
    pub fn absf(v: f32) -> f32;
    #[link_name = "llvm.nvvm.round.d"]
    pub fn round(v: f64) -> f64;
    #[link_name = "llvm.nvvm.round.f"]
    pub fn roundf(v: f32) -> f32;
    #[link_name = "llvm.nvvm.trunc.d"]
    pub fn trunc(v: f64) -> f64;
    #[link_name = "llvm.nvvm.trunc.f"]
    pub fn truncf(v: f32) -> f32;

    #[link_name = "llvm.nvvm.sin.approx.f"]
    pub fn sinf_approx(v: f32) -> f32;
    #[link_name = "llvm.nvvm.cos.approx.f"]
    pub fn cosf_approx(v: f32) -> f32;
    #[link_name = "llvm.nvvm.ex2.approx.d"]
    pub fn exp2_approx(v: f64) -> f64;
    #[link_name = "llvm.nvvm.ex2.approx.f"]
    pub fn exp2f_approx(v: f32) -> f32;
    #[link_name = "llvm.nvvm.lg2.approx.d"]
    pub fn lg2_approx(v: f64) -> f64;
    #[link_name = "llvm.nvvm.lg2.approx.f"]
    pub fn lg2f_approx(v: f32) -> f32;
}
