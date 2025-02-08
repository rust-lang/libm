#[macro_use]
pub mod macros;
mod big;
mod float_traits;
pub mod hex_float;
mod int_traits;

#[allow(unused_imports)]
pub use big::{i256, u256};
#[allow(unused_imports)]
pub use float_traits::{DFloat, Float, HFloat, IntTy};
pub(crate) use float_traits::{f32_from_bits, f64_from_bits};
#[cfg(f16_enabled)]
#[allow(unused_imports)]
pub use hex_float::hf16;
#[cfg(f128_enabled)]
#[allow(unused_imports)]
pub use hex_float::hf128;
#[allow(unused_imports)]
pub use hex_float::{Hexf, hf32, hf64};
pub use int_traits::{CastFrom, CastInto, DInt, HInt, Int, MinInt};

/// Hint to the compiler that the current path is cold.
pub fn cold_path() {
    #[cfg(intrinsics_enabled)]
    core::intrinsics::cold_path();
}

/// Return `x`, first raising `FE_INVALID`.
pub fn raise_invalid<F: Float>(x: F) -> F {
    (x - x) / (x - x)
}
