//fixme: type are wrong should use long double
use super::signgam;
use core::{f64, i32};
use libc::{c_double, c_int, c_long, c_longlong};

#[no_mangle]
pub extern "C" fn acosl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn acoshl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn asinl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn asinhl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn atanl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn atan2l(y: c_double, x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn atanhl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn cbrtl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ceill(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn cosl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn coshl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn expl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn exp2l(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn expm1l(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn exp10l(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn erfl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn erfcl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fabsl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fdiml(x: c_double, y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn floorl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fmal(x: c_double, y: c_double, z: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fmodl(numer: c_double, denom: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fmaxl(x: c_double, y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fminl(x: c_double, y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn hypotl(x: c_double, y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn logl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn log10l(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn log1pl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn log2l(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn powl(base: c_double, exponent: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn pow10l(exponent: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn roundl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn scalbnl(x: c_double, n: c_int) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sinl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sinhl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sqrtl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn tanl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn tanhl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn truncl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lgammal(arg: c_double) -> c_double {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn tgammal(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ilogbl(arg: c_double) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn llroundl(arg: c_double) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lroundl(arg: c_double) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn llrintl(arg: c_double) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lrintl(arg: c_double) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rintl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nearbyintl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sincosl(x: c_double, sin: *mut c_double, cos: *mut c_double) {
    unimplemented!()
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalblnl(x: c_double, n: c_long) -> c_double {
    unimplemented!()
}

// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn logbl(x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn j0l(x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn y0l(x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn j1l(x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn y1l(x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn jnl(n: c_int, x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ynl(n: c_int, x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nextafterl(x: c_double, y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn remquol(numer: c_double, denom: c_double, quot: *mut c_int) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn remainderl(numer: c_double, denom: c_double) -> c_double {
    unimplemented!()
}

// todo : add a newlib test cfg feature flag?
#[no_mangle]
pub extern "C" fn __isfinitel(x: c_double) -> c_int {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn __isnormall(x: c_double) -> c_int {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn __fpclassifydl(x: c_double) -> c_int {
    unimplemented!()
}
