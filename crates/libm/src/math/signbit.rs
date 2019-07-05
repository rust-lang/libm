#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) extern "C" fn signbit(x: f64) -> i32 {
    (x.to_bits() >> 63) as i32
}
