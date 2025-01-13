/* SPDX-License-Identifier: MIT
 * origin: musl src/math/ceil.c */

use super::super::{CastInto, Float};

pub fn ceil<F: Float>(x: F) -> F {
    let toint = F::ONE / F::EPSILON;

    // NB: using `exp` here and comparing to values adjusted by `EXP_BIAS` has better
    // perf than using `exp_unbiased` here.
    let e = x.exp();
    let y: F;

    // If the represented value has no fractional part, no truncation is needed.
    if e >= (F::SIG_BITS + F::EXP_BIAS).cast() || x == F::ZERO {
        return x;
    }

    let neg = x.is_sign_negative();

    // y = int(x) - x, where int(x) is an integer neighbor of x.
    // The `x - t + t - x` method is a way to expose non-round-to-even modes.
    y = if neg { x - toint + toint - x } else { x + toint - toint - x };

    // Exp < 0;  special case because of non-nearest rounding modes
    if e < F::EXP_BIAS.cast() {
        // Raise `FE_INEXACT`
        force_eval!(y);
        return if neg { F::NEG_ZERO } else { F::ONE };
    }

    if y < F::ZERO { x + y + F::ONE } else { x + y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check_f64() {
        assert_eq!(ceil(1.1f64), 2.0);
        assert_eq!(ceil(2.9f64), 3.0);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/ceil
    #[test]
    fn spec_tests_f64() {
        // Not Asserted: that the current rounding mode has no effect.
        assert!(ceil(f64::NAN).is_nan());
        for f in [0.0, -0.0, f64::INFINITY, f64::NEG_INFINITY].iter().copied() {
            assert_eq!(ceil(f), f);
        }
    }

    #[test]
    fn sanity_check_f32() {
        assert_eq!(ceil(1.1f32), 2.0);
        assert_eq!(ceil(2.9f32), 3.0);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/ceil
    #[test]
    fn spec_tests_f32() {
        // Not Asserted: that the current rounding mode has no effect.
        assert!(ceil(f32::NAN).is_nan());
        for f in [0.0, -0.0, f32::INFINITY, f32::NEG_INFINITY].iter().copied() {
            assert_eq!(ceil(f), f);
        }
    }
}
