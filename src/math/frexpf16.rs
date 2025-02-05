/// Decompose a float into a normalized value within the range `[0.5, 1)`, and a power of 2.
///
/// That is, `x * 2^p` will represent the input value.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn frexpf16(x: f16) -> (f16, i32) {
    super::generic::frexp(x)
}
