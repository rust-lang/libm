#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fmaf128(x: f128, y: f128, z: f128) -> f128 {
    super::generic::fma(x, y, z)
}
