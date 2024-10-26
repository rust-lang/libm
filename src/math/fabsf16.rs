/// Absolute value (magnitude) of a `f16` value.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fabsf16(x: f16) -> f16 {
    super::generic::abs::abs(x)
}
