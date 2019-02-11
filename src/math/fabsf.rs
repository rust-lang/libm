use crate::math::consts::*;

#[cfg(all(target_os = "cuda", not(feature = "stable")))]
use super::cuda_intrinsics;

#[inline]
pub fn fabsf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.abs` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::fabsf32(x) }
        }
    }
    llvm_intrinsically_optimized! {
        #[cfg(target_os = "cuda")] {
            return unsafe { cuda_intrinsics::absf(x) }
        }
    }
    f32::from_bits(x.to_bits() & UF_ABS)
}
