/* origin: FreeBSD /usr/src/lib/msun/src/k_tan.c */
/*
 * ====================================================
 * Copyright 2004 Sun Microsystems, Inc.  All Rights Reserved.
 *
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

/* |tan(x)/x - t(x)| < 2**-25.5 (~[-2e-08, 2e-08]). */
const T: [f64; 6] = [
    0.333_331_395_030_791_399_758,    /* 0x_1555_4d3418c99f.0p-54 */
    0.133_392_002_712_976_742_718,    /* 0x_1112_fd38999f72.0p-55 */
    0.053_381_237_844_567_039_352_3,  /* 0x_1b54_c91d865afe.0p-57 */
    0.024_528_318_116_654_727_887_3,  /* 0x_191d_f3908c33ce.0p-58 */
    0.002_974_357_433_599_673_049_27, /* 0x_185d_adfcecf44e.0p-61 */
    0.009_465_647_849_436_731_667_28, /* 0x_1362_b9bf971bcd.0p-59 */
];

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_tanf(x: f64, odd: bool) -> f32 {
    let z = x * x;
    /*
     * Split up the polynomial into small independent terms to give
     * opportunities for parallel evaluation.  The chosen splitting is
     * micro-optimized for Athlons (XP, X64).  It costs 2 multiplications
     * relative to Horner's method on sequential machines.
     *
     * We add the small terms from lowest degree up for efficiency on
     * non-sequential machines (the lowest degree terms tend to be ready
     * earlier).  Apart from this, we don't care about order of
     * operations, and don't need to to care since we have precision to
     * spare.  However, the chosen splitting is good for accuracy too,
     * and would give results as accurate as Horner's method if the
     * small terms were added from highest degree down.
     */
    let mut r = T[4] + z * T[5];
    let t = T[2] + z * T[3];
    let w = z * z;
    let s = z * x;
    let u = T[0] + z * T[1];
    r = (x + s * u) + (s * w) * (t + w * r);
    (if odd { -1. / r } else { r }) as f32
}
