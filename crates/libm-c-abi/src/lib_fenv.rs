use libc::{c_int, c_ulong};

//const FE_ALL_EXCEPT: c_int = 0;
const FE_TONEAREST: c_int = 0;

#[repr(C)]
pub struct fenv_t {
    __cw: c_ulong,
}

#[no_mangle]
pub extern "C" fn feclearexcept(_mask: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn feraiseexcept(_mask: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fetestexcept(_mask: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fegetround() -> c_int {
    FE_TONEAREST
}

#[no_mangle]
extern "C" fn __fesetround(_r: c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fegetenv(_envp: *const fenv_t) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fesetenv(_envp: *const fenv_t) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn fesetround(r: c_int) -> c_int {
    __fesetround(r)
}
