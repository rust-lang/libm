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
        let outputs_nan = all_nan(&[actual, expected]);

        match &ctx.basis {
            CheckBasis::Musl => match ctx.fname {
                // We return +NaN, Musl returns -NaN
                "tgammaf" => input.0 < 0.0,
                _ => false,
            },
            CheckBasis::MultiPrecision => match ctx.fname {
                // For almost everything we return -NaN but MPFR does +NaN
                _ if (input.0.is_nan() || input.0.is_infinite()) && outputs_nan => true,
                // x86 MacOS NaN doesn't seem to depend on input sign
                _ if cfg!(x86_macos) && outputs_nan => true,
                // Out of domain we return +NaN, MPFR returns -NaN
                "atanhf" => input.0 < -1.0 && outputs_nan,
                // We return -NaN, MPFR says +NaN
                "tgammaf" => input.0 < 0.0 && outputs_nan,
                _ => false,
            },
        }
    }

    fn xfail_int<I: Int>(input: (f32,), actual: I, expected: I, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => match ctx.fname {
                // We set -1, MPFR sets +1
                "lgammaf_r" => input.0 == f32::NEG_INFINITY && actual.abs() == expected.abs(),
                _ => false,
            },
        }
    }
}

impl IgnoreCase<(f64,)> for XFail {
    fn xfail_float<F: Float>(input: (f64,), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        let outputs_nan = all_nan(&[actual, expected]);

        // See the `f32` version for notes about what is skipped
        match &ctx.basis {
            CheckBasis::Musl => match ctx.fname {
                "tgamma" => input.0 < 0.0,
                _ => false,
            },
            CheckBasis::MultiPrecision => match ctx.fname {
                _ if (input.0.is_nan() || input.0.is_infinite()) && outputs_nan => true,
                _ if cfg!(x86_macos) && outputs_nan => true,
                "atanh" => input.0 < -1.0 && outputs_nan,
                "tgamma" => input.0 < 0.0 && outputs_nan,
                _ => false,
            },
        }
    }

    fn xfail_int<I: Int>(input: (f64,), actual: I, expected: I, ctx: &CheckCtx) -> bool {
        // See the `f32` version for notes about what is skipped
        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => match ctx.fname {
                "lgamma_r" => input.0 == f64::NEG_INFINITY && actual.abs() == expected.abs(),
                _ => false,
            },
        }
    }
}

impl IgnoreCase<(f32, f32)> for XFail {
    fn xfail_float<F: Float>(input: (f32, f32), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        let outputs_nan = all_nan(&[actual, expected]);

        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => {
                (all_nan(&[input.0, input.1]) && outputs_nan) || (cfg!(x86_macos) && outputs_nan)
            }
        }
    }
}

impl IgnoreCase<(f64, f64)> for XFail {
    fn xfail_float<F: Float>(input: (f64, f64), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        let outputs_nan = all_nan(&[actual, expected]);

        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => {
                (all_nan(&[input.0, input.1]) && outputs_nan) || (cfg!(x86_macos) && outputs_nan)
            }
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
        let outputs_nan = all_nan(&[actual, expected]);

        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => {
                (all_nan(&[input.0, input.1, input.2]) && outputs_nan)
                    || (cfg!(x86_macos) && outputs_nan)
            }
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
        let outputs_nan = all_nan(&[actual, expected]);

        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => {
                (all_nan(&[input.0, input.1, input.2]) && outputs_nan)
                    || (cfg!(x86_macos) && outputs_nan)
            }
        }
    }
}

impl IgnoreCase<(i32, f32)> for XFail {
    fn xfail_float<F: Float>(input: (i32, f32), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => match ctx.fname {
                _ if input.1.is_nan() && all_nan(&[actual, expected]) => true,
                // We return +0.0, MPFR returns -0.0
                "jnf" => input.1 == f32::NEG_INFINITY && actual == F::ZERO && expected == F::ZERO,
                _ => false,
            },
        }
    }
}

impl IgnoreCase<(i32, f64)> for XFail {
    fn xfail_float<F: Float>(input: (i32, f64), actual: F, expected: F, ctx: &CheckCtx) -> bool {
        match &ctx.basis {
            CheckBasis::Musl => false,
            CheckBasis::MultiPrecision => match ctx.fname {
                _ if input.1.is_nan() && all_nan(&[actual, expected]) => true,
                "jn" => input.1 == f64::NEG_INFINITY && actual == F::ZERO && expected == F::ZERO,
                _ => false,
            },
        }
    }
}

impl IgnoreCase<(f32, i32)> for XFail {}
impl IgnoreCase<(f64, i32)> for XFail {}

/// Convenience to check if all values are NaN
fn all_nan<F: Float>(v1: &[F]) -> bool {
    v1.iter().all(|v| v.is_nan())
}
