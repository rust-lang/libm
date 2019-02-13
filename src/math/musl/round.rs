#[cfg(all(target_os = "cuda", not(feature = "stable")))]
use super::cuda_intrinsics;
use core::f64;

const TOINT: f64 = 1. / f64::EPSILON;

#[inline]
pub fn round(mut x: f64) -> f64 {
    llvm_intrinsically_optimized! {
        #[cfg(target_os = "cuda")] {
            return unsafe { cuda_intrinsics::round(x) }
        }
    }

    let (f, i) = (x, x.to_bits());
    let e: u64 = i >> 52 & 0x7ff;
    let mut y: f64;

    if e >= 0x3ff + 52 {
        return x;
    }
    if i >> 63 != 0 {
        x = -x;
    }
    if e < 0x3ff - 1 {
        // raise inexact if x!=0
        force_eval!(x + TOINT);
        return 0. * f;
    }
    y = x + TOINT - TOINT - x;
    if y > 0.5 {
        y += x - 1.;
    } else if y <= -0.5 {
        y += x + 1.;
    } else {
        y += x;
    }

    if i >> 63 != 0 {
        -y
    } else {
        y
    }
}
