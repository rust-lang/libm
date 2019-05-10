const FP_ILOGBNAN: i32 = -1 - 0x_7fff_ffff;
const FP_ILOGB0: i32 = FP_ILOGBNAN;
use core::f64;

/// Get exponent (f32)
///
/// All nonzero, normal numbers can be described as `m*2^p`.
/// Examines the argument `x`, and returns *p*.
pub fn ilogbf(x: f32) -> i32 {
    let mut i = x.to_bits();
    let e = ((i >> 23) & 0xff) as i32;

    if e == 0 {
        i <<= 9;
        if i == 0 {
            force_eval!(f64::NAN);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x7f;
        while (i >> 31) == 0 {
            e -= 1;
            i <<= 1;
        }
        e
    } else if e == 0xff {
        force_eval!(f64::NAN);
        if (i << 9) != 0 {
            FP_ILOGBNAN
        } else {
            i32::max_value()
        }
    } else {
        e - 0x7f
    }
}
