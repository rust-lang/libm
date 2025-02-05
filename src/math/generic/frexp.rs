use super::super::{CastFrom, Float, MinInt};

pub fn frexp<F: Float>(x: F) -> (F, i32) {
    let mut ix = x.to_bits();
    let ee = x.exp();

    if ee == 0 {
        if x != F::ZERO {
            // normalize via multiplication; 1p64 for `f64`
            let magic = F::from_parts(false, F::EXP_BIAS + F::BITS, F::Int::ZERO);
            magic.to_bits();

            let (x, e) = frexp(x * magic);
            return (x, e - F::BITS as i32);
        }
        return (x, 0);
    } else if ee == F::EXP_SAT {
        return (x, 0);
    }

    let e = ee as i32 - (F::EXP_BIAS as i32 - 1);
    ix &= F::SIGN_MASK | F::SIG_MASK;
    ix |= F::Int::cast_from(F::EXP_BIAS - 1) << F::SIG_BITS;
    (F::from_bits(ix), e)
}
