#![allow(unused)]

use core::ops::{Shl, Shr};

use super::super::fenv::{
    FE_INEXACT, FE_TONEAREST, FE_UNDERFLOW, feclearexcept, fegetround, feraiseexcept, fetestexcept,
};
use super::super::support::{DInt, HInt, Int};
use super::super::{CastFrom, CastInto, Float, IntTy, MinInt};

const ZEROINFNAN: i32 = 0x7ff - 0x3ff - 52 - 1;

/// Fused multiply add.
pub fn fma<F: Float>(x: F, y: F, z: F, scbn: impl FnOnce(F, i32) -> F) -> F
where
    F::Int: CastFrom<u32>,
    F::Int: HInt,
    F::Int: Shr<i32, Output = F::Int>,
    F::Int: Shl<i32, Output = F::Int>,
    F::SignedInt: CastInto<F>,
    u32: CastInto<F::Int>,
    bool: CastInto<F::Int>,
{
    let one = F::Int::ONE;
    let zero = F::Int::ZERO;

    let nx = Norm::from_float(x);
    let ny = Norm::from_float(y);
    let nz = Norm::from_float(z);

    if nx.e >= ZEROINFNAN || ny.e >= ZEROINFNAN {
        return x * y + z;
    }
    if nz.e >= ZEROINFNAN {
        if nz.e > ZEROINFNAN {
            /* z==0 */
            return x * y + z;
        }
        return z;
    }

    let zhi: F::Int;
    let zlo: F::Int;

    let (mut rlo, mut rhi) = nx.m.widen_mul(ny.m).lo_hi();

    let mut e: i32 = nx.e + ny.e;
    let mut d: i32 = nz.e - e;

    let fbits = F::BITS as i32;

    if d > 0 {
        if d < fbits {
            zlo = nz.m << d;
            zhi = nz.m >> (fbits - d);
        } else {
            zlo = zero;
            zhi = nz.m;
            e = nz.e - fbits;
            d -= fbits;
            if d == 0 {
            } else if d < fbits {
                rlo = (rhi << (fbits - d)) | (rlo >> d) | ((rlo << (fbits - d)) != zero).cast();
                rhi = rhi >> d;
            } else {
                rlo = one;
                rhi = zero;
            }
        }
    } else {
        zhi = zero;
        d = -d;
        if d == 0 {
            zlo = nz.m;
        } else if d < fbits {
            zlo = (nz.m >> d) | ((nz.m << (fbits - d)) != zero).cast();
        } else {
            zlo = one;
        }
    }

    /* add */
    let mut neg: bool = nx.neg ^ ny.neg;
    let samesign: bool = neg ^ nz.neg;
    let mut nonzero: i32 = 1;
    if samesign {
        /* r += z */
        rlo = rlo.wrapping_add(zlo);
        rhi += zhi + (rlo < zlo).cast();
    } else {
        /* r -= z */
        let (res, borrow) = rlo.overflowing_sub(zlo);
        rlo = res;
        rhi = rhi.wrapping_sub(zhi.wrapping_add(borrow.cast()));
        if (rhi >> (F::BITS - 1)) != zero {
            rlo = (rlo.signed()).wrapping_neg().unsigned();
            rhi = (rhi.signed()).wrapping_neg().unsigned() - (rlo != zero).cast();
            neg = !neg;
        }
        nonzero = (rhi != zero) as i32;
    }

    /* set rhi to top 63bit of the result (last bit is sticky) */
    if nonzero != 0 {
        e += fbits;
        d = rhi.leading_zeros() as i32 - 1;
        /* note: d > 0 */
        rhi = (rhi << d) | (rlo >> (fbits - d)) | ((rlo << d) != zero).cast();
    } else if rlo != zero {
        d = rlo.leading_zeros() as i32 - 1;
        if d < 0 {
            rhi = (rlo >> 1) | (rlo & one);
        } else {
            rhi = rlo << d;
        }
    } else {
        /* exact +-0 */
        return x * y + z;
    }
    e -= d;

    /* convert to double */
    let mut i: F::SignedInt = rhi.signed(); /* i is in [1<<62,(1<<63)-1] */
    if neg {
        i = -i;
    }
    let mut r: F = i.cast(); /* |r| is in [0x1p62,0x1p63] */

    if e < -1022 - 62 {
        /* result is subnormal before rounding */
        if e == -1022 - 63 {
            let mut c: F = foo::<F>();
            if neg {
                c = -c;
            }
            if r == c {
                /* min normal after rounding, underflow depends
                on arch behaviour which can be imitated by
                a double to float conversion */
                // let fltmin: f32 = (x0_ffffff8p_63 * f32::MIN_POSITIVE as f64 * r) as f32;
                // return f64::MIN_POSITIVE / f32::MIN_POSITIVE as f64 * fltmin as f64;
                todo!()
            }
            /* one bit is lost when scaled, add another top bit to
            only round once at conversion if it is inexact */
            if (rhi << (F::SIG_BITS + 1)) != zero {
                let tmp: F::Int = (rhi >> 1) | (rhi & one) | (one << (F::BITS - 2));
                i = tmp.signed();
                if neg {
                    i = -i;
                }
                r = i.cast();
                r = (F::ONE + F::ONE) * r - c; /* remove top bit */

                /* raise underflow portably, such that it
                cannot be optimized away */
                {
                    // let tiny: f64 = f64::MIN_POSITIVE / f32::MIN_POSITIVE as f64 * r;
                    // r += (tiny * tiny) * (r - r);
                    todo!()
                }
            }
        } else {
            /* only round once when scaled */
            d = 10;
            i = (((rhi >> d) | ((rhi << (fbits - d)) != zero).cast()) << d).signed();
            if neg {
                i = -i;
            }
            r = i.cast();
        }
    }

    // todo!()
    //
    scbn(r, e)
}

struct Norm<F: Float> {
    m: F::Int,
    e: i32,
    neg: bool,
}

impl<F: Float> Norm<F> {
    fn from_float(x: F) -> Self
    where
        F::Int: CastFrom<u32>,
        u32: CastInto<F::Int>,
    {
        let mut ix = x.to_bits();
        let mut e = x.exp();
        let neg = x.is_sign_negative();
        if e.is_zero() {
            ix = (x * foo::<F>()).to_bits();
            e = x.exp();
            e = if e != 0 { e - (F::BITS as i32) } else { 0x800 };
        }
        ix &= F::SIG_MASK;
        ix |= F::IMPLICIT_BIT;
        ix <<= 1;
        e -= 0x3ff + 52 + 1;

        Self { m: ix, e, neg }
    }
}

// 1p63 magic number
fn foo<F: Float>() -> F
where
    u32: CastInto<F::Int>,
{
    F::from_parts(false, (F::BITS - 1).cast(), F::Int::ZERO)
}

/// FMA implementation when there is a larger float type available.
pub fn fma_big<F, B>(x: F, y: F, z: F) -> F
where
    F: Float + CastInto<B>,
    B: Float + CastInto<F> + CastFrom<F>,
    B::Int: CastInto<i32>,
    i32: CastFrom<i32>,
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
