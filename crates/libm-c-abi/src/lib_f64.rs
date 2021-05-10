use super::signgam;
use core::{f64, i32};
use libc::{c_double, c_int, c_long, c_longlong};
use libm;

#[no_mangle]
pub extern "C" fn acos(arg: c_double) -> c_double {
    libm::acos(arg)
}

#[no_mangle]
pub extern "C" fn acosh(arg: c_double) -> c_double {
    libm::acosh(arg)
}

#[no_mangle]
pub extern "C" fn asin(arg: c_double) -> c_double {
    libm::asin(arg)
}

#[no_mangle]
pub extern "C" fn asinh(arg: c_double) -> c_double {
    libm::asinh(arg)
}

#[no_mangle]
pub extern "C" fn atan(arg: c_double) -> c_double {
    libm::atan(arg)
}

#[no_mangle]
pub extern "C" fn atan2(y: c_double, x: c_double) -> c_double {
    libm::atan2(y, x)
}

#[no_mangle]
pub extern "C" fn atanh(arg: c_double) -> c_double {
    libm::atanh(arg)
}

#[no_mangle]
pub extern "C" fn cbrt(arg: c_double) -> c_double {
    libm::cbrt(arg)
}

#[no_mangle]
pub extern "C" fn ceil(arg: c_double) -> c_double {
    libm::ceil(arg)
}

#[no_mangle]
pub extern "C" fn cos(arg: c_double) -> c_double {
    libm::cos(arg)
}

#[no_mangle]
pub extern "C" fn cosh(arg: c_double) -> c_double {
    libm::cosh(arg)
}

#[no_mangle]
pub extern "C" fn exp(arg: c_double) -> c_double {
    libm::exp(arg)
}

#[no_mangle]
pub extern "C" fn exp2(arg: c_double) -> c_double {
    libm::exp2(arg)
}

#[no_mangle]
pub extern "C" fn expm1(arg: c_double) -> c_double {
    libm::expm1(arg)
}

#[no_mangle]
pub extern "C" fn exp10(arg: c_double) -> c_double {
    libm::exp10(arg)
}

#[no_mangle]
pub extern "C" fn erf(arg: c_double) -> c_double {
    libm::erf(arg)
}

#[no_mangle]
pub extern "C" fn erfc(arg: c_double) -> c_double {
    libm::erf(arg)
}

#[no_mangle]
pub extern "C" fn fabs(arg: c_double) -> c_double {
    libm::fabs(arg)
}

#[no_mangle]
pub extern "C" fn fdim(x: c_double, y: c_double) -> c_double {
    libm::fdim(x, y)
}

#[no_mangle]
pub extern "C" fn floor(arg: c_double) -> c_double {
    libm::floor(arg)
}

#[no_mangle]
pub extern "C" fn fma(x: c_double, y: c_double, z: c_double) -> c_double {
    libm::fma(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmod(numer: c_double, denom: c_double) -> c_double {
    libm::fmod(numer, denom)
}

#[no_mangle]
pub extern "C" fn fmax(x: c_double, y: c_double) -> c_double {
    (x as f64).max(y)
}

#[no_mangle]
pub extern "C" fn fmin(x: c_double, y: c_double) -> c_double {
    (x as f64).min(y)
}

#[no_mangle]
pub extern "C" fn hypot(x: c_double, y: c_double) -> c_double {
    libm::hypot(x, y)
}

#[no_mangle]
pub extern "C" fn log(arg: c_double) -> c_double {
    libm::log(arg)
}

#[no_mangle]
pub extern "C" fn log10(arg: c_double) -> c_double {
    libm::log10(arg)
}

#[no_mangle]
pub extern "C" fn log1p(arg: c_double) -> c_double {
    libm::log1p(arg)
}

#[no_mangle]
pub extern "C" fn log2(arg: c_double) -> c_double {
    libm::log2(arg)
}

#[no_mangle]
pub extern "C" fn pow(base: c_double, exponent: c_double) -> c_double {
    libm::pow(base, exponent)
}

#[no_mangle]
pub extern "C" fn pow10(exponent: c_double) -> c_double {
    libm::pow(10.0, exponent)
}

#[no_mangle]
pub extern "C" fn round(arg: c_double) -> c_double {
    libm::round(arg)
}

#[no_mangle]
pub extern "C" fn scalbn(x: c_double, n: c_int) -> c_double {
    libm::scalbn(x, n)
}

#[no_mangle]
pub extern "C" fn sin(arg: c_double) -> c_double {
    libm::sin(arg)
}

#[no_mangle]
pub extern "C" fn sinh(arg: c_double) -> c_double {
    libm::sinh(arg)
}

#[no_mangle]
pub extern "C" fn sqrt(arg: c_double) -> c_double {
    libm::sqrt(arg)
}

#[no_mangle]
pub extern "C" fn tan(arg: c_double) -> c_double {
    libm::tan(arg)
}

#[no_mangle]
pub extern "C" fn tanh(arg: c_double) -> c_double {
    libm::tanh(arg)
}

#[no_mangle]
pub extern "C" fn trunc(arg: c_double) -> c_double {
    libm::trunc(arg)
}

#[no_mangle]
pub extern "C" fn lgamma(arg: c_double) -> c_double {
    let (res, err) = libm::lgamma_r(arg);
    unsafe { signgam = err };
    res
}
#[no_mangle]
pub extern "C" fn tgamma(arg: c_double) -> c_double {
    libm::tgamma(arg)
}

#[no_mangle]
pub extern "C" fn ilogb(arg: c_double) -> c_int {
    libm::ilogb(arg) as c_int
}

#[no_mangle]
pub extern "C" fn llround(arg: c_double) -> c_longlong {
    libm::round(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lround(arg: c_double) -> c_long {
    libm::round(arg) as c_long
}

#[no_mangle]
pub extern "C" fn llrint(_arg: c_double) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lrint(_arg: c_double) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rint(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nearbyint(_arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sincos(x: c_double, sin: *mut c_double, cos: *mut c_double) {
    let (rsin, rcos) = libm::sincos(x);
    unsafe {
        *sin = rsin;
        *cos = rcos;
    }
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalbln(x: c_double, n: c_long) -> c_double {
    if n > i32::max_value() as i64 {
        libm::scalbn(x, i32::max_value())
    } else if n < i32::min_value() as i64 {
        libm::scalbn(x, i32::min_value())
    } else {
        libm::scalbn(x, n as i32)
    }
}

// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn logb(x: c_double) -> c_double {
    if (x as f64).is_infinite() {
        return x * x;
    }
    if x == 0.0 {
        return -1.0 / (x * x);
    }
    return libm::ilogb(x) as f64;
}

#[no_mangle]
pub extern "C" fn j0(x: c_double) -> c_double {
    return libm::j0(x);
}

#[no_mangle]
pub extern "C" fn y0(x: c_double) -> c_double {
    return libm::y0(x);
}

#[no_mangle]
pub extern "C" fn j1(x: c_double) -> c_double {
    return libm::j1(x);
}

#[no_mangle]
pub extern "C" fn y1(x: c_double) -> c_double {
    return libm::y1(x);
}

#[no_mangle]
pub extern "C" fn jn(n: c_int, x: c_double) -> c_double {
    return libm::jn(n, x);
}

#[no_mangle]
pub extern "C" fn yn(n: c_int, x: c_double) -> c_double {
    return libm::yn(n, x);
}

#[no_mangle]
pub extern "C" fn nextafter(x: c_double, y: c_double) -> c_double {
    return libm::nextafter(x, y);
}

#[no_mangle]
pub extern "C" fn remquo(numer: c_double, denom: c_double, quot: *mut c_int) -> c_double {
    let (res, q) = libm::remquo(numer, denom);
    unsafe {
        *quot = q as i32;
    }
    res
}

#[no_mangle]
pub extern "C" fn remainder(numer: c_double, denom: c_double) -> c_double {
    libm::remquo(numer, denom).0
}

// todo : add a newlib test cfg feature flag?
#[no_mangle]
pub extern "C" fn __isfinite(x: c_double) -> c_int {
    if (x as f64).is_finite() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __isnormal(x: c_double) -> c_int {
    if (x as f64).is_normal() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __fpclassifyd(x: c_double) -> c_int {
    const FP_NORMAL: i32 = 0x4;
    const FP_INFINITE: i32 = 0x1;
    const FP_ZERO: i32 = 0x10;
    const FP_SUBNORMAL: i32 = 0x8;
    const FP_NAN: i32 = 0x2;

    let u = (x as f64).to_bits();
    let e = u >> 52 & 0x7ff;
    if e == 0 {
        return if u << 1 != 0 { FP_SUBNORMAL } else { FP_ZERO };
    }
    if e == 0x7ff {
        return if u << 12 != 0 { FP_NAN } else { FP_INFINITE };
    }
    return FP_NORMAL;
}
