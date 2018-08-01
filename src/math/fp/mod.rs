mod consts {
    pub const Z_ROOTEPS_F: f32 = 1.7263349182589107e-4;

    pub const Z_HUGEVAL_F: u32 = 0x7f800000;
    pub const Z_INFINITY_F: u32 = 0x7f800000;
    pub const Z_NOTANUM_F: u32 = 0xffd00000;

    pub const PI: f32 = 3.14159265358979323846;
    pub const SQRT_HALF: f32 = 0.70710678118654752440;
    pub const PI_OVER_TWO: f32 = 1.57079632679489661923132;
}
#[derive(Clone, Copy, PartialEq)]
pub enum NumState {
    Zero = 0,
    Inf = 1,
    Nan = 2,
    Num = 3,
}

mod asinef;
mod numtestf;
mod sinef;

use self::asinef::asinef;
use self::numtestf::numtestf;
use self::sinef::sinef;

#[inline]
pub fn asinf(x: f32) -> f32 {
    asinef(x, false)
}

#[inline]
pub fn acosf(x: f32) -> f32 {
    asinef(x, true)
}

#[inline]
pub fn sinf(x: f32) -> f32 {
    sinef(x, false)
}

#[inline]
pub fn cosf(x: f32) -> f32 {
    sinef(x, true)
}
