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

// Public modules for f32
mod acosf;
#[cfg(not(feature = "newlib"))]
mod asinf;
mod atan2f;
mod atanf;
mod cbrtf;
mod ceilf;
#[cfg(not(feature = "newlib"))]
mod cosf;
mod coshf;
#[cfg(not(feature = "newlib"))]
mod exp2f;
mod expf;
mod expm1f;
mod fabsf;
mod fdimf;
#[cfg(not(feature = "newlib"))]
mod floorf;
mod fmaf;
mod fmodf;
#[cfg(not(feature = "newlib"))]
mod hypotf;
mod log10f;
mod log1pf;
mod log2f;
mod logf;
mod powf;
mod roundf;
mod scalbnf;
#[cfg(not(feature = "newlib"))]
mod sinf;
mod sinhf;
mod sqrtf;
#[cfg(not(feature = "newlib"))]
mod tanf;
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

#[cfg(not(feature = "newlib"))]
pub use self::{
    asinf::asinf, cosf::cosf, exp2f::exp2f, floorf::floorf, hypotf::hypotf, sinf::sinf, tanf::tanf,
};

pub mod newlib;
#[cfg(feature = "newlib")]
pub use self::newlib::*;

pub mod fp;

pub use self::acosf::acosf;
pub use self::atan2f::atan2f;
pub use self::atanf::atanf;
pub use self::cbrt::cbrt;
pub use self::cbrtf::cbrtf;
pub use self::ceilf::ceilf;
pub use self::coshf::coshf;
pub use self::expm1f::expm1f;
pub use self::fabsf::fabsf;
pub use self::fdimf::fdimf;
pub use self::fmaf::fmaf;
pub use self::fmodf::fmodf;
pub use self::log10f::log10f;
pub use self::log1pf::log1pf;
pub use self::log2f::log2f;
pub use self::logf::logf;
pub use self::powf::powf;
pub use self::roundf::roundf;
pub use self::scalbnf::scalbnf;
pub use self::sinhf::sinhf;
pub use self::sqrtf::sqrtf;
pub use self::tanhf::tanhf;
pub use self::truncf::truncf;

pub use self::acos::acos;
pub use self::asin::asin;
pub use self::atan::atan;
pub use self::atan2::atan2;
pub use self::ceil::ceil;
pub use self::cos::cos;
pub use self::cosh::cosh;
pub use self::exp::exp;
pub use self::exp2::exp2;
pub use self::expf::expf;
pub use self::expm1::expm1;
pub use self::fabs::fabs;
pub use self::fdim::fdim;
pub use self::floor::floor;
pub use self::fma::fma;
pub use self::fmod::fmod;
pub use self::hypot::hypot;
pub use self::log::log;
pub use self::log10::log10;
pub use self::log1p::log1p;
pub use self::log2::log2;
pub use self::pow::pow;
pub use self::round::round;
pub use self::scalbn::scalbn;
pub use self::sin::sin;
pub use self::sinh::sinh;
pub use self::sqrt::sqrt;
pub use self::tan::tan;
pub use self::tanh::tanh;
pub use self::trunc::trunc;

// Private modules
mod expo2;
mod fenv;
mod k_cos;
#[cfg(not(feature = "newlib"))]
mod k_cosf;
mod k_expo2;
mod k_expo2f;
mod k_sin;
#[cfg(not(feature = "newlib"))]
mod k_sinf;
mod k_tan;
#[cfg(not(feature = "newlib"))]
mod k_tanf;
mod rem_pio2;
mod rem_pio2_large;
#[cfg(not(feature = "newlib"))]
mod rem_pio2f;

// Private re-imports
use self::expo2::expo2;
use self::k_cos::k_cos;
use self::k_expo2::k_expo2;
use self::k_expo2f::k_expo2f;
use self::k_sin::k_sin;
use self::k_tan::k_tan;
use self::rem_pio2::rem_pio2;
use self::rem_pio2_large::rem_pio2_large;

#[cfg(not(feature = "newlib"))]
use self::{k_cosf::k_cosf, k_sinf::k_sinf, k_tanf::k_tanf, rem_pio2f::rem_pio2f};

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
