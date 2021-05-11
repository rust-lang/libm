use libc::{c_float, c_int, c_long, c_longlong};

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
pub extern "C" fn __fpclassifyf(_x: c_float) -> c_int {
    const FP_NORMAL: i32 = 0x4;
    FP_NORMAL
}
