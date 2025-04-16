use super::super::{CastFrom, Float, Int, MinInt};

/// Given the bits of a positive float, clamp the exponent field to [0,1]
fn collapse_exponent<F: Float>(bits: F::Int) -> F::Int {
    let sig = bits & F::SIG_MASK;
    if sig == bits { sig } else { sig | F::IMPLICIT_BIT }
}

/// Computes (x << e) % y
fn reduction<I: Int>(mut x: I, e: u32, y: I) -> I {
    x %= y;
    for _ in 0..e {
        x <<= 1;
        x = x.checked_sub(y).unwrap_or(x);
    }
    x
}

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fmod<F: Float>(x: F, y: F) -> F {
    let _1 = F::Int::ONE;
    let mut ix = x.to_bits();
    let mut iy = y.to_bits();

    let sx = ix & F::SIGN_MASK;
    ix &= !F::SIGN_MASK;
    iy &= !F::SIGN_MASK;

    if ix >= F::EXP_MASK {
        // x is nan or inf
        return F::NAN;
    }

    if iy.wrapping_sub(_1) >= F::EXP_MASK {
        // y is nan or zero
        return F::NAN;
    }

    if ix < iy {
        // |x| < |y|
        return x;
    };

    let ex: u32 = x.ex().saturating_sub(1);
    let ey: u32 = y.ex().saturating_sub(1);

    let num = collapse_exponent::<F>(ix);
    let div = collapse_exponent::<F>(iy);

    let num = reduction(num, ex - ey, div);

    if num.is_zero() {
        F::from_bits(sx)
    } else {
        let ilog = num.ilog2();
        let shift = (ey + ilog).min(F::SIG_BITS) - ilog;
        let scale = (ey + ilog).saturating_sub(F::SIG_BITS);

        let normalized = num << shift;
        let scaled = normalized + (F::Int::cast_from(scale) << F::SIG_BITS);
        F::from_bits(sx | scaled)
    }
}
