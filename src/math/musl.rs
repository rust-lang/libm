// Public modules
pub mod acosf;
pub mod asinf;
pub mod cbrtf;
pub mod cosf;
pub mod exp2f;
pub mod floorf;
pub mod hypotf;
pub mod lgammaf;
pub mod sinf;
pub mod sincosf;
pub mod tanf;

// Public modules for f32
pub mod acoshf;
pub mod asinhf;
pub mod atan2f;
pub mod atanf;
pub mod atanhf;
pub mod ceilf;
pub mod copysignf;
pub mod coshf;
pub mod erff;
pub mod expf;
pub mod exp10f;
pub mod expm1f;
pub mod fabsf;
pub mod fdimf;
pub mod fmaf;
pub mod fmodf;
pub mod frexpf;
pub mod ilogbf;
pub mod j0f;
pub mod j1f;
pub mod jnf;
pub mod log10f;
pub mod log1pf;
pub mod log2f;
pub mod logf;
pub mod modff;
pub mod powf;
pub mod remquof;
pub mod roundf;
pub mod scalbnf;
pub mod sinhf;
pub mod sqrtf;
pub mod tanhf;
pub mod tgammaf;
pub mod truncf;

// Public modules for f64
pub mod acos;
pub mod acosh;
pub mod asin;
pub mod asinh;
pub mod atan;
pub mod atan2;
pub mod atanh;
pub mod cbrt;
pub mod ceil;
pub mod copysign;
pub mod cos;
pub mod cosh;
pub mod erf;
pub mod exp;
pub mod exp10;
pub mod exp2;
pub mod expm1;
pub mod fabs;
pub mod fdim;
pub mod floor;
pub mod fma;
pub mod fmod;
pub mod frexp;
pub mod hypot;
pub mod ilogb;
pub mod j0;
pub mod j1;
pub mod jn;
pub mod lgamma;
pub mod log;
pub mod log10;
pub mod log1p;
pub mod log2;
pub mod modf;
pub mod pow;
pub mod remquo;
pub mod round;
pub mod scalbn;
pub mod sin;
pub mod sincos;
pub mod sinh;
pub mod sqrt;
pub mod tan;
pub mod tanh;
pub mod tgamma;
pub mod trunc;

#[rustfmt::skip]
#[cfg(feature = "musl")]
pub use self::{
    acosf::acosf,
    asinf::asinf,
    cbrtf::cbrtf,
    cosf::cosf,
    floorf::floorf,
    hypotf::hypotf,
    lgammaf::lgammaf,
    lgammaf::lgammaf_r,
    sincosf::sincosf,
    sinf::sinf,
    tanf::tanf,
};

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
    exp2f::exp2f,
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

// Private modules
mod k_cosf;
mod k_sinf;
mod k_tanf;
mod rem_pio2f;

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
// Private re-imports
#[rustfmt::skip]
use self::{
    k_cosf::k_cosf,
    k_sinf::k_sinf,
    k_tanf::k_tanf,
    rem_pio2f::rem_pio2f,
};
