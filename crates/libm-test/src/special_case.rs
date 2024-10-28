//! Configuration for skipping or changing the result for individual test cases (inputs) rather
//! than ignoring entire tests.

use core::f32;

use crate::{CheckBasis, CheckCtx, Float, Int, TestResult};

/// Type implementing [`IgnoreCase`].
pub struct SpecialCase;

/// Don't run further validation on this test case.
const SKIP: Option<TestResult> = Some(Ok(()));

/// Return this to skip checks on a test that currently fails but shouldn't. Looks
/// the same as skip, but we keep them separate to better indicate purpose.
const XFAIL: Option<TestResult> = Some(Ok(()));

/// Allow overriding the outputs of specific test cases.
///
/// There are some cases where we want to xfail specific cases or handle certain inputs
/// differently than the rest of calls to `validate`. This provides a hook to do that.
///
/// If `None` is returned, checks will proceed as usual. If `Some(result)` is returned, checks
/// are skipped and the provided result is returned instead.
///
/// This gets implemented once per input type, then the functions provide further filtering
/// based on function name and values.
///
/// `ulp` can also be set to adjust the ULP for that specific test, even if `None` is still
/// returned.
pub trait MaybeOverride<Input> {
    fn check_float<F: Float>(
        _input: Input,
        _actual: F,
        _expected: F,
        _ulp: &mut u32,
        _ctx: &CheckCtx,
    ) -> Option<TestResult> {
        None
    }

    fn check_int<I: Int>(
        _input: Input,
        _actual: I,
        _expected: I,
        _ctx: &CheckCtx,
    ) -> Option<TestResult> {
        None
    }
}

impl MaybeOverride<(f32,)> for SpecialCase {
    fn check_float<F: Float>(
        input: (f32,),
        actual: F,
        expected: F,
        _ulp: &mut u32,
        ctx: &CheckCtx,
    ) -> Option<TestResult> {
        if ctx.basis == CheckBasis::Musl {
            if ctx.fname == "acoshf" && input.0 < -1.0 {
                // acoshf is undefined for x <= 1.0, but we return a random result at lower
                // values.
                return XFAIL;
            }

            if ctx.fname == "sincosf" {
                let factor_frac_pi_2 = input.0.abs() / f32::consts::FRAC_PI_2;
                if (factor_frac_pi_2 - factor_frac_pi_2.round()).abs() < 1e-2 {
                    // we have a bad approximation near multiples of pi/2
                    return XFAIL;
                }
            }

            if ctx.fname == "expm1f" && input.0 > 80.0 && actual.is_infinite() {
                // we return infinity but the number is representable
                return XFAIL;
            }

            if ctx.fname == "sinhf" && input.0.abs() > 80.0 && actual.is_nan() {
                // we return some NaN that should be real values or infinite
                // doesn't seem to happen on x86
                return XFAIL;
            }

            if ctx.fname == "lgammaf" || ctx.fname == "lgammaf_r" && input.0 < 0.0 {
                // loggamma should not be defined for x < 0, yet we both return results
                return XFAIL;
            }
        }

        maybe_check_nan_bits(actual, expected, ctx)
    }
}

impl MaybeOverride<(f64,)> for SpecialCase {
    fn check_float<F: Float>(
        input: (f64,),
        actual: F,
        expected: F,
        _ulp: &mut u32,
        ctx: &CheckCtx,
    ) -> Option<TestResult> {
        if ctx.basis == CheckBasis::Musl {
            if cfg!(target_arch = "x86") && ctx.fname == "acosh" && input.0 < 1.0 {
                // The function is undefined, both implementations return random results
                return SKIP;
            }

            if cfg!(x86_no_sse)
                && ctx.fname == "ceil"
                && input.0 < 0.0
                && input.0 > -1.0
                && expected == F::ZERO
                && actual == F::ZERO
            {
                // musl returns -0.0, we return +0.0
                return XFAIL;
            }

            if ctx.fname == "lgamma" || ctx.fname == "lgamma_r" && input.0 < 0.0 {
                // loggamma should not be defined for x < 0, yet we both return results
                return XFAIL;
            }
        }

        maybe_check_nan_bits(actual, expected, ctx)
    }
}

/// Check NaN bits if the function requires it
fn maybe_check_nan_bits<F: Float>(actual: F, expected: F, ctx: &CheckCtx) -> Option<TestResult> {
    if !(ctx.canonical_name == "fabs" || ctx.canonical_name == "copysign") {
        return None;
    }

    // The musl implementations seem to set the top bit of the mantissa for any NaN on i686.
    if cfg!(target_arch = "x86") && ctx.basis == CheckBasis::Musl && ctx.canonical_name == "fabs" {
        return SKIP;
    }
    // abs and copysign require signaling NaNs to be propagated, so verify bit equality.
    if actual.to_bits() == expected.to_bits() {
        return SKIP;
    } else {
        Some(Err(anyhow::anyhow!("NaNs have different bitpatterns")))
    }
}

impl MaybeOverride<(f32, f32)> for SpecialCase {
    fn check_float<F: Float>(
        input: (f32, f32),
        _actual: F,
        expected: F,
        _ulp: &mut u32,
        ctx: &CheckCtx,
    ) -> Option<TestResult> {
        maybe_skip_min_max_nan(input, expected, ctx)
    }
}
impl MaybeOverride<(f64, f64)> for SpecialCase {
    fn check_float<F: Float>(
        input: (f64, f64),
        _actual: F,
        expected: F,
        _ulp: &mut u32,
        ctx: &CheckCtx,
    ) -> Option<TestResult> {
        maybe_skip_min_max_nan(input, expected, ctx)
    }
}

/// Musl propagates NaNs if one is provided as the input, but we return the other input.
// F1 and F2 are always the same type, this is just to please generics
fn maybe_skip_min_max_nan<F1: Float, F2: Float>(
    input: (F1, F1),
    expected: F2,
    ctx: &CheckCtx,
) -> Option<TestResult> {
    if (ctx.canonical_name == "fmax" || ctx.canonical_name == "fmin")
        && (input.0.is_nan() || input.1.is_nan())
        && expected.is_nan()
    {
        return XFAIL;
    } else {
        None
    }
}

impl MaybeOverride<(i32, f32)> for SpecialCase {
    fn check_float<F: Float>(
        input: (i32, f32),
        _actual: F,
        _expected: F,
        ulp: &mut u32,
        ctx: &CheckCtx,
    ) -> Option<TestResult> {
        bessel_prec_dropoff(input, ulp, ctx)
    }
}
impl MaybeOverride<(i32, f64)> for SpecialCase {
    fn check_float<F: Float>(
        input: (i32, f64),
        _actual: F,
        _expected: F,
        ulp: &mut u32,
        ctx: &CheckCtx,
    ) -> Option<TestResult> {
        bessel_prec_dropoff(input, ulp, ctx)
    }
}

/// Our bessel functions blow up with large N values
fn bessel_prec_dropoff<F: Float>(
    input: (i32, F),
    ulp: &mut u32,
    ctx: &CheckCtx,
) -> Option<TestResult> {
    if ctx.canonical_name == "jn" {
        if input.0 > 4000 {
            return XFAIL;
        } else if input.0 > 2000 {
            // *ulp = 20_000;
            *ulp = 20000;
        } else if input.0 > 1000 {
            *ulp = 4000;
        }
    }

    None
}

impl MaybeOverride<(f32, f32, f32)> for SpecialCase {}
impl MaybeOverride<(f64, f64, f64)> for SpecialCase {}
impl MaybeOverride<(f32, i32)> for SpecialCase {}
impl MaybeOverride<(f64, i32)> for SpecialCase {}
