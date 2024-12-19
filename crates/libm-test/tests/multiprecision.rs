//! Test with "infinite precision"

#![cfg(feature = "test-multiprecision")]

use libm_test::domain::HasDomain;
use libm_test::gen::{CachedInput, domain_logspace, random};
use libm_test::mpfloat::MpOp;
use libm_test::{CheckBasis, CheckCtx, CheckOutput, GenerateInput, MathOp, TupleCall};

/// Test against MPFR with random inputs.
macro_rules! mp_rand_tests {
    (
        fn_name: $fn_name:ident,
        attrs: [$($meta:meta)*]
    ) => {
        paste::paste! {
            #[test]
            $(#[$meta])*
            fn [< mp_random_ $fn_name >]() {
                test_one_random::<libm_test::op::$fn_name::Routine>();
            }
        }
    };
}

/// Test a single routine with random inputs
fn test_one_random<Op>()
where
    Op: MathOp + MpOp,
    CachedInput: GenerateInput<Op::RustArgs>,
{
    let mut mp_vals = Op::new_mp();
    let ctx = CheckCtx::new(Op::IDENTIFIER, CheckBasis::Mpfr);
    let cases = random::get_test_cases::<Op::RustArgs>(&ctx);

    for input in cases {
        let mp_res = Op::run(&mut mp_vals, input);
        let crate_res = input.call(Op::ROUTINE);

        crate_res.validate(mp_res, input, &ctx).unwrap();
    }
}

libm_macros::for_each_function! {
    callback: mp_rand_tests,
    attributes: [
        // Also an assertion failure on i686: at `MPFR_ASSERTN (! mpfr_erangeflag_p ())`
        #[ignore = "large values are infeasible in MPFR"]
        [jn, jnf],
    ],
    skip: [
        // FIXME: MPFR tests needed
        frexp,
        frexpf,
        ilogb,
        ilogbf,
        ldexp,
        ldexpf,
        modf,
        modff,
        remquo,
        remquof,
        scalbn,
        scalbnf,

        // FIXME: test needed, see
        // https://github.com/rust-lang/libm/pull/311#discussion_r1818273392
        nextafter,
        nextafterf,
    ],
}

/// Test against MPFR with generators from a domain.
macro_rules! mp_domain_tests {
    (
        fn_name: $fn_name:ident,
        attrs: [$($meta:meta)*]
    ) => {
        paste::paste! {
            #[test]
            $(#[$meta])*
            fn [< mp_logspace_ $fn_name >]() {
                type Op = libm_test::op::$fn_name::Routine;
                domain_test_runner::<Op>(domain_logspace::get_test_cases::<Op>());
            }
        }
    };
}

/// Test a single routine against domaine-aware inputs.
fn domain_test_runner<Op>(cases: impl Iterator<Item = (Op::FTy,)>)
where
    // Complicated generics...
    // The operation must take a single float argument (unary only)
    Op: MathOp<RustArgs = (<Op as MathOp>::FTy,)>,
    // It must also support multiprecision operations
    Op: MpOp,
    // And it must have a domain specified
    Op: HasDomain<Op::FTy>,
    // The single float argument tuple must be able to call the `RustFn` and return `RustRet`
    (<Op as MathOp>::FTy,): TupleCall<<Op as MathOp>::RustFn, Output = <Op as MathOp>::RustRet>,
{
    let mut mp_vals = Op::new_mp();
    let ctx = CheckCtx::new(Op::IDENTIFIER, CheckBasis::Mpfr);

    for input in cases {
        let mp_res = Op::run(&mut mp_vals, input);
        let crate_res = input.call(Op::ROUTINE);

        crate_res.validate(mp_res, input, &ctx).unwrap();
    }
}

libm_macros::for_each_function! {
    callback: mp_domain_tests,
    attributes: [],
    skip: [
        // All functions with more than one input must be disabled
        atan2f,
        copysignf,
        fdimf,
        fmaxf,
        fminf,
        fmodf,
        hypotf,
        nextafterf,
        powf,
        remainderf,
        atan2,
        copysign,
        fdim,
        fmax,
        fmin,
        fmod,
        hypot,
        nextafter,
        pow,
        remainder,
        fmaf,
        fma,
        ilogbf,
        ilogb,
        modf,
        modff,
        jnf,
        jn,
        remquo,
        remquof,
        tgamma,
        tgammaf,
        lgammaf_r,
        lgamma_r,
        lgamma,
        lgammaf,
        sincos,
        sincosf,
        frexp,
        frexpf,
        ilogb,
        ilogbf,
        ldexp,
        ldexpf,
        modf,
        modff,
        remquo,
        remquof,
        scalbn,
        scalbnf,
        nextafter,
        nextafterf,
    ],
}
