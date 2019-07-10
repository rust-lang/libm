#![cfg(all(test, feature = "musl-reference-tests"))]

use core::{f32, f64};
use libm::*;

/// Approximate equality with 1 ULP of tolerance
#[inline]
fn _eqf(a: f32, b: f32) -> Result<(), u32> {
    if a.is_nan() && b.is_nan() {
        Ok(())
    } else {
        let err = (a.to_bits() as i32).wrapping_sub(b.to_bits() as i32).abs();

        if err <= 1 {
            Ok(())
        } else {
            Err(err as u32)
        }
    }
}

#[inline]
fn _eq(a: f64, b: f64) -> Result<(), u64> {
    if a.is_nan() && b.is_nan() {
        Ok(())
    } else {
        let err = (a.to_bits() as i64).wrapping_sub(b.to_bits() as i64).abs();

        if err <= 1 {
            Ok(())
        } else {
            Err(err as u64)
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/musl-tests.rs"));
