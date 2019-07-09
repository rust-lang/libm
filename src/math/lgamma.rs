use super::lgamma_r;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lgamma(x: f64) -> f64 {
    lgamma_r(x).0
}
