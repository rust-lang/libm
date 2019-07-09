//! Fake compiler-builtins crate
//!
//! This is used to test that we can source import `libm` into the compiler-builtins crate.

#![allow(dead_code)]
#![no_std]
#![cfg_attr(
    all(target_arch = "wasm32", not(feature = "stable")),
    feature(core_intrinsics)
)]

#[path = "../../../src/math/mod.rs"]
mod libm;
