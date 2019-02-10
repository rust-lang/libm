/* origin: FreeBSD /usr/src/lib/msun/src/k_cosf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Debugged and optimized by Bruce D. Evans.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

/* |cos(x) - c(x)| < 2**-34.1 (~[-5.37e-11, 5.295e-11]). */
const C0: f64 = -0.499_999_997_251_031_003_120; /* -0x_1fff_fffd0c5e81.0p-54 */
const C1: f64 = 0.041_666_623_323_739_063_189_4; /*  0x_1555_53e1053a42.0p-57 */
const C2: f64 = -0.001_388_676_377_460_992_946_92; /* -0x_16c0_87e80f1e27.0p-62 */
const C3: f64 = 0.000_024_390_448_796_277_409_065_4; /*  0x_1993_42e0ee5069.0p-68 */

#[inline]
pub fn k_cosf(x: f64) -> f32 {
    let z = x * x;
    let w = z * z;
    let r = C2 + z * C3;
    (((1.0 + z * C0) + w * C1) + (w * z) * r) as f32
}
