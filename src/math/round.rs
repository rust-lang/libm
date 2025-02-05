/// Round `x` to the nearest integer, breaking ties away from zero.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn round(x: f64) -> f64 {
    select_implementation! {
        name: round,
        use_arch: all(target_arch = "aarch64", target_feature = "neon"),
        args: x,
    }

    super::generic::round(x)
}
