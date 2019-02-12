// Public modules
pub mod fp;

pub mod acosf;
pub mod asinf;
pub mod cbrtf;
pub mod cosf;
pub mod floorf;
pub mod hypotf;
pub mod sinf;
pub mod tanf;

#[rustfmt::skip]
#[cfg(feature = "newlib")]
pub use self::{
    acosf::acosf,
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

use crate::math::powf;
#[inline]
pub fn exp2f(x: f32) -> f32 {
    powf(2., x)
}

//pub use crate::math::musl::acosf; //temporary
