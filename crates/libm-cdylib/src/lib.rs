#![cfg(
    // The tests are only enabled on x86 32/64-bit linux/macos:
    all(unstable_rust,
        any(target_os = "linux", target_os = "macos"),
        any(target_arch = "x86", target_arch = "x86_64")
    )
)]
#![allow(dead_code)]
#![cfg_attr(not(test), feature(core_intrinsics, lang_items))]
#![cfg_attr(not(test), no_std)]

#[path = "../../../src/math/mod.rs"]
mod libm;

#[macro_use]
mod macros;

#[cfg(test)]
mod test_utils;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: just call libc::abort - fails to link
    // unsafe { libc::abort() }
    unsafe { core::intrinsics::abort() }
}

// TODO: there has to be a way to avoid this
#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

// This macro exports the functions that are part of the C ABI.
//
// It generates tests that replace the implementation of the
// function with a specific value, that then is used to check
// that the library is properly linked.

export! {
    fn acos(x: f64) -> f64: (42.) -> 42.;
    fn acosf(x: f32) -> f32: (42.) -> 42.;
    fn acosh(x: f64) -> f64: (42.) -> 42.;
    fn acoshf(x: f32) -> f32: (42.) -> 42.;
    fn asin(x: f64) -> f64: (42.) -> 42.;
    fn asinf(x: f32) -> f32: (42.) -> 42.;
    fn asinh(x: f64) -> f64: (42.) -> 42.;
    fn asinhf(x: f32) -> f32: (42.) -> 42.;
    // FIXME: fails to link. Missing symbol: _memcpy
    // fn atan(x: f64) -> f64: (42.) -> 42.;
    fn atanf(x: f32) -> f32: (42.) -> 42.;
    fn atanh(x: f64) -> f64: (42.) -> 42.;
    fn atanhf(x: f32) -> f32: (42.) -> 42.;
    fn cbrt(x: f64) -> f64: (42.) -> 42.;
    fn cbrtf(x: f32) -> f32: (42.) -> 42.;
    fn ceil(x: f64) -> f64: (42.) -> 42.;
    fn ceilf(x: f32) -> f32: (42.) -> 42.;
    fn copysign(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn copysignf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;
    // FIXME: fails to link. Missing symbols
    //fn cos(x: f64) -> f64: (42.) -> 42.;
    //fn cosf(x: f32) -> f32: (42.) -> 42.;
    fn cosh(x: f64) -> f64: (42.) -> 42.;
    fn coshf(x: f32) -> f32: (42.) -> 42.;
    fn erf(x: f64) -> f64: (42.) -> 42.;
    fn erfc(x: f64) -> f64: (42.) -> 42.;
    fn erff(x: f32) -> f32: (42.) -> 42.;
    fn erfcf(x: f32) -> f32: (42.) -> 42.;
    fn exp(x: f64) -> f64: (42.) -> 42.;
    fn expf(x: f32) -> f32: (42.) -> 42.;
    // FIXME: not in C:
    // fn exp10(x: f64) -> f64: (42.) -> 42.;
    // fn exp10f(x: f32) -> f32: (42.) -> 42.;
    fn exp2(x: f64) -> f64: (42.) -> 42.;
    fn exp2f(x: f32) -> f32: (42.) -> 42.;
    fn expm1(x: f64) -> f64: (42.) -> 42.;
    fn expm1f(x: f32) -> f32: (42.) -> 42.;
    fn fabs(x: f64) -> f64: (42.) -> 42.;
    fn fabsf(x: f32) -> f32: (42.) -> 42.;
    fn fdim(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn fdimf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;
    fn floor(x: f64) -> f64: (42.) -> 42.;
    fn floorf(x: f32) -> f32: (42.) -> 42.;
    fn fma(x: f64, y: f64, z: f64) -> f64: (42., 42., 42.) -> 42.;
    fn fmaf(x: f32, y: f32, z: f32) -> f32: (42., 42., 42.) -> 42.;
    fn fmax(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn fmaxf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;
    fn fmin(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn fminf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;
    fn fmod(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn fmodf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;

    // TODO: different ABI than in C - need a more elaborate wrapper
    // fn frexp(x: f64) -> (f64, i32): (42.) -> (42., 42);
    // fn frexpf(x: f32) -> (f32, i32): (42.) -> (42., 42);

    fn hypot(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn hypotf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;
    fn ilogb(x: f64) -> i32: (42.) -> 42;
    fn ilogbf(x: f32) -> i32: (42.) -> 42;

    // FIXME: fails to link. Missing symbols
    // fn j0(x: f64) -> f64: (42.) -> 42.;
    // fn j0f(x: f32) -> f32: (42.) -> 42.;
    // fn j1(x: f64) -> f64: (42.) -> 42.;
    // fn j1f(x: f32) -> f32: (42.) -> 42.;
    // fn jn(n: i32, x: f64) -> f64: (42, 42.) -> 42.;
    // fn jnf(n: i32, x: f32) -> f32: (42, 42.) -> 42.;

    fn ldexp(x: f64, n: i32) -> f64: (42, 42.) -> 42.;
    fn ldexpf(x: f32, n: i32) -> f32: (42, 42.) -> 42.;
    fn lgamma(x: f64) -> f64: (42.) -> 42.;
    fn lgammaf(x: f32) -> f32: (42.) -> 42.;

    // TODO: different ABI than in C - need a more elaborate wrapper
    // fn lgamma_r(x: f64) -> (f64, i32): (42.) -> (42., 42);
    // fn lgammaf_r(x: f32) -> (f32, i32): (42.) -> (42., 42);

    fn log(x: f64) -> f64: (42.) -> 42.;
    fn logf(x: f32) -> f32: (42.) -> 42.;
    fn log10(x: f64) -> f64: (42.) -> 42.;
    fn log10f(x: f32) -> f32: (42.) -> 42.;
    fn log1p(x: f64) -> f64: (42.) -> 42.;
    fn log1pf(x: f32) -> f32: (42.) -> 42.;
    fn log2(x: f64) -> f64: (42.) -> 42.;
    fn log2f(x: f32) -> f32: (42.) -> 42.;
    fn pow(x: f64, y: f64) -> f64: (42., 42.) -> 42.;
    fn powf(x: f32, y: f32) -> f32: (42., 42.) -> 42.;

    // FIXME: different ABI than in C - need a more elaborate wrapper
    // fn modf(x: f64) -> (f64, f64): (42.) -> (42., 42.);
    // fn modff(x: f32) -> (f32, f32): (42.) -> (42., 42.);
    // remquo
    // remquof

    fn round(x: f64) -> f64: (42.) -> 42.;
    fn roundf(x: f32) -> f32: (42.) -> 42.;
    fn scalbn(x: f64, n: i32) -> f64: (42., 42) -> 42.;
    fn scalbnf(x: f32, n: i32) -> f32: (42., 42) -> 42.;

    // FIXME: different ABI than in C - need a more elaborate wrapper
    // fn sincos
    // fn sincosf

    // FIXME: missing symbols - fails to link
    // fn sin(x: f64) -> f64: (42.) -> 42.;
    // fn sinf(x: f32) -> f32: (42.) -> 42.;

    fn sinh(x: f64) -> f64: (42.) -> 42.;
    fn sinhf(x: f32) -> f32: (42.) -> 42.;
    fn sqrt(x: f64) -> f64: (42.) -> 42.;
    fn sqrtf(x: f32) -> f32: (42.) -> 42.;
    // FIXME: missing symbols - fails to link
    // fn tan(x: f64) -> f64: (42.) -> 42.;
    // fn tanf(x: f32) -> f32: (42.) -> 42.;
    fn tanh(x: f64) -> f64: (42.) -> 42.;
    fn tanhf(x: f32) -> f32: (42.) -> 42.;
    // FIXME: missing symbols - fails to link
    // fn tgamma(x: f64) -> f64: (42.) -> 42.;
    // fn tgammaf(x: f32) -> f32: (42.) -> 42.;
    fn trunc(x: f64) -> f64: (42.) -> 42.;
    fn truncf(x: f32) -> f32: (42.) -> 42.;
}
