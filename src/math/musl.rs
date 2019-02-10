// Public modules
mod acosf;
mod asinf;
mod cbrtf;
mod cosf;
mod exp2f;
mod floorf;
mod hypotf;
mod lgammaf;
mod sinf;
mod sincosf;
mod tanf;

#[rustfmt::skip]
pub use self::{
    acosf::acosf,
    asinf::asinf,
    cbrtf::cbrtf,
    cosf::cosf,
    exp2f::exp2f,
    floorf::floorf,
    hypotf::hypotf,
    lgammaf::lgammaf,
    lgammaf::lgammaf_r,
    sincosf::sincosf,
    sinf::sinf,
    tanf::tanf,
};

// Private modules
mod k_cosf;
mod k_sinf;
mod k_tanf;
mod rem_pio2f;

// Private re-imports
#[rustfmt::skip]
use self::{
    k_cosf::k_cosf,
    k_sinf::k_sinf,
    k_tanf::k_tanf,
    rem_pio2f::rem_pio2f,
};
