use core::f32;

const TOINT: f32 = 1. / f32::EPSILON;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn roundf(mut x: f32) -> f32 {
    let i = x.to_bits();
    let e: u32 = i >> 23 & 0xff;
    let mut y: f32;

    if e >= 0x7f + 23 {
        return x;
    }
    if i >> 31 != 0 {
        x = -x;
    }
    if e < 0x7f - 1 {
        force_eval!(x + TOINT);
        return 0. * x;
    }
    y = x + TOINT - TOINT - x;
    y += if y > 0.5 {
        x - 1.
    } else if y <= -0.5 {
        x + 1.
    } else {
        x
    };
    if i >> 31 != 0 {
        -y
    } else {
        y
    }
}
