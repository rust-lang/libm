/* SPDX-License-Identifier: MIT */
/* origin: core-math/src/binary64/cbrt/cbrt.c
 * Copyright (c) 2021-2022 Alexei Sibidanov.
 * Ported to Rust in 2025 by Trevor Gross.
 */

use core::cmp::Ordering;

use super::support::{DInt, HInt, cold_path};

/// The natural logarithm of `x` (f64).
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log(x: f64) -> f64 {
    return cr_log(x);
}

fn cr_log(x: f64) -> f64 {
    let mut v = x.to_bits();
    let mut e: i32 = (v >> 52) as i32 - 0x3ff;
    if e >= 0x400 || e == -0x3ff {
        /* x <= 0 or NaN/Inf or subnormal */
        if x <= 0.0 {
            /* f(x<0) is NaN, f(+/-0) is -Inf and raises DivByZero */
            if x < 0.0 {
                return 0.0 / 0.0;
            } else {
                return 1.0 / -0.0;
            }
        }

        if e == 0x400 || e == 0xc00 {
            /* +Inf or NaN */
            return x + x;
        }
        if e == -0x3ff {
            /* subnormal */
            v = (f64::from_bits(v) * hf64!("0x1p52")).to_bits();
            e = (v >> 52) as i32 - 0x3ff - 52;
        }
    }
    /* now x > 0 */
    /* normalize v in [1,2) */
    v = (0x3ffu64 << 52) | (v & 0xfffffffffffff);
    /* now x = m*2^e with 1 <= m < 2 (m = v.f) and -1074 <= e <= 1023 */
    if v == 0x3ff0000000000000u64 && e == 0 {
        cold_path();
        return 0.0;
    }

    let (h, l) = cr_log_fast(e, v);

    let err = hf64!("0x1.b6p-69"); /* maximal absolute error from cr_log_fast */

    /* Note: the error analysis is quite tight since if we replace the 0x1.b6p-69
    bound by 0x1.3fp-69, it fails for x=0x1.71f7c59ede8ep+125 (rndz) */

    let left: f64 = h + (l - err);
    let right: f64 = h + (l + err);
    if left == right {
        return left;
    }

    /* the probability of failure of the fast path is about 2^-11.5 */
    cr_log_accurate(x)
}

/* Given 1 <= x < 2, where x = v.f, put in h+l a double-double approximation
   of log(2^e*x), with absolute error bounded by 2^-68.22 (details below).
*/
fn cr_log_fast(mut e: i32, v: u64) -> (f64, f64) {
    let m: u64 = 0x10000000000000 + (v & 0xfffffffffffff);
    /* x = m/2^52 */
    /* if x > sqrt(2), we divide it by 2 to avoid cancellation */
    let c: i32 = (m >= 0x16a09e667f3bcd) as i32;
    e += c; /* now -1074 <= e <= 1024 */
    let cy = [1.0f64, 0.5];
    let cm = [43u64, 44];

    let i: i32 = (m >> cm[c as usize]) as i32;
    let y: f64 = f64::from_bits(v) * cy[c as usize];

    let offset = 362;

    let r: f64 = INVERSE[i as usize - offset];
    let l1: f64 = LOG_INV[i as usize - offset].0;
    let l2: f64 = LOG_INV[i as usize - offset].1;
    let z: f64 = fmaf64(r, y, -1.0); /* exact */

    /* evaluate P(z), for |z| < 0.00212097167968735 */
    let mut ph: f64; /* will hold the value of P(z)-z */
    let z2: f64 = z * z; /* |z2| < 4.5e-6 thus the rounding error on z2 is
    bounded by ulp(4.5e-6) = 2^-70. */
    let p45: f64 = fmaf64(P[5], z, P[4]);
    /* |P[5]| < 0.167, |z| < 0.0022, |P[4]| < 0.21 thus |p45| < 0.22:
    the rounding (and total) error on p45 is bounded by ulp(0.22) = 2^-55 */
    let p23: f64 = fmaf64(P[3], z, P[2]);
    /* |P[3]| < 0.26, |z| < 0.0022, |P[2]| < 0.34 thus |p23| < 0.35:
    the rounding (and total) error on p23 is bounded by ulp(0.35) = 2^-54 */
    ph = fmaf64(p45, z2, p23);
    /* |p45| < 0.22, |z2| < 4.5e-6, |p23| < 0.35 thus |ph| < 0.36:
    the rounding error on ph is bounded by ulp(0.36) = 2^-54.
    Adding the error on p45 multiplied by z2, that on z2 multiplied by p45,
    and that on p23 (ignoring low order errors), we get for the total error
    on ph the following bound:
    2^-54 + err(p45)*4.5e-6 + 0.22*err(z2) + err(p23) <
    2^-54 + 2^-55*4.5e-6 + 0.22*2^-70 + 2^-54 < 2^-52.99 */
    ph = fmaf64(ph, z, P[1]);
    /* let ph0 be the value at input, and ph1 the value at output:
    |ph0| < 0.36, |z| < 0.0022, |P[1]| < 0.5 thus |ph1| < 0.501:
    the rounding error on ph1 is bounded by ulp(0.501) = 2^-53.
    Adding the error on ph0 multiplied by z, we get for the total error
    on ph1 the following bound:
    2^-53 + err(ph0)*0.0022 < 2^-53 + 2^-52.99*0.0022 < 2^-52.99 */
    ph *= z2;
    /* let ph2 be the value at output of the above instruction:
    |ph2| < |z2| * |ph1| < 4.5e-6 * 0.501 < 2.26e-6 thus the
    rounding error on ph2 is bounded by ulp(2.26e-6) = 2^-71.
    Adding the error on ph1 multiplied by z2, and the error on z2
    multiplied by ph1, we get for the total error on ph2 the following bound:
    2^-71 + err(ph1)*z2 + ph1*err(z2) <
    2^-71 + 2^-52.99*4.5e-6 + 0.501*2^-70 < 2^-69.32. */

    /* Add e*log(2) to (h,l), where -1074 <= e <= 1023, thus e has at most
    11 bits. log2_h is an integer multiple of 2^-42, so that e*log2_h
    is exact. */
    let log2_h: f64 = hf64!("0x1.62e42fefa38p-1");
    let log2_l = hf64!("0x1.ef35793c7673p-45");
    /* |log(2) - (h+l)| < 2^-102.01 */
    /* let hh = e * log2_h: hh is an integer multiple of 2^-42,
    with |hh| <= 1074*log2_h
    = 3274082061039582*2^-42. l1 is also an integer multiple of 2^-42,
    with |l1| <= 1524716581803*2^-42. Thus hh+l1 is an integer multiple of
    2^-42, with 2^42*|hh+l1| <= 3275606777621385 < 2^52, thus hh+l1 is exactly
    representable. */

    let ee: f64 = e as f64;
    let (h, mut l) = fast_two_sum(fmaf64(ee, log2_h, l1), z);
    /* here |hh+l1|+|z| <= 3275606777621385*2^-42 + 0.0022 < 745
    thus |h| < 745, and the additional error from the fast_two_sum() call is
    bounded by 2^-105*745 < 2^-95.4. */
    /* add ph + l2 to l */
    l = ph + (l + l2);
    /* here |ph| < 2.26e-6, |l| < ulp(h) = 2^-43, and |l2| < 2^-43,
    thus |*l + l2| < 2^-42, and the rounding error on *l + l2 is bounded
    by ulp(2^-43) = 2^-95 (*l + l2 cannot be >= 2^-42).
    Now |ph + (*l + l2)| < 2.26e-6 + 2^-42 < 2^-18.7, thus the rounding
    error on ph + ... is bounded by ulp(2^-18.7) = 2^-71, which yields a
    cumulated error bound of 2^-71 + 2^-95 < 2^-70.99. */

    l = fmaf64(ee, log2_l, l);
    /* let l_in be the input value of *l, and l_out the output value.
    We have |l_in| < 2^-18.7 (from above)
    and |e*log2_l| <= 1074*0x1.ef35793c7673p-45
    thus |l_out| < 2^-18.69 and err(l_out) <= ulp(2^-18.69) = 2^-71 */

    /* The absolute error on h + l is bounded by:
       2^-70.278 from the error in the Sollya polynomial
       2^-91.94 for the maximal difference |e*(log(2)-(log2_h + log2_l))|
                (|e| <= 1074 and |log(2)-(log2_h + log2_l)| < 2^-102.01)
       2^-97 for the maximal difference |l1 + l2 - (-log(r))|
       2^-69.32 from the rounding errors in the polynomial evaluation
       2^-95.4 from the fast_two_sum call
       2^-70.99 from the *l = ph + (*l + l2) instruction
       2^-71 from the last fmaf64 call.
       This gives an absolute error bounded by < 2^-68.22.
    */

    /* Absolute error bounded by 2^-68.22 < 0x1.b8p-69.
    Using the Gappa tool (https://gappa.gitlabpages.inria.fr/) we can
    improve the bound to 2.89253666698316e-21 < 0x1.b6p-69
    (see file gappa.sage).

    What we proved with gappa (see file log1_template.g):
    for each interval i, 362 <= i <= 724:
    if y is a binary64 number in the range i*2^-9 <= y < (i+1)*2^-9,
    if -1074 <= e <= 1024, assuming the absolute error from the Sollya
    polynomial is bounded by 2^-70.278, the difference between log(2)
    and log2_h + log2_l is bounded by 1.95853e-31, and the maximal
    difference between -log(r) and l1+l2 is bounded by 2^-96, then
    (a) z is exact
    (b) we have the following bounds:
    -2.4696201316824195e-21 <= h + l - log(2^e*y) <= 2.89253666698316e-21
    with the largest bounds obtained for i=369, RNDD (left bound) and
    RNDZ (right bound). */

    (h, l)
}

fn cr_log_accurate(x: f64) -> f64 {
    if x == 1.0 {
        return 0.0;
    }

    let dx = DInt64::fromd(x);
    /* x = (-1)^sgn*2^ex*(hi/2^63+lo/2^127) */
    let dy = log_2(&dx);
    dy.tod()
}

fn log_2(x: &DInt64) -> DInt64 {
    let mut x = x.clone();
    let mut e = x.ex;

    // Find the lookup index
    let mut i: u16 = (x.hi >> 55) as u16;

    if x.hi > 0xb504f333f9de6484 {
        e += 1;
        i = i >> 1;
    }

    x.ex = x.ex - e;

    let inv2 = &INVERSE_2[(i - 128) as usize];
    let mut z = x.mul(inv2);

    z = DInt64::M_ONE.add(&z);

    // E·log(2)
    let r = DInt64::LOG2.mul2(e);

    let mut p = z.pow2();
    p = LOG_INV_2[(i - 128) as usize].add(&p);

    p.add(&r)
}

/// Float represented as a 128-bit significand and 64-bit exponent.
#[derive(Clone)]
struct DInt64 {
    lo: u64,
    hi: u64,
    ex: i64,
    sign: u64,
}

impl DInt64 {
    const M_ONE: Self = Self { hi: 0x8000000000000000, lo: 0x0, ex: 0, sign: 0x1 };

    /* the following is an approximation of log(2), with absolute error less
    than 2^-129.97 */
    const LOG2: Self = Self { hi: 0xb17217f7d1cf79ab, lo: 0xc9e3b39803f2f6af, ex: -1, sign: 0x0 };

    #[allow(unused)]
    const LOG2_INV: Self =
        Self { hi: 0xb8aa3b295c17f0bb, lo: 0xbe87fed0691d3e89, ex: 12, sign: 0x0 };

    const ZERO: Self = Self { hi: 0x0, lo: 0x0, ex: 0, sign: 0x0 };

    fn new(sig: u128, ex: i64, sign: u64) -> Self {
        Self { lo: sig as u64, hi: (sig >> 64) as u64, ex, sign }
    }

    fn u128(&self) -> u128 {
        ((self.hi as u128) << 64) | self.lo as u128
    }

    fn fromd(x: f64) -> Self {
        let (mut ex, mut hi) = fast_extract(x);
        let t = hi.leading_zeros();

        let sign = (x < 0.0) as u64;
        hi = hi << t;
        ex = ex - if t > 11 { t as i64 - 12 } else { 0 };
        let lo = 0;
        Self { lo, hi, ex, sign }
    }

    fn tod(self) -> f64 {
        let mut r: u64 = (self.hi >> 11) | (0x3ffu64 << 52);
        /* r contains the upper 53 bits of a->hi, 1 <= r < 2 */

        let mut rd: f64 = 0.0;
        /* if round bit is 1, add 2^-53 */
        if (self.hi >> 10) & 0x1 != 0 {
            rd += hf64!("0x1p-53");
        }
        /* if trailing bits after the rounding bit are non zero, add 2^-54 */
        if (self.hi & 0x3ff) != 0 || self.lo != 0 {
            rd += hf64!("0x1p-54");
        }

        r |= self.sign << 63;
        let mut r_f = f64::from_bits(r);

        r_f += if self.sign == 0 { rd } else { -rd };

        /* For log, the result is always in the normal range,
        thus a->ex > -1023. Similarly, we cannot have a->ex > 1023. */

        let e: u64 = (((self.ex + 1023) & 0x7ff) as u64) << 52;

        return r_f * f64::from_bits(e);
    }

    fn add(&self, other: &Self) -> Self {
        if (self.hi | self.lo) == 0 {
            return other.clone();
        }

        if (other.hi | other.lo) == 0 {
            return self.clone();
        }

        match self.cmp(other) {
            Ordering::Equal => {
                if (self.sign ^ other.sign) != 0 {
                    return DInt64::ZERO;
                }
                let mut ret = self.clone();
                ret.ex += 1;
                return ret;
            }
            Ordering::Less => {
                return other.add(self);
            }
            Ordering::Greater => (),
        }

        let ai = self.u128();
        let mut bi = other.u128();
        let mut m_ex = self.ex;

        if self.ex > other.ex {
            let sh: i32 = (self.ex - other.ex) as i32;
            // round to nearest
            if sh <= 128 {
                bi += 0x1 & (bi >> (sh - 1));
            }
            if sh < 128 {
                bi = bi >> sh;
            } else {
                bi = 0;
            }
        }

        let sign = self.sign as u8;
        let mut c: u128;

        if (self.sign ^ other.sign) != 0 {
            c = ai - bi;
        } else {
            let oflow;
            (c, oflow) = ai.overflowing_add(bi);
            if oflow {
                c += c & 0x1;
                c = (1u128 << 127) | (c >> 1);
                m_ex += 1;
            }
        }

        let ex: u64 = if (c >> 64) as u64 != 0 {
            ((c >> 64) as u64).leading_zeros() as u64
        } else {
            64 + if (c & u64::MAX as u128) != 0 {
                ((c & u64::MAX as u128) as u64).leading_zeros() as u64
            } else {
                self.ex as u64
            }
        };
        c <<= ex;

        Self::new(c, m_ex - ex as i64, sign as u64)
    }

    fn mul(&self, other: &Self) -> Self {
        let mut t: u128 = self.hi as u128 * other.hi as u128;
        let m1: u128 = self.hi as u128 * other.lo as u128;
        let m2: u128 = self.lo as u128 * other.hi as u128;

        // If we only garantee 127 bits of accuracy, we improve the simplicity of the
        // code uint64_t l = ((u128)(a->lo) * (u128)(b->lo)) >> 64; m.l += l; m.h +=
        // (m.l < l);
        let (m, ovf) = m1.overflowing_add(m2);
        t = t.wrapping_add((ovf as u128) << 64);
        t = t.wrapping_add(m.hi().widen());
        // t = t.wrapping_add(m & ((u64::MAX as u128) << 64));

        // Ensure that r->hi starts with a 1
        let ex: u64 = ((t >> (64 + 63)) == 0) as u64;
        if ex != 0 {
            t <<= 1;
        }
        t += (m & u64::MAX as u128) >> 63;

        Self {
            lo: t as u64,
            hi: (t >> 64) as u64,
            ex: self.ex + other.ex - ex as i64 + 1,
            sign: self.sign ^ other.sign,
        }
    }

    fn mul2(&self, b: i64) -> Self {
        if b == 0 {
            return DInt64::ZERO;
        }

        let c: u64 = if b < 0 { -b } else { b } as u64;
        let sign = if b < 0 { (self.sign == 0) as u64 } else { self.sign };

        let mut t = self.hi as u128 * c as u128;

        let mut m = if t >> 64 != 0 { t.hi().leading_zeros() } else { 64 };

        t <<= m;

        // Will pose issues if b is too large but for now we assume it never happens
        // TODO: FIXME
        let mut l: u128 = self.lo as u128 * c as u128;
        l = (l << (m - 1)) >> 63;

        let oflow;
        (t, oflow) = l.overflowing_add(t);
        if oflow {
            t += t & 0x1;
            t = 1u128 << 127 | t >> 1;
            m -= 1;
        }

        Self::new(t, self.ex + 64 - m as i64, sign)
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.ex != other.ex {
            self.ex.cmp(&other.ex)
        } else if self.hi != other.hi {
            self.hi.cmp(&other.hi)
        } else {
            self.lo.cmp(&other.lo)
        }
    }

    fn pow2(&self) -> Self {
        let z = self;
        let mut r = P_2[0].clone();

        r = z.mul(&r);
        r = P_2[1].add(&r);

        r = z.mul(&r);
        r = P_2[2].add(&r);

        r = z.mul(&r);
        r = P_2[3].add(&r);

        r = z.mul(&r);
        r = P_2[4].add(&r);

        r = z.mul(&r);
        r = P_2[5].add(&r);

        r = z.mul(&r);
        r = P_2[6].add(&r);

        r = z.mul(&r);
        r = P_2[7].add(&r);

        r = z.mul(&r);
        r = P_2[8].add(&r);

        r = z.mul(&r);
        r = P_2[9].add(&r);

        r = z.mul(&r);
        r = P_2[10].add(&r);

        r = z.mul(&r);
        r = P_2[11].add(&r);

        r = z.mul(&r);
        r = P_2[12].add(&r);

        r = z.mul(&r);

        r
    }
}

// Extract both the significand and exponent of a double
fn fast_extract(x: f64) -> (i64, u64) {
    let xi = x.to_bits();
    let e = (xi >> 52) & 0x7ff;
    let m = (xi & (u64::MAX >> 12)) + (if e != 0 { 1u64 << 52 } else { 0 });
    let e = e as i64 - 0x3ff;
    (e, m)
}

fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let hi = a + b;
    let e = hi - a; /* exact */
    let lo = b - e; /* exact */
    (hi, lo)
    /* Now hi + lo = a + b exactly for rounding to nearest.
    For directed rounding modes, this is not always true.
    Take for example a = 1, b = 2^-200, and rounding up,
    then hi = 1 + 2^-52, e = 2^-52 (it can be proven that
    e is always exact), and lo = -2^52 + 2^-105, thus
    hi + lo = 1 + 2^-105 <> a + b = 1 + 2^-200.
    A bound on the error is given
    in "Note on FastTwoSum with Directed Roundings"
    by Paul Zimmermann, https://hal.inria.fr/hal-03798376, 2022.
    Theorem 1 says that
    the difference between a+b and hi+lo is bounded by 2u^2|a+b|
    and also by 2u^2|hi|. Here u=2^-53, thus we get:
    |(a+b)-(hi+lo)| <= 2^-105 min(|a+b|,|hi|) */
}

fn fmaf64(x: f64, y: f64, z: f64) -> f64 {
    #[cfg(intrinsics_enabled)]
    {
        return unsafe { core::intrinsics::fmaf64(x, y, z) };
    }

    #[cfg(not(intrinsics_enabled))]
    {
        return super::fma(x, y, z);
    }
}

/* The following is a degree-6 polynomial generated by Sollya over
[-0.00202941894531250,0.00212097167968735],
with absolute error < 2^-70.278.
The polynomial is P[0]*x + P[1]*x^2 + ... + P[5]*x^6.
The algorithm assumes that P[0]=1. */
static P: [f64; 6] = [
    hf64!("0x1p0"),                 /* degree 1 */
    hf64!("-0x1.ffffffffffffap-2"), /* degree 2 */
    hf64!("0x1.555555554f4d8p-2"),  /* degree 3 */
    hf64!("-0x1.0000000537df6p-2"), /* degree 4 */
    hf64!("0x1.999a14758b084p-3"),  /* degree 5 */
    hf64!("-0x1.55362255e0f63p-3"), /* degree 6 */
];

/* For 362 <= i <= 724, r[i] = _INVERSE[i-362] is a 10-bit approximation of
1/x[i], where i*2^-9 <= x[i] < (i+1)*2^-9.
More precisely r[i] is a 10-bit value such that r[i]*y-1 is representable
exactly on 53 bits for any y, i*2^-9 <= y < (i+1)*2^-9.
Moreover |r[i]*y-1| <= 0.00212097167968735. */
static INVERSE: [f64; 363] = [
    hf64!("0x1.698p+0"),
    hf64!("0x1.688p+0"),
    hf64!("0x1.678p+0"),
    hf64!("0x1.668p+0"),
    hf64!("0x1.658p+0"),
    hf64!("0x1.648p+0"),
    hf64!("0x1.638p+0"),
    hf64!("0x1.63p+0"),
    hf64!("0x1.62p+0"),
    hf64!("0x1.61p+0"),
    hf64!("0x1.6p+0"),
    hf64!("0x1.5fp+0"),
    hf64!("0x1.5ep+0"),
    hf64!("0x1.5dp+0"),
    hf64!("0x1.5cp+0"),
    hf64!("0x1.5bp+0"),
    hf64!("0x1.5a8p+0"),
    hf64!("0x1.598p+0"),
    hf64!("0x1.588p+0"),
    hf64!("0x1.578p+0"),
    hf64!("0x1.568p+0"),
    hf64!("0x1.56p+0"),
    hf64!("0x1.55p+0"),
    hf64!("0x1.54p+0"),
    hf64!("0x1.53p+0"),
    hf64!("0x1.52p+0"),
    hf64!("0x1.518p+0"),
    hf64!("0x1.508p+0"),
    hf64!("0x1.4f8p+0"),
    hf64!("0x1.4fp+0"),
    hf64!("0x1.4ep+0"),
    hf64!("0x1.4dp+0"),
    hf64!("0x1.4cp+0"),
    hf64!("0x1.4b8p+0"),
    hf64!("0x1.4a8p+0"),
    hf64!("0x1.4ap+0"),
    hf64!("0x1.49p+0"),
    hf64!("0x1.48p+0"),
    hf64!("0x1.478p+0"),
    hf64!("0x1.468p+0"),
    hf64!("0x1.458p+0"),
    hf64!("0x1.45p+0"),
    hf64!("0x1.44p+0"),
    hf64!("0x1.43p+0"),
    hf64!("0x1.428p+0"),
    hf64!("0x1.418p+0"),
    hf64!("0x1.41p+0"),
    hf64!("0x1.4p+0"),
    hf64!("0x1.3f8p+0"),
    hf64!("0x1.3e8p+0"),
    hf64!("0x1.3ep+0"),
    hf64!("0x1.3dp+0"),
    hf64!("0x1.3cp+0"),
    hf64!("0x1.3b8p+0"),
    hf64!("0x1.3a8p+0"),
    hf64!("0x1.3ap+0"),
    hf64!("0x1.39p+0"),
    hf64!("0x1.388p+0"),
    hf64!("0x1.378p+0"),
    hf64!("0x1.37p+0"),
    hf64!("0x1.36p+0"),
    hf64!("0x1.358p+0"),
    hf64!("0x1.35p+0"),
    hf64!("0x1.34p+0"),
    hf64!("0x1.338p+0"),
    hf64!("0x1.328p+0"),
    hf64!("0x1.32p+0"),
    hf64!("0x1.31p+0"),
    hf64!("0x1.308p+0"),
    hf64!("0x1.3p+0"),
    hf64!("0x1.2fp+0"),
    hf64!("0x1.2e8p+0"),
    hf64!("0x1.2d8p+0"),
    hf64!("0x1.2dp+0"),
    hf64!("0x1.2c8p+0"),
    hf64!("0x1.2b8p+0"),
    hf64!("0x1.2bp+0"),
    hf64!("0x1.2ap+0"),
    hf64!("0x1.298p+0"),
    hf64!("0x1.29p+0"),
    hf64!("0x1.28p+0"),
    hf64!("0x1.278p+0"),
    hf64!("0x1.27p+0"),
    hf64!("0x1.26p+0"),
    hf64!("0x1.258p+0"),
    hf64!("0x1.25p+0"),
    hf64!("0x1.24p+0"),
    hf64!("0x1.238p+0"),
    hf64!("0x1.23p+0"),
    hf64!("0x1.228p+0"),
    hf64!("0x1.218p+0"),
    hf64!("0x1.21p+0"),
    hf64!("0x1.208p+0"),
    hf64!("0x1.2p+0"),
    hf64!("0x1.1fp+0"),
    hf64!("0x1.1e8p+0"),
    hf64!("0x1.1ep+0"),
    hf64!("0x1.1dp+0"),
    hf64!("0x1.1c8p+0"),
    hf64!("0x1.1cp+0"),
    hf64!("0x1.1b8p+0"),
    hf64!("0x1.1bp+0"),
    hf64!("0x1.1ap+0"),
    hf64!("0x1.198p+0"),
    hf64!("0x1.19p+0"),
    hf64!("0x1.188p+0"),
    hf64!("0x1.18p+0"),
    hf64!("0x1.17p+0"),
    hf64!("0x1.168p+0"),
    hf64!("0x1.16p+0"),
    hf64!("0x1.158p+0"),
    hf64!("0x1.15p+0"),
    hf64!("0x1.14p+0"),
    hf64!("0x1.138p+0"),
    hf64!("0x1.13p+0"),
    hf64!("0x1.128p+0"),
    hf64!("0x1.12p+0"),
    hf64!("0x1.118p+0"),
    hf64!("0x1.11p+0"),
    hf64!("0x1.1p+0"),
    hf64!("0x1.0f8p+0"),
    hf64!("0x1.0fp+0"),
    hf64!("0x1.0e8p+0"),
    hf64!("0x1.0ep+0"),
    hf64!("0x1.0d8p+0"),
    hf64!("0x1.0dp+0"),
    hf64!("0x1.0c8p+0"),
    hf64!("0x1.0cp+0"),
    hf64!("0x1.0bp+0"),
    hf64!("0x1.0a8p+0"),
    hf64!("0x1.0ap+0"),
    hf64!("0x1.098p+0"),
    hf64!("0x1.09p+0"),
    hf64!("0x1.088p+0"),
    hf64!("0x1.08p+0"),
    hf64!("0x1.078p+0"),
    hf64!("0x1.07p+0"),
    hf64!("0x1.068p+0"),
    hf64!("0x1.06p+0"),
    hf64!("0x1.058p+0"),
    hf64!("0x1.05p+0"),
    hf64!("0x1.048p+0"),
    hf64!("0x1.04p+0"),
    hf64!("0x1.038p+0"),
    hf64!("0x1.03p+0"),
    hf64!("0x1.028p+0"),
    hf64!("0x1.02p+0"),
    hf64!("0x1.018p+0"),
    hf64!("0x1.01p+0"),
    hf64!("0x1.008p+0"),
    hf64!("0x1.ff8p-1"),
    hf64!("0x1.fe8p-1"),
    hf64!("0x1.fd8p-1"),
    hf64!("0x1.fc8p-1"),
    hf64!("0x1.fb8p-1"),
    hf64!("0x1.fa8p-1"),
    hf64!("0x1.f98p-1"),
    hf64!("0x1.f88p-1"),
    hf64!("0x1.f78p-1"),
    hf64!("0x1.f68p-1"),
    hf64!("0x1.f58p-1"),
    hf64!("0x1.f5p-1"),
    hf64!("0x1.f4p-1"),
    hf64!("0x1.f3p-1"),
    hf64!("0x1.f2p-1"),
    hf64!("0x1.f1p-1"),
    hf64!("0x1.fp-1"),
    hf64!("0x1.efp-1"),
    hf64!("0x1.eep-1"),
    hf64!("0x1.edp-1"),
    hf64!("0x1.ec8p-1"),
    hf64!("0x1.eb8p-1"),
    hf64!("0x1.ea8p-1"),
    hf64!("0x1.e98p-1"),
    hf64!("0x1.e88p-1"),
    hf64!("0x1.e78p-1"),
    hf64!("0x1.e7p-1"),
    hf64!("0x1.e6p-1"),
    hf64!("0x1.e5p-1"),
    hf64!("0x1.e4p-1"),
    hf64!("0x1.e3p-1"),
    hf64!("0x1.e28p-1"),
    hf64!("0x1.e18p-1"),
    hf64!("0x1.e08p-1"),
    hf64!("0x1.df8p-1"),
    hf64!("0x1.dfp-1"),
    hf64!("0x1.dep-1"),
    hf64!("0x1.ddp-1"),
    hf64!("0x1.dcp-1"),
    hf64!("0x1.db8p-1"),
    hf64!("0x1.da8p-1"),
    hf64!("0x1.d98p-1"),
    hf64!("0x1.d9p-1"),
    hf64!("0x1.d8p-1"),
    hf64!("0x1.d7p-1"),
    hf64!("0x1.d6p-1"),
    hf64!("0x1.d58p-1"),
    hf64!("0x1.d48p-1"),
    hf64!("0x1.d38p-1"),
    hf64!("0x1.d3p-1"),
    hf64!("0x1.d2p-1"),
    hf64!("0x1.d1p-1"),
    hf64!("0x1.d08p-1"),
    hf64!("0x1.cf8p-1"),
    hf64!("0x1.ce8p-1"),
    hf64!("0x1.cep-1"),
    hf64!("0x1.cdp-1"),
    hf64!("0x1.cc8p-1"),
    hf64!("0x1.cb8p-1"),
    hf64!("0x1.ca8p-1"),
    hf64!("0x1.cap-1"),
    hf64!("0x1.c9p-1"),
    hf64!("0x1.c88p-1"),
    hf64!("0x1.c78p-1"),
    hf64!("0x1.c68p-1"),
    hf64!("0x1.c6p-1"),
    hf64!("0x1.c5p-1"),
    hf64!("0x1.c48p-1"),
    hf64!("0x1.c38p-1"),
    hf64!("0x1.c3p-1"),
    hf64!("0x1.c2p-1"),
    hf64!("0x1.c18p-1"),
    hf64!("0x1.c08p-1"),
    hf64!("0x1.bf8p-1"),
    hf64!("0x1.bfp-1"),
    hf64!("0x1.bep-1"),
    hf64!("0x1.bd8p-1"),
    hf64!("0x1.bc8p-1"),
    hf64!("0x1.bcp-1"),
    hf64!("0x1.bbp-1"),
    hf64!("0x1.ba8p-1"),
    hf64!("0x1.b98p-1"),
    hf64!("0x1.b9p-1"),
    hf64!("0x1.b8p-1"),
    hf64!("0x1.b78p-1"),
    hf64!("0x1.b68p-1"),
    hf64!("0x1.b6p-1"),
    hf64!("0x1.b58p-1"),
    hf64!("0x1.b48p-1"),
    hf64!("0x1.b4p-1"),
    hf64!("0x1.b3p-1"),
    hf64!("0x1.b28p-1"),
    hf64!("0x1.b18p-1"),
    hf64!("0x1.b1p-1"),
    hf64!("0x1.bp-1"),
    hf64!("0x1.af8p-1"),
    hf64!("0x1.afp-1"),
    hf64!("0x1.aep-1"),
    hf64!("0x1.ad8p-1"),
    hf64!("0x1.ac8p-1"),
    hf64!("0x1.acp-1"),
    hf64!("0x1.ab8p-1"),
    hf64!("0x1.aa8p-1"),
    hf64!("0x1.aap-1"),
    hf64!("0x1.a9p-1"),
    hf64!("0x1.a88p-1"),
    hf64!("0x1.a8p-1"),
    hf64!("0x1.a7p-1"),
    hf64!("0x1.a68p-1"),
    hf64!("0x1.a6p-1"),
    hf64!("0x1.a5p-1"),
    hf64!("0x1.a48p-1"),
    hf64!("0x1.a4p-1"),
    hf64!("0x1.a3p-1"),
    hf64!("0x1.a28p-1"),
    hf64!("0x1.a2p-1"),
    hf64!("0x1.a1p-1"),
    hf64!("0x1.a08p-1"),
    hf64!("0x1.ap-1"),
    hf64!("0x1.9fp-1"),
    hf64!("0x1.9e8p-1"),
    hf64!("0x1.9ep-1"),
    hf64!("0x1.9dp-1"),
    hf64!("0x1.9c8p-1"),
    hf64!("0x1.9cp-1"),
    hf64!("0x1.9bp-1"),
    hf64!("0x1.9a8p-1"),
    hf64!("0x1.9ap-1"),
    hf64!("0x1.998p-1"),
    hf64!("0x1.988p-1"),
    hf64!("0x1.98p-1"),
    hf64!("0x1.978p-1"),
    hf64!("0x1.968p-1"),
    hf64!("0x1.96p-1"),
    hf64!("0x1.958p-1"),
    hf64!("0x1.95p-1"),
    hf64!("0x1.94p-1"),
    hf64!("0x1.938p-1"),
    hf64!("0x1.93p-1"),
    hf64!("0x1.928p-1"),
    hf64!("0x1.92p-1"),
    hf64!("0x1.91p-1"),
    hf64!("0x1.908p-1"),
    hf64!("0x1.9p-1"),
    hf64!("0x1.8f8p-1"),
    hf64!("0x1.8e8p-1"),
    hf64!("0x1.8ep-1"),
    hf64!("0x1.8d8p-1"),
    hf64!("0x1.8dp-1"),
    hf64!("0x1.8c8p-1"),
    hf64!("0x1.8b8p-1"),
    hf64!("0x1.8bp-1"),
    hf64!("0x1.8a8p-1"),
    hf64!("0x1.8ap-1"),
    hf64!("0x1.898p-1"),
    hf64!("0x1.888p-1"),
    hf64!("0x1.88p-1"),
    hf64!("0x1.878p-1"),
    hf64!("0x1.87p-1"),
    hf64!("0x1.868p-1"),
    hf64!("0x1.86p-1"),
    hf64!("0x1.85p-1"),
    hf64!("0x1.848p-1"),
    hf64!("0x1.84p-1"),
    hf64!("0x1.838p-1"),
    hf64!("0x1.83p-1"),
    hf64!("0x1.828p-1"),
    hf64!("0x1.82p-1"),
    hf64!("0x1.81p-1"),
    hf64!("0x1.808p-1"),
    hf64!("0x1.8p-1"),
    hf64!("0x1.7f8p-1"),
    hf64!("0x1.7fp-1"),
    hf64!("0x1.7e8p-1"),
    hf64!("0x1.7ep-1"),
    hf64!("0x1.7d8p-1"),
    hf64!("0x1.7c8p-1"),
    hf64!("0x1.7cp-1"),
    hf64!("0x1.7b8p-1"),
    hf64!("0x1.7bp-1"),
    hf64!("0x1.7a8p-1"),
    hf64!("0x1.7ap-1"),
    hf64!("0x1.798p-1"),
    hf64!("0x1.79p-1"),
    hf64!("0x1.788p-1"),
    hf64!("0x1.78p-1"),
    hf64!("0x1.778p-1"),
    hf64!("0x1.77p-1"),
    hf64!("0x1.76p-1"),
    hf64!("0x1.758p-1"),
    hf64!("0x1.75p-1"),
    hf64!("0x1.748p-1"),
    hf64!("0x1.74p-1"),
    hf64!("0x1.738p-1"),
    hf64!("0x1.73p-1"),
    hf64!("0x1.728p-1"),
    hf64!("0x1.72p-1"),
    hf64!("0x1.718p-1"),
    hf64!("0x1.71p-1"),
    hf64!("0x1.708p-1"),
    hf64!("0x1.7p-1"),
    hf64!("0x1.6f8p-1"),
    hf64!("0x1.6fp-1"),
    hf64!("0x1.6e8p-1"),
    hf64!("0x1.6ep-1"),
    hf64!("0x1.6d8p-1"),
    hf64!("0x1.6dp-1"),
    hf64!("0x1.6c8p-1"),
    hf64!("0x1.6cp-1"),
    hf64!("0x1.6b8p-1"),
    hf64!("0x1.6bp-1"),
    hf64!("0x1.6a8p-1"),
    hf64!("0x1.6ap-1"),
];

/* For 362 <= i <= 724, (h,l) = _LOG_INV[i-362] is a double-double
approximation of -log(r) with r=INVERSE[i-362]), with h an integer multiple
of 2^-42, and |l| < 2^-43. The maximal difference between -log(r) and h+l
is bounded by 1/2 ulp(l) < 2^-97. */
static LOG_INV: [(f64, f64); 363] = [
    (hf64!("-0x1.615ddb4becp-2"), hf64!("-0x1.3c7ca90bc04b2p-46")),
    (hf64!("-0x1.5e87b20c29p-2"), hf64!("-0x1.527d18f7738fap-44")),
    (hf64!("-0x1.5baf846aa2p-2"), hf64!("0x1.39ae8f873fa41p-44")),
    (hf64!("-0x1.58d54f86ep-2"), hf64!("-0x1.791f30a795215p-45")),
    (hf64!("-0x1.55f9107a44p-2"), hf64!("0x1.1e64778df4a62p-46")),
    (hf64!("-0x1.531ac457eep-2"), hf64!("-0x1.df83b7d931501p-44")),
    (hf64!("-0x1.503a682cb2p-2"), hf64!("0x1.a68c8f16f9b5dp-45")),
    (hf64!("-0x1.4ec97326p-2"), hf64!("-0x1.34d7aaf04d104p-45")),
    (hf64!("-0x1.4be5f95778p-2"), hf64!("0x1.d7c92cd9ad824p-44")),
    (hf64!("-0x1.4900680401p-2"), hf64!("0x1.8bccffe1a0f8cp-44")),
    (hf64!("-0x1.4618bc21c6p-2"), hf64!("0x1.3d82f484c84ccp-46")),
    (hf64!("-0x1.432ef2a04fp-2"), hf64!("0x1.fb129931715adp-44")),
    (hf64!("-0x1.404308686ap-2"), hf64!("-0x1.f8ef43049f7d3p-44")),
    (hf64!("-0x1.3d54fa5c1fp-2"), hf64!("-0x1.c3e1cd9a395e3p-44")),
    (hf64!("-0x1.3a64c55694p-2"), hf64!("-0x1.7a71cbcd735dp-44")),
    (hf64!("-0x1.3772662bfep-2"), hf64!("0x1.e9436ac53b023p-44")),
    (hf64!("-0x1.35f865c933p-2"), hf64!("0x1.b07de4ea1a54ap-44")),
    (hf64!("-0x1.3302c16586p-2"), hf64!("-0x1.6217dc2a3e08bp-44")),
    (hf64!("-0x1.300aead063p-2"), hf64!("-0x1.42f568b75fcacp-44")),
    (hf64!("-0x1.2d10dec508p-2"), hf64!("-0x1.60c61f7088353p-44")),
    (hf64!("-0x1.2a1499f763p-2"), hf64!("0x1.0dbbf51f3aadcp-44")),
    (hf64!("-0x1.2895a13de8p-2"), hf64!("-0x1.a8d7ad24c13fp-44")),
    (hf64!("-0x1.2596010df7p-2"), hf64!("-0x1.8e7bc224ea3e3p-44")),
    (hf64!("-0x1.22941fbcf8p-2"), hf64!("0x1.a6976f5eb0963p-44")),
    (hf64!("-0x1.1f8ff9e48ap-2"), hf64!("-0x1.7946c040cbe77p-45")),
    (hf64!("-0x1.1c898c169ap-2"), hf64!("0x1.81410e5c62affp-44")),
    (hf64!("-0x1.1b05791f08p-2"), hf64!("0x1.2dd466dc55e2dp-44")),
    (hf64!("-0x1.17fb98e151p-2"), hf64!("0x1.a8a8ba74a2684p-44")),
    (hf64!("-0x1.14ef67f887p-2"), hf64!("0x1.e97a65dfc9794p-44")),
    (hf64!("-0x1.136870293bp-2"), hf64!("0x1.d3e8499d67123p-44")),
    (hf64!("-0x1.1058bf9ae5p-2"), hf64!("0x1.4ab9d817d52cdp-44")),
    (hf64!("-0x1.0d46b579abp-2"), hf64!("-0x1.d2c81f640e1e6p-44")),
    (hf64!("-0x1.0a324e2739p-2"), hf64!("-0x1.c6bee7ef4030ep-47")),
    (hf64!("-0x1.08a73667c5p-2"), hf64!("-0x1.ebc1d40c5a329p-44")),
    (hf64!("-0x1.058f3c703fp-2"), hf64!("0x1.0e866bcd236adp-44")),
    (hf64!("-0x1.0402594b4dp-2"), hf64!("-0x1.036b89ef42d7fp-48")),
    (hf64!("-0x1.00e6c45ad5p-2"), hf64!("-0x1.cc68d52e01203p-50")),
    (hf64!("-0x1.fb9186d5e4p-3"), hf64!("0x1.d572aab993c87p-47")),
    (hf64!("-0x1.f871b28956p-3"), hf64!("0x1.f75fd6a526efep-44")),
    (hf64!("-0x1.f22e5e72f2p-3"), hf64!("0x1.f454f1417e41fp-44")),
    (hf64!("-0x1.ebe61f4dd8p-3"), hf64!("0x1.3d45330fdca4dp-45")),
    (hf64!("-0x1.e8c0252aa6p-3"), hf64!("0x1.6805b80e8e6ffp-45")),
    (hf64!("-0x1.e27076e2bp-3"), hf64!("0x1.a342c2af0003cp-44")),
    (hf64!("-0x1.dc1bca0abep-3"), hf64!("-0x1.8fac1a628ccc6p-44")),
    (hf64!("-0x1.d8ef91af32p-3"), hf64!("0x1.5105fc364c784p-46")),
    (hf64!("-0x1.d293581b6cp-3"), hf64!("0x1.83270128aaa5fp-44")),
    (hf64!("-0x1.cf6354e09cp-3"), hf64!("-0x1.771239a07d55bp-45")),
    (hf64!("-0x1.c8ff7c79aap-3"), hf64!("0x1.7794f689f8434p-45")),
    (hf64!("-0x1.c5cba543aep-3"), hf64!("-0x1.0929decb454fcp-45")),
    (hf64!("-0x1.bf601bb0e4p-3"), hf64!("-0x1.386a947c378b5p-45")),
    (hf64!("-0x1.bc286742d8p-3"), hf64!("-0x1.9ac53f39d121cp-44")),
    (hf64!("-0x1.b5b519e8fcp-3"), hf64!("0x1.4b722ec011f31p-44")),
    (hf64!("-0x1.af3c94e80cp-3"), hf64!("0x1.a4e633fcd9066p-52")),
    (hf64!("-0x1.abfe5ae462p-3"), hf64!("0x1.b68f5395f139dp-44")),
    (hf64!("-0x1.a57df28244p-3"), hf64!("-0x1.b99c8ca1d9abbp-44")),
    (hf64!("-0x1.a23bc1fe2cp-3"), hf64!("0x1.539cd91dc9f0bp-44")),
    (hf64!("-0x1.9bb362e7ep-3"), hf64!("0x1.1f2a8a1ce0ffcp-45")),
    (hf64!("-0x1.986d322818p-3"), hf64!("-0x1.93b564dd44p-48")),
    (hf64!("-0x1.91dcc8c34p-3"), hf64!("-0x1.7bc6abddeff46p-44")),
    (hf64!("-0x1.8e928de886p-3"), hf64!("-0x1.a8154b13d72d5p-44")),
    (hf64!("-0x1.87fa06520cp-3"), hf64!("-0x1.22120401202fcp-44")),
    (hf64!("-0x1.84abb75866p-3"), hf64!("0x1.d8daadf4e2bd2p-44")),
    (hf64!("-0x1.815c0a1436p-3"), hf64!("0x1.02a52f9201ce8p-44")),
    (hf64!("-0x1.7ab890210ep-3"), hf64!("0x1.bdb9072534a58p-45")),
    (hf64!("-0x1.7764c128f2p-3"), hf64!("-0x1.274903479e3d1p-47")),
    (hf64!("-0x1.70b8f97a1ap-3"), hf64!("-0x1.4ea64f6a95befp-44")),
    (hf64!("-0x1.6d60fe719ep-3"), hf64!("0x1.bc6e557134767p-44")),
    (hf64!("-0x1.66acd4272ap-3"), hf64!("-0x1.aa1bdbfc6c785p-44")),
    (hf64!("-0x1.6350a28aaap-3"), hf64!("-0x1.d5ec0ab8163afp-45")),
    (hf64!("-0x1.5ff3070a7ap-3"), hf64!("0x1.8586f183bebf2p-44")),
    (hf64!("-0x1.59338d9982p-3"), hf64!("-0x1.0ba68b7555d4ap-48")),
    (hf64!("-0x1.55d1ad4232p-3"), hf64!("-0x1.add94dda647e8p-44")),
    (hf64!("-0x1.4f099f4a24p-3"), hf64!("0x1.e9bf2fafeaf27p-44")),
    (hf64!("-0x1.4ba36f39a6p-3"), hf64!("0x1.4354bb3f219e5p-44")),
    (hf64!("-0x1.483bccce6ep-3"), hf64!("-0x1.eea52723f6369p-46")),
    (hf64!("-0x1.41682bf728p-3"), hf64!("0x1.10047081f849dp-45")),
    (hf64!("-0x1.3dfc2b0eccp-3"), hf64!("-0x1.8a72a62b8c13fp-45")),
    (hf64!("-0x1.371fc201e8p-3"), hf64!("-0x1.ee8779b2d8abcp-44")),
    (hf64!("-0x1.33af57577p-3"), hf64!("-0x1.c9ecca2fe72a5p-44")),
    (hf64!("-0x1.303d718e48p-3"), hf64!("0x1.680b5ce3ecb05p-50")),
    (hf64!("-0x1.29552f82p-3"), hf64!("0x1.5b967f4471dfcp-44")),
    (hf64!("-0x1.25ded0abc6p-3"), hf64!("-0x1.5a3854f176449p-44")),
    (hf64!("-0x1.2266f190a6p-3"), hf64!("0x1.4d20ab840e7f6p-45")),
    (hf64!("-0x1.1b72ad52f6p-3"), hf64!("-0x1.e80a41811a396p-45")),
    (hf64!("-0x1.17f6458fcap-3"), hf64!("-0x1.843fad093c8dcp-45")),
    (hf64!("-0x1.1478584674p-3"), hf64!("-0x1.563451027c75p-46")),
    (hf64!("-0x1.0d77e7cd08p-3"), hf64!("-0x1.cb2cd2ee2f482p-44")),
    (hf64!("-0x1.09f561ee72p-3"), hf64!("0x1.8f3057157d1a8p-45")),
    (hf64!("-0x1.0671512ca6p-3"), hf64!("0x1.a47579cdc0a3dp-45")),
    (hf64!("-0x1.02ebb42bf4p-3"), hf64!("0x1.5a8fa5ce00e5dp-46")),
    (hf64!("-0x1.f7b79fec38p-4"), hf64!("0x1.10987e897ed01p-47")),
    (hf64!("-0x1.f0a30c0118p-4"), hf64!("0x1.d599e83368e91p-44")),
    (hf64!("-0x1.e98b54967p-4"), hf64!("-0x1.4677489c50e97p-44")),
    (hf64!("-0x1.e27076e2bp-4"), hf64!("0x1.a342c2af0003cp-45")),
    (hf64!("-0x1.d4313d66ccp-4"), hf64!("0x1.9454379135713p-45")),
    (hf64!("-0x1.cd0cdbf8cp-4"), hf64!("-0x1.3e14db50dd743p-44")),
    (hf64!("-0x1.c5e548f5bcp-4"), hf64!("-0x1.d0c57585fbe06p-46")),
    (hf64!("-0x1.b78c82bb1p-4"), hf64!("0x1.25ef7bc3987e7p-44")),
    (hf64!("-0x1.b05b49bee4p-4"), hf64!("-0x1.ff22c18f84a5ep-47")),
    (hf64!("-0x1.a926d3a4acp-4"), hf64!("-0x1.563650bd22a9cp-44")),
    (hf64!("-0x1.a1ef1d806p-4"), hf64!("-0x1.cd4176df97bcbp-44")),
    (hf64!("-0x1.9ab4246204p-4"), hf64!("0x1.8a64826787061p-45")),
    (hf64!("-0x1.8c345d6318p-4"), hf64!("-0x1.b20f5acb42a66p-44")),
    (hf64!("-0x1.84ef898e84p-4"), hf64!("0x1.7d5cd246977c9p-44")),
    (hf64!("-0x1.7da766d7bp-4"), hf64!("-0x1.2cc844480c89bp-44")),
    (hf64!("-0x1.765bf23a6cp-4"), hf64!("0x1.ecbc035c4256ap-48")),
    (hf64!("-0x1.6f0d28ae58p-4"), hf64!("0x1.4b4641b664613p-44")),
    (hf64!("-0x1.60658a9374p-4"), hf64!("-0x1.0c3b1dee9c4f8p-44")),
    (hf64!("-0x1.590cafdfp-4"), hf64!("-0x1.c284f5722abaap-44")),
    (hf64!("-0x1.51b073f06p-4"), hf64!("-0x1.83f69278e686ap-44")),
    (hf64!("-0x1.4a50d3aa1cp-4"), hf64!("0x1.f7fe1308973e2p-45")),
    (hf64!("-0x1.42edcbea64p-4"), hf64!("-0x1.bc0eeea7c9acdp-46")),
    (hf64!("-0x1.341d7961bcp-4"), hf64!("-0x1.1d0929983761p-44")),
    (hf64!("-0x1.2cb0283f5cp-4"), hf64!("-0x1.e1ee2ca657021p-44")),
    (hf64!("-0x1.253f62f0ap-4"), hf64!("-0x1.416f8fb69a701p-44")),
    (hf64!("-0x1.1dcb263dbp-4"), hf64!("-0x1.9444f5e9e8981p-44")),
    (hf64!("-0x1.16536eea38p-4"), hf64!("0x1.47c5e768fa309p-46")),
    (hf64!("-0x1.0ed839b554p-4"), hf64!("0x1.901f46d48abb4p-44")),
    (hf64!("-0x1.075983599p-4"), hf64!("0x1.b8ecfe4b59987p-44")),
    (hf64!("-0x1.f0a30c0118p-5"), hf64!("0x1.d599e83368e91p-45")),
    (hf64!("-0x1.e19070c278p-5"), hf64!("0x1.fea4664629e86p-45")),
    (hf64!("-0x1.d276b8adbp-5"), hf64!("-0x1.6a423c78a64bp-46")),
    (hf64!("-0x1.c355dd092p-5"), hf64!("-0x1.f2ccc9abf8388p-45")),
    (hf64!("-0x1.b42dd71198p-5"), hf64!("0x1.c827ae5d6704cp-46")),
    (hf64!("-0x1.a4fe9ffa4p-5"), hf64!("0x1.6e584a0402925p-44")),
    (hf64!("-0x1.95c830ec9p-5"), hf64!("0x1.c148297c5feb8p-45")),
    (hf64!("-0x1.868a83084p-5"), hf64!("0x1.2623a134ac693p-46")),
    (hf64!("-0x1.77458f633p-5"), hf64!("0x1.181dce586af09p-44")),
    (hf64!("-0x1.58a5bafc9p-5"), hf64!("0x1.b2b739570ad39p-45")),
    (hf64!("-0x1.494acc34d8p-5"), hf64!("-0x1.11c78a56fd247p-45")),
    (hf64!("-0x1.39e87b9fe8p-5"), hf64!("-0x1.eafd480ad9015p-44")),
    (hf64!("-0x1.2a7ec2215p-5"), hf64!("0x1.78ce77a9163fep-45")),
    (hf64!("-0x1.1b0d98924p-5"), hf64!("0x1.3401e9ae889bbp-44")),
    (hf64!("-0x1.0b94f7c198p-5"), hf64!("0x1.e89896f022783p-45")),
    (hf64!("-0x1.f829b0e78p-6"), hf64!("-0x1.980267c7e09e4p-45")),
    (hf64!("-0x1.d91a66c54p-6"), hf64!("-0x1.e61f1658cfb9ap-45")),
    (hf64!("-0x1.b9fc027bp-6"), hf64!("0x1.b9a010ae6922ap-44")),
    (hf64!("-0x1.9ace7551dp-6"), hf64!("0x1.d75d97ec7c41p-45")),
    (hf64!("-0x1.7b91b07d6p-6"), hf64!("0x1.3b955b602ace4p-44")),
    (hf64!("-0x1.5c45a51b9p-6"), hf64!("0x1.63bb6216d87d8p-45")),
    (hf64!("-0x1.3cea44347p-6"), hf64!("0x1.6a2c432d6a40bp-44")),
    (hf64!("-0x1.1d7f7eb9fp-6"), hf64!("0x1.4193a83fcc7a6p-46")),
    (hf64!("-0x1.fc0a8b0fcp-7"), hf64!("-0x1.f1e7cf6d3a69cp-50")),
    (hf64!("-0x1.bcf712c74p-7"), hf64!("-0x1.c25e097bd9771p-46")),
    (hf64!("-0x1.7dc475f82p-7"), hf64!("0x1.eb1245b5da1f5p-44")),
    (hf64!("-0x1.3e7295d26p-7"), hf64!("0x1.609c1ff29a114p-45")),
    (hf64!("-0x1.fe02a6b1p-8"), hf64!("-0x1.9e23f0dda40e4p-46")),
    (hf64!("-0x1.7ee11ebd8p-8"), hf64!("-0x1.749d3c2d23a07p-47")),
    (hf64!("-0x1.ff00aa2bp-9"), hf64!("-0x1.0bc04a086b56ap-45")),
    (hf64!("-0x1.ff802a9bp-10"), hf64!("0x1.3bc661d61c5ebp-44")),
    (hf64!("0x1.00200556p-10"), hf64!("0x1.56224cd5f35f8p-44")),
    (hf64!("0x1.809048288p-9"), hf64!("0x1.85c0696a70c0cp-45")),
    (hf64!("0x1.40c8a7478p-8"), hf64!("0x1.e3871df070002p-46")),
    (hf64!("0x1.c189cbb1p-8"), hf64!("-0x1.d80551258856p-44")),
    (hf64!("0x1.2145e939ep-7"), hf64!("0x1.e3d1238c4eap-44")),
    (hf64!("0x1.61e77e8b6p-7"), hf64!("-0x1.8073eeaf8eaf3p-44")),
    (hf64!("0x1.a2a9c6c18p-7"), hf64!("-0x1.f73bc4d6d3472p-44")),
    (hf64!("0x1.e38ce3034p-7"), hf64!("-0x1.9de88a3da281ap-44")),
    (hf64!("0x1.12487a55p-6"), hf64!("0x1.fdbe5fed4b393p-44")),
    (hf64!("0x1.32db0ea13p-6"), hf64!("0x1.710cb130895fcp-45")),
    (hf64!("0x1.537e3f45fp-6"), hf64!("0x1.ab259d2d7f253p-45")),
    (hf64!("0x1.63d617869p-6"), hf64!("0x1.7abf389596542p-47")),
    (hf64!("0x1.8492528c9p-6"), hf64!("-0x1.aa0ba325a0c34p-45")),
    (hf64!("0x1.a55f548c6p-6"), hf64!("-0x1.de0709f2d03c9p-45")),
    (hf64!("0x1.c63d2ec15p-6"), hf64!("-0x1.5439ce030a687p-44")),
    (hf64!("0x1.e72bf2814p-6"), hf64!("-0x1.8d75149774d47p-45")),
    (hf64!("0x1.0415d89e78p-5"), hf64!("-0x1.dddc7f461c516p-44")),
    (hf64!("0x1.149e3e4008p-5"), hf64!("-0x1.2b98a9a4168fdp-44")),
    (hf64!("0x1.252f32f8dp-5"), hf64!("0x1.83e9ae021b67bp-45")),
    (hf64!("0x1.35c8bfaa1p-5"), hf64!("0x1.8357d5ef9eb35p-44")),
    (hf64!("0x1.3e18c1ca08p-5"), hf64!("0x1.748ed3f6e378ep-44")),
    (hf64!("0x1.4ebf4334ap-5"), hf64!("-0x1.d9150f73be773p-45")),
    (hf64!("0x1.5f6e73079p-5"), hf64!("-0x1.0485a8012494cp-45")),
    (hf64!("0x1.70265a551p-5"), hf64!("-0x1.888df11fd5ce7p-45")),
    (hf64!("0x1.80e7023d9p-5"), hf64!("-0x1.99dc16f28bf45p-44")),
    (hf64!("0x1.91b073efd8p-5"), hf64!("-0x1.9d7c53f76ca96p-46")),
    (hf64!("0x1.9a187b574p-5"), hf64!("-0x1.0c22e4ec4d90dp-44")),
    (hf64!("0x1.aaef2d0fbp-5"), hf64!("0x1.0fc1a353bb42ep-45")),
    (hf64!("0x1.bbcebfc69p-5"), hf64!("-0x1.7bf868c317c2ap-46")),
    (hf64!("0x1.ccb73cddd8p-5"), hf64!("0x1.965c36e09f5fep-44")),
    (hf64!("0x1.dda8adc68p-5"), hf64!("-0x1.1b1ac64d9e42fp-45")),
    (hf64!("0x1.e624c4a0b8p-5"), hf64!("-0x1.0f25c74676689p-44")),
    (hf64!("0x1.f723b518p-5"), hf64!("-0x1.d6eb0dd5610d3p-44")),
    (hf64!("0x1.0415d89e74p-4"), hf64!("0x1.111c05cf1d753p-46")),
    (hf64!("0x1.0c9e615ac4p-4"), hf64!("0x1.c2da80974d976p-45")),
    (hf64!("0x1.10e45b3cbp-4"), hf64!("-0x1.7cf69284a3465p-44")),
    (hf64!("0x1.1973bd1464p-4"), hf64!("0x1.566d154f930b3p-44")),
    (hf64!("0x1.2207b5c784p-4"), hf64!("0x1.49d8cfc10c7bfp-44")),
    (hf64!("0x1.2aa04a447p-4"), hf64!("0x1.7a48ba8b1cb41p-44")),
    (hf64!("0x1.2eee507b4p-4"), hf64!("0x1.8081edd77c86p-47")),
    (hf64!("0x1.378dd7f748p-4"), hf64!("0x1.7141128f1facap-44")),
    (hf64!("0x1.403207b414p-4"), hf64!("0x1.6fd84aa8157cp-45")),
    (hf64!("0x1.4485e03dbcp-4"), hf64!("0x1.fad46e8d26ab7p-44")),
    (hf64!("0x1.4d3115d208p-4"), hf64!("-0x1.53a2582f4e1efp-48")),
    (hf64!("0x1.55e10050ep-4"), hf64!("0x1.c1d740c53c72ep-47")),
    (hf64!("0x1.5e95a4d978p-4"), hf64!("0x1.1cb7ce1d17171p-44")),
    (hf64!("0x1.62f1be7d78p-4"), hf64!("-0x1.179957ed63c4ep-45")),
    (hf64!("0x1.6bad83c188p-4"), hf64!("0x1.daf3cc08926aep-47")),
    (hf64!("0x1.746e100228p-4"), hf64!("-0x1.126d16e1e21d2p-44")),
    (hf64!("0x1.78d02263d8p-4"), hf64!("0x1.69b5794b69fb7p-47")),
    (hf64!("0x1.8197e2f41p-4"), hf64!("-0x1.c0fe460d20041p-44")),
    (hf64!("0x1.8a6477a91cp-4"), hf64!("0x1.c28c0af9bd6dfp-44")),
    (hf64!("0x1.8ecc933aecp-4"), hf64!("-0x1.22f39be67f7aap-45")),
    (hf64!("0x1.97a07024ccp-4"), hf64!("-0x1.8bcc1732093cep-48")),
    (hf64!("0x1.a0792e9278p-4"), hf64!("-0x1.a9ce6c9ad51bfp-47")),
    (hf64!("0x1.a4e7640b1cp-4"), hf64!("-0x1.e42b6b94407c8p-47")),
    (hf64!("0x1.adc77ee5bp-4"), hf64!("-0x1.573b209c31904p-44")),
    (hf64!("0x1.b23965a53p-4"), hf64!("-0x1.ff64eea137079p-49")),
    (hf64!("0x1.bb20e936d8p-4"), hf64!("-0x1.68ba835459b8ep-44")),
    (hf64!("0x1.c40d6425a4p-4"), hf64!("0x1.cb1121d1930ddp-44")),
    (hf64!("0x1.c885801bc4p-4"), hf64!("0x1.646d1c65aacd3p-45")),
    (hf64!("0x1.d179788218p-4"), hf64!("0x1.36433b5efbeedp-44")),
    (hf64!("0x1.d5f556592p-4"), hf64!("0x1.0e239cc185469p-44")),
    (hf64!("0x1.def0d8d468p-4"), hf64!("-0x1.24750412e9a74p-44")),
    (hf64!("0x1.e7f1691a34p-4"), hf64!("-0x1.2c1c59bc77bfap-44")),
    (hf64!("0x1.ec739830ap-4"), hf64!("0x1.11fcba80cdd1p-44")),
    (hf64!("0x1.f57bc7d9p-4"), hf64!("0x1.76a6c9ea8b04ep-46")),
    (hf64!("0x1.fa01c9db58p-4"), hf64!("-0x1.8f351fa48a73p-47")),
    (hf64!("0x1.0188d2ecf6p-3"), hf64!("0x1.3f9651cff9dfep-47")),
    (hf64!("0x1.03cdc0a51ep-3"), hf64!("0x1.81a9cf169fc5cp-44")),
    (hf64!("0x1.08598b59e4p-3"), hf64!("-0x1.7e5dd7009902cp-45")),
    (hf64!("0x1.0aa0691268p-3"), hf64!("-0x1.45519d7032129p-44")),
    (hf64!("0x1.0f301717dp-3"), hf64!("-0x1.e09b441ae86c5p-44")),
    (hf64!("0x1.13c2605c3ap-3"), hf64!("-0x1.cf5fdd94f6509p-45")),
    (hf64!("0x1.160c8024b2p-3"), hf64!("0x1.ec2d2a9009e3dp-45")),
    (hf64!("0x1.1aa2b7e24p-3"), hf64!("-0x1.1ac38dde3b366p-44")),
    (hf64!("0x1.1ceed09854p-3"), hf64!("-0x1.15c1c39192af9p-44")),
    (hf64!("0x1.2188fd9808p-3"), hf64!("-0x1.b3a1e7f50c701p-44")),
    (hf64!("0x1.23d712a49cp-3"), hf64!("0x1.00d238fd3df5cp-46")),
    (hf64!("0x1.28753bc11ap-3"), hf64!("0x1.7494e359302e6p-44")),
    (hf64!("0x1.2ac55095f6p-3"), hf64!("-0x1.d3466d0c6c8a8p-46")),
    (hf64!("0x1.2f677cbbcp-3"), hf64!("0x1.52b302160f40dp-44")),
    (hf64!("0x1.31b994d3a4p-3"), hf64!("0x1.f098ee3a5081p-44")),
    (hf64!("0x1.365fcb015ap-3"), hf64!("-0x1.fd3a0afb9691bp-44")),
    (hf64!("0x1.38b3e9e028p-3"), hf64!("-0x1.70ef0545c17f9p-44")),
    (hf64!("0x1.3d5e3126bcp-3"), hf64!("0x1.3fb2f85096c4bp-46")),
    (hf64!("0x1.3fb45a5992p-3"), hf64!("0x1.19713c0cae559p-44")),
    (hf64!("0x1.420b32741p-3"), hf64!("-0x1.16282c85a0884p-46")),
    (hf64!("0x1.46baf0f9f6p-3"), hf64!("-0x1.249cd0790841ap-46")),
    (hf64!("0x1.4913d8333cp-3"), hf64!("-0x1.53e43558124c4p-44")),
    (hf64!("0x1.4dc7b897bcp-3"), hf64!("0x1.c79b60ae1ff0fp-47")),
    (hf64!("0x1.5022b292f6p-3"), hf64!("0x1.48a05ff36a25bp-44")),
    (hf64!("0x1.54dabc261p-3"), hf64!("0x1.746fee5c8d0d8p-45")),
    (hf64!("0x1.5737cc9018p-3"), hf64!("0x1.9baa7a6b887f6p-44")),
    (hf64!("0x1.5bf406b544p-3"), hf64!("-0x1.27023eb68981cp-46")),
    (hf64!("0x1.5e533144c2p-3"), hf64!("-0x1.1ce0bf3b290eap-44")),
    (hf64!("0x1.60b3100b0ap-3"), hf64!("-0x1.71456c988f814p-44")),
    (hf64!("0x1.6574ebe8c2p-3"), hf64!("-0x1.98c1d34f0f462p-44")),
    (hf64!("0x1.67d6e9d786p-3"), hf64!("-0x1.11e8830a706d3p-44")),
    (hf64!("0x1.6c9d07d204p-3"), hf64!("-0x1.c73fafd9b2dcap-50")),
    (hf64!("0x1.6f0128b756p-3"), hf64!("0x1.577390d31ef0fp-44")),
    (hf64!("0x1.716600c914p-3"), hf64!("0x1.51b157cec3838p-49")),
    (hf64!("0x1.7631d82936p-3"), hf64!("-0x1.5e77dc7c5f3e1p-45")),
    (hf64!("0x1.7898d85444p-3"), hf64!("0x1.8e67be3dbaf3fp-44")),
    (hf64!("0x1.7d6903caf6p-3"), hf64!("-0x1.4c06b17c301d7p-45")),
    (hf64!("0x1.7fd22ff59ap-3"), hf64!("-0x1.58bebf457b7d2p-46")),
    (hf64!("0x1.823c16551ap-3"), hf64!("0x1.e0ddb9a631e83p-46")),
    (hf64!("0x1.871213750ep-3"), hf64!("0x1.328eb42f9af75p-44")),
    (hf64!("0x1.897e2b17b2p-3"), hf64!("-0x1.96b37380cbe9ep-45")),
    (hf64!("0x1.8beafeb39p-3"), hf64!("-0x1.73d54aae92cd1p-47")),
    (hf64!("0x1.90c6db9fccp-3"), hf64!("-0x1.935f57718d7cap-46")),
    (hf64!("0x1.9335e5d594p-3"), hf64!("0x1.3115c3abd47dap-44")),
    (hf64!("0x1.95a5adcf7p-3"), hf64!("0x1.7f22858a0ff6fp-47")),
    (hf64!("0x1.9a8778debap-3"), hf64!("0x1.470fa3efec39p-44")),
    (hf64!("0x1.9cf97cdcep-3"), hf64!("0x1.d862f10c414e3p-44")),
    (hf64!("0x1.9f6c40708ap-3"), hf64!("-0x1.337d94bcd3f43p-44")),
    (hf64!("0x1.a454082e6ap-3"), hf64!("0x1.60a77c81f7171p-44")),
    (hf64!("0x1.a6c90d44b8p-3"), hf64!("-0x1.f63b7f037b0c6p-44")),
    (hf64!("0x1.a93ed3c8aep-3"), hf64!("-0x1.8724350562169p-45")),
    (hf64!("0x1.ae2ca6f672p-3"), hf64!("0x1.7a8d5ae54f55p-44")),
    (hf64!("0x1.b0a4b48fc2p-3"), hf64!("-0x1.2e72d5c3998edp-45")),
    (hf64!("0x1.b31d8575bcp-3"), hf64!("0x1.c794e562a63cbp-44")),
    (hf64!("0x1.b811730b82p-3"), hf64!("0x1.e90683b9cd768p-46")),
    (hf64!("0x1.ba8c90ae4ap-3"), hf64!("0x1.a32e7f44432dap-44")),
    (hf64!("0x1.bd087383bep-3"), hf64!("-0x1.d4bc4595412b6p-45")),
    (hf64!("0x1.c2028ab18p-3"), hf64!("-0x1.92e0ee55c7ac6p-45")),
    (hf64!("0x1.c480c0005cp-3"), hf64!("0x1.9a294d5e44e76p-44")),
    (hf64!("0x1.c6ffbc6fp-3"), hf64!("0x1.ee138d3a69d43p-44")),
    (hf64!("0x1.c97f8079d4p-3"), hf64!("0x1.3b161a8c6e6c5p-45")),
    (hf64!("0x1.ce816157f2p-3"), hf64!("-0x1.9e0aba2099515p-45")),
    (hf64!("0x1.d1037f2656p-3"), hf64!("-0x1.84a7e75b6f6e4p-47")),
    (hf64!("0x1.d38666872p-3"), hf64!("-0x1.73650b38932bcp-44")),
    (hf64!("0x1.d88e93fb3p-3"), hf64!("-0x1.75f280234bf51p-44")),
    (hf64!("0x1.db13db0d48p-3"), hf64!("0x1.2806a847527e6p-44")),
    (hf64!("0x1.dd99edaf6ep-3"), hf64!("-0x1.02ec669c756ebp-44")),
    (hf64!("0x1.e020cc6236p-3"), hf64!("-0x1.52b00adb91424p-45")),
    (hf64!("0x1.e530effe72p-3"), hf64!("-0x1.fdbdbb13f7c18p-44")),
    (hf64!("0x1.e7ba35eb78p-3"), hf64!("-0x1.d5eee23793649p-47")),
    (hf64!("0x1.ea4449f04ap-3"), hf64!("0x1.5e91663732a36p-44")),
    (hf64!("0x1.eccf2c8feap-3"), hf64!("-0x1.bec63a3e7564p-44")),
    (hf64!("0x1.ef5ade4ddp-3"), hf64!("-0x1.a211565bb8e11p-51")),
    (hf64!("0x1.f474b134ep-3"), hf64!("-0x1.bae49f1df7b5ep-44")),
    (hf64!("0x1.f702d36778p-3"), hf64!("-0x1.0819516673e23p-46")),
    (hf64!("0x1.f991c6cb3cp-3"), hf64!("-0x1.90d04cd7cc834p-44")),
    (hf64!("0x1.fc218be62p-3"), hf64!("0x1.4bba46f1cf6ap-44")),
    (hf64!("0x1.00a1c6addap-2"), hf64!("0x1.1cd8d688b9e18p-44")),
    (hf64!("0x1.01eae5626cp-2"), hf64!("0x1.a43dcfade85aep-44")),
    (hf64!("0x1.03346e0106p-2"), hf64!("0x1.89ff8a966395cp-48")),
    (hf64!("0x1.047e60cde8p-2"), hf64!("0x1.dbdf10d397f3cp-45")),
    (hf64!("0x1.05c8be0d96p-2"), hf64!("0x1.ad0f1c77ccb58p-45")),
    (hf64!("0x1.085eb8f8aep-2"), hf64!("0x1.e5d513f45fe7bp-44")),
    (hf64!("0x1.09aa572e6cp-2"), hf64!("0x1.b50a1e1734342p-44")),
    (hf64!("0x1.0af660eb9ep-2"), hf64!("0x1.3c7c3f528d80ap-45")),
    (hf64!("0x1.0c42d67616p-2"), hf64!("0x1.7188b163ceae9p-45")),
    (hf64!("0x1.0d8fb813ebp-2"), hf64!("0x1.ee8c88753fa35p-46")),
    (hf64!("0x1.102ac0a35dp-2"), hf64!("-0x1.f1fbddfdfd686p-45")),
    (hf64!("0x1.1178e8227ep-2"), hf64!("0x1.1ef78ce2d07f2p-44")),
    (hf64!("0x1.12c77cd007p-2"), hf64!("0x1.3b2948a11f797p-46")),
    (hf64!("0x1.14167ef367p-2"), hf64!("0x1.e0c07824daaf5p-44")),
    (hf64!("0x1.1565eed456p-2"), hf64!("-0x1.e75adfb6aba25p-49")),
    (hf64!("0x1.16b5ccbadp-2"), hf64!("-0x1.23299042d74bfp-44")),
    (hf64!("0x1.1956d3b9bcp-2"), hf64!("0x1.7d2f73ad1aa14p-45")),
    (hf64!("0x1.1aa7fd638dp-2"), hf64!("0x1.9f60a9616f7ap-45")),
    (hf64!("0x1.1bf99635a7p-2"), hf64!("-0x1.1ac89575c2125p-44")),
    (hf64!("0x1.1d4b9e796cp-2"), hf64!("0x1.22a667c42e56dp-45")),
    (hf64!("0x1.1e9e16788ap-2"), hf64!("-0x1.82eaed3c8b65ep-44")),
    (hf64!("0x1.1ff0fe7cf4p-2"), hf64!("0x1.e9d5b513ff0c1p-44")),
    (hf64!("0x1.214456d0ecp-2"), hf64!("-0x1.caf0428b728a3p-44")),
    (hf64!("0x1.23ec5991ecp-2"), hf64!("-0x1.6dbe448a2e522p-44")),
    (hf64!("0x1.25410494e5p-2"), hf64!("0x1.b1d7ac0ef77f2p-44")),
    (hf64!("0x1.269621134ep-2"), hf64!("-0x1.1b61f10522625p-44")),
    (hf64!("0x1.27ebaf58d9p-2"), hf64!("-0x1.b198800b4bda7p-45")),
    (hf64!("0x1.2941afb187p-2"), hf64!("-0x1.210c2b730e28bp-44")),
    (hf64!("0x1.2a982269a4p-2"), hf64!("-0x1.2058e557285cfp-45")),
    (hf64!("0x1.2bef07cdc9p-2"), hf64!("0x1.a9cfa4a5004f4p-45")),
    (hf64!("0x1.2d46602addp-2"), hf64!("-0x1.88d0ddcd54196p-45")),
    (hf64!("0x1.2ff66b04ebp-2"), hf64!("-0x1.8aed2541e6e2ep-44")),
    (hf64!("0x1.314f1e1d36p-2"), hf64!("-0x1.8e27ad3213cb8p-45")),
    (hf64!("0x1.32a8456512p-2"), hf64!("0x1.4f928139af5d6p-47")),
    (hf64!("0x1.3401e12aedp-2"), hf64!("-0x1.17c73556e291dp-44")),
    (hf64!("0x1.355bf1bd83p-2"), hf64!("-0x1.ba99b8964f0e8p-45")),
    (hf64!("0x1.36b6776be1p-2"), hf64!("0x1.16ecdb0f177c8p-46")),
    (hf64!("0x1.3811728565p-2"), hf64!("-0x1.a71e493a0702bp-45")),
    (hf64!("0x1.396ce359bcp-2"), hf64!("-0x1.5839c5663663dp-47")),
    (hf64!("0x1.3ac8ca38e6p-2"), hf64!("-0x1.d0befbc02be4ap-45")),
    (hf64!("0x1.3c25277333p-2"), hf64!("0x1.83b54b606bd5cp-46")),
    (hf64!("0x1.3d81fb5947p-2"), hf64!("-0x1.22c7c2a9d37a4p-45")),
    (hf64!("0x1.3edf463c17p-2"), hf64!("-0x1.f067c297f2c3fp-44")),
    (hf64!("0x1.419b423d5fp-2"), hf64!("-0x1.ce379226de3ecp-44")),
    (hf64!("0x1.42f9f3ff62p-2"), hf64!("0x1.906440f7d3354p-44")),
    (hf64!("0x1.44591e053ap-2"), hf64!("-0x1.6e95892923d88p-47")),
    (hf64!("0x1.45b8c0a17ep-2"), hf64!("-0x1.d9120e7d0a853p-47")),
    (hf64!("0x1.4718dc271cp-2"), hf64!("0x1.06c18fb4c14c5p-44")),
    (hf64!("0x1.487970e958p-2"), hf64!("0x1.dc1b8465cf25fp-44")),
    (hf64!("0x1.49da7f3bccp-2"), hf64!("0x1.07b334daf4b9ap-44")),
    (hf64!("0x1.4b3c077268p-2"), hf64!("-0x1.65b4681052b9fp-46")),
    (hf64!("0x1.4c9e09e173p-2"), hf64!("-0x1.e20891b0ad8a4p-45")),
    (hf64!("0x1.4e0086dd8cp-2"), hf64!("-0x1.4d692a1e44788p-44")),
    (hf64!("0x1.4f637ebbaap-2"), hf64!("-0x1.fc158cb3124b9p-44")),
    (hf64!("0x1.50c6f1d11cp-2"), hf64!("-0x1.a0e6b7e827c2cp-44")),
    (hf64!("0x1.522ae0738ap-2"), hf64!("0x1.ebe708164c759p-45")),
    (hf64!("0x1.538f4af8f7p-2"), hf64!("0x1.7ec02e45547cep-45")),
    (hf64!("0x1.54f431b7bep-2"), hf64!("0x1.a8954c0910952p-46")),
    (hf64!("0x1.5659950695p-2"), hf64!("0x1.4c5fd2badc774p-46")),
    (hf64!("0x1.57bf753c8dp-2"), hf64!("0x1.fadedee5d40efp-46")),
    (hf64!("0x1.5925d2b113p-2"), hf64!("-0x1.69bf5a7a56f34p-44")),
    (hf64!("0x1.5a8cadbbeep-2"), hf64!("-0x1.7c79b0af7ecf8p-48")),
    (hf64!("0x1.5bf406b544p-2"), hf64!("-0x1.27023eb68981cp-45")),
    (hf64!("0x1.5d5bddf596p-2"), hf64!("-0x1.a0b2a08a465dcp-47")),
    (hf64!("0x1.5ec433d5c3p-2"), hf64!("0x1.6b71a1229d17fp-44")),
    (hf64!("0x1.602d08af09p-2"), hf64!("0x1.ebe9176df3f65p-46")),
    (hf64!("0x1.61965cdb03p-2"), hf64!("-0x1.f08ad603c488ep-45")),
    (hf64!("0x1.630030b3abp-2"), hf64!("-0x1.db623e731aep-45")),
];

const INVERSE_2: &[DInt64] = &[
    DInt64 { hi: 0x8000000000000000, lo: 0x0, ex: 1, sign: 0x0 },
    DInt64 { hi: 0xfe03f80fe03f80ff, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xfc0fc0fc0fc0fc10, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xfa232cf252138ac0, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xf83e0f83e0f83e10, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xf6603d980f6603da, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xf4898d5f85bb3951, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xf2b9d6480f2b9d65, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xf0f0f0f0f0f0f0f1, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xef2eb71fc4345239, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xed7303b5cc0ed731, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xebbdb2a5c1619c8c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xea0ea0ea0ea0ea0f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xe865ac7b7603a197, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xe6c2b4481cd8568a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xe525982af70c880f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xe38e38e38e38e38f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xe1fc780e1fc780e2, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xe070381c0e070382, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xdee95c4ca037ba58, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xdd67c8a60dd67c8b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xdbeb61eed19c5958, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xda740da740da740e, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd901b2036406c80e, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd79435e50d79435f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd62b80d62b80d62c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd4c77b03531dec0e, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd3680d3680d3680e, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd20d20d20d20d20e, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xd0b69fcbd2580d0c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xcf6474a8819ec8ea, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xce168a7725080ce2, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xcccccccccccccccd, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xcb8727c065c393e1, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xca4587e6b74f032a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc907da4e871146ad, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc7ce0c7ce0c7ce0d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc6980c6980c6980d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc565c87b5f9d4d1c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc4372f855d824ca6, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc30c30c30c30c30d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc1e4bbd595f6e948, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xc0c0c0c0c0c0c0c1, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xbfa02fe80bfa02ff, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xbe82fa0be82fa0bf, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xbd69104707661aa3, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xbc52640bc52640bd, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xbb3ee721a54d880c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xba2e8ba2e8ba2e8c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb92143fa36f5e02f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb81702e05c0b8171, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb70fbb5a19be3659, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb60b60b60b60b60c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb509e68a9b948220, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb40b40b40b40b40c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb30f63528917c80c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb21642c8590b2165, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb11fd3b80b11fd3c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xb02c0b02c0b02c0c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xaf3addc680af3ade, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xae4c415c9882b932, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xad602b580ad602b6, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xac7691840ac76919, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xab8f69e28359cd12, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xaaaaaaaaaaaaaaab, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa9c84a47a07f5638, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa8e83f5717c0a8e9, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa80a80a80a80a80b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa72f05397829cbc2, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa655c4392d7b73a8, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa57eb50295fad40b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa4a9cf1d96833752, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa3d70a3d70a3d70b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa3065e3fae7cd0e1, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa237c32b16cfd773, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa16b312ea8fc377d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xa0a0a0a0a0a0a0a1, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9fd809fd809fd80a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9f1165e7254813e3, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9e4cad23dd5f3a21, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9d89d89d89d89d8a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9cc8e160c3fb19b9, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9c09c09c09c09c0a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9b4c6f9ef03a3caa, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9a90e7d95bc609aa, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x99d722dabde58f07, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x991f1a515885fb38, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9868c809868c8099, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x97b425ed097b425f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x97012e025c04b80a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x964fda6c0964fda7, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x95a02568095a0257, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x94f2094f2094f20a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x9445809445809446, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x939a85c40939a85d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x92f113840497889d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x924924924924924a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x91a2b3c4d5e6f80a, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x90fdbc090fdbc091, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x905a38633e06c43b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8fb823ee08fb823f, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8f1779d9fdc3a219, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8e78356d1408e784, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8dda520237694809, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8d3dcb08d3dcb08e, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8ca29c046514e024, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8c08c08c08c08c09, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8b70344a139bc75b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8ad8f2fba9386823, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8a42f8705669db47, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x89ae4089ae4089af, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x891ac73ae9819b51, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8888888888888889, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x87f78087f78087f8, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8767ab5f34e47ef2, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x86d905447a34acc7, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x864b8a7de6d1d609, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x85bf37612cee3c9b, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8534085340853409, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x84a9f9c8084a9f9d, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8421084210842109, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x839930523fbe3368, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x83126e978d4fdf3c, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x828cbfbeb9a020a4, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8208208208208209, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x81848da8faf0d278, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8102040810204082, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8000000000000000, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0x8000000000000000, lo: 0x0, ex: 0, sign: 0x0 },
    DInt64 { hi: 0xff00ff00ff00ff02, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xfe03f80fe03f80ff, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xfd08e5500fd08e56, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xfc0fc0fc0fc0fc11, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xfb18856506ddaba7, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xfa232cf252138ac1, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf92fb2211855a866, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf83e0f83e0f83e11, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf74e3fc22c700f76, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf6603d980f6603db, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf57403d5d00f5741, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf4898d5f85bb3951, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf3a0d52cba872337, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf2b9d6480f2b9d66, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf1d48bcee0d399fb, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf0f0f0f0f0f0f0f2, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xf00f00f00f00f010, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xef2eb71fc4345239, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xee500ee500ee5010, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xed7303b5cc0ed731, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xec979118f3fc4da3, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xebbdb2a5c1619c8d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xeae56403ab959010, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xea0ea0ea0ea0ea10, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe939651fe2d8d35d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe865ac7b7603a198, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe79372e225fe30da, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe6c2b4481cd8568a, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe5f36cb00e5f36cc, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe525982af70c880f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe45932d7dc52100f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe38e38e38e38e38f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe2c4a6886a4c2e11, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe1fc780e1fc780e3, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe135a9c97500e137, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xe070381c0e070383, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xdfac1f74346c5760, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xdee95c4ca037ba58, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xde27eb2c41f3d9d2, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xdd67c8a60dd67c8b, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xdca8f158c7f91ab9, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xdbeb61eed19c5959, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xdb2f171df770291a, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xda740da740da740f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd9ba4256c0366e92, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd901b2036406c80f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd84a598ec9151f44, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd79435e50d79435f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd6df43fca482f00e, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd62b80d62b80d62d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd578e97c3f5fe552, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd4c77b03531dec0e, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd4173289870ac52f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd3680d3680d3680e, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd2ba083b445250ac, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd20d20d20d20d20e, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd161543e28e50275, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd0b69fcbd2580d0c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xd00d00d00d00d00e, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xcf6474a8819ec8ea, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xcebcf8bb5b4169cc, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xce168a7725080ce2, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xcd712752a886d243, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xccccccccccccccce, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xcc29786c7607f9a0, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xcb8727c065c393e1, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xcae5d85f1bbd6c96, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xca4587e6b74f032a, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc9a633fcd967300e, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc907da4e871146ae, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc86a78900c86a78a, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc7ce0c7ce0c7ce0d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc73293d789b9f839, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc6980c6980c6980d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc5fe740317f9d00d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc565c87b5f9d4d1d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc4ce07b00c4ce07c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc4372f855d824ca7, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc3a13de60495c774, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc30c30c30c30c30d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc2780613c0309e03, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc1e4bbd595f6e948, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc152500c152500c2, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc0c0c0c0c0c0c0c2, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xc0300c0300c0300d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbfa02fe80bfa0300, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbf112a8ad278e8de, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbe82fa0be82fa0c0, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbdf59c91700bdf5b, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbd69104707661aa4, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbcdd535db1cc5b7c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbc52640bc52640bd, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbbc8408cd63069a2, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbb3ee721a54d880d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xbab656100bab6562, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xba2e8ba2e8ba2e8d, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb9a7862a0ff46589, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb92143fa36f5e02f, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb89bc36ce3e0453b, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb81702e05c0b8171, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb79300b79300b794, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb70fbb5a19be365a, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb68d31340e4307d9, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb60b60b60b60b60c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb58a485518d1e7e5, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb509e68a9b948220, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb48a39d44685fe98, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb40b40b40b40b40c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb38cf9b00b38cf9c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb30f63528917c80c, lo: 0x0, ex: -1, sign: 0x0 },
    DInt64 { hi: 0xb2927c29da5519d0, lo: 0x0, ex: -1, sign: 0x0 },
];

const LOG_INV_2: &[DInt64] = &[
    DInt64 { hi: 0xb17217f7d1cf79ab, lo: 0xc9e3b39803f2f6af, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xaf74155120c9011d, lo: 0x46d235ee63073dc, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xad7a02e1b24efd32, lo: 0x160864fd949b4bd3, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xab83d135dc633301, lo: 0xffe6607ba902ef3b, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xa991713433c2b999, lo: 0xba4aea614d05700, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xa7a2d41ad270c9d7, lo: 0xcd362382a7688479, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xa5b7eb7cb860fb89, lo: 0x7b6a62a0dec6e072, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xa3d0a93f45169a4b, lo: 0x9594fab088c0d64, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xa1ecff97c91e267b, lo: 0x1b7efae08e597e16, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xa00ce1092e5498c4, lo: 0x69879c5a30cd1241, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x9e304061b5fda91a, lo: 0x4603d87b6df81ac, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x9c5710b8cbb73a42, lo: 0xaa554b2dd4619e63, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x9a81456cec642e10, lo: 0x4d49f9aaea3cb5e0, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x98aed221a03458b6, lo: 0x732f89321647b358, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x96dfaabd86fa1647, lo: 0xd61188fbc94e2f14, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x9513c36876083696, lo: 0xb5cbc416a2418011, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x934b1089a6dc93c2, lo: 0xbf5bb3b60554e151, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x918586c5f5e4bf01, lo: 0x9f92199ed1a4bab0, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x8fc31afe30b2c6de, lo: 0xe300bf167e95da66, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x8e03c24d7300395a, lo: 0xcddae1ccce247837, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x8c47720791e53314, lo: 0x762ad19415fe25a5, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x8a8e1fb794b09134, lo: 0x9eb628dba173c82d, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x88d7c11e3ad53cdc, lo: 0x8a3111a707b6de2c, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x87244c308e670a66, lo: 0x85e005d06dbfa8f7, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x8573b71682a7d21b, lo: 0xb21f9f89c1ab80b2, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x83c5f8299e2b4091, lo: 0xb8f6fafe8fbb68b8, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x821b05f3b01d6774, lo: 0xdb0d58c3f7e2ea1e, ex: -1, sign: 0x1 },
    DInt64 { hi: 0x8072d72d903d588c, lo: 0x7dd1b09c70c40109, ex: -1, sign: 0x1 },
    DInt64 { hi: 0xfd9ac57bd2442180, lo: 0xaf05924d258c14c4, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xfa553f7018c966f4, lo: 0x2780a545a1b54dce, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xf7150ab5a09f27f6, lo: 0xa470250d40ebe8e, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xf3da161eed6b9ab1, lo: 0x248d42f78d3e65d2, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xf0a450d139366ca7, lo: 0x7c66eb6408ff6432, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xed73aa4264b0adeb, lo: 0x5391cf4b33e42996, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xea481236f7d35bb2, lo: 0x39a767a80d6d97e6, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xe72178c0323a1a0f, lo: 0xcc4e1653e71d9973, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xe3ffce3a2aa64923, lo: 0x8eadb651b49ac539, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xe0e30349fd1cec82, lo: 0x3e8e1802aba24d5, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xddcb08dc0717d85c, lo: 0x940a666c87842842, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xdab7d02231484a93, lo: 0xbec20cca6efe2ac4, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xd7a94a92466e833c, lo: 0xcd88bba7d0cee8df, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xd49f69e456cf1b7b, lo: 0x7f53bd2e406e66e6, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xd19a201127d3c646, lo: 0x279d79f51dcc7301, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xce995f50af69d863, lo: 0x432f3f4f861ad6a8, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xcb9d1a189ab56e77, lo: 0x7d7e9307c70c0667, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xc8a5431adfb44ca6, lo: 0x48ce7c1a75e341a, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xc5b1cd44596fa51f, lo: 0xf218fb8f9f9ef27f, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xc2c2abbb6e5fd570, lo: 0x3337789d592e296, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xbfd7d1dec0a8df70, lo: 0x37eda996244bccaf, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xbcf13343e7d9ec7f, lo: 0x2afd17781bb3afea, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xba0ec3b633dd8b0b, lo: 0x91dc60b2b059a609, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xb730773578cb90b3, lo: 0xaa1116c3466beb6c, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xb45641f4e350a0d4, lo: 0xe756eba00bc33976, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xb1801859d56249de, lo: 0x98ce51fff99479cb, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xaeadeefacaf97d37, lo: 0x9dd6e688ebb13b01, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xabdfba9e468fd6f9, lo: 0x472ea07749ce6bd1, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xa9157039c51ebe72, lo: 0xe164c759686a2207, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xa64f04f0b961df78, lo: 0x54f5275c2d15c21e, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xa38c6e138e20d834, lo: 0xd698298adddd7f30, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xa0cda11eaf46390e, lo: 0x632438273918db7d, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x9e1293b9998c1dad, lo: 0x3b035eae273a855c, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x9b5b3bb5f088b768, lo: 0x5078bbe3d392be24, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x98a78f0e9ae71d87, lo: 0x64dec34784707838, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x95f783e6e49a9cfc, lo: 0x25004f3ef063312, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x934b1089a6dc93c2, lo: 0xdf5bb3b60554e151, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x90a22b6875c6a1f8, lo: 0x8e91aeba609c8876, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x8dfccb1ad35ca6ef, lo: 0x9947bdb6ddcaf59a, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x8b5ae65d67db9acf, lo: 0x7ba5168126a58b99, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x88bc74113f23def3, lo: 0xbc5a0fe396f40f1c, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x86216b3b0b17188c, lo: 0x363ceae88f720f1d, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x8389c3026ac3139d, lo: 0x6adda9d2270fa1f3, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x80f572b1363487bc, lo: 0xedbd0b5b3479d5f2, ex: -2, sign: 0x1 },
    DInt64 { hi: 0xfcc8e3659d9bcbf1, lo: 0x8a0cdf301431b60b, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xf7ad6f26e7ff2efc, lo: 0x9cd2238f75f969ad, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xf29877ff38809097, lo: 0x2b020fa1820c948d, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xed89ed86a44a01ab, lo: 0x9d49f96cb88317a, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xe881bf932af3dac3, lo: 0x2524848e3443e03f, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xe37fde37807b84e3, lo: 0x5e9a750b6b68781c, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xde8439c1dec5687c, lo: 0x9d57da945b5d0aa6, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xd98ec2bade71e53e, lo: 0xd0a98f2ad65bee96, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xd49f69e456cf1b7a, lo: 0x5f53bd2e406e66e7, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xcfb6203844b3209b, lo: 0x18cb02f33f79c16b, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xcad2d6e7b80bf915, lo: 0xcc507fb7a3d0bf69, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xc5f57f59c7f46156, lo: 0x9a8b6997a402bf30, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xc11e0b2a8d1e0de1, lo: 0xda631e830fd308fe, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xbc4c6c2a226399f6, lo: 0x276ebcfb2016a433, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xb780945bab55dcea, lo: 0xb4c7bc3d32750fd9, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xb2ba75f46099cf8f, lo: 0x243c2e77904afa76, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xadfa035aa1ed8fdd, lo: 0x549767e410316d2b, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xa93f2f250dac67d5, lo: 0x9ad2fb8d48054add, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xa489ec199dab06f4, lo: 0x59fb6cf0ecb411b7, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x9fda2d2cc9465c52, lo: 0x6b2b9565f5355180, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x9b2fe580ac80b182, lo: 0x11a5b944aca8705, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x968b08643409ceb9, lo: 0xd5c0da506a088482, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x91eb89524e100d28, lo: 0xbfd3df5c52d67e77, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x8d515bf11fb94f22, lo: 0xa0713268840cbcbb, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x88bc74113f23def7, lo: 0x9c5a0fe396f40f19, ex: -3, sign: 0x1 },
    DInt64 { hi: 0x842cc5acf1d0344b, lo: 0x6fecdfa819b96092, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xff4489cedeab2ca6, lo: 0xe17bd40d8d9291ec, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xf639cc185088fe62, lo: 0x5066e87f2c0f733d, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xed393b1c22351281, lo: 0xff4e2e660317d55f, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xe442c00de2591b4c, lo: 0xe96ab34ce0bccd10, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xdb56446d6ad8df09, lo: 0x28112e35a60e636f, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xd273b2058de1bd4b, lo: 0x36bbf837b4d320c6, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xc99af2eaca4c457b, lo: 0xeaf51f66692844b2, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xc0cbf17a071f80e9, lo: 0x396ffdf76a147cc2, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xb8069857560707a7, lo: 0xa677b4c8bec22e0, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xaf4ad26cbc8e5bef, lo: 0x9e8b8b88a14ff0c9, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xa6988ae903f562f1, lo: 0x7e858f08597b3a68, ex: -4, sign: 0x1 },
    DInt64 { hi: 0x9defad3e8f732186, lo: 0x476d3b5b45f6ca02, ex: -4, sign: 0x1 },
    DInt64 { hi: 0x9550252238bd2468, lo: 0x658e5a0b811c596d, ex: -4, sign: 0x1 },
    DInt64 { hi: 0x8cb9de8a32ab3694, lo: 0x97c9859530a4514c, ex: -4, sign: 0x1 },
    DInt64 { hi: 0x842cc5acf1d0344c, lo: 0x1fecdfa819b96094, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xf7518e0035c3dd92, lo: 0x606d89093278a931, ex: -5, sign: 0x1 },
    DInt64 { hi: 0xe65b9e6eed965c4f, lo: 0x609f5fe2058d5ff2, ex: -5, sign: 0x1 },
    DInt64 { hi: 0xd5779687d887e0ee, lo: 0x49dda17056e45ebb, ex: -5, sign: 0x1 },
    DInt64 { hi: 0xc4a550a4fd9a19bb, lo: 0x3e97660a23cc5402, ex: -5, sign: 0x1 },
    DInt64 { hi: 0xb3e4a796a5dac213, lo: 0x7cca0bcc06c2f8e, ex: -5, sign: 0x1 },
    DInt64 { hi: 0xa33576a16f1f4c79, lo: 0x121016bd904dc95a, ex: -5, sign: 0x1 },
    DInt64 { hi: 0x9297997c68c1f4e6, lo: 0x610db3d4dd423bc9, ex: -5, sign: 0x1 },
    DInt64 { hi: 0x820aec4f3a222397, lo: 0xb9e3aea6c444eef6, ex: -5, sign: 0x1 },
    DInt64 { hi: 0xe31e9760a5578c6d, lo: 0xf9eb2f284f31c35a, ex: -6, sign: 0x1 },
    DInt64 { hi: 0xc24929464655f482, lo: 0xda5f3cc0b3251da6, ex: -6, sign: 0x1 },
    DInt64 { hi: 0xa195492cc0660519, lo: 0x4a18dff7cdb4ae33, ex: -6, sign: 0x1 },
    DInt64 { hi: 0x8102b2c49ac23a86, lo: 0x91d082dce3ddcd08, ex: -6, sign: 0x1 },
    DInt64 { hi: 0xc122451c45155150, lo: 0xb16137f09a002b0e, ex: -7, sign: 0x1 },
    DInt64 { hi: 0x8080abac46f389c4, lo: 0x662d417ced0079c9, ex: -7, sign: 0x1 },
    DInt64 { hi: 0x0, lo: 0x0, ex: 127, sign: 0x0 },
    DInt64 { hi: 0x0, lo: 0x0, ex: 127, sign: 0x0 },
    DInt64 { hi: 0xff805515885e014e, lo: 0x435ab4da6a5bb50f, ex: -9, sign: 0x0 },
    DInt64 { hi: 0xff015358833c4762, lo: 0xbb481c8ee1416999, ex: -8, sign: 0x0 },
    DInt64 { hi: 0xbee23afc0853b6a8, lo: 0xa89782c20df350c2, ex: -7, sign: 0x0 },
    DInt64 { hi: 0xfe054587e01f1e2b, lo: 0xf6d3a69bd5eab72f, ex: -7, sign: 0x0 },
    DInt64 { hi: 0x9e75221a352ba751, lo: 0x452b7ea62f2198ea, ex: -6, sign: 0x0 },
    DInt64 { hi: 0xbdc8d83ead88d518, lo: 0x7faa638b5e00ee90, ex: -6, sign: 0x0 },
    DInt64 { hi: 0xdcfe013d7c8cbfc5, lo: 0x632dbac46f30d009, ex: -6, sign: 0x0 },
    DInt64 { hi: 0xfc14d873c1980236, lo: 0xc7e09e3de453f5fc, ex: -6, sign: 0x0 },
    DInt64 { hi: 0x8d86cc491ecbfe03, lo: 0xf1776453b7e82558, ex: -5, sign: 0x0 },
    DInt64 { hi: 0x9cf43dcff5eafd2f, lo: 0x2ad90155c8a7236a, ex: -5, sign: 0x0 },
    DInt64 { hi: 0xac52dd7e4726a456, lo: 0xa47a963a91bb3018, ex: -5, sign: 0x0 },
    DInt64 { hi: 0xbba2c7b196e7e224, lo: 0xe7950f7252c163cf, ex: -5, sign: 0x0 },
    DInt64 { hi: 0xcae41876471f5bde, lo: 0x91d00a417e330f8e, ex: -5, sign: 0x0 },
    DInt64 { hi: 0xda16eb88cb8df5fb, lo: 0x28a63ecfb66e94c0, ex: -5, sign: 0x0 },
    DInt64 { hi: 0xe93b5c56d85a9083, lo: 0xce2992bfea38e76b, ex: -5, sign: 0x0 },
    DInt64 { hi: 0xf85186008b1532f9, lo: 0xe64b8b7759978998, ex: -5, sign: 0x0 },
    DInt64 { hi: 0x83acc1acc7238978, lo: 0x5a5333c45b7f442e, ex: -4, sign: 0x0 },
    DInt64 { hi: 0x8b29b7751bd7073b, lo: 0x2e0b9ee992f2372, ex: -4, sign: 0x0 },
    DInt64 { hi: 0x929fb17850a0b7be, lo: 0x5b4d3807660516a4, ex: -4, sign: 0x0 },
    DInt64 { hi: 0x9a0ebcb0de8e848e, lo: 0x2c1bb082689ba814, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xa176e5f5323781d2, lo: 0xdcf935996c92e8d4, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xa8d839f830c1fb40, lo: 0x4c7343517c8ac264, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xb032c549ba861d83, lo: 0x774e27bc92ce3373, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xb78694572b5a5cd3, lo: 0x24cdcf68cdb2067c, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xbed3b36bd8966419, lo: 0x7c0644d7d9ed08b4, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xc61a2eb18cd907a1, lo: 0xe5a1532f6d5a1ac1, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xcd5a1231019d66d7, lo: 0x761e3e7b171e44b2, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xd49369d256ab1b1f, lo: 0x9e9154e1d5263cda, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xdbc6415d876d0839, lo: 0x3e33c0c9f8824f54, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xe2f2a47ade3a18a8, lo: 0xa0bf7c0b0d8bb4ef, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xea189eb3659aeaeb, lo: 0x93b2a3b21f448259, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xf1383b7157972f48, lo: 0x543fff0ff4f0aaf1, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xf85186008b153302, lo: 0x5e4b8b7759978993, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xff64898edf55d548, lo: 0x428ccfc99271dffa, ex: -4, sign: 0x0 },
    DInt64 { hi: 0x8338a89652cb714a, lo: 0xb247eb86498c2ce7, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x86bbf3e68472cb2f, lo: 0xb8bd20615747126, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x8a3c2c233a156341, lo: 0x9027c74fe0e6f64f, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x8db956a97b3d0143, lo: 0xf023472cd739f9e1, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x913378c852d65be6, lo: 0x977e3013d10f7525, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x94aa97c0ffa91a5d, lo: 0x4ee3880fb7d34429, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x981eb8c723fe97f2, lo: 0x1f1c134fb702d433, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x9b8fe100f47ba1d8, lo: 0x4b62af189fcba0d, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x9efe158766314e4f, lo: 0x4d71827efe892fc8, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xa2695b665be8f338, lo: 0x4eca87c3f0f06211, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xa5d1b79cd2af2aca, lo: 0x8837986ceabfbed6, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xa9372f1d0da1bd10, lo: 0x580eb71e58cd36e5, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xac99c6ccc1042e94, lo: 0x3dd557528315838d, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xaff983853c9e9e40, lo: 0x5f105039091dd7f5, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xb3566a13956a86f4, lo: 0x471b1e1574d9fd55, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xb6b07f38ce90e463, lo: 0x7bb2e265d0de37e1, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xba07c7aa01bd2648, lo: 0x43f9d57b324bd05f, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xbd5c481086c848db, lo: 0xbb596b5030403242, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xc0ae050a1abf56ad, lo: 0x2f7f8c5fa9c50d76, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xc3fd03290648847d, lo: 0x30480bee4cbbd698, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xc74946f4436a054e, lo: 0xf4f5cb531201c0d3, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xca92d4e7a2b5a3ad, lo: 0xc983a9c5c4b3b135, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xcdd9b173efdc1aaa, lo: 0x8863e007c184a1e7, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xd11de0ff15ab18c6, lo: 0xd88d83d4cc613f21, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xd45f67e44178c612, lo: 0x5486e73c615158b4, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xd79e4a7405ff96c3, lo: 0x1300c9be67ae5da0, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xdada8cf47dad236d, lo: 0xdffb833c3409ee7e, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xde1433a16c66b14c, lo: 0xde744870f54f0f18, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xe14b42ac60c60512, lo: 0x4e38eb8092a01f06, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xe47fbe3cd4d10d5b, lo: 0x2ec0f797fdcd125c, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xe7b1aa704e2ee240, lo: 0xb40faab6d2ad0841, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xeae10b5a7ddc8ad8, lo: 0x806b2fc9a8038790, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xee0de5055f63eb01, lo: 0x90a33316df83ba5a, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xf1383b7157972f4a, lo: 0xb43fff0ff4f0aaf1, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xf460129552d2ff41, lo: 0xe62e3201bb2bbdce, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xf7856e5ee2c9b28a, lo: 0x76f2a1b84190a7dc, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xfaa852b25bd9b833, lo: 0xa6dbfa03186e0666, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xfdc8c36af1f15468, lo: 0xa3361bca696504a, ex: -3, sign: 0x0 },
    DInt64 { hi: 0x8073622d6a80e631, lo: 0xe897009015316073, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x82012ca5a68206d5, lo: 0x8fde85afdd2bc88a, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x838dc2fe6ac868e7, lo: 0x1a3fcbdef40100cb, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x851927139c871af8, lo: 0x67bd00c38061c51f, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x86a35abcd5ba5901, lo: 0x5481c3cbd925ccd2, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x882c5fcd7256a8c1, lo: 0x39055a6598e7c29e, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x89b438149d4582f5, lo: 0x34531dba493eb5a6, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x8b3ae55d5d30701a, lo: 0xc63eab8837170480, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x8cc0696ea11b7b36, lo: 0x94361c9a28d38a6a, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x8e44c60b4ccfd7dc, lo: 0x1473aa01c7778679, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x8fc7fcf24517946a, lo: 0x380cbe769f2c6793, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x914a0fde7bcb2d0e, lo: 0xc429ed3aea197a60, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x92cb0086fbb1cf75, lo: 0xa29d47c50b1182d0, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x944ad09ef4351af1, lo: 0xa49827e081cb16ba, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x95c981d5c4e924ea, lo: 0x45404f5aa577d6b4, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x974715d708e984dd, lo: 0x6648d42840d9e6fb, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x98c38e4aa20c27d2, lo: 0x846767ec990d7333, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x9a3eecd4c3eaa6ae, lo: 0xdb3a7f6e6087b947, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x9bb93315fec2d790, lo: 0x7f589fba0865790f, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x9d3262ab4a2f4e37, lo: 0xa1ae6ba06846fae0, ex: -2, sign: 0x0 },
    DInt64 { hi: 0x9eaa7d2e0fb87c35, lo: 0xff472bc6ce648a7d, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa0218434353f1de4, lo: 0xd493efa632530acc, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa197795027409daa, lo: 0x1dd1d4a6df960357, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa30c5e10e2f613e4, lo: 0x9bd9bd99e39a20b3, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa4803402004e865c, lo: 0x31cbe0e8824116cd, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa5f2fcabbbc506d8, lo: 0x68ca4fb7ec323d74, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa764b99300134d79, lo: 0xd04d10474301862, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xa8d56c396fc1684c, lo: 0x1eb067d578c4756, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xaa45161d6e93167b, lo: 0x9b081cf72249f5b2, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xabb3b8ba2ad362a1, lo: 0x1db6506cc17a01f5, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xad215587a67f0cdf, lo: 0xe890422cb86b7cb1, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xae8dedfac04e5282, lo: 0xac707b8ffc22b3e8, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xaff983853c9e9e3f, lo: 0xc5105039091dd7f8, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xb1641795ce3ca978, lo: 0xfaf915300e517393, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xb2cdab981f0f940b, lo: 0xc857c77dc1df600f, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xb43640f4d8a5761f, lo: 0xf5f080a71c34b25d, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xb59dd911aca1ec48, lo: 0x1d2664cf09a0c1bf, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xb70475515d0f1c5e, lo: 0x4c98c6b8be17818d, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xb86a1713c491aeaa, lo: 0xd37ee2872a6f1cd6, ex: -2, sign: 0x0 },
];

const P_2: &[DInt64] = &[
    DInt64 { hi: 0x99df88a0430813ca, lo: 0xa1cffb6e966a70f6, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xaaa02d43f696c3e4, lo: 0x4dbe754667b6bc48, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xba2e7a1eaf856174, lo: 0x70e5c5a5ebbe0226, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xccccccb9ec017492, lo: 0xf934e28d924e76d4, ex: -4, sign: 0x1 },
    DInt64 { hi: 0xe38e38e3807cfa4b, lo: 0xc976e6cbd22e203f, ex: -4, sign: 0x0 },
    DInt64 { hi: 0xfffffffffff924cc, lo: 0x5b308e39fa7dfb5, ex: -4, sign: 0x1 },
    DInt64 { hi: 0x924924924924911d, lo: 0x862bc3d33abb3649, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xaaaaaaaaaaaaaaaa, lo: 0x6637fd4b19743eec, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xcccccccccccccccc, lo: 0xccc2ca18b08fe343, ex: -3, sign: 0x0 },
    DInt64 { hi: 0xffffffffffffffff, lo: 0xffffff2245823ae0, ex: -3, sign: 0x1 },
    DInt64 { hi: 0xaaaaaaaaaaaaaaaa, lo: 0xaaaaaaaaa5c48b54, ex: -2, sign: 0x0 },
    DInt64 { hi: 0xffffffffffffffff, lo: 0xffffffffffffebd8, ex: -2, sign: 0x1 },
    DInt64 { hi: 0x8000000000000000, lo: 0x0, ex: 0, sign: 0x0 },
];
