/* SPDX-License-Identifier: MIT */
/* origin: musl src/math/sqrt.c. */

use core::ops;

use super::super::support::{IntTy, cold_path, raise_invalid};
use super::super::{CastFrom, CastInto, DInt, Float, HInt, Int, MinInt};

pub fn sqrt<F: Float>(x: F) -> F
where
    F::Int: DInt + HInt + CastInto<u32>,
    F::Int: ops::Rem<Output = F::Int>,
    u32: CastInto<F::Int>,
    u8: CastInto<F::Int>,
{
    let zero = IntTy::<F>::ZERO;
    let one = IntTy::<F>::ONE;

    let mut ix = x.to_bits();
    // Exponent and sign
    let mut top = u32::cast_from(ix >> F::SIG_BITS);

    if top.wrapping_sub(1) >= F::EXP_MAX - 1 {
        cold_path();
        //
        if ix.overflowing_mul(f_int::<F, _>(2u8)).0 == zero {
            return x;
        }

        // Positive infinity
        if ix == F::EXP_MASK {
            return x;
        }

        // NaN or negative
        if ix > F::EXP_MASK {
            return raise_invalid(x);
        }

        let scaled = x * F::from_parts(false, (F::SIG_BITS + F::EXP_BIAS) as i32, zero);
        ix = scaled.to_bits();
        top = scaled.exp().unsigned();
        top = top.wrapping_sub(F::SIG_BITS);
    }

    let even = (top & 1) != 0;
    let mut m = (ix << F::EXP_BITS) | (one << (F::BITS - 1));
    if even {
        m >>= 1;
    }
    top = (top.wrapping_add(F::EXP_MAX >> 1)) >> 1;

    // 32-bit three
    let three = f_int::<F, _>(0b11u8) << ((F::BITS / 2) - 2);

    let mut r: F::Int;
    let mut s: F::Int;
    let mut d: F::Int;
    let mut u: F::Int;
    let i: F::Int;

    // 17 for f32, 46 for f64
    i = (ix >> 46) % f_int::<F, _>(128u8);
    // i = (ix >> todo!()) % f_int::<F, _>(128u8);
    r = f_int::<F, _>(RSQRT_TAB[usize::cast_from(i)]) << 16;
    // TODO: can some of this casting back and forth be removed?
    s = wmulh::<u32>(u32::cast_from(m >> 32), u32::cast_from(r)).cast();
    d = wmulh::<u32>(s.cast(), r.cast()).cast();
    u = three - d;

    r = wmulh::<u32>(r.cast(), u.cast()).cast() << 1;
    s = wmulh::<u32>(s.cast(), u.cast()).cast() << 1;
    d = wmulh::<u32>(s.cast(), r.cast()).cast();
    u = three - d;
    r = wmulh::<u32>(r.cast(), u.cast()).cast() << 1;
    /* |r sqrt(m) - 1| < 0x1.3704p-29 (measured worst-case) */
    r <<= 32;
    s = mul64(m, r);
    d = mul64(s, r);
    u = (three << 32) - d;
    s = mul64(s, u); /* repr: 3.61 */
    /* -0x1p-57 < s - sqrt(m) < 0x1.8001p-61 */
    s = (s - 2u8.cast()) >> 9; /* repr: 12.52 */
    // if F::BITS > 32 {
    // } else {
    //     //
    // }
    //
    //
    //

    let d0: F::Int;
    let d1: F::Int;
    // let d2: F::Int;

    let y: F;
    // let t: F;

    d0 = (m << 42).wrapping_sub(s.overflowing_mul(s).0);
    d1 = s.wrapping_sub(d0);
    // d2 = d1.wrapping_add(s).wrapping_add(one);
    s += d1 >> F::BITS - 1;
    s &= F::SIG_MASK;
    // s &= 0x000fffffffffffff;
    s |= f_int::<F, _>(top) << F::SIG_BITS;
    y = F::from_bits(s);
    // if (FENV_SUPPORT) {
    // 	/* handle rounding modes and inexact exception:
    // 	   only (s+1)^2 == 2^42 m case is exact otherwise
    // 	   add a tiny value to cause the fenv effects.  */
    // 	uint64_t tiny = predict_false(d2==0) ? 0 : 0x0010000000000000;
    // 	tiny |= (d1^d2) & 0x8000000000000000;
    // 	t = asdouble(tiny);
    // 	y = eval_as_double(y + t);
    // }
    y
}

fn f_int<F: Float, T: Copy>(x: T) -> F::Int
where
    F::Int: CastFrom<T>,
{
    F::Int::cast_from(x)
}

/// Widen multiply, returning the high half.
fn wmulh<I: HInt>(a: I, b: I) -> I {
    a.widen_mul(b).hi()
}

fn mul64<I: DInt + HInt>(a: I, b: I) -> I {
    let ahi: I = a.hi().widen();
    let alo: I = a.lo().widen();
    let bhi: I = b.hi().widen();
    let blo: I = b.lo().widen();

    (ahi * bhi) + (ahi * blo).hi().widen() + (alo * bhi).hi().widen()
}
// fn mul64(a: u32, b: u32) -> u32 {
//     a.hi() & b.hi()

//     a.widen_mul(b).hi()
// }

#[rustfmt::skip]
const RSQRT_TAB: [u16; 128] = [
    0xb451,0xb2f0,0xb196,0xb044,0xaef9,0xadb6,0xac79,0xab43,
    0xaa14,0xa8eb,0xa7c8,0xa6aa,0xa592,0xa480,0xa373,0xa26b,
    0xa168,0xa06a,0x9f70,0x9e7b,0x9d8a,0x9c9d,0x9bb5,0x9ad1,
    0x99f0,0x9913,0x983a,0x9765,0x9693,0x95c4,0x94f8,0x9430,
    0x936b,0x92a9,0x91ea,0x912e,0x9075,0x8fbe,0x8f0a,0x8e59,
    0x8daa,0x8cfe,0x8c54,0x8bac,0x8b07,0x8a64,0x89c4,0x8925,
    0x8889,0x87ee,0x8756,0x86c0,0x862b,0x8599,0x8508,0x8479,
    0x83ec,0x8361,0x82d8,0x8250,0x81c9,0x8145,0x80c2,0x8040,
    0xff02,0xfd0e,0xfb25,0xf947,0xf773,0xf5aa,0xf3ea,0xf234,
    0xf087,0xeee3,0xed47,0xebb3,0xea27,0xe8a3,0xe727,0xe5b2,
    0xe443,0xe2dc,0xe17a,0xe020,0xdecb,0xdd7d,0xdc34,0xdaf1,
    0xd9b3,0xd87b,0xd748,0xd61a,0xd4f1,0xd3cd,0xd2ad,0xd192,
    0xd07b,0xcf69,0xce5b,0xcd51,0xcc4a,0xcb48,0xca4a,0xc94f,
    0xc858,0xc764,0xc674,0xc587,0xc49d,0xc3b7,0xc2d4,0xc1f4,
    0xc116,0xc03c,0xbf65,0xbe90,0xbdbe,0xbcef,0xbc23,0xbb59,
    0xba91,0xb9cc,0xb90a,0xb84a,0xb78c,0xb6d0,0xb617,0xb560,
];
