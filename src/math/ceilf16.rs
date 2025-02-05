/// Ceil (f16)
///
/// Finds the nearest integer greater than or equal to `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ceilf16(x: f16) -> f16 {
    select_implementation! {
        name: ceilf16,
        use_arch: all(target_arch = "aarch64", target_feature = "fp16"),
        args: x,
    }

    super::generic::ceil(x)
}
