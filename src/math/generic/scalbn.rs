use super::super::{CastFrom, CastInto, Float, IntTy, MinInt};

pub fn scalbn<F: Float>(mut x: F, mut n: i32) -> F
where
    u32: CastInto<F::Int>,
    F::Int: CastFrom<i32>,
    F::Int: CastFrom<u32>,
{
    // Bits including the implicit bit
    let sig_total_bits = F::SIG_BITS + 1;

    // Maximum and minimum values when biased
    let exp_max: i32 = F::EXP_BIAS as i32;
    let exp_min = -(exp_max - 1);

    // 2 ^ Emax, where Emax is the maximum biased exponent value (1023 for f64)
    let f_exp_max = F::from_bits(F::Int::cast_from(F::EXP_BIAS << 1) << F::SIG_BITS);
    // 2 ^ Emin, where Emin is the minimum biased exponent value (-1022 for f64)
    let f_exp_min = F::from_bits(IntTy::<F>::ONE << F::SIG_BITS);
    // 2 ^ sig_total_bits, representation of what can be accounted for with subnormals
    let f_exp_subnorm = F::from_bits((F::EXP_BIAS + sig_total_bits).cast() << F::SIG_BITS);

    if n > exp_max {
        x *= f_exp_max;
        n -= exp_max;
        if n > exp_max {
            x *= f_exp_max;
            n -= exp_max;
            if n > exp_max {
                n = exp_max;
            }
        }
    } else if n < exp_min {
        let mul = f_exp_min * f_exp_subnorm;
        let add = (exp_max - 1) - sig_total_bits as i32;

        x *= mul;
        n += add;
        if n < exp_min {
            x *= mul;
            n += add;
            if n < exp_min {
                n = exp_min;
            }
        }
    }

    x * F::from_bits(F::Int::cast_from(F::EXP_BIAS as i32 + n) << F::SIG_BITS)
}
