use super::signbit::signbit;
use super::signbitf::signbitf;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn nexttowardf(x: f32, y: f64) -> f32 {
    let mut ux_i = x.to_bits();
    if x.is_nan() || y.is_nan() {
        return (x as f64 + y) as f32;
    }

    if x == y as f32 {
        return y as f32;
    }
    if x == 0.0 {
        ux_i = 1;
        if signbit(y) != 0 {
            ux_i |= 0x8000_0000_u32;
        }
    } else if x < (y as f32) {
        if signbitf(x) != 0 {
            ux_i -= 1;
        } else {
            ux_i += 1;
        }
    } else {
        if signbitf(x) != 0 {
            ux_i += 1;
        } else {
            ux_i -= 1;
        }
    }
    let e = ux_i.wrapping_shr(0x7f80_0000_u32);
    // raise overflow if ux.f is infinite and x is finite
    if e == 0x7f80_0000_u32 {
        force_eval!(x + x);
    }
    // raise underflow if ux.f is subnormal or zero
    let ux_f = f32::from_bits(ux_i);
    if e == 0 {
        force_eval!(x * x + ux_f * ux_f);
    }
    ux_f
}
