/* SPDX-License-Identifier: MIT */
/* origin: musl src/math/{fma,fmaf}.c. Ported to generic Rust algorithm in 2025, TG. */

use core::{f32, f64};

use super::super::fenv::{
    FE_INEXACT, FE_TONEAREST, FE_UNDERFLOW, feclearexcept, fegetround, feraiseexcept, fetestexcept,
};
use super::super::support::{DInt, HInt, IntTy};
use super::super::{CastFrom, CastInto, DFloat, Float, HFloat, Int, MinInt};

/// Fused multiply-add that works when there is not a larger float size available. Currently this
/// is still specialized only for `f64`. Computes `(x * y) + z`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fma<F>(x: F, y: F, z: F) -> F
where
    F: Float + FmaHelper,
    F: CastFrom<F::SignedInt>,
    F: CastFrom<i8>,
    F::Int: HInt,
    u32: CastInto<F::Int>,
{
    let one = IntTy::<F>::ONE;
    let zero = IntTy::<F>::ZERO;

    // Normalize such that the top of the mantissa is zero and we have a guard bit.
    let nx = Norm::from_float(x);
    let ny = Norm::from_float(y);
    let nz = Norm::from_float(z);

    if nx.is_zero_nan_inf() || ny.is_zero_nan_inf() {
        // Value will overflow, defer to non-fused operations.
        return x * y + z;
    }

    if nz.is_zero_nan_inf() {
        if nz.is_zero() {
            // Empty add component means we only need to multiply.
            return x * y;
        }
        // `z` is NaN or infinity, which sets the result.
        return z;
    }

    // multiply: r = x * y
    let zhi: F::Int;
    let zlo: F::Int;
    let (mut rlo, mut rhi) = nx.m.widen_mul(ny.m).lo_hi();

    // Exponent result of multiplication
    let mut e: i32 = nx.e + ny.e;
    // Needed shift to align `z` to the multiplication result
    let mut d: i32 = nz.e - e;
    let sbits = F::BITS as i32;

    // Scale `z`. Shift `z <<= kz`, `r >>= kr`, so `kz+kr == d`, set `e = e+kr` (== ez-kz)
    if d > 0 {
        // The magnitude of `z` is larger than `x * y`
        if d < sbits {
            // Maximum shift of one `F::BITS` means shifted `z` will fit into `2 * F::BITS`. Shift
            // it into `(zhi, zlo)`. No exponent adjustment necessary.
            zlo = nz.m << d;
            zhi = nz.m >> (sbits - d);
        } else {
            // Shift larger than `sbits`, `z` only needs the top half `zhi`. Place it there (acts
            // as a shift by `sbits`).
            zlo = zero;
            zhi = nz.m;
            d -= sbits;

            // `z`'s exponent is large enough that it now needs to be taken into account.
            e = nz.e - sbits;

            if d == 0 {
                // Exactly `sbits`, nothing to do
            } else if d < sbits {
                // Remaining shift fits within `sbits`. Leave `z` in place, shift `x * y`
                rlo = (rhi << (sbits - d)) | (rlo >> d);
                // Set the sticky bit
                rlo |= IntTy::<F>::from((rlo << (sbits - d)) != zero);
                rhi = rhi >> d;
            } else {
                // `z`'s magnitude is enough that `x * y` is irrelevant. It was nonzero, so set
                // the sticky bit.
                rlo = one;
                rhi = zero;
            }
        }
    } else {
        // `z`'s magnitude once shifted fits entirely within `zlo`
        zhi = zero;
        d = -d;
        if d == 0 {
            // No shift needed
            zlo = nz.m;
        } else if d < sbits {
            // Shift s.t. `nz.m` fits into `zlo`
            let sticky = IntTy::<F>::from((nz.m << (sbits - d)) != zero);
            zlo = (nz.m >> d) | sticky;
        } else {
            // Would be entirely shifted out, only set the sticky bit
            zlo = one;
        }
    }

    /* addition */

    let mut neg = nx.neg ^ ny.neg;
    let samesign: bool = !neg ^ nz.neg;
    let mut rhi_nonzero = true;

    if samesign {
        // r += z
        rlo = rlo.wrapping_add(zlo);
        rhi += zhi + IntTy::<F>::from(rlo < zlo);
    } else {
        // r -= z
        let (res, borrow) = rlo.overflowing_sub(zlo);
        rlo = res;
        rhi = rhi.wrapping_sub(zhi.wrapping_add(IntTy::<F>::from(borrow)));
        if (rhi >> (F::BITS - 1)) != zero {
            rlo = rlo.signed().wrapping_neg().unsigned();
            rhi = rhi.signed().wrapping_neg().unsigned() - IntTy::<F>::from(rlo != zero);
            neg = !neg;
        }
        rhi_nonzero = rhi != zero;
    }

    /* Construct result */

    // Shift result into `rhi`, left-aligned. Last bit is sticky
    if rhi_nonzero {
        // `d` > 0, need to shift both `rhi` and `rlo` into result
        e += sbits;
        d = rhi.leading_zeros() as i32 - 1;
        rhi = (rhi << d) | (rlo >> (sbits - d));
        // Update sticky
        rhi |= IntTy::<F>::from((rlo << d) != zero);
    } else if rlo != zero {
        // `rhi` is zero, `rlo` is the entire result and needs to be shifted
        d = rlo.leading_zeros() as i32 - 1;
        if d < 0 {
            // Shift and set sticky
            rhi = (rlo >> 1) | (rlo & one);
        } else {
            rhi = rlo << d;
        }
    } else {
        // exact +/- 0.0
        return x * y + z;
    }

    e -= d;

    // Use int->float conversion to populate the significand.
    // i is in [1 << (BITS - 2), (1 << (BITS - 1)) - 1]
    let mut i: F::SignedInt = rhi.signed();

    if neg {
        i = -i;
    }

    // `|r|` is in `[0x1p62,0x1p63]` for `f64`
    let mut r: F = F::cast_from_lossy(i);

    /* Account for subnormal and rounding */

    // Unbiased exponent for the maximum value of `r`
    let max_pow = F::BITS - 1 + F::EXP_BIAS;

    if e < -(max_pow as i32 - 2) {
        // Result is subnormal before rounding
        if e == -(max_pow as i32 - 1) {
            let mut c = F::from_parts(false, max_pow, zero);
            if neg {
                c = -c;
            }

            if r == c {
                // Min normal after rounding,
                return r.raise_underflow_as_min_positive();
            }

            if (rhi << (F::SIG_BITS + 1)) != zero {
                // Account for truncated bits. One bit will be lost in the `scalbn` call, add
                // another top bit to avoid double rounding if inexact.
                let iu: F::Int = (rhi >> 1) | (rhi & one) | (one << (F::BITS - 2));
                i = iu.signed();

                if neg {
                    i = -i;
                }

                r = F::cast_from_lossy(i);

                // Remove the top bit
                r = F::cast_from(2i8) * r - c;
                r += r.raise_underflow_ret_zero();
            }
        } else {
            // Only round once when scaled
            d = F::EXP_BITS as i32 - 1;
            let sticky = IntTy::<F>::from(rhi << (F::BITS as i32 - d) != zero);
            i = (((rhi >> d) | sticky) << d).signed();

            if neg {
                i = -i;
            }

            r = F::cast_from_lossy(i);
        }
    }

    // Use our exponent to scale the final value.
    super::scalbn(r, e)
}

/// Fma implementation when a hardware-backed larger float type is available. For `f32` and `f64`,
/// `f64` has enough precision to represent the `f32` in its entirety, except for double rounding.
pub fn fma_wide<F, B>(x: F, y: F, z: F) -> F
where
    F: Float + HFloat<D = B>,
    B: Float + DFloat<H = F>,
    B::Int: CastInto<i32>,
    i32: CastFrom<i32>,
{
    let one = IntTy::<B>::ONE;

    let xy: B = x.widen() * y.widen();
    let mut result: B = xy + z.widen();
    let mut ui: B::Int = result.to_bits();
    let re = result.exp();
    let zb: B = z.widen();

    let prec_diff = B::SIG_BITS - F::SIG_BITS;
    let excess_prec = ui & ((one << prec_diff) - one);
    let halfway = one << (prec_diff - 1);

    // Common case: the larger precision is fine if...
    // This is not a halfway case
    if excess_prec != halfway
        // Or the result is NaN
        || re == B::EXP_SAT
        // Or the result is exact
        || (result - xy == zb && result - zb == xy)
        // Or the mode is something other than round to nearest
        || fegetround() != FE_TONEAREST
    {
        let min_inexact_exp = (B::EXP_BIAS as i32 + F::EXP_MIN_SUBNORM) as u32;
        let max_inexact_exp = (B::EXP_BIAS as i32 + F::EXP_MIN) as u32;

        if (min_inexact_exp..max_inexact_exp).contains(&re) && fetestexcept(FE_INEXACT) != 0 {
            feclearexcept(FE_INEXACT);
            // prevent `xy + vz` from being CSE'd with `xy + z` above
            let vz: F = force_eval!(z);
            result = xy + vz.widen();
            if fetestexcept(FE_INEXACT) != 0 {
                feraiseexcept(FE_UNDERFLOW);
            } else {
                feraiseexcept(FE_INEXACT);
            }
        }

        return result.narrow();
    }

    let neg = ui >> (B::BITS - 1) != IntTy::<B>::ZERO;
    let err = if neg == (zb > xy) { xy - result + zb } else { zb - result + xy };
    if neg == (err < B::ZERO) {
        ui += one;
    } else {
        ui -= one;
    }

    B::from_bits(ui).narrow()
}

/// Representation of `F` that has handled subnormals.
#[derive(Clone, Copy, Debug)]
struct Norm<F: Float> {
    /// Normalized significand with one guard bit, unsigned.
    m: F::Int,
    /// Exponent of the mantissa such that `m * 2^e = x`. Accounts for the shift in the mantissa
    /// and the guard bit; that is, 1.0 will normalize as `m = 1 << 53` and `e = -53`.
    e: i32,
    neg: bool,
}

impl<F: Float> Norm<F> {
    /// Unbias the exponent and account for the mantissa's precision, including the guard bit.
    const EXP_UNBIAS: u32 = F::EXP_BIAS + F::SIG_BITS + 1;

    /// Values greater than this had a saturated exponent (infinity or NaN), OR were zero and we
    /// adjusted the exponent such that it exceeds this threashold.
    const ZERO_INF_NAN: u32 = F::EXP_SAT - Self::EXP_UNBIAS;

    fn from_float(x: F) -> Self {
        let mut ix = x.to_bits();
        let mut e = x.exp() as i32;
        let neg = x.is_sign_negative();
        if e == 0 {
            // Normalize subnormals by multiplication
            let scale_i = F::BITS - 1;
            let scale_f = F::from_parts(false, scale_i + F::EXP_BIAS, F::Int::ZERO);
            let scaled = x * scale_f;
            ix = scaled.to_bits();
            e = scaled.exp() as i32;
            e = if e == 0 {
                // If the exponent is still zero, the input was zero. Artifically set this value
                // such that the final `e` will exceed `ZERO_INF_NAN`.
                1 << F::EXP_BITS
            } else {
                // Otherwise, account for the scaling we just did.
                e - scale_i as i32
            };
        }

        e -= Self::EXP_UNBIAS as i32;

        // Absolute  value, set the implicit bit, and shift to create a guard bit
        ix &= F::SIG_MASK;
        ix |= F::IMPLICIT_BIT;
        ix <<= 1;

        Self { m: ix, e, neg }
    }

    /// True if the value was zero, infinity, or NaN.
    fn is_zero_nan_inf(self) -> bool {
        self.e >= Self::ZERO_INF_NAN as i32
    }

    /// The only value we have
    fn is_zero(self) -> bool {
        // The only exponent that strictly exceeds this value is our sentinel value for zero.
        self.e > Self::ZERO_INF_NAN as i32
    }
}

/// Type-specific helpers that are not needed outside of fma.
pub trait FmaHelper {
    /// Raise underflow and return the minimum positive normal value with the sign of `self`.
    fn raise_underflow_as_min_positive(self) -> Self;
    /// Raise underflow and return zero.
    fn raise_underflow_ret_zero(self) -> Self;
}

impl FmaHelper for f64 {
    fn raise_underflow_as_min_positive(self) -> Self {
        /* min normal after rounding, underflow depends
         * on arch behaviour which can be imitated by
         * a double to float conversion */
        let fltmin: f32 = (hf64!("0x0.ffffff8p-63") * f32::MIN_POSITIVE as f64 * self) as f32;
        f64::MIN_POSITIVE / f32::MIN_POSITIVE as f64 * fltmin as f64
    }

    fn raise_underflow_ret_zero(self) -> Self {
        /* raise underflow portably, such that it
         * cannot be optimized away */
        let tiny: f64 = f64::MIN_POSITIVE / f32::MIN_POSITIVE as f64 * self;
        (tiny * tiny) * (self - self)
    }
}

#[cfg(f128_enabled)]
impl FmaHelper for f128 {
    fn raise_underflow_as_min_positive(self) -> Self {
        f128::MIN_POSITIVE.copysign(self)
    }

    fn raise_underflow_ret_zero(self) -> Self {
        f128::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec_test<F>()
    where
        F: Float + FmaHelper,
        F: CastFrom<F::SignedInt>,
        F: CastFrom<i8>,
        F::Int: HInt,
        u32: CastInto<F::Int>,
    {
        let x = F::from_bits(F::Int::ONE);
        let y = F::from_bits(F::Int::ONE);
        let z = F::ZERO;

        // 754-2020 says "When the exact result of (a × b) + c is non-zero yet the result of
        // fusedMultiplyAdd is zero because of rounding, the zero result takes the sign of the
        // exact result"
        assert_biteq!(fma(x, y, z), F::ZERO);
        assert_biteq!(fma(x, -y, z), F::NEG_ZERO);
        assert_biteq!(fma(-x, y, z), F::NEG_ZERO);
        assert_biteq!(fma(-x, -y, z), F::ZERO);
    }

    #[test]
    fn spec_test_f64() {
        spec_test::<f64>();
    }

    #[test]
    #[cfg(f128_enabled)]
    fn spec_test_f128() {
        spec_test::<f128>();
    }
}
