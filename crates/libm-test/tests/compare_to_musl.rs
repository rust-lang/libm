// Targets that we can't compile musl for
#![cfg(not(any(target_env = "msvc", target_family = "wasm")))]
// These tests seem to overflow the stack pretty easily on Windows
#![cfg(not(all(target_family = "windows", not(optimizations_enabled))))]
// FIXME(ppc,crash): LE PPC crashes calling the musl version of some of these and are disabled. It
// seems like a qemu bug but should be investigated further at some point. See
// <https://github.com/rust-lang/libm/issues/309>.
#![cfg(not(all(target_arch = "powerpc64", target_endian = "little")))]

use std::ffi::c_int;
use std::sync::LazyLock;

use libm_test::CheckOutput;
use libm_test::GetVal;
use libm_test::TestCases;
use libm_test::TupleCall;
use musl_math_sys as musl;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const SEED: [u8; 32] = *b"3.141592653589793238462643383279";

const NTESTS: usize = if cfg!(optimizations_enabled) {
    1000
} else {
    100
} * if cfg!(target_pointer_width = "64") {
    5
} else {
    // Tests can be pretty slow on non-64-bit targets
    1
};

/// ULP allowed to differ from musl (note that musl may not be accurate)
const ALLOWED_ULP: u32 = 2;

const ULP_OVERRIDES: &[(&str, u32)] = &[
    // The gamma functions deviate more from musl for whatever reason
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

const ALLOWED_SKIPS: &[&str] = &[
    // Not a generic test function
    "fenv",
    // Nonpublic functions
    "expo2",
    "k_cos",
    "k_cosf",
    "k_expo2",
    "k_expo2f",
    "k_sin",
    "k_sinf",
    "k_tan",
    "k_tanf",
    "rem_pio2",
    "rem_pio2_large",
    "rem_pio2f",
];

static TEST_CASES: LazyLock<TestCases> = LazyLock::new(|| make_test_cases(NTESTS));

/// The first argument to `jn` and `jnf` is the number of iterations. Make this a reasonable
/// value.
static TEST_CASES_JN: LazyLock<TestCases> = LazyLock::new(|| {
    // It is easy to overflow the stack with these in debug mode
    let iterations = if cfg!(optimizations_enabled) && cfg!(target_pointer_width = "64") {
        0xffff
    } else if cfg!(windows) {
        0x00ff
    } else {
        0x0fff
    };

    let mut cases = (&*TEST_CASES).clone();
    for case in cases.inputs_i32_f32.iter_mut() {
        case.0 = iterations;
    }
    for case in cases.inputs_i32_f64.iter_mut() {
        case.0 = iterations;
    }
    cases
});

fn make_test_cases(ntests: usize) -> TestCases {
    let mut rng = ChaCha8Rng::from_seed(SEED);

    let inputs_f32 = (0..ntests).map(|_| rng.gen::<(f32,)>()).collect();
    let inputs_f64 = (0..ntests).map(|_| rng.gen::<(f64,)>()).collect();
    let inputs_f32_f32 = (0..ntests).map(|_| rng.gen::<(f32, f32)>()).collect();
    let inputs_f64_f64 = (0..ntests).map(|_| rng.gen::<(f64, f64)>()).collect();
    let inputs_f32_f32_f32 = (0..ntests).map(|_| rng.gen::<(f32, f32, f32)>()).collect();
    let inputs_f64_f64_f64 = (0..ntests).map(|_| rng.gen::<(f64, f64, f64)>()).collect();
    let inputs_i32_f32 = (0..ntests).map(|_| rng.gen::<(i32, f32)>()).collect();
    let inputs_i32_f64 = (0..ntests).map(|_| rng.gen::<(i32, f64)>()).collect();
    let inputs_f32_i32 = (0..ntests).map(|_| rng.gen::<(f32, i32)>()).collect();
    let inputs_f64_i32 = (0..ntests).map(|_| rng.gen::<(f64, i32)>()).collect();

    TestCases {
        inputs_f32,
        inputs_f64,
        inputs_f32_f32,
        inputs_f64_f64,
        inputs_f32_f32_f32,
        inputs_f64_f64_f64,
        inputs_i32_f32,
        inputs_i32_f64,
        inputs_f32_i32,
        inputs_f64_i32,
    }
}

// The macro should save all functions registered, and take an env from build.rs
// that prints the paths of all files. Add a test that asserts all files have tests.

// TODO: put this into libm as a hidden macro in lib.rs, that takes a callback macro.
// callback macro gets invoked like `@single_test` does here. Then we can reuse this for
// benchmarks, tests, etc. Call it `__for_all_functions`.

macro_rules! make_tests {
    ( $(
        ($($arg:ty),+) => $retty:ty $(| ($($arg2:ty),+) => $retty2:ty)? {
            $(
                $(#[$fn_meta:meta])* // applied to the test
                $name:ident;
            )*
        };
    )* ) => {
        const TESTED_FUNCTIONS: &[&str] = &[
            $(
                $( stringify!($name), )*
            )*
        ];

        $(
            make_tests!{
                @single_signature
                // Always use the crate arg types if available since that will be more accurate
                ArgsTy: make_tests!(@coalesce [($($arg),+ ,)] $( [($($arg2),+ ,)] )? ),
                SysFnTy: fn($($arg),+) -> $retty,
                CrateFnTy: make_tests!(@coalesce [fn($($arg),+) -> $retty] $([fn($($arg2),+) -> $retty2])?),
                functions: [$( {
                    attrs: [$($fn_meta),*],
                    fn_name: $name,
                } ),*],
            }
        )*
    };


    (@single_signature
        ArgsTy: $argty:ty,
        SysFnTy: $fnty_sys:ty,
        CrateFnTy: $fnty_crate:ty,
        functions: [$( {
            attrs: [$($fn_meta:meta),*],
            fn_name: $name:ident,
        } ),*],
    ) => {
        $(
            #[test]
            $(#[$fn_meta])*
            fn $name() {
                let fname = stringify!($name);
                let cases = if fname == "jn" || fname == "jnf" {
                    &TEST_CASES_JN
                } else {
                    &TEST_CASES
                };

                let ulp = match ULP_OVERRIDES.iter().find(|(name, _val)| name == &fname) {
                    Some((_name, val)) => *val,
                    None => ALLOWED_ULP,
                };

                let cases = <$argty>::get_cases(cases);

                for input in cases.iter().copied() {
                    let mres = input.call(musl::$name as $fnty_sys);
                    let cres = input.call(libm::$name as $fnty_crate);

                    mres.validate(cres, input, ulp);
                }
            }
        )*
    };

    // Macro helper to return the second item if two are provided, otherwise the default
    (@coalesce [$($tt1:tt)*]) => { $($tt1)* } ;
    (@coalesce [$($tt1:tt)*] [$($tt2:tt)*]) => { $($tt2)* } ;

}

make_tests! {
    (f32) => f32 {
        acosf;
        acoshf;
        asinf;
        #[cfg_attr(x86_no_sse, ignore)] // FIXME(precision): i586 exceeds minimum ULP
        asinhf;
        atanf;
        atanhf;
        cbrtf;
        ceilf;
        cosf;
        coshf;
        erff;
        #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
        exp10f;
        #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
        exp2f;
        expf;
        expm1f;
        fabsf;
        floorf;
        j0f;
        j1f;
        lgammaf;
        log10f;
        log1pf;
        log2f;
        logf;
        rintf;
        roundf;
        sinf;
        sinhf;
        sqrtf;
        tanf;
        tanhf;
        tgammaf;
        truncf;
    };

    (f64) => f64 {
        acos;
        acosh;
        asin;
        asinh;
        atan;
        atanh;
        cbrt;
        ceil;
        cos;
        cosh;
        erf;
        #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
        exp10;
        #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
        exp2;
        exp;
        expm1;
        fabs;
        floor;
        j0;
        j1;
        lgamma;
        log10;
        log1p;
        log2;
        log;
        rint;
        round;
        sin;
        sinh;
        sqrt;
        tan;
        tanh;
        tgamma;
        trunc;
    };

    (f32, f32) => f32 {
        atan2f;
        copysignf;
        fdimf;
        fmaxf;
        fminf;
        fmodf;
        hypotf;
        nextafterf;
        powf;
        remainderf;
    };

    (f64, f64) => f64 {
        atan2;
        copysign;
        fdim;
        fmax;
        fmin;
        fmod;
        hypot;
        nextafter;
        pow;
        remainder;
    };

    (f32, f32, f32) => f32 {
        fmaf;
    };

    (f64, f64, f64) => f64 {
        fma;
    };

    (f32) => i32 {
        ilogbf;
    };

    (f64) => i32 {
        ilogb;
    };

    (i32, f32) => f32 {
        jnf;
    };

    (f32, i32) => f32 {
        scalbnf;
        ldexpf;
    };

    (i32, f64) => f64 {
        jn;
    };

    (f64, i32) => f64 {
        scalbn;
        ldexp;
    };

    (f32, &mut f32) => f32 | (f32) => (f32, f32) {
        modff;
    };

    (f64, &mut f64) => f64 | (f64) => (f64, f64) {
        modf;
    };

    (f32, &mut c_int) => f32 | (f32) => (f32, i32) {
        frexpf;
        lgammaf_r;
    };

    (f64, &mut c_int) => f64 | (f64) => (f64, i32) {
        frexp;
        lgamma_r;
    };

    (f32, f32, &mut c_int) => f32 | (f32, f32) => (f32, i32) {
        remquof;
    };

    (f64, f64, &mut c_int) => f64 | (f64, f64) => (f64, i32) {
        remquo;
    };

    (f32, &mut f32, &mut f32) => () | (f32) => (f32, f32) {
        sincosf;
    };

    (f64, &mut f64, &mut f64) => () | (f64) => (f64, f64) {
        sincos;
    };

}

#[test]
fn verify_everything_tested() {
    let mut missing = Vec::new();

    for f in libm_test::MATH_FILES {
        if !TESTED_FUNCTIONS.contains(f) && !ALLOWED_SKIPS.contains(f) {
            missing.push(f)
        }
    }

    if !missing.is_empty() {
        panic!("missing tests for the following: {missing:#?}");
    }
}
