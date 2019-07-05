//! libm in pure Rust
#![deny(warnings)]
//#![no_std]
#![cfg_attr(
    all(target_arch = "wasm32", not(feature = "stable")),
    feature(core_intrinsics)
)]

mod math;
pub use self::math::*;
