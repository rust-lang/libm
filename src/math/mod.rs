macro_rules! force_eval {
    ($e:expr) => {
        unsafe {
            ::core::ptr::read_volatile(&$e);
        }
    };
}

mod asin;
mod ceilf;
mod expf;
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
mod sqrt;
mod sqrtf;
mod trunc;
mod truncf;

//mod service;

pub use self::{
    asin::asin, ceilf::ceilf, expf::expf, fabs::fabs, fabsf::fabsf, floor::floor, floorf::floorf,
    fmodf::fmodf, hypot::hypot, hypotf::hypotf, log::log, log10::log10, log10f::log10f,
    log1p::log1p, log1pf::log1pf, log2::log2, log2f::log2f, logf::logf, powf::powf, round::round,
    roundf::roundf, scalbn::scalbn, scalbnf::scalbnf, sqrt::sqrt, sqrtf::sqrtf, trunc::trunc,
    truncf::truncf,
};

#[inline]
pub fn get_high_word(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

#[inline]
pub fn get_low_word(x: f64) -> u32 {
    x.to_bits() as u32
}

#[inline]
pub fn with_set_high_word(f: f64, hi: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0x00000000_ffffffff;
    tmp |= (hi as u64) << 32;
    f64::from_bits(tmp)
}

#[inline]
pub fn with_set_low_word(f: f64, lo: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0xffffffff_00000000;
    tmp |= lo as u64;
    f64::from_bits(tmp)
}

fn isnanf(x: f32) -> bool {
    x.to_bits() & 0x7fffffff > 0x7f800000
}
