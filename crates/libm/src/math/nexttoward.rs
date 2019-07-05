use super::signbit::signbit;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn nexttoward(x: f64, y: /*long double*/ f64) -> f64 {
    let mut ux_i = x.to_bits();
    if x.is_nan() || y.is_nan() {
        return x + y;
    }

    if x == y {
        return y;
    }
    if x == 0.0 {
        ux_i = 1;
        if signbit(y) != 0 {
            ux_i |= 1_u64 << 63;
        }
    } else if x < y {
        if signbit(x) != 0 {
            ux_i -= 1;
        } else {
            ux_i += 1;
        }
    } else {
        if signbit(x) != 0 {
            ux_i += 1;
        } else {
            ux_i -= 1;
        }
    }
    let e = ux_i >> 52 & 0x7ff;
    // raise overflow if ux.f is infinite and x is finite
    if e == 0x7ff {
        force_eval!(x + x);
    }
    // raise underflow if ux.f is subnormal or zero
    let ux_f = f64::from_bits(ux_i);
    if e == 0 {
        force_eval!(x * x + ux_f * ux_f);
    }
    ux_f
}
