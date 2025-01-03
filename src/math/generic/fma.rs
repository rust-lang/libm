#![allow(unused)]

use super::super::fenv::{
    FE_INEXACT, FE_TONEAREST, FE_UNDERFLOW, feclearexcept, fegetround, feraiseexcept, fetestexcept,
};
use super::super::{CastFrom, CastInto, Float, IntTy, MinInt};

/// Fused multiply add.
pub fn fma<F: Float>(x: F, y: F, z: F) -> F {
    todo!()
}

pub fn fma_big<F: Float, B: Float>(x: F, y: F, z: F) -> F
where
    F: CastInto<B>,
    B: CastInto<F>,
    B::Int: CastInto<i32>,
    i32: CastFrom<i32>,
    B: CastFrom<F>,
{
    let one = IntTy::<B>::ONE;

    let xy: B;
    let mut result: B;
    let mut ui: B::Int;
    let e: i32;

    xy = x.cast() * y.cast();
    result = xy + z.cast();
    ui = result.to_bits();
    e = i32::cast_from(ui >> F::SIG_BITS) & F::EXP_MAX as i32;
    let zb = B::cast_from(z);

    let prec_diff = B::SIG_BITS - F::SIG_BITS;
    let excess_prec = ui & ((one << prec_diff) - one);
    let x = one << (prec_diff - 1);

    // Common case: the larger precision is fine
    if excess_prec != x
        || e == i32::cast_from(F::EXP_MAX)
        || (result - xy == zb && result - zb == xy)
        || fegetround() != FE_TONEAREST
    {
        // TODO: feclearexcept

        return result.cast();
    }

    let neg = ui & B::SIGN_MASK > IntTy::<B>::ZERO;
    let err = if neg == (zb > xy) { xy - result + zb } else { zb - result + xy };
    if neg == (err < B::ZERO) {
        ui += one;
    } else {
        ui -= one;
    }

    B::from_bits(ui).cast()
}
