use super::consts::*;
use core::f32;
use core::u32;

/// Floating-point remainder (modulo) (f32)
///
/// Computes the floating-point remainder of `x/y` (`x` modulo `y`).
/// Returns the value `x-i*y`, for the largest integer `i` such that,
/// if `y` is nonzero, the result has the same sign as `x` and magnitude less than the magnitude of `y`.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fmodf(x: f32, y: f32) -> f32 {
    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let mut ex = (uxi >> 23 & 0xff) as i32;
    let mut ey = (uyi >> 23 & 0xff) as i32;
    let sx = uxi & UF_SIGN;
    let mut i;

    if uyi << 1 == 0 || y.is_nan() || ex == 0xff {
        return (x * y) / (x * y);
    }

    if uxi << 1 <= uyi << 1 {
        if uxi << 1 == uyi << 1 {
            return 0. * x;
        }

        return x;
    }

    /* normalize x and y */
    if ex == 0 {
        i = uxi << 9;
        while i >> 31 == 0 {
            ex -= 1;
            i <<= 1;
        }

        uxi <<= -ex + 1;
    } else {
        uxi &= u32::MAX >> 9;
        uxi |= 1 << 23;
    }

    if ey == 0 {
        i = uyi << 9;
        while i >> 31 == 0 {
            ey -= 1;
            i <<= 1;
        }

        uyi <<= -ey + 1;
    } else {
        uyi &= u32::MAX >> 9;
        uyi |= 1 << 23;
    }

    /* x mod y */
    while ex > ey {
        i = uxi.wrapping_sub(uyi);
        if i >> 31 == 0 {
            if i == 0 {
                return 0. * x;
            }
            uxi = i;
        }
        uxi <<= 1;

        ex -= 1;
    }

    i = uxi.wrapping_sub(uyi);
    if i >> 31 == 0 {
        if i == 0 {
            return 0. * x;
        }
        uxi = i;
    }

    while uxi >> 23 == 0 {
        uxi <<= 1;
        ex -= 1;
    }

    /* scale result up */
    if ex > 0 {
        uxi -= 1 << 23;
        uxi |= (ex as u32) << 23;
    } else {
        uxi >>= -ex + 1;
    }
    uxi |= sx;

    f32::from_bits(uxi)
}
