// f64 modules
mod acos;
mod asin;
mod atan;
mod atan2;
mod cbrt;
mod ceil;
mod cos;
mod cosh;
mod exp;
mod expm1;
mod fabs;
mod fdim;
mod floor;
mod fma;
mod fmod;
mod hypot;
mod ldexp;
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

// f32 modules
mod acosf;
mod asinf;
mod atan2f;
mod atanf;
mod cbrtf;
mod ceilf;
mod cosf;
mod coshf;
mod exp2;
mod exp2f;
mod expf;
mod expm1f;
mod fabsf;
mod fdimf;
mod floorf;
mod fmaf;
mod fmodf;
mod hypotf;
mod ldexpf;
mod log10f;
mod log1pf;
mod log2f;
mod logf;
mod powf;
mod roundf;
mod scalbnf;
mod sinf;
mod sinhf;
mod sqrtf;
mod tanf;
mod tanhf;
mod truncf;

// Public f64 functions
#[rustfmt::skip]
pub use self::{
    acos::acos,
    asin::asin,
    atan::atan,
    atan2::atan2,
    cbrt::cbrt,
    ceil::ceil,
    cos::cos,
    cosh::cosh,
    exp::exp,
    exp2::exp2,
    expm1::expm1,
    fabs::fabs,
    fdim::fdim,
    floor::floor,
    fma::fma,
    fmod::fmod,
    hypot::hypot,
    ldexp::ldexp,
    log::log,
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

// Public f32 functions
#[rustfmt::skip]
pub use self::{
    acosf::acosf,
    asinf::asinf,
    atan2f::atan2f,
    atanf::atanf,
    cbrtf::cbrtf,
    ceilf::ceilf,
    cosf::cosf,
    coshf::coshf,
    exp2f::exp2f,
    expf::expf,
    expm1f::expm1f,
    fabsf::fabsf,
    fdimf::fdimf,
    floorf::floorf,
    fmaf::fmaf,
    fmodf::fmodf,
    hypotf::hypotf,
    ldexpf::ldexpf,
    log10::log10,
    log10f::log10f,
    log1p::log1p,
    log1pf::log1pf,
    log2f::log2f,
    logf::logf,
    powf::powf,
    roundf::roundf,
    scalbnf::scalbnf,
    sinf::sinf,
    sinhf::sinhf,
    sqrtf::sqrtf,
    tanf::tanf,
    tanhf::tanhf,
    truncf::truncf,
};

// Private f64 modules
mod expo2;
mod fenv;
mod k_cos;
mod k_expo2;
mod k_sin;
mod k_tan;
mod rem_pio2;
mod rem_pio2_large;

// Private f32 modules
mod k_cosf;
mod k_expo2f;
mod k_sinf;
mod k_tanf;
mod rem_pio2f;

// Private f64 re-imports
#[rustfmt::skip]
use self::{
    expo2::expo2,
    k_cos::k_cos,
    k_sin::k_sin,
    k_tan::k_tan,
    rem_pio2::rem_pio2,
    rem_pio2_large::rem_pio2_large,
};

// Private f32 re-imports
#[rustfmt::skip]
use self::{
    k_cosf::k_cosf,
    k_expo2::k_expo2,
    k_expo2f::k_expo2f,
    k_sinf::k_sinf,
    k_tanf::k_tanf,
    rem_pio2f::rem_pio2f,
};
