// origin: FreeBSD /usr/src/lib/msun/src/k_sin.c
//
// ====================================================
// Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
//
// Developed at SunSoft, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this
// software is freely granted, provided that this notice
// is preserved.
// ====================================================

const S1: f64 = -1.666_666_666_666_663_243_48_e-01; /* 0x_BFC5_5555, 0x_5555_5549 */
const S2: f64 = 8.333_333_333_322_489_461_24_e-03; /* 0x_3F81_1111, 0x_1110_F8A6 */
const S3: f64 = -1.984_126_982_985_794_931_34_e-04; /* 0x_BF2A_01A0, 0x_19C1_61D5 */
const S4: f64 = 2.755_731_370_707_006_767_89_e-06; /* 0x_3EC7_1DE3, 0x_57B1_FE7D */
const S5: f64 = -2.505_076_025_340_686_341_95_e-08; /* 0x_BE5A_E5E6, 0x_8A2B_9CEB */
const S6: f64 = 1.589_690_995_211_550_102_21_e-10; /* 0x_3DE5_D93A, 0x_5ACF_D57C */

// kernel sin function on ~[-pi/4, pi/4] (except on -0), pi/4 ~ 0.7854
// Input x is assumed to be bounded by ~pi/4 in magnitude.
// Input y is the tail of x.
// Input iy indicates whether y is 0. (if iy=0, y assume to be 0).
//
// Algorithm
//      1. Since sin(-x) = -sin(x), we need only to consider positive x.
//      2. Callers must return sin(-0) = -0 without calling here since our
//         odd polynomial is not evaluated in a way that preserves -0.
//         Callers may do the optimization sin(x) ~ x for tiny x.
//      3. sin(x) is approximated by a polynomial of degree 13 on
//         [0,pi/4]
//                               3            13
//              sin(x) ~ x + S1*x + ... + S6*x
//         where
//
//      |sin(x)         2     4     6     8     10     12  |     -58
//      |----- - (1+S1*x +S2*x +S3*x +S4*x +S5*x  +S6*x   )| <= 2
//      |  x                                               |
//
//      4. sin(x+y) = sin(x) + sin'(x')*y
//                  ~ sin(x) + (1-x*x/2)*y
//         For better accuracy, let
//                   3      2      2      2      2
//              r = x *(S2+x *(S3+x *(S4+x *(S5+x *S6))))
//         then                   3    2
//              sin(x) = x + (S1*x + (x *(r-y/2)+y))
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_sin(x: f64, y: f64, iy: i32) -> f64 {
    let z = x * x;
    let w = z * z;
    let r = S2 + z * (S3 + z * S4) + z * w * (S5 + z * S6);
    let v = z * x;
    if iy == 0 {
        x + v * (S1 + z * r)
    } else {
        x - ((z * (0.5 * y - v * r) - y) - v * S1)
    }
}
