//! Configuration for skipping individual test cases (inputs) rather than ignoring entire tests.
//!
//! One common case here is that our NaNs usually have the sign bit set (platform dependent),
//! while MPFR seems to put meaning on the signedness of NaNs and zeros.

#![allow(unused)]
use crate::{CheckBasis, CheckCtx, Float, Int};

/// Type implementing [`IgnoreCase`].
pub struct XFail;

/// If the relevant function returns true, the input has a mismatch but should still be skipped.
///
/// This gets implemented once per input type, then the functions provide further filtering
/// based on function name and values.
pub trait IgnoreCase<Input> {
    fn xfail_float<F: Float>(_input: Input, _actual: F, _expected: F, _ctx: &CheckCtx) -> bool {
        false
    }

    fn xfail_int<I: Int>(_input: Input, _actual: I, _expected: I, _ctx: &CheckCtx) -> bool {
        false
    }
}

impl IgnoreCase<(f32,)> for XFail {
    fn xfail_float<F: Float>(input: (f32,), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => match ctx.fname {
                // We return +NaN, Musl returns -NaN
                "tgammaf" => input.0 < 0.0,
                _ => false,
            },
        }
    }

    fn xfail_int<I: Int>(input: (f32,), actual: I, expected: I, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(f64,)> for XFail {
    fn xfail_float<F: Float>(input: (f64,), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        // See the `f32` version for notes about what is skipped
        match &ctx.basis {
            CheckBasis::Musl => match ctx.fname {
                "tgamma" => input.0 < 0.0,
                _ => false,
            },
        }
    }

    fn xfail_int<I: Int>(input: (f64,), actual: I, expected: I, ctx: &CheckCtx) -> bool {
        // See the `f32` version for notes about what is skipped
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(f32, f32)> for XFail {
    fn xfail_float<F: Float>(input: (f32, f32), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(f64, f64)> for XFail {
    fn xfail_float<F: Float>(input: (f64, f64), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(f32, f32, f32)> for XFail {
    fn xfail_float<F: Float>(
        input: (f32, f32, f32),
        actual: F,
        expected: F,
        ctx: &CheckCtx,
    ) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}
impl IgnoreCase<(f64, f64, f64)> for XFail {
    fn xfail_float<F: Float>(
        input: (f64, f64, f64),
        actual: F,
        expected: F,
        ctx: &CheckCtx,
    ) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(i32, f32)> for XFail {
    fn xfail_float<F: Float>(input: (i32, f32), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(i32, f64)> for XFail {
    fn xfail_float<F: Float>(input: (i32, f64), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
        }
    }
}

impl IgnoreCase<(f32, i32)> for XFail {}
impl IgnoreCase<(f64, i32)> for XFail {}

/// Convenience to check if all values are NaN
fn all_nan<F: Float>(v1: &[F]) -> bool {
    v1.iter().all(|v| v.is_nan())
}
