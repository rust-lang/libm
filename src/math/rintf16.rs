/// Round `x` to the nearest integer, breaking ties toward even.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn rintf16(x: f16) -> f16 {
    select_implementation! {
        name: rintf16,
        use_arch: all(target_arch = "aarch64", target_feature = "fp16"),
        args: x,
    }

    super::generic::rint(x)
}
