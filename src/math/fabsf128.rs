/// Absolute value (magnitude) (f128)
///
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fabsf128(x: f128) -> f128 {
    select_implementation! {
        name: fabsf,
        use_intrinsic: target_arch = "wasm32",
        args: x,
    }

    super::generic::fabs(x)
}

// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(fabsf128(-1.0), 1.0);
        assert_eq!(fabsf128(2.8), 2.8);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
    #[test]
    fn spec_tests() {
        assert!(fabsf128(f128::NAN).is_nan());
        for f in [0.0, -0.0].iter().copied() {
            assert_eq!(fabsf128(f), 0.0);
        }
        for f in [f128::INFINITY, f128::NEG_INFINITY].iter().copied() {
            assert_eq!(fabsf128(f), f128::INFINITY);
        }
    }
}
