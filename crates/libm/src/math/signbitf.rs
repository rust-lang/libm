#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) extern "C" fn signbitf(x: f32) -> i32 {
    (x.to_bits() >> 31) as i32
}
