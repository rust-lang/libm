//! Platforms where LLVM is known to lower to assembly. Currently only wasm32 for most things.

#[cfg(target_arch = "wasm32")]
pub mod ceil;

#[cfg(target_arch = "wasm32")]
pub mod ceilf;

#[cfg(target_arch = "wasm32")]
pub mod fabs;

#[cfg(target_arch = "wasm32")]
pub mod fabsf;

#[cfg(target_arch = "wasm32")]
pub mod floor;

#[cfg(target_arch = "wasm32")]
pub mod floorf;

#[cfg(target_arch = "wasm32")]
pub mod sqrt;

#[cfg(target_arch = "wasm32")]
pub mod sqrtf;

#[cfg(target_arch = "wasm32")]
pub mod trunc;

#[cfg(target_arch = "wasm32")]
pub mod truncf;
