/* origin: FreeBSD /usr/src/lib/msun/src/s_fmaf.c */

use core::f32;
use core::ptr::read_volatile;

use super::fenv::{
    feclearexcept, fegetround, feraiseexcept, fesetround, fetestexcept, FE_INEXACT, FE_TONEAREST,
    FE_TOWARDZERO, FE_UNDERFLOW,
};

/// Floating multiply add (f32)
///
/// Computes `(x*y)+z`, rounded as one ternary operation:
/// Computes the value (as if) to infinite precision and rounds once to the result format,
/// according to the rounding mode characterized by the value of FLT_ROUNDS.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fmaf(x: f32, y: f32, mut z: f32) -> f32 {
    let xy: f64;
    let mut result: f64;
    let mut ui: u64;
    let e: i32;

    xy = x as f64 * y as f64;
    result = xy + z as f64;
    ui = result.to_bits();
    e = (ui >> 52) as i32 & 0x7ff;
    /* Common case: The double precision result is fine. */
    if (
        /* not a halfway case */
        ui & 0x1fffffff) != 0x10000000 ||
        /* NaN */
        e == 0x7ff ||
        /* exact */
        (result - xy == z as f64 && result - z as f64 == xy) ||
        /* not round-to-nearest */
        fegetround() != FE_TONEAREST
    {
        /*
            underflow may not be raised correctly, example:
            fmaf(0x1p-120f, 0x1p-120f, 0x1p-149f)
        */
        if e < 0x3ff - 126 && e >= 0x3ff - 149 && fetestexcept(FE_INEXACT) != 0 {
            feclearexcept(FE_INEXACT);
            // prevent `xy + vz` from being CSE'd with `xy + z` above
            let vz: f32 = unsafe { read_volatile(&z) };
            result = xy + vz as f64;
            if fetestexcept(FE_INEXACT) != 0 {
                feraiseexcept(FE_UNDERFLOW);
            } else {
                feraiseexcept(FE_INEXACT);
            }
        }
        z = result as f32;
        return z;
    }

    /*
     * If result is inexact, and exactly halfway between two float values,
     * we need to adjust the low-order bit in the direction of the error.
     */
    fesetround(FE_TOWARDZERO);
    // prevent `vxy + z` from being CSE'd with `xy + z` above
    let vxy: f64 = unsafe { read_volatile(&xy) };
    let mut adjusted_result: f64 = vxy + z as f64;
    fesetround(FE_TONEAREST);
    if result == adjusted_result {
        ui = adjusted_result.to_bits();
        ui += 1;
        adjusted_result = f64::from_bits(ui);
    }
    z = adjusted_result as f32;
    z
}
