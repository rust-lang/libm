use super::super::{CastFrom, CastInto, Float, IntTy, MinInt};

/// Scale the exponent.
///
/// From N3220:
///
/// > The scalbn and scalbln functions compute `x * b^n`, where `b = FLT_RADIX` if the return type
/// > of the function is a standard floating type, or `b = 10` if the return type of the function
/// > is a decimal floating type. A range error occurs for some finite x, depending on n.
/// >
/// > [...]
/// >
/// > * `scalbn(±0, n)` returns `±0`.
/// > * `scalbn(x, 0)` returns `x`.
/// > * `scalbn(±∞, n)` returns `±∞`.
/// >
/// > If the calculation does not overflow or underflow, the returned value is exact and
/// > independent of the current rounding direction mode.
pub fn scalbn<F: Float>(mut x: F, mut n: i32) -> F
where
    u32: CastInto<F::Int>,
    F::Int: CastFrom<i32>,
    F::Int: CastFrom<u32>,
{
    if n == 0 || x == F::ZERO || x.is_nan() || x.is_infinite() {
        return x;
    }

    // Bits including the implicit bit
    let sig_total_bits = F::SIG_BITS + 1;

    // Maximum and minimum values when biased
    let exp_max: i32 = F::EXP_BIAS as i32;
    let exp_min = -(exp_max - 1);
    let exp_min_with_subnorm = -((F::EXP_BIAS + F::SIG_BITS + 1) as i32);

    // let x_exp = x.exp();
    // let x_sig = x.frac();

    if n > exp_max {
        return F::INFINITY * x.signum();
    }

    if n < exp_min_with_subnorm {
        return F::ZERO * x.signum();
    }

    // 2 ^ Emax, where Emax is the maximum biased exponent value (1023 for f64)
    let f_exp_max = F::from_bits(F::Int::cast_from(F::EXP_BIAS << 1) << F::SIG_BITS);
    // 2 ^ Emin, where Emin is the minimum biased exponent value (-1022 for f64)
    let f_exp_min = F::from_bits(IntTy::<F>::ONE << F::SIG_BITS);
    // 2 ^ sig_total_bits, representation of what can be accounted for with subnormals
    let f_exp_subnorm = F::from_bits((F::EXP_BIAS + sig_total_bits).cast() << F::SIG_BITS);

    // std::println!("{exp_max} {exp_min} {n}");
    // std::dbg!(x, exp_max, exp_min, n);

    if n > exp_max {
        x *= f_exp_max;
        n -= exp_max;
        // std::dbg!(11, x, n);
        if n > exp_max {
            x *= f_exp_max;
            n -= exp_max;
            // std::dbg!(12, x, n);
            if n > exp_max {
                n = exp_max;
                // std::dbg!(13, x, n);
            }
        }
    } else if n < exp_min {
        let mul = f_exp_min * f_exp_subnorm;
        let add = (exp_max - 1) - sig_total_bits as i32;

        x *= mul;
        n += add;
        // std::dbg!(21, x, n);
        if n < exp_min {
            x *= mul;
            n += add;
            // std::dbg!(22, x, n);
            if n < exp_min {
                n = exp_min;
                // std::dbg!(23, x, n);
            }
        }
    }

    x * F::from_bits(F::Int::cast_from(F::EXP_BIAS as i32 + n) << F::SIG_BITS)
}

// DELETE

extern crate std;

#[test]
fn testme() {
    assert_eq!(scalbn::<f16>(f16::from_bits(0x6ecb), -1336428830), f16::ZERO);
}

#[test]
fn testme2() {
    // assert_eq!(scalbn(-f64::INFINITY, -2147033648), f64::ZERO);
}
