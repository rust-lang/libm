use super::lgammaf_r;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn lgammaf(x: f32) -> f32 {
    lgammaf_r(x).0
}
