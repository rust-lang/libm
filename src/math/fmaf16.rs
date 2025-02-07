#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fmaf16(x: f16, y: f16, z: f16) -> f16 {
    super::generic::fma_wide::<f16, f32>(x, y, z)
}
