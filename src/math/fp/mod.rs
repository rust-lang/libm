mod consts {
    pub const Z_ROOTEPS_F: f32 = 1.7263349182589107e-4;

    pub const Z_HUGEVAL_F: u32 = 0x7f800000;
    pub const Z_INFINITY_F: u32 = 0x7f800000;
    pub const Z_NOTANUM_F: u32 = 0xffd00000;
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

pub use self::asinef::asinef;
use self::numtestf::numtestf;
