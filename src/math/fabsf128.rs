/// Absolute value (magnitude) of a `f128` value.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fabsf128(x: f128) -> f128 {
    super::generic::abs::abs(x)
}
