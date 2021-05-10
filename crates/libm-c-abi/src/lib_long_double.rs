//fixme: type are wrong should use long double

use libc::{c_double, c_int, c_long, c_longlong};

#[no_mangle]
pub extern "C" fn acosl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn acoshl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn asinl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn asinhl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn atanl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn atan2l(_y: c_double, _x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn atanhl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn cbrtl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ceill(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn cosl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn coshl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn expl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn exp2l(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn expm1l(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn exp10l(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn erfl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn erfcl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fabsl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fdiml(_x: c_double, _y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn floorl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fmal(_x: c_double, _y: c_double, _z: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fmodl(_numer: c_double, _denom: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fmaxl(_x: c_double, _y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fminl(_x: c_double, _y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn hypotl(_x: c_double, _y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn logl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn log10l(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn log1pl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn log2l(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn powl(_base: c_double, _exponent: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn pow10l(_exponent: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn roundl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn scalbnl(_x: c_double, _n: c_int) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sinl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sinhl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sqrtl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn tanl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn tanhl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn truncl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lgammal(_arg: c_double) -> c_double {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn tgammal(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ilogbl(_arg: c_double) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn llroundl(_arg: c_double) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lroundl(_arg: c_double) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn llrintl(_arg: c_double) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lrintl(_arg: c_double) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rintl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nearbyintl(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sincosl(_x: c_double, _si_n: *mut c_double, _cos: *mut c_double) {
    unimplemented!()
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalblnl(_x: c_double, _n: c_long) -> c_double {
    unimplemented!()
}

// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn logbl(_x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn j0l(_x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn y0l(_x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn j1l(_x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn y1l(_x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn jnl(_n: c_int, _x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn ynl(_n: c_int, _x: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nextafterl(_x: c_double, _y: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn remquol(_numer: c_double, _denom: c_double, _quot: *mut c_int) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn remainderl(_numer: c_double, _denom: c_double) -> c_double {
    unimplemented!()
}

// todo : add a newlib test cfg feature flag?
#[no_mangle]
pub extern "C" fn __isfinitel(_x: c_double) -> c_int {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn __isnormall(_x: c_double) -> c_int {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn __fpclassifydl(_x: c_double) -> c_int {
    unimplemented!()
}
