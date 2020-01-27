use super::signgam;
use core::{f32, i32};
use libc::{c_float, c_int, c_long, c_longlong};
use libm;

#[no_mangle]
pub extern "C" fn acosf(arg: c_float) -> c_float {
    libm::acosf(arg)
}

#[no_mangle]
pub extern "C" fn acoshf(arg: c_float) -> c_float {
    libm::acoshf(arg)
}

#[no_mangle]
pub extern "C" fn asinf(arg: c_float) -> c_float {
    libm::asinf(arg)
}

#[no_mangle]
pub extern "C" fn asinhf(arg: c_float) -> c_float {
    libm::asinhf(arg)
}

#[no_mangle]
pub extern "C" fn atanf(arg: c_float) -> c_float {
    libm::atanf(arg)
}

#[no_mangle]
pub extern "C" fn atan2f(y: c_float, x: c_float) -> c_float {
    libm::atan2f(y, x)
}

#[no_mangle]
pub extern "C" fn atanhf(arg: c_float) -> c_float {
    libm::atanhf(arg)
}

#[no_mangle]
pub extern "C" fn cbrtf(arg: c_float) -> c_float {
    libm::cbrtf(arg)
}

#[no_mangle]
pub extern "C" fn ceilf(arg: c_float) -> c_float {
    libm::ceilf(arg)
}

#[no_mangle]
pub extern "C" fn cosf(arg: c_float) -> c_float {
    libm::cosf(arg)
}

#[no_mangle]
pub extern "C" fn coshf(arg: c_float) -> c_float {
    libm::coshf(arg)
}

#[no_mangle]
pub extern "C" fn expf(arg: c_float) -> c_float {
    libm::expf(arg)
}

#[no_mangle]
pub extern "C" fn exp2f(arg: c_float) -> c_float {
    libm::exp2f(arg)
}

#[no_mangle]
pub extern "C" fn expm1f(arg: c_float) -> c_float {
    libm::expm1f(arg)
}

#[no_mangle]
pub extern "C" fn exp10f(arg: c_float) -> c_float {
    libm::exp10f(arg)
}

#[no_mangle]
pub extern "C" fn erff(arg: c_float) -> c_float {
    libm::erff(arg)
}

#[no_mangle]
pub extern "C" fn erfcf(arg: c_float) -> c_float {
    libm::erff(arg)
}

#[no_mangle]
pub extern "C" fn fabsf(arg: c_float) -> c_float {
    libm::fabsf(arg)
}

#[no_mangle]
pub extern "C" fn fdimf(x: c_float, y: c_float) -> c_float {
    libm::fdimf(x, y)
}

#[no_mangle]
pub extern "C" fn floorf(arg: c_float) -> c_float {
    libm::floorf(arg)
}

#[no_mangle]
pub extern "C" fn fmaf(x: c_float, y: c_float, z: c_float) -> c_float {
    libm::fmaf(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmodf(numer: c_float, denom: c_float) -> c_float {
    libm::fmodf(numer, denom)
}

#[no_mangle]
pub extern "C" fn fmaxf(x: c_float, y: c_float) -> c_float {
    (x as f32).max(y)
}

#[no_mangle]
pub extern "C" fn fminf(x: c_float, y: c_float) -> c_float {
    (x as f32).min(y)
}

#[no_mangle]
pub extern "C" fn hypotf(x: c_float, y: c_float) -> c_float {
    libm::hypotf(x, y)
}

#[no_mangle]
pub extern "C" fn logf(arg: c_float) -> c_float {
    libm::logf(arg)
}

#[no_mangle]
pub extern "C" fn log10f(arg: c_float) -> c_float {
    libm::log10f(arg)
}

#[no_mangle]
pub extern "C" fn log1pf(arg: c_float) -> c_float {
    libm::log1pf(arg)
}

#[no_mangle]
pub extern "C" fn log2f(arg: c_float) -> c_float {
    libm::log2f(arg)
}

#[no_mangle]
pub extern "C" fn powf(base: c_float, exponent: c_float) -> c_float {
    libm::powf(base, exponent)
}

#[no_mangle]
pub extern "C" fn pow10f(exponent: c_float) -> c_float {
    libm::powf(10.0, exponent)
}

#[no_mangle]
pub extern "C" fn roundf(arg: c_float) -> c_float {
    libm::roundf(arg)
}

#[no_mangle]
pub extern "C" fn scalbnf(x: c_float, n: c_int) -> c_float {
    libm::scalbnf(x, n)
}

#[no_mangle]
pub extern "C" fn sinf(arg: c_float) -> c_float {
    libm::sinf(arg)
}

#[no_mangle]
pub extern "C" fn sinhf(arg: c_float) -> c_float {
    libm::sinhf(arg)
}

#[no_mangle]
pub extern "C" fn sqrtf(arg: c_float) -> c_float {
    libm::sqrtf(arg)
}

#[no_mangle]
pub extern "C" fn tanf(arg: c_float) -> c_float {
    libm::tanf(arg)
}

#[no_mangle]
pub extern "C" fn tanhf(arg: c_float) -> c_float {
    libm::tanhf(arg)
}

#[no_mangle]
pub extern "C" fn truncf(arg: c_float) -> c_float {
    libm::truncf(arg)
}

#[no_mangle]
pub extern "C" fn lgammaf(arg: c_float) -> c_float {
    let (res, err) = libm::lgammaf_r(arg);
    unsafe { signgam = err };
    res
}
#[no_mangle]
pub extern "C" fn tgammaf(arg: c_float) -> c_float {
    libm::tgammaf(arg)
}

#[no_mangle]
pub extern "C" fn ilogbf(arg: c_float) -> c_int {
    libm::ilogbf(arg) as c_int
}

#[no_mangle]
pub extern "C" fn llroundf(arg: c_float) -> c_longlong {
    libm::roundf(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lroundf(arg: c_float) -> c_long {
    libm::roundf(arg) as c_long
}

#[no_mangle]
pub extern "C" fn llrintf(arg: c_float) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lrintf(arg: c_float) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rintf(arg: c_float) -> c_float {
    unimplemented!()
}
#[no_mangle]
pub extern "C" fn nearbyintf(arg: c_float) -> c_float {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sincosf(x: c_float, sin: *mut c_float, cos: *mut c_float) {
    let (rsin, rcos) = libm::sincosf(x);
    unsafe {
        *sin = rsin;
        *cos = rcos;
    }
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalblnf(x: c_float, n: c_long) -> c_float {
    if n > i32::max_value() as i64 {
        libm::scalbnf(x, i32::max_value())
    } else if n < i32::min_value() as i64 {
        libm::scalbnf(x, i32::min_value())
    } else {
        libm::scalbnf(x, n as i32)
    }
}

// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn logbf(x: c_float) -> c_float {
    if (x as f32).is_infinite() {
        return x * x;
    }
    if x == 0.0 {
        return -1.0 / (x * x);
    }
    return libm::ilogbf(x) as f32;
}

#[no_mangle]
pub extern "C" fn j0f(x: c_float) -> c_float {
    return libm::j0f(x);
}

#[no_mangle]
pub extern "C" fn y0f(x: c_float) -> c_float {
    return libm::y0f(x);
}

#[no_mangle]
pub extern "C" fn j1f(x: c_float) -> c_float {
    return libm::j1f(x);
}

#[no_mangle]
pub extern "C" fn y1f(x: c_float) -> c_float {
    return libm::y1f(x);
}

#[no_mangle]
pub extern "C" fn jnf(n: c_int, x: c_float) -> c_float {
    return libm::jnf(n, x);
}

#[no_mangle]
pub extern "C" fn ynf(n: c_int, x: c_float) -> c_float {
    return libm::ynf(n, x);
}

#[no_mangle]
pub extern "C" fn nextafterf(x: c_float, y: c_float) -> c_float {
    return libm::nextafterf(x, y);
}

#[no_mangle]
pub extern "C" fn remquof(numer: c_float, denom: c_float, quot: *mut c_int) -> c_float {
    let (res, q) = libm::remquof(numer, denom);
    unsafe {
        *quot = q as i32;
    }
    res
}

#[no_mangle]
pub extern "C" fn remainderf(numer: c_float, denom: c_float) -> c_float {
    libm::remquof(numer, denom).0
}

// todo : add a newlib test cfg feature flag?
#[no_mangle]
pub extern "C" fn __isfinitef(x: c_float) -> c_int {
    if (x as f32).is_finite() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __isnormalf(x: c_float) -> c_int {
    if (x as f32).is_normal() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn __fpclassifyf(x: c_float) -> c_int {
    const FP_NORMAL: i32 = 0x4;
    FP_NORMAL
}
