//! Compare our implementations with the result of musl functions, as provided by `musl-math-sys`.
//!
//! Currently this only tests randomized inputs. In the future this may be improved to test edge
//! cases or run exhaustive tests.
//!
//! Note that musl functions do not always provide 0.5ULP rounding, so our functions can do better
//! than these results.

// There are some targets we can't build musl for
#![cfg(feature = "build-musl")]

use libm_test::generate::{case_list, edge_cases, random, spaced};
use libm_test::{CheckBasis, CheckCtx, CheckOutput, GeneratorKind, MathOp, TupleCall};

const BASIS: CheckBasis = CheckBasis::Musl;

fn musl_runner<Op: MathOp>(
    ctx: &CheckCtx,
    cases: impl Iterator<Item = Op::RustArgs>,
    musl_fn: Op::CFn,
) {
    for input in cases {
        let musl_res = input.call(musl_fn);
        let crate_res = input.call_intercept_panics(Op::ROUTINE);

        crate_res.validate(musl_res, input, ctx).unwrap();
    }
}

/// Test against musl with generators from a domain.
macro_rules! musl_tests {
    (
        fn_name: $fn_name:ident,
        attrs: [$($attr:meta),*],
    ) => {
        paste::paste! {
            #[test]
            $(#[$attr])*
            fn [< musl_case_list_ $fn_name >]() {
                type Op = libm_test::op::$fn_name::Routine;
                let ctx = CheckCtx::new(Op::IDENTIFIER, BASIS, GeneratorKind::List);
                let cases = case_list::get_test_cases_basis::<Op>(&ctx).0;
                musl_runner::<Op>(&ctx, cases, musl_math_sys::$fn_name);
            }

            #[test]
            $(#[$attr])*
            fn [< musl_random_ $fn_name >]() {
                type Op = libm_test::op::$fn_name::Routine;
                let ctx = CheckCtx::new(Op::IDENTIFIER, BASIS, GeneratorKind::Random);
                let cases = random::get_test_cases::<<Op as MathOp>::RustArgs>(&ctx).0;
                musl_runner::<Op>(&ctx, cases, musl_math_sys::$fn_name);
            }

            #[test]
            $(#[$attr])*
            fn [< musl_edge_case_ $fn_name >]() {
                type Op = libm_test::op::$fn_name::Routine;
                let ctx = CheckCtx::new(Op::IDENTIFIER, BASIS, GeneratorKind::EdgeCases);
                let cases = edge_cases::get_test_cases::<Op>(&ctx).0;
                musl_runner::<Op>(&ctx, cases, musl_math_sys::$fn_name);
            }

            #[test]
            $(#[$attr])*
            fn [< musl_quickspace_ $fn_name >]() {
                type Op = libm_test::op::$fn_name::Routine;
                let ctx = CheckCtx::new(Op::IDENTIFIER, BASIS, GeneratorKind::QuickSpaced);
                let cases = spaced::get_test_cases::<Op>(&ctx).0;
                musl_runner::<Op>(&ctx, cases, musl_math_sys::$fn_name);
            }
        }
    };
}

libm_macros::for_each_function! {
    callback: musl_tests,
    attributes: [],
    skip: [
        // TODO integer inputs
        jn,
        jnf,
        ldexp,
        ldexpf,
        scalbn,
        scalbnf,
        yn,
        ynf,

        // Not provided by musl
        // verify-sorted-start
        ceilf128,
        ceilf16,
        copysignf128,
        copysignf16,
        fabsf128,
        fabsf16,
        fdimf128,
        fdimf16,
        floorf128,
        floorf16,
        fmaf128,
        fmaxf128,
        fmaxf16,
        fmaximum,
        fmaximum_num,
        fmaximum_numf,
        fmaximum_numf128,
        fmaximum_numf16,
        fmaximumf,
        fmaximumf128,
        fmaximumf16,
        fminf128,
        fminf16,
        fminimum,
        fminimum_num,
        fminimum_numf,
        fminimum_numf128,
        fminimum_numf16,
        fminimumf,
        fminimumf128,
        fminimumf16,
        fmodf128,
        fmodf16,
        ldexpf128,
        ldexpf16,
        rintf128,
        rintf16,
        roundeven,
        roundevenf,
        roundevenf128,
        roundevenf16,
        roundf128,
        roundf16,
        scalbnf128,
        scalbnf16,
        sqrtf128,
        sqrtf16,
        truncf128,
        truncf16,
        // verify-sorted-end
    ],
}
