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

use libm_test::gen::random;
use libm_test::musl_allowed_ulp;
use libm_test::{CheckOutput, TupleCall};
use musl_math_sys as musl;

macro_rules! musl_rand_tests {
    (
        fn_name: $fn_name:ident,
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
            let ulp = musl_allowed_ulp(fname);

            let cases = random::get_test_cases::<$RustArgs>(fname);
            for input in cases {
                let musl_res = input.call(musl::$fn_name as $CFn);
                let crate_res = input.call(libm::$fn_name as $RustFn);

                musl_res.validate(crate_res, input, ulp).unwrap();
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
}
