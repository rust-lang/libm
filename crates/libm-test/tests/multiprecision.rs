//! Test with "infinite precision"

#![cfg(feature = "test-multiprecision")]

use libm_test::domain::HasDomain;
use libm_test::gen::{CachedInput, domain, random};
use libm_test::mpfloat::MpOp;
use libm_test::{CheckBasis, CheckCtx, CheckOutput, Float, GenerateInput, MathOp, TupleCall};

/// Implement a test against MPFR with random inputs.
macro_rules! multiprec_rand_tests {
    (
        fn_name: $fn_name:ident,
        attrs: [$($meta:meta)*]
    ) => {
        paste::paste! {
            #[test]
            $(#[$meta])*
            fn [< multiprec_random_ $fn_name >]() {
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
    callback: multiprec_rand_tests,
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

/// Implement a test against MPFR with domain-aware inputs.
macro_rules! multiprec_domain_tests {
    (
        fn_name: $fn_name:ident,
        attrs: [$($meta:meta)*]
    ) => {
        paste::paste! {
            #[test]
            $(#[$meta])*
            fn [< multiprec_domain_ $fn_name >]() {
                test_one_domain::<libm_test::op::$fn_name::Routine>();
            }
        }
    };
}

/// Test a single routine against domaine-aware inputs
fn test_one_domain<Op>()
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
    // And then we need to be able to cast integers (required by `Domain`).
    <Op::FTy as Float>::Int: TryFrom<usize>,
{
    let name = Op::NAME_STR;

    let ulp = multiprec_allowed_ulp(name);
    let mut mp_vals = Op::new_mp();
    let ctx = CheckCtx::new(ulp, name, CheckBasis::Mpfr);
    let cases = domain::get_test_cases::<Op>();

    for input in cases {
        let mp_res = Op::run(&mut mp_vals, input);
        let crate_res = input.call(Op::ROUTINE);

        crate_res.validate(mp_res, input, &ctx).unwrap();
    }
}

libm_macros::for_each_function! {
    callback: multiprec_domain_tests,
    attributes: [],
    skip: [
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
