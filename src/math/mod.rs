macro_rules! force_eval {
    ($e:expr) => {
        unsafe {
            ::core::ptr::read_volatile(&$e);
        }
    };
}

mod acosf;
mod asinf;
mod atanf;
mod ceilf;
mod cosf;
mod expf;
mod expm1f;
mod fabs;
mod fabsf;
mod floor;
mod floorf;
mod fmodf;
mod hypot;
mod hypotf;
mod log;
mod log10;
mod log10f;
mod log1p;
mod log1pf;
mod log2;
mod log2f;
mod logf;
mod powf;
mod round;
mod roundf;
mod scalbn;
mod scalbnf;
mod sinf;
mod sqrt;
mod sqrtf;
mod tanf;
mod trunc;
mod truncf;

pub use self::{
    acosf::acosf, asinf::asinf, atanf::atanf, ceilf::ceilf, cosf::cosf, expf::expf, expm1f::expm1f,
    fabs::fabs, fabsf::fabsf, floor::floor, floorf::floorf, fmodf::fmodf, hypot::hypot,
    hypotf::hypotf, log::log, log10::log10, log10f::log10f, log1p::log1p, log1pf::log1pf,
    log2::log2, log2f::log2f, logf::logf, powf::powf, round::round, roundf::roundf, scalbn::scalbn,
    scalbnf::scalbnf, sinf::sinf, sqrt::sqrt, sqrtf::sqrtf, tanf::tanf, trunc::trunc,
    truncf::truncf,
};

mod k_cosf;
mod k_sinf;
mod k_tanf;
mod rem_pio2_large;
mod rem_pio2f;

use self::{
    k_cosf::k_cosf, k_sinf::k_sinf, k_tanf::k_tanf, rem_pio2_large::rem_pio2_large,
    rem_pio2f::rem_pio2f,
};

fn isnanf(x: f32) -> bool {
    x.to_bits() & 0x7fffffff > 0x7f800000
}
