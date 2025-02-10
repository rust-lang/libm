/* SPDX-License-Identifier: MIT OR Apache-2.0 */
//! IEEE 754-2019 `maximumNumber`.
//!
//! Per the spec, returns:
//! - `x` if `x > y`
//! - `y` if `y > x`
//! - Non-NaN if one operand is NaN
//! - Logic following +0.0 > -0.0
//! - Either `x` or `y` if `x == y` and the signs are the same
//! - qNaN if either operand is a NaN
//!
//! Excluded from our implementation is sNaN handling.

use super::super::Float;

pub fn fmaximum_num<F: Float>(x: F, y: F) -> F {
    let res =
        if x.is_nan() || x < y || (x.to_bits() == F::NEG_ZERO.to_bits() && y.is_sign_positive()) {
            y
        } else {
            x
        };

    // Canonicalize
    res * F::ONE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::{Hexf, Int};

    fn spec_test<F: Float>() {
        let cases = [
            (F::ZERO, F::ZERO, F::ZERO),
            (F::ONE, F::ONE, F::ONE),
            (F::ZERO, F::ONE, F::ONE),
            (F::ONE, F::ZERO, F::ONE),
            (F::ZERO, F::NEG_ONE, F::ZERO),
            (F::NEG_ONE, F::ZERO, F::ZERO),
            (F::INFINITY, F::ZERO, F::INFINITY),
            (F::NEG_INFINITY, F::ZERO, F::ZERO),
            (F::NAN, F::ZERO, F::ZERO),
            (F::ZERO, F::NAN, F::ZERO),
            (F::NAN, F::NAN, F::NAN),
            (F::ZERO, F::NEG_ZERO, F::ZERO),
            (F::NEG_ZERO, F::ZERO, F::ZERO),
        ];

        for (x, y, res) in cases {
            let val = fmaximum_num(x, y);
            assert_biteq!(val, res, "fmaximum_num({}, {})", Hexf(x), Hexf(y));
        }
    }

    #[test]
    #[cfg(f16_enabled)]
    fn spec_tests_f16() {
        spec_test::<f16>();
    }

    #[test]
    fn spec_tests_f32() {
        spec_test::<f32>();
    }

    #[test]
    fn spec_tests_f64() {
        spec_test::<f64>();
    }

    #[test]
    #[cfg(f128_enabled)]
    fn spec_tests_f128() {
        spec_test::<f128>();
    }
}
