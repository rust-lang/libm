use core::f64;

#[allow(clippy::eq_op)]
const FP_ILOGBNAN: isize = -1 - ((!0) >> 1);
const FP_ILOGB0: isize = FP_ILOGBNAN;

/// Get exponent (f64)
///
/// All nonzero, normal numbers can be described as `m*2^p`.
/// Examines the argument `x`, and returns *p*.
pub fn ilogb(x: f64) -> isize {
    let mut i: u64 = x.to_bits();
    let e = ((i >> 52) & 0x7ff) as isize;

    if e == 0 {
        i <<= 12;
        if i == 0 {
            force_eval!(f64::NAN);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x3ff;
        while (i >> 63) == 0 {
            e -= 1;
            i <<= 1;
        }
        return e;
    }
    if e == 0x7ff {
        force_eval!(f64::NAN);
        if (i << 12) != 0 {
            return FP_ILOGBNAN;
        } else {
            return isize::max_value();
        }
    }
    e - 0x3ff
}
