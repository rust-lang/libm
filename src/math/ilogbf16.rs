/// Extract the binary exponent of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ilogbf16(x: f16) -> i32 {
    super::generic::ilogb(x)
}
