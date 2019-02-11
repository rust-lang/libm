macro_rules! force_eval {
    ($e:expr) => {
        unsafe {
            ::core::ptr::read_volatile(&$e);
        }
    };
}

#[cfg(not(feature = "checked"))]
macro_rules! i {
    ($array:expr, $index:expr) => {
        unsafe { *$array.get_unchecked($index) }
    };
    ($array:expr, $index:expr, = , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) = $rhs;
        }
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) += $rhs;
        }
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) -= $rhs;
        }
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) &= $rhs;
        }
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        unsafe { *$array.get_unchecked_mut($index) == $rhs }
    };
}

#[cfg(feature = "checked")]
macro_rules! i {
    ($array:expr, $index:expr) => {
        *$array.get($index).unwrap()
    };
    ($array:expr, $index:expr, = , $rhs:expr) => {
        *$array.get_mut($index).unwrap() = $rhs;
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        *$array.get_mut($index).unwrap() -= $rhs;
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        *$array.get_mut($index).unwrap() += $rhs;
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        *$array.get_mut($index).unwrap() &= $rhs;
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        *$array.get_mut($index).unwrap() == $rhs
    };
}

macro_rules! llvm_intrinsically_optimized {
    (#[cfg($($clause:tt)*)] $e:expr) => {
        #[cfg(all(not(feature = "stable"), $($clause)*))]
        {
            if true { // thwart the dead code lint
                $e
            }
        }
    };
}

mod consts;

// Public modules for f32
mod acoshf;
mod asinhf;
mod atan2f;
mod atanf;
mod atanhf;
mod ceilf;
mod copysignf;
mod coshf;
mod erff;
mod expf;
mod exp10f;
mod expm1f;
mod fabsf;
mod fdimf;
mod fmaf;
mod fmodf;
mod frexpf;
mod ilogbf;
mod j0f;
mod j1f;
mod jnf;
mod log10f;
mod log1pf;
mod log2f;
mod logf;
mod modff;
mod powf;
mod remquof;
mod roundf;
mod scalbnf;
mod sinhf;
mod sqrtf;
mod tanhf;
mod tgammaf;
mod truncf;

// Public modules for f64
mod acos;
mod acosh;
mod asin;
mod asinh;
mod atan;
mod atan2;
mod atanh;
mod cbrt;
mod ceil;
mod copysign;
mod cos;
mod cosh;
mod erf;
mod exp;
mod exp10;
mod exp2;
mod expm1;
mod fabs;
mod fdim;
mod floor;
mod fma;
mod fmod;
mod frexp;
mod hypot;
mod ilogb;
mod j0;
mod j1;
mod jn;
mod lgamma;
mod log;
mod log10;
mod log1p;
mod log2;
mod modf;
mod pow;
mod remquo;
mod round;
mod scalbn;
mod sin;
mod sincos;
mod sinh;
mod sqrt;
mod tan;
mod tanh;
mod tgamma;
mod trunc;

pub mod newlib;
#[cfg(feature = "newlib")]
pub use self::newlib::*;

pub mod musl;
#[cfg(not(feature = "newlib"))]
pub use self::musl::*;

#[rustfmt::skip]
pub use self::{
    acoshf::acoshf,
    asinhf::asinhf,
    atan2f::atan2f,
    atanf::atanf,
    atanhf::atanhf,
    cbrt::cbrt,
    ceilf::ceilf,
    copysignf::copysignf,
    coshf::coshf,
    erff::erff,
    erff::erfcf,
    exp10f::exp10f,
    expm1f::expm1f,
    fabsf::fabsf,
    fdimf::fdimf,
    fmaf::fmaf,
    fmodf::fmodf,
    frexpf::frexpf,
    ilogbf::ilogbf,
    j0f::j0f,
    j0f::y0f,
    j1f::j1f,
    j1f::y1f,
    jnf::jnf,
    jnf::ynf,
    log10f::log10f,
    log1pf::log1pf,
    log2f::log2f,
    logf::logf,
    modff::modff,
    powf::powf,
    remquof::remquof,
    roundf::roundf,
    scalbnf::scalbnf,
    sinhf::sinhf,
    sqrtf::sqrtf,
    tanhf::tanhf,
    tgammaf::tgammaf,
    truncf::truncf,
};

#[rustfmt::skip]
pub use self::{
    acos::acos,
    acosh::acosh,
    asinh::asinh,
    asin::asin,
    atan::atan,
    atan2::atan2,
    atanh::atanh,
    ceil::ceil,
    copysign::copysign,
    cos::cos,
    cosh::cosh,
    erf::erf,
    erf::erfc,
    exp::exp,
    exp10::exp10,
    exp2::exp2,
    expf::expf,
    expm1::expm1,
    fabs::fabs,
    fdim::fdim,
    floor::floor,
    fma::fma,
    fmod::fmod,
    frexp::frexp,
    hypot::hypot,
    ilogb::ilogb,
    j0::j0,
    j0::y0,
    j1::j1,
    j1::y1,
    jn::jn,
    jn::yn,
    lgamma::lgamma,
    lgamma::lgamma_r,
    log::log,
    log10::log10,
    log1p::log1p,
    log2::log2,
    modf::modf,
    pow::pow,
    remquo::remquo,
    round::round,
    scalbn::scalbn,
    sin::sin,
    sincos::sincos,
    sinh::sinh,
    sqrt::sqrt,
    tan::tan,
    tanh::tanh,
    tgamma::tgamma,
    trunc::trunc,
};

// Private modules
mod expo2;
mod fenv;
mod k_cos;
mod k_expo2;
mod k_expo2f;
mod k_sin;
mod k_tan;
mod rem_pio2;
mod rem_pio2_large;

// Private re-imports
#[rustfmt::skip]
use self::{
    expo2::expo2,
    k_cos::k_cos,
    k_expo2::k_expo2,
    k_expo2f::k_expo2f,
    k_sin::k_sin,
    k_tan::k_tan,
    rem_pio2::rem_pio2,
    rem_pio2_large::rem_pio2_large,
};

#[inline]
fn get_high_word(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

#[inline]
fn get_low_word(x: f64) -> u32 {
    x.to_bits() as u32
}

#[inline]
fn with_set_high_word(f: f64, hi: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0x_0000_0000_ffff_ffff;
    tmp |= (hi as u64) << 32;
    f64::from_bits(tmp)
}

#[inline]
fn with_set_low_word(f: f64, lo: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0x_ffff_ffff_0000_0000;
    tmp |= lo as u64;
    f64::from_bits(tmp)
}

#[inline]
fn combine_words(hi: u32, lo: u32) -> f64 {
    f64::from_bits((hi as u64) << 32 | lo as u64)
}
