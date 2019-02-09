#![rustfmt::skip]

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

mod consts {
    pub const UF_1 : u32 = 0x3f800000;
}

// Public modules for f32
mod atan2f;
mod atanf;
mod ceilf;
mod coshf;
mod expf;
mod expm1f;
mod fabsf;
mod fdimf;
mod fmaf;
mod fmodf;
mod log10f;
mod log1pf;
mod log2f;
mod logf;
mod powf;
mod roundf;
mod scalbnf;
mod sinhf;
mod sqrtf;
mod tanhf;
mod truncf;

// Public modules for f64
mod acos;
mod asin;
mod atan;
mod atan2;
mod cbrt;
mod ceil;
mod cos;
mod cosh;
mod exp;
mod exp2;
mod expm1;
mod fabs;
mod fdim;
mod floor;
mod fma;
mod fmod;
mod hypot;
mod log;
mod log10;
mod log1p;
mod log2;
mod pow;
mod round;
mod scalbn;
mod sin;
mod sinh;
mod sqrt;
mod tan;
mod tanh;
mod trunc;

pub mod newlib;
#[cfg(feature = "newlib")]
pub use self::newlib::*;

pub mod musl;
#[cfg(not(feature = "newlib"))]
pub use self::musl::*;

pub mod fp;

pub use self::{
    atan2f::atan2f,
    atanf::atanf,
    cbrt::cbrt,
    ceilf::ceilf,
    coshf::coshf,
    expm1f::expm1f,
    fabsf::fabsf,
    fdimf::fdimf,
    fmaf::fmaf,
    fmodf::fmodf,
    log10f::log10f,
    log1pf::log1pf,
    log2f::log2f,
    logf::logf,
    powf::powf,
    roundf::roundf,
    scalbnf::scalbnf,
    sinhf::sinhf,
    sqrtf::sqrtf,
    tanhf::tanhf,
    truncf::truncf,
};

pub use self::{
    acos::acos,
    asin::asin,
    atan::atan,
    atan2::atan2,
    ceil::ceil,
    cos::cos,
    cosh::cosh,
    exp::exp,
    exp2::exp2,
    expf::expf,
    expm1::expm1,
    fabs::fabs,
    fdim::fdim,
    floor::floor,
    fma::fma,
    fmod::fmod,
    hypot::hypot,
    log::log,
    log10::log10,
    log1p::log1p,
    log2::log2,
    pow::pow,
    round::round,
    scalbn::scalbn,
    sin::sin,
    sinh::sinh,
    sqrt::sqrt,
    tan::tan,
    tanh::tanh,
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
    tmp &= 0x00000000_ffffffff;
    tmp |= (hi as u64) << 32;
    f64::from_bits(tmp)
}

#[inline]
fn with_set_low_word(f: f64, lo: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0xffffffff_00000000;
    tmp |= lo as u64;
    f64::from_bits(tmp)
}

#[inline]
fn combine_words(hi: u32, lo: u32) -> f64 {
    f64::from_bits((hi as u64) << 32 | lo as u64)
}
