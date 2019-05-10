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

// Public modules
mod acos;
mod acosf;
mod acosh;
mod acoshf;
mod asin;
mod asinf;
mod asinh;
mod asinhf;
mod atan;
mod atan2;
mod atan2f;
mod atanf;
mod atanh;
mod atanhf;
mod cbrt;
mod cbrtf;
mod ceil;
mod ceilf;
mod copysign;
mod copysignf;
mod cos;
mod cosf;
mod cosh;
mod coshf;
mod erf;
mod erff;
mod exp;
mod exp10;
mod exp10f;
mod exp2;
mod exp2f;
mod expf;
mod expm1;
mod expm1f;
mod fabs;
mod fabsf;
mod fdim;
mod fdimf;
mod floor;
mod floorf;
mod fma;
mod fmaf;
mod fmod;
mod fmodf;
mod frexp;
mod frexpf;
mod hypot;
mod hypotf;
mod ilogb;
mod ilogbf;
mod j0;
mod j0f;
mod j1;
mod j1f;
mod jn;
mod jnf;
mod ldexp;
mod ldexpf;
mod lgamma;
mod lgamma_r;
mod lgammaf;
mod lgammaf_r;
mod log;
mod log10;
mod log10f;
mod log1p;
mod log1pf;
mod log2;
mod log2f;
mod logf;
mod modf;
mod modff;
mod pow;
mod powf;
mod remquo;
mod remquof;
mod round;
mod roundf;
mod scalbn;
mod scalbnf;
mod sin;
mod sincos;
mod sincosf;
mod sinf;
mod sinh;
mod sinhf;
mod sqrt;
mod sqrtf;
mod tan;
mod tanf;
mod tanh;
mod tanhf;
mod tgamma;
mod tgammaf;
mod trunc;
mod truncf;

// Use separated imports instead of {}-grouped imports for easier merging.
#[rustfmt::skip]
pub use self::{
    acos::acos,
    acosf::acosf,
    acosh::acosh,
    acoshf::acoshf,
    asin::asin,
    asinf::asinf,
    asinh::asinh,
    asinhf::asinhf,
    atan::atan,
    atan2::atan2,
    atan2f::atan2f,
    atanf::atanf,
    atanh::atanh,
    atanhf::atanhf,
    cbrt::cbrt,
    cbrtf::cbrtf,
    ceil::ceil,
    ceilf::ceilf,
    copysign::copysign,
    copysignf::copysignf,
    cos::cos,
    cosf::cosf,
    cosh::cosh,
    coshf::coshf,
    erf::erf,
    erf::erfc,
    erff::erfcf,
    erff::erff,
    exp::exp,
    exp10::exp10,
    exp10f::exp10f,
    exp2::exp2,
    exp2f::exp2f,
    expf::expf,
    expm1::expm1,
    expm1f::expm1f,
    fabs::fabs,
    fabsf::fabsf,
    fdim::fdim,
    fdimf::fdimf,
    floor::floor,
    floorf::floorf,
    fma::fma,
    fmaf::fmaf,
    fmod::fmod,
    fmodf::fmodf,
    frexp::frexp,
    frexpf::frexpf,
    hypot::hypot,
    hypotf::hypotf,
    ilogb::ilogb,
    ilogbf::ilogbf,
    j0::j0,
    j0::y0,
    j0f::j0f,
    j0f::y0f,
    j1::j1,
    j1::y1,
    j1f::j1f,
    j1f::y1f,
    jn::jn,
    jn::yn,
    jnf::jnf,
    jnf::ynf,
    ldexp::ldexp,
    ldexpf::ldexpf,
    lgamma::lgamma,
    lgamma_r::lgamma_r,
    lgammaf::lgammaf,
    lgammaf_r::lgammaf_r,
    log::log,
    log10::log10,
    log10f::log10f,
    log1p::log1p,
    log1pf::log1pf,
    log2::log2,
    log2f::log2f,
    logf::logf,
    modf::modf,
    modff::modff,
    pow::pow,
    powf::powf,
    remquo::remquo,
    remquof::remquof,
    round::round,
    roundf::roundf,
    scalbn::scalbn,
    scalbnf::scalbnf,
    sin::sin,
    sincos::sincos,
    sincosf::sincosf,
    sinf::sinf,
    sinh::sinh,
    sinhf::sinhf,
    sqrt::sqrt,
    sqrtf::sqrtf,
    tan::tan,
    tanf::tanf,
    tanh::tanh,
    tanhf::tanhf,
    tgamma::tgamma,
    tgammaf::tgammaf,
    trunc::trunc,
    truncf::truncf,
};

// Private modules
mod expo2;
mod fenv;
mod k_cos;
mod k_cosf;
mod k_expo2;
mod k_expo2f;
mod k_sin;
mod k_sinf;
mod k_tan;
mod k_tanf;
mod rem_pio2;
mod rem_pio2_large;
mod rem_pio2f;

// Private re-imports
#[rustfmt::skip]
use self::{
    expo2::expo2,
    k_cos::k_cos,
    k_cosf::k_cosf,
    k_expo2::k_expo2,
    k_expo2f::k_expo2f,
    k_sin::k_sin,
    k_sinf::k_sinf,
    k_tan::k_tan,
    k_tanf::k_tanf,
    rem_pio2::rem_pio2,
    rem_pio2_large::rem_pio2_large,
    rem_pio2f::rem_pio2f,
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
