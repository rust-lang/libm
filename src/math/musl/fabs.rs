#[cfg(all(target_os = "cuda", not(feature = "stable")))]
use super::cuda_intrinsics;
use core::u64;

/// Absolute value (magnitude) (f64)
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[inline]
pub fn fabs(x: f64) -> f64 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f64.abs` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::fabsf64(x) }
        }
    }
    llvm_intrinsically_optimized! {
        #[cfg(target_os = "cuda")] {
            return unsafe { cuda_intrinsics::abs(x) }
        }
    }
    f64::from_bits(x.to_bits() & (u64::MAX / 2))
}
