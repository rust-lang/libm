/* origin: FreeBSD /usr/src/lib/msun/src/s_cbrt.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 *
 * Optimized by Bruce D. Evans.
 */
/*
 * Copyright (c) 2021-2022 Alexei Sibidanov.
 *
 * This file is part of the CORE-MATH project
 * (https://core-math.gitlabpages.inria.fr/).
*/
/* cbrt(x)
 * Return cube root of x
 */

// #![allow(unused)]

use core::{f64, intrinsics};

const B1: u32 = 715094163; /* B1 = (1023-1023/3-0.03306235651)*2**20 */
const B2: u32 = 696219795; /* B2 = (1023-1023/3-54/3-0.03306235651)*2**20 */

/* |1/cbrt(x) - p(x)| < 2**-23.5 (~[-7.93e-8, 7.929e-8]). */
const P0: f64 = 1.87595182427177009643; /* 0x3ffe03e6, 0x0f61e692 */
const P1: f64 = -1.88497979543377169875; /* 0xbffe28e0, 0x92f02420 */
const P2: f64 = 1.621429720105354466140; /* 0x3ff9f160, 0x4a49d6c2 */
const P3: f64 = -0.758397934778766047437; /* 0xbfe844cb, 0xbee751d9 */
const P4: f64 = 0.145996192886612446982; /* 0x3fc2b000, 0xd4e4edd7 */

fn __builtin_expect<T>(v: T, _exp: T) -> T {
    v
}

fn __builtin_fabs(x: f64) -> f64 {
    unsafe { intrinsics::fabsf64(x) }
}

fn __builtin_copysign(x: f64, y: f64) -> f64 {
    unsafe { intrinsics::copysignf64(x, y) }
}

type FExcept = u32;

fn get_rounding_mode(_flag: &mut FExcept) -> i32 {
    // Always nearest
    0
}

fn set_flags(_flag: &FExcept) {}

fn cr_cbrt(x: f64) -> f64 {
    const ESCALE: [f64; 3] = [
        1.0,
        hf64!("0x1.428a2f98d728bp+0"), /* 2^(1/3) */
        hf64!("0x1.965fea53d6e3dp+0"), /* 2^(2/3) */
    ];

    /* the polynomial c0+c1*x+c2*x^2+c3*x^3 approximates x^(1/3) on [1,2]
    with maximal error < 9.2e-5 (attained at x=2) */
    const C: [f64; 4] = [
        hf64!("0x1.1b0babccfef9cp-1"),
        hf64!("0x1.2c9a3e94d1da5p-1"),
        hf64!("-0x1.4dc30b1a1ddbap-3"),
        hf64!("0x1.7a8d3e4ec9b07p-6"),
    ];

    let u0: f64 = hf64!("0x1.5555555555555p-2");
    let u1: f64 = hf64!("0x1.c71c71c71c71cp-3");

    let rsc = [1.0, -1.0, 0.5, -0.5, 0.25, -0.25];

    let off = [hf64!("0x1p-53"), 0.0, 0.0, 0.0];

    let mut flag: FExcept = 0;
    let rm = get_rounding_mode(&mut flag);

    // /* rm=0 for rounding to nearest, and other values for directed roundings */
    let hx: u64 = x.to_bits();
    let mut mant: u64 = hx & (u64::MAX >> 12);
    let sign: u64 = hx >> 63;

    // unsigned e = (hx>>52)&0x7ff;
    let mut e: u32 = (hx >> 52) as u32 & 0x7ff;
    // if(__builtin_expect(((e+1)&0x7ff)<2, 0)){
    if __builtin_expect(((e + 1) & 0x7ff) < 2, false) {
        // uint64_t ix = hx&((~(uint64_t)0)>>1);
        let ix: u64 = hx & (u64::MAX >> 1);
        /* 0, inf, nan: we return x + x instead of simply x,
        to that for x a signaling NaN, it correctly triggers
        the invalid exception. */
        if e == 0x7ff || ix == 0 {
            return x + x;
        }

        // /* use __builtin_clzll otherwise ix might be truncated to 32 bits
        //    on 32-bit processors */
        let nz = ix.leading_zeros() - 11; /* subnormal */
        mant <<= nz;
        mant &= u64::MAX >> 12;
        e -= nz - 1
    }

    e += 3072;
    let cvt1: u64 = mant | (0x3ffu64 << 52);
    let mut cvt5: u64 = cvt1;

    let et: u32 = e / 3;
    let it: u32 = e % 3;

    /* 2^(3k+it) <= x < 2^(3k+it+1), with 0 <= it <= 3 */
    cvt5 += u64::from(it) << 52;
    cvt5 |= sign << 63;
    let zz: f64 = f64::from_bits(cvt5);
    /* cbrt(x) = cbrt(zz)*2^(et-1365) where 1 <= zz < 8 */
    // uint64_t isc = ((const uint64_t*)escale)[it];

    let mut isc: u64 = ESCALE[it as usize].to_bits(); // todo: index
    isc |= sign << 63;
    let cvt2: u64 = isc;
    let z: f64 = f64::from_bits(cvt1);
    /* cbrt(zz) = cbrt(z)*isc, where isc encodes 1, 2^(1/3) or 2^(2/3),
    and 1 <= z < 2 */
    let r: f64 = 1.0 / z;
    let rr: f64 = r * rsc[(it as usize) << 1 | sign as usize];
    let z2: f64 = z * z;
    let c0: f64 = C[0] + z * C[1];
    let c2: f64 = C[2] + z * C[3];
    let mut y: f64 = c0 + z2 * c2;
    let mut y2: f64 = y * y;
    /* y is an approximation of z^(1/3) */
    let mut h: f64 = y2 * (y * r) - 1.0;
    /* h determines the error between y and z^(1/3) */
    y -= (h * y) * (u0 - u1 * h);
    /* The correction y -= (h*y)*(u0 - u1*h) corresponds to a cubic variant
    of Newton's method, with the function f(y) = 1-z/y^3. */
    y *= f64::from_bits(cvt2);
    /* Now y is an approximation of zz^(1/3),
    and rr an approximation of 1/zz. We now perform another iteration of
    Newton-Raphson, this time with a linear approximation only. */
    y2 = y * y;
    let mut y2l: f64 = unsafe { intrinsics::fmaf64(y, y, -y2) };
    /* y2 + y2l = y^2 exactly */
    let mut y3: f64 = y2 * y;
    let mut y3l: f64 = unsafe { intrinsics::fmaf64(y, y2, -y3) } + y * y2l;
    /* y3 + y3l approximates y^3 with about 106 bits of accuracy */
    h = ((y3 - zz) + y3l) * rr;
    let mut dy: f64 = h * (y * u0);
    /* the approximation of zz^(1/3) is y - dy */
    let mut y1: f64 = y - dy;
    dy = (y - y1) - dy;
    /* the approximation of zz^(1/3) is now y1 + dy, where |dy| < 1/2 ulp(y)
    (for rounding to nearest) */
    let mut ady: f64 = unsafe { intrinsics::fabsf64(dy) };
    /* For directed roundings, ady0 is tiny when dy is tiny, or ady0 is near
    from ulp(1);
    for rounding to nearest, ady0 is tiny when dy is near from 1/2 ulp(1),
    or from 3/2 ulp(1). */
    let mut ady0: f64 = unsafe { intrinsics::fabsf64(ady - off[rm as usize]) };
    let mut ady1: f64 = unsafe { intrinsics::fabsf64(ady - (hf64!("0x1p-52") + off[rm as usize])) };
    if __builtin_expect(ady0 < hf64!("0x1p-75") || ady1 < hf64!("0x1p-75"), false) {
        y2 = y1 * y1;
        y2l = unsafe { intrinsics::fmaf64(y1, y1, -y2) };
        y3 = y2 * y1;
        y3l = unsafe { intrinsics::fmaf64(y1, y2, -y3) } + y1 * y2l;
        h = ((y3 - zz) + y3l) * rr;
        dy = h * (y1 * u0);
        y = y1 - dy;
        dy = (y1 - y) - dy;
        y1 = y;
        ady = __builtin_fabs(dy);
        ady0 = __builtin_fabs(ady - off[rm as usize]);
        ady1 = __builtin_fabs(ady - (hf64!("0x1p-52") + off[rm as usize]));
        if __builtin_expect(ady0 < hf64!("0x1p-98") || ady1 < hf64!("0x1p-98"), false) {
            let azz: f64 = __builtin_fabs(zz);

            // ~ 0x1.79d15d0e8d59b80000000000000ffc3dp+0
            if azz == hf64!("0x1.9b78223aa307cp+1") {
                y1 = __builtin_copysign(hf64!("0x1.79d15d0e8d59cp+0"), zz);
            }

            // ~ 0x1.de87aa837820e80000000000001c0f08p+0
            if azz == hf64!("0x1.a202bfc89ddffp+2") {
                y1 = __builtin_copysign(hf64!("0x1.de87aa837820fp+0"), zz);
            }

            if rm > 0 {
                let wlist = [
                    (hf64!("0x1.3a9ccd7f022dbp+0"), hf64!("0x1.1236160ba9b93p+0")), // ~ 0x1.1236160ba9b930000000000001e7e8fap+0
                    (hf64!("0x1.7845d2faac6fep+0"), hf64!("0x1.23115e657e49cp+0")), // ~ 0x1.23115e657e49c0000000000001d7a799p+0
                    (hf64!("0x1.d1ef81cbbbe71p+0"), hf64!("0x1.388fb44cdcf5ap+0")), // ~ 0x1.388fb44cdcf5a0000000000002202c55p+0
                    (hf64!("0x1.0a2014f62987cp+1"), hf64!("0x1.46bcbf47dc1e8p+0")), // ~ 0x1.46bcbf47dc1e8000000000000303aa2dp+0
                    (hf64!("0x1.fe18a044a5501p+1"), hf64!("0x1.95decfec9c904p+0")), // ~ 0x1.95decfec9c9040000000000000159e8ep+0
                    (hf64!("0x1.a6bb8c803147bp+2"), hf64!("0x1.e05335a6401dep+0")), // ~ 0x1.e05335a6401de00000000000027ca017p+0
                    (hf64!("0x1.ac8538a031cbdp+2"), hf64!("0x1.e281d87098de8p+0")), // ~ 0x1.e281d87098de80000000000000ee9314p+0
                ];
                for i in 0..7 {
                    if azz == wlist[i].0 {
                        let tmp = if rm as u64 + sign == 2 {
                            hf64!("0x1p-52")
                        } else {
                            0.0
                        };
                        y1 = __builtin_copysign(wlist[i].1 + tmp, zz);
                    }
                }
            }
        }
    }

    let mut cvt3: u64 = y1.to_bits();
    cvt3 = cvt3.wrapping_add(((et.wrapping_sub(342).wrapping_sub(1023)) as u64) << 52);
    let m0: u64 = cvt3 << 30;
    let m1 = m0 >> 63;
    if __builtin_expect((m0 ^ m1) <= (1u64 << 30), false) {
        let mut cvt4: u64 = y1.to_bits();
        cvt4 = (cvt4 + (164 << 15)) & 0xffffffffffff0000u64;
        if __builtin_fabs((f64::from_bits(cvt4) - y1) - dy) < hf64!("0x1p-60")
            || __builtin_fabs(zz) == 1.0
        {
            cvt3 = (cvt3 + (1u64 << 15)) & 0xffffffffffff0000u64;
            set_flags(&flag);
        }
    }

    f64::from_bits(cvt3)
}

// Cube root (f64)
///
/// Computes the cube root of the argument.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn cbrt(x: f64) -> f64 {
    if true {
        return cr_cbrt(x);
    }

    let x1p54 = f64::from_bits(0x4350000000000000); // 0x1p54 === 2 ^ 54

    let mut ui: u64 = x.to_bits();
    let mut r: f64;
    let s: f64;
    let mut t: f64;
    let w: f64;
    let mut hx: u32 = (ui >> 32) as u32 & 0x7fffffff;

    if hx >= 0x7ff00000 {
        /* cbrt(NaN,INF) is itself */
        return x + x;
    }

    /*
     * Rough cbrt to 5 bits:
     *    cbrt(2**e*(1+m) ~= 2**(e/3)*(1+(e%3+m)/3)
     * where e is integral and >= 0, m is real and in [0, 1), and "/" and
     * "%" are integer division and modulus with rounding towards minus
     * infinity.  The RHS is always >= the LHS and has a maximum relative
     * error of about 1 in 16.  Adding a bias of -0.03306235651 to the
     * (e%3+m)/3 term reduces the error to about 1 in 32. With the IEEE
     * floating point representation, for finite positive normal values,
     * ordinary integer divison of the value in bits magically gives
     * almost exactly the RHS of the above provided we first subtract the
     * exponent bias (1023 for doubles) and later add it back.  We do the
     * subtraction virtually to keep e >= 0 so that ordinary integer
     * division rounds towards minus infinity; this is also efficient.
     */
    if hx < 0x00100000 {
        /* zero or subnormal? */
        ui = (x * x1p54).to_bits();
        hx = (ui >> 32) as u32 & 0x7fffffff;
        if hx == 0 {
            return x; /* cbrt(0) is itself */
        }
        hx = hx / 3 + B2;
    } else {
        hx = hx / 3 + B1;
    }
    ui &= 1 << 63;
    ui |= (hx as u64) << 32;
    t = f64::from_bits(ui);

    /*
     * New cbrt to 23 bits:
     *    cbrt(x) = t*cbrt(x/t**3) ~= t*P(t**3/x)
     * where P(r) is a polynomial of degree 4 that approximates 1/cbrt(r)
     * to within 2**-23.5 when |r - 1| < 1/10.  The rough approximation
     * has produced t such than |t/cbrt(x) - 1| ~< 1/32, and cubing this
     * gives us bounds for r = t**3/x.
     *
     * Try to optimize for parallel evaluation as in __tanf.c.
     */
    r = (t * t) * (t / x);
    t = t * ((P0 + r * (P1 + r * P2)) + ((r * r) * r) * (P3 + r * P4));

    /*
     * Round t away from zero to 23 bits (sloppily except for ensuring that
     * the result is larger in magnitude than cbrt(x) but not much more than
     * 2 23-bit ulps larger).  With rounding towards zero, the error bound
     * would be ~5/6 instead of ~4/6.  With a maximum error of 2 23-bit ulps
     * in the rounded t, the infinite-precision error in the Newton
     * approximation barely affects third digit in the final error
     * 0.667; the error in the rounded t can be up to about 3 23-bit ulps
     * before the final error is larger than 0.667 ulps.
     */
    ui = t.to_bits();
    ui = (ui + 0x80000000) & 0xffffffffc0000000;
    t = f64::from_bits(ui);

    /* one step Newton iteration to 53 bits with error < 0.667 ulps */
    s = t * t; /* t*t is exact */
    r = x / s; /* error <= 0.5 ulps; |r| < |t| */
    w = t + t; /* t+t is exact */
    r = (r - t) / (w + r); /* r-t is exact; w+r ~= 3*t */
    t = t + t * r; /* error <= 0.5 + 0.5/3 + epsilon */
    t
}
