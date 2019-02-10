mod consts {
    pub const Z_ROOTEPS_F: f32 = 1.726_334_918_258_910_7_e-4;

    pub const Z_NOTANUM_F: u32 = 0x_ffd0_0000;

    pub const PI: f32 = 3.141_592_653_589_793_238_46;
    //pub const SQRT_HALF: f32 = 0.707_106_781_186_547_524_40;
    //pub const PI_OVER_TWO: f32 = 1.570_796_326_794_896_619_231_32;
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
