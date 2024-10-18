//! Compare our implementations with the result of musl functions, as provided by `musl-math-sys`.
//!
//! Currently this only tests randomized inputs. In the future this may be improved to test edge
//! cases or run exhaustive tests.
//!
//! Note that musl functions do not always provide 0.5ULP rounding, so our functions can do better
//! than these results.

// Targets that we can't compile musl for
#![cfg(not(any(target_env = "msvc", target_family = "wasm")))]
// These wind up with stack overflows
#![cfg(not(all(target_family = "windows", target_env = "gnu")))]
// FIXME(#309): LE PPC crashes calling the musl version of some of these and are disabled. It
// seems like a qemu bug but should be investigated further at some point. See
// <https://github.com/rust-lang/libm/issues/309>.
#![cfg(not(all(target_arch = "powerpc64", target_endian = "little")))]

use std::sync::LazyLock;

use libm_test::gen::CachedInput;
use libm_test::{CheckOutput, GenerateInput, TupleCall};
use musl_math_sys as musl;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const SEED: [u8; 32] = *b"3.141592653589793238462643383279";

const NTESTS: usize = {
    let mut ntests = if cfg!(optimizations_enabled) {
        5000
    } else {
        500
    };

    // Tests can be pretty slow on non-64-bit targets and, for some reason, ppc.
    if !cfg!(target_pointer_width = "64") || cfg!(target_arch = "powerpc64") {
        ntests /= 5;
    }

    ntests
};

/// ULP allowed to differ from musl (note that musl itself may not be accurate).
const ALLOWED_ULP: u32 = 2;

/// Certain functions have different allowed ULP (consider these xfail).
///
/// Currently this includes:
/// - gamma functions that have higher errors
/// - 32-bit functions fall back to a less precise algorithm.
const ULP_OVERRIDES: &[(&str, u32)] = &[
    #[cfg(x86_no_sse)]
    ("asinhf", 6),
    ("lgamma", 6),
    ("lgamma_r", 6),
    ("lgammaf", 6),
    ("lgammaf_r", 6),
    ("tanh", 4),
    ("tgamma", 8),
    #[cfg(not(target_pointer_width = "64"))]
    ("exp10", 4),
    #[cfg(not(target_pointer_width = "64"))]
    ("exp10f", 4),
];

/// Tested inputs.
static TEST_CASES: LazyLock<CachedInput> = LazyLock::new(|| make_test_cases(NTESTS));

/// The first argument to `jn` and `jnf` is the number of iterations. Make this a reasonable
/// value so tests don't run forever.
static TEST_CASES_JN: LazyLock<CachedInput> = LazyLock::new(|| {
    // It is easy to overflow the stack with these in debug mode
    let iterations = if cfg!(optimizations_enabled) && cfg!(target_pointer_width = "64") {
        0xffff
    } else if cfg!(windows) {
        0x00ff
    } else {
        0x0fff
    };

    let mut cases = (&*TEST_CASES).clone();
    for case in cases.inputs_i32.iter_mut() {
        case.0 = iterations;
    }
    for case in cases.inputs_i32.iter_mut() {
        case.0 = iterations;
    }
    cases
});

fn make_test_cases(ntests: usize) -> CachedInput {
    let mut rng = ChaCha8Rng::from_seed(SEED);

    let inputs_i32 = (0..ntests).map(|_| rng.gen::<(i32, i32, i32)>()).collect();
    let inputs_f32 = (0..ntests).map(|_| rng.gen::<(f32, f32, f32)>()).collect();
    let inputs_f64 = (0..ntests).map(|_| rng.gen::<(f64, f64, f64)>()).collect();

    CachedInput {
        inputs_f32,
        inputs_f64,
        inputs_i32,
    }
}

macro_rules! musl_rand_tests {
    (
        fn_name: $fn_name:ident,
        extra: [],
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: $RustFn:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        attrs: [$($meta:meta)*]
    ) => { paste::paste! {
        #[test]
        $(#[$meta])*
        fn [< musl_random_ $fn_name >]() {
            let fname = stringify!($fn_name);
            let inputs = if fname == "jn" || fname == "jnf" {
                &TEST_CASES_JN
            } else {
                &TEST_CASES
            };

            let ulp = match ULP_OVERRIDES.iter().find(|(name, _val)| name == &fname) {
                Some((_name, val)) => *val,
                None => ALLOWED_ULP,
            };

            let cases = <CachedInput as GenerateInput<$RustArgs>>::get_cases(inputs);
            for input in cases {
                let mres = input.call(musl::$fn_name as $CFn);
                let cres = input.call(libm::$fn_name as $RustFn);

                mres.validate(cres, input, ulp);
            }
        }
    } };
}

libm_macros::for_each_function! {
    callback: musl_rand_tests,
    skip: [],
    attributes: [
        #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
        [exp10, exp10f, exp2, exp2f]
    ],
    extra: [],
}
