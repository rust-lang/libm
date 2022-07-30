#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub const fn remainder(x: f64, y: f64) -> f64 {
    let (result, _) = super::remquo(x, y);
    result
}
