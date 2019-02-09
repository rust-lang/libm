// Public modules
mod asinf;
mod cbrtf;
mod cosf;
mod floorf;
mod hypotf;
mod sinf;
mod tanf;

#[rustfmt::skip]
pub use self::{
    asinf::asinf,
    cbrtf::cbrtf,
    cosf::cosf,
    floorf::floorf,
    hypotf::hypotf,
    sinf::sinf,
    tanf::tanf,
};

// Private modules
mod k_cosf;
mod k_rem_pio2f;
mod k_sinf;
mod k_tanf;
mod rem_pio2f;

// Private re-imports
#[rustfmt::skip]
use self::{
    k_cosf::k_cosf,
    k_rem_pio2f::k_rem_pio2f,
    k_rem_pio2f::Precision,
    k_sinf::k_sinf,
    k_tanf::k_tanf,
    rem_pio2f::rem_pio2f,
};

use math::powf;
#[inline]
pub fn exp2f(x: f32) -> f32 {
    powf(2.0, x)
}

pub use math::musl::acosf; //temporary
