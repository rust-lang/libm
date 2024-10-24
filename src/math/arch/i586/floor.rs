pub fn floor(x: f64) -> f64 {
    //use an alternative implementation on x86, because the
    //main implementation fails with the x87 FPU used by
    //debian i386, probably due to excess precision issues.
    //basic implementation taken from https://github.com/rust-lang/libm/issues/219
    use crate::fabs;
    if fabs(x).to_bits() < 4503599627370496.0_f64.to_bits() {
        let truncated = x as i64 as f64;
        if truncated > x {
            return truncated - 1.0;
        } else {
            return truncated;
        }
    } else {
        return x;
    }
}
