/* origin: FreeBSD /usr/src/lib/msun/src/e_atan2.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 *
 */
/* atan2(y,x)
 * Method :
 *      1. Reduce y to positive by atan2(y,x)=-atan2(-y,x).
 *      2. Reduce x to positive by (if x and y are unexceptional):
 *              ARG (x+iy) = arctan(y/x)           ... if x > 0,
 *              ARG (x+iy) = pi - arctan[y/(-x)]   ... if x < 0,
 *
 * Special cases:
 *
 *      ATAN2((anything), NaN ) is NaN;
 *      ATAN2(NAN , (anything) ) is NaN;
 *      ATAN2(+-0, +(anything but NaN)) is +-0  ;
 *      ATAN2(+-0, -(anything but NaN)) is +-pi ;
 *      ATAN2(+-(anything but 0 and NaN), 0) is +-pi/2;
 *      ATAN2(+-(anything but INF and NaN), +INF) is +-0 ;
 *      ATAN2(+-(anything but INF and NaN), -INF) is +-pi;
 *      ATAN2(+-INF,+INF ) is +-pi/4 ;
 *      ATAN2(+-INF,-INF ) is +-3pi/4;
 *      ATAN2(+-INF, (anything but,0,NaN, and INF)) is +-pi/2;
 *
 * Constants:
 * The hexadecimal values are the intended ones for the following
 * constants. The decimal values may be used, provided that the
 * compiler will convert from decimal to binary accurately enough
 * to produce the hexadecimal values shown.
 */

use super::{atan, fabs};
use crate::math::consts::*;
use core::f64::{
    self,
    consts::{FRAC_PI_2, FRAC_PI_4, PI}, /* 0x_4009_21FB, 0x_5444_2D18 */
};

const PI_LO: f64 = 1.224_646_799_147_353_177_2_e-16; /* 0x_3CA1_A626, 0x_3314_5C07 */

/// Arctangent of y/x (f64)
///
/// Computes the inverse tangent (arc tangent) of `y/x`.
/// Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
/// Returns a value in radians, in the range of -pi to pi.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn atan2(y: f64, x: f64) -> f64 {
    if x.is_nan() || y.is_nan() {
        return x + y;
    }
    let mut ix = (x.to_bits() >> 32) as u32;
    let lx = x.to_bits() as u32;
    let mut iy = (y.to_bits() >> 32) as u32;
    let ly = y.to_bits() as u32;
    if ((ix.wrapping_sub(0x_3ff0_0000)) | lx) == 0 {
        /* x = 1. */
        return atan(y);
    }
    let m = ((iy >> 31) & 1) | ((ix >> 30) & 2); /* 2*sign(x)+sign(y) */
    ix &= UF_ABS;
    iy &= UF_ABS;

    /* when y = 0 */
    if (iy | ly) == 0 {
        return match m {
            0 | 1 => y, /* atan(+-0,+anything)=+-0 */
            2 => PI,    /* atan(+0,-anything) = PI */
            _ => -PI,   /* atan(-0,-anything) =-PI */
        };
    }
    /* when x = 0 */
    if (ix | lx) == 0 {
        return if m & 1 != 0 { -FRAC_PI_2 } else { FRAC_PI_2 };
    }
    /* when x is INF */
    if ix == 0x_7ff0_0000 {
        if iy == 0x_7ff0_0000 {
            return match m {
                0 => FRAC_PI_4,       /* atan(+INF,+INF) */
                1 => -FRAC_PI_4,      /* atan(-INF,+INF) */
                2 => 3. * FRAC_PI_4,  /* atan(+INF,-INF) */
                _ => -3. * FRAC_PI_4, /* atan(-INF,-INF) */
            };
        } else {
            return match m {
                0 => 0.,  /* atan(+...,+INF) */
                1 => -0., /* atan(-...,+INF) */
                2 => PI,  /* atan(+...,-INF) */
                _ => -PI, /* atan(-...,-INF) */
            };
        }
    }
    /* |y/x| > 0x1p64 */
    if ix.wrapping_add(64 << 20) < iy || iy == 0x_7ff0_0000 {
        return if m & 1 != 0 { -FRAC_PI_2 } else { FRAC_PI_2 };
    }

    /* z = atan(|y/x|) without spurious underflow */
    let z = if (m & 2 != 0) && iy.wrapping_add(64 << 20) < ix {
        /* |y/x| < 0x1p-64, x<0 */
        0.
    } else {
        atan(fabs(y / x))
    };
    match m {
        0 => z,                /* atan(+,+) */
        1 => -z,               /* atan(-,+) */
        2 => PI - (z - PI_LO), /* atan(+,-) */
        _ => (z - PI_LO) - PI, /* atan(-,-) */
    }
}

#[test]
fn sanity_check() {
    assert_eq!(atan2(0., 1.), 0.);
    assert_eq!(atan2(0., -1.), PI);
    assert_eq!(atan2(-0., -1.), -PI);
    assert_eq!(atan2(3., 2.), atan(3. / 2.));
    assert_eq!(atan2(2., -1.), atan(2. / -1.) + PI);
    assert_eq!(atan2(-2., -1.), atan(-2. / -1.) - PI);
}
