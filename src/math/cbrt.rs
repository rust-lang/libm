/* SPDX-License-Identifier: MIT */
/* origin: core-math/src/binary64/cbrt/cbrt.c
 * Copyright (c) 2021-2022 Alexei Sibidanov.
 * Ported to Rust in 2025 by Trevor Gross.
 */

use super::Float;
use super::support::{CastFrom, FpResult, Int, MinInt, Round, cold_path};

/// Compute the cube root of the argument.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn cbrt(x: f64) -> f64 {
    cbrt_round(x, Round::Nearest).val
}

// /// Compute the cube root of the argument.
// #[cfg(f128_enabled)]
// #[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
// pub fn cbrtf128(x: f128) -> f128 {
//     cbrt_round(x, Round::Nearest).val
// }

/// Correctly rounded cube root.
///
/// Algorithm:
/// - Minimax initial approximation
/// - `F`-sized newton iteration
/// - `2xF`-sized newton iteration
pub fn cbrt_round<F: Float + CbrtHelper>(x: F, round: Round) -> FpResult<F>
where
    F::Int: CastFrom<u64>,
    F::Int: CastFrom<u32>,
    F::Int: From<u8>,
{
    let zero = F::Int::ZERO;
    let one = F::Int::ONE;
    let u0: F = F::U0;
    let u1: F = F::U1;
    let off = F::OFF;

    let hx = x.to_bits();
    let mut mant: F::Int = hx & F::SIG_MASK;
    let sign: F::Int = hx & F::SIGN_MASK;
    let neg = x.is_sign_negative();

    let mut e: u32 = x.exp();

    // Handle 0, infinity, NaN, and subnormals
    if ((e + 1) & F::EXP_SAT) < 2 {
        cold_path();

        let ix = hx & !F::SIGN_MASK;

        if e == F::EXP_SAT || ix == zero {
            // 0, infinity, NaN; use x + x to trigger exceptions
            return FpResult::ok(x + x);
        }

        // Normalize subnormals
        let nz = ix.leading_zeros() - F::EXP_BITS;
        mant <<= nz;
        mant &= F::SIG_MASK;
        e = e.wrapping_sub(nz - 1);
    }

    e = e.wrapping_add(3072);
    // Set the exponent to 0, z is now [1, 2)
    let iz = mant | (F::Int::cast_from(F::EXP_BIAS) << F::SIG_BITS);

    let et: u32 = e / 3;
    let it: u32 = e % 3;

    // 2^(3k+it) <= x < 2^(3k+it+1), with 0 <= it <= 3
    // `zz` is `x` reduced to [1, 8)
    let izz = (iz + (F::Int::cast_from(it) << F::SIG_BITS)) | sign;
    let zz: F = F::from_bits(izz);

    /* cbrt(x) = cbrt(zz)*2^(et-1365) where 1 <= zz < 8 */
    let isc = F::ESCALE[it as usize].to_bits() | sign;
    let z: F = F::from_bits(iz);

    /* cbrt(zz) = cbrt(z)*isc, where isc encodes 1, 2^(1/3) or 2^(2/3),
    and 1 <= z < 2 */
    let r: F = F::ONE / z;
    let rr: F = r * F::RSCALE[((it as usize) << 1) | neg as usize];
    let z2: F = z * z;
    let c0: F = F::C[0] + z * F::C[1];
    let c2: F = F::C[2] + z * F::C[3];

    /* y is an approximation of z^(1/3) */
    let mut y: F = c0 + z2 * c2;
    let mut y2: F = y * y;

    /* h determines the error between y and z^(1/3) */
    let mut h: F = y2 * (y * r) - F::ONE;

    /* The correction y -= (h*y)*(u0 - u1*h) corresponds to a cubic variant
    of Newton's method, with the function f(y) = 1-z/y^3. */
    y -= (h * y) * (u0 - u1 * h);

    y *= F::from_bits(isc);

    /* Now y is an approximation of zz^(1/3),
     * and rr an approximation of 1/zz. We now perform another iteration of
     * Newton-Raphson, this time with a linear approximation only. */
    y2 = y * y;
    let mut y2l: F = y.fma(y, -y2);

    /* y2 + y2l = y^2 exactly */
    let mut y3: F = y2 * y;
    let mut y3l: F = y.fma(y2, -y3) + y * y2l;

    /* y3 + y3l approximates y^3 with about 106 bits of accuracy */
    h = ((y3 - zz) + y3l) * rr;
    let mut dy: F = h * (y * u0);

    /* the approximation of zz^(1/3) is y - dy */
    let mut y1: F = y - dy;
    dy = (y - y1) - dy;

    /* the approximation of zz^(1/3) is now y1 + dy, where |dy| < 1/2 ulp(y)
     * (for rounding to nearest) */
    let mut ady: F = dy.abs();

    /* For directed roundings, ady0 is tiny when dy is tiny, or ady0 is near
     * from ulp(1);
     * for rounding to nearest, ady0 is tiny when dy is near from 1/2 ulp(1),
     * or from 3/2 ulp(1). */
    let mut ady0: F = (ady - off[round as usize]).abs();
    let mut ady1: F = (ady - (F::TWO_POW_NEG_SIG_BITS + off[round as usize])).abs();

    let magic = F::from_parts(false, (-75 + F::EXP_BIAS as i32) as u32, zero);

    if ady0 < magic || ady1 < magic {
        cold_path();

        y2 = y1 * y1;
        y2l = y1.fma(y1, -y2);
        y3 = y2 * y1;
        y3l = y1.fma(y2, -y3) + y1 * y2l;
        h = ((y3 - zz) + y3l) * rr;
        dy = h * (y1 * u0);
        y = y1 - dy;
        dy = (y1 - y) - dy;
        y1 = y;
        ady = dy.abs();
        ady0 = (ady - off[round as usize]).abs();
        ady1 = (ady - (F::TWO_POW_NEG_SIG_BITS + off[round as usize])).abs();

        let magic2 = F::from_parts(false, (-98 + F::EXP_BIAS as i32) as u32, zero);
        if ady0 < magic2 || ady1 < magic2 {
            cold_path();
            let azz: F = zz.abs();

            // ~ 0x1.79d15d0e8d59b80000000000000ffc3dp+0
            if azz == F::AZMAGIC1 {
                y1 = F::AZMAGIC2.copysign(zz);
            }

            // ~ 0x1.de87aa837820e80000000000001c0f08p+0
            if azz == F::AZMAGIC3 {
                y1 = F::AZMAGIC4.copysign(zz);
            }

            if round != Round::Nearest {
                for (a, b) in F::WLIST {
                    if azz == a {
                        let tmp = if F::Int::from(round as u8 + neg as u8) == F::Int::cast_from(2) {
                            F::TWO_POW_NEG_SIG_BITS
                        } else {
                            F::ZERO
                        };
                        y1 = (b + tmp).copysign(zz);
                    }
                }
            }
        }
    }

    let mut cvt3 = y1.to_bits();
    cvt3 = cvt3.wrapping_add((F::Int::cast_from(et.wrapping_sub(342).wrapping_sub(1023))) << 52);
    let m0 = cvt3 << 30;
    let m1 = m0 >> 63;

    if (m0 ^ m1) <= (one << 30) {
        cold_path();

        let mut cvt4 = y1.to_bits();
        cvt4 = (cvt4 + (F::Int::cast_from(164) << 15)) & F::Int::cast_from(0xffffffffffff0000u64);

        let magic3 = F::from_parts(false, (-60 + F::EXP_BIAS as i32) as u32, zero);
        if ((F::from_bits(cvt4) - y1) - dy).abs() < magic3 || (zz).abs() == F::ONE {
            cvt3 = (cvt3 + (one << 15)) & F::Int::cast_from(0xffffffffffff0000u64);
        }
    }

    FpResult::ok(F::from_bits(cvt3))
}

pub trait CbrtHelper: Float {
    /// 2^(n / 3) for n = [0, 1, 2]
    const ESCALE: [Self; 3];
    /// The polynomial `c0+c1*x+c2*x^2+c3*x^3` approximates `x^(1/3)` on `[1,2]`
    /// with maximal error < 9.2e-5 (attained at x=2)
    const C: [Self; 4];
    const U0: Self;
    const U1: Self;
    const RSCALE: [Self; 6];
    const OFF: [Self; 4];
    const WLIST: [(Self, Self); 7];
    const AZMAGIC1: Self;
    const AZMAGIC2: Self;
    const AZMAGIC3: Self;
    const AZMAGIC4: Self;
    fn fma(self, y: Self, z: Self) -> Self;
}

impl CbrtHelper for f64 {
    const ESCALE: [Self; 3] = [
        1.0,
        hf64!("0x1.428a2f98d728bp+0"), /* 2^(1/3) */
        hf64!("0x1.965fea53d6e3dp+0"), /* 2^(2/3) */
    ];

    const C: [Self; 4] = [
        // hf64!("0x1.1b850259b99ddp-1"),
        // hf64!("0x1.2b9762efeffecp-1"),
        // hf64!("-0x1.4af8eb64ea1ecp-3"),
        // hf64!("0x1.7590cccfad50bp-6"),
        hf64!("0x1.1b0babccfef9cp-1"),
        hf64!("0x1.2c9a3e94d1da5p-1"),
        hf64!("-0x1.4dc30b1a1ddbap-3"),
        hf64!("0x1.7a8d3e4ec9b07p-6"),
    ];

    // 0.33333333...
    const U0: Self = hf64!("0x1.5555555555555p-2");

    // 0.22222222...
    const U1: Self = hf64!("0x1.c71c71c71c71cp-3");

    const RSCALE: [Self; 6] = [1.0, -1.0, 0.5, -0.5, 0.25, -0.25];

    const OFF: [Self; 4] = [hf64!("0x1p-53"), 0.0, 0.0, 0.0];

    const WLIST: [(Self, Self); 7] = [
        (hf64!("0x1.3a9ccd7f022dbp+0"), hf64!("0x1.1236160ba9b93p+0")), // ~ 0x1.1236160ba9b930000000000001e7e8fap+0
        (hf64!("0x1.7845d2faac6fep+0"), hf64!("0x1.23115e657e49cp+0")), // ~ 0x1.23115e657e49c0000000000001d7a799p+0
        (hf64!("0x1.d1ef81cbbbe71p+0"), hf64!("0x1.388fb44cdcf5ap+0")), // ~ 0x1.388fb44cdcf5a0000000000002202c55p+0
        (hf64!("0x1.0a2014f62987cp+1"), hf64!("0x1.46bcbf47dc1e8p+0")), // ~ 0x1.46bcbf47dc1e8000000000000303aa2dp+0
        (hf64!("0x1.fe18a044a5501p+1"), hf64!("0x1.95decfec9c904p+0")), // ~ 0x1.95decfec9c9040000000000000159e8ep+0
        (hf64!("0x1.a6bb8c803147bp+2"), hf64!("0x1.e05335a6401dep+0")), // ~ 0x1.e05335a6401de00000000000027ca017p+0
        (hf64!("0x1.ac8538a031cbdp+2"), hf64!("0x1.e281d87098de8p+0")), // ~ 0x1.e281d87098de80000000000000ee9314p+0
    ];

    const AZMAGIC1: Self = hf64!("0x1.9b78223aa307cp+1");
    const AZMAGIC2: Self = hf64!("0x1.79d15d0e8d59cp+0");
    const AZMAGIC3: Self = hf64!("0x1.a202bfc89ddffp+2");
    const AZMAGIC4: Self = hf64!("0x1.de87aa837820fp+0");

    fn fma(self, y: Self, z: Self) -> Self {
        #[cfg(intrinsics_enabled)]
        {
            return unsafe { core::intrinsics::fmaf64(self, y, z) };
        }

        #[cfg(not(intrinsics_enabled))]
        {
            return super::fma(self, y, z);
        }
    }
}

#[cfg(f128_enabled)]
impl CbrtHelper for f128 {
    const ESCALE: [Self; 3] = [
        1.0,
        hf128!("0x1.428a2f98d728acf826cc8664b665p+0"), /* 2^(1/3) */
        hf128!("0x1.965fea53d6e3c53be1ca3482bf3ap+0"), /* 2^(2/3) */
    ];

    const C: [Self; 4] = [
        hf128!("0x1.1b850223b8bf644fcef50feeced1p-1"),
        hf128!("0x1.2b97635e9e17d5240965cb56dc73p-1"),
        hf128!("-0x1.4af8ec964bbc3767a6cf8ac456cbp-3"),
        hf128!("0x1.7590ceecbb8c4c40d8c5e8b64d6bp-6"),
    ];

    const U0: Self = 0.3333333333333333333333333333333333333333;

    const U1: Self = 0.2222222222222222222222222222222222222222;

    const RSCALE: [Self; 6] = [1.0, -1.0, 0.5, -0.5, 0.25, -0.25];

    const OFF: [Self; 4] = [hf128!("0x1p-53"), 0.0, 0.0, 0.0];

    // Other rounding modes unsupported for f128
    const WLIST: [(Self, Self); 7] =
        [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)];

    const AZMAGIC1: Self = hf128!("0x1.9b78223aa307cp+1");
    const AZMAGIC2: Self = hf128!("0x1.79d15d0e8d59cp+0");
    const AZMAGIC3: Self = hf128!("0x1.a202bfc89ddffp+2");
    const AZMAGIC4: Self = hf128!("0x1.de87aa837820fp+0");

    fn fma(self, y: Self, z: Self) -> Self {
        #[cfg(intrinsics_enabled)]
        {
            return unsafe { core::intrinsics::fmaf128(self, y, z) };
        }

        #[cfg(not(intrinsics_enabled))]
        {
            return super::fmaf128(self, y, z);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spot_checks() {
        if !cfg!(x86_no_sse) {
            // Exposes a rounding mode problem. Ignored on i586 because of inaccurate FMA.
            assert_biteq!(
                cbrt(f64::from_bits(0xf7f792b28f600000)),
                f64::from_bits(0xd29ce68655d962f3)
            );
        }
    }
}
