/******************************************************************
 * The following routines are coded directly from the algorithms
 * and coefficients given in "Software Manual for the Elementary
 * Functions" by William J. Cody, Jr. and William Waite, Prentice
 * Hall, 1980.
 ******************************************************************/

use core::f32;

use math::fabsf;

use super::consts::*;
use super::{numtestf, NumState};

const HALF_PI: f32 = 1.570_796_326;
const ONE_OVER_PI: f32 = 0.318_309_886;
const R: [f32; 4] = [
    -0.166_666_566_8,
    0.833_302_513_9_e-02,
    -0.198_074_187_2_e-03,
    0.260_190_303_6_e-5,
];

///  sine generator
///
///  Input:
///    x - floating point value
///    cosine - indicates cosine value
///
///  Output:
///    Sine of x.
///
///  Description:
///    This routine calculates sines and cosines.
#[inline]
pub fn sinef(x: f32, cosine: bool) -> f32 {
    const YMAX: f32 = 210_828_714.;

    match numtestf(x) {
        NumState::Nan => {
            //errno = EDOM;
            return x;
        }
        NumState::Inf => {
            //errno = EDOM;
            return f32::INFINITY;
        }
        _ => {}
    }
    let mut sgn: i32;
    /* Use sin and cos properties to ease computations. */
    let mut y = if cosine {
        sgn = 1;
        fabsf(x) + HALF_PI
    } else if x < 0. {
        sgn = -1;
        -x
    } else {
        sgn = 1;
        x
    };

    /* Check for values of y that will overflow here. */
    if y > YMAX {
        //errno = ERANGE;
        return x;
    }

    /* Calculate the exponent. */
    let n = if y < 0. {
        (y * ONE_OVER_PI - 0.5) as i32
    } else {
        (y * ONE_OVER_PI + 0.5) as i32
    };
    let mut xn = n as f32;

    if (n & 1) != 0 {
        sgn = -sgn;
    }

    if cosine {
        xn -= 0.5;
    }

    y = fabsf(x) - xn * PI;
    let res = if (-Z_ROOTEPS_F < y) && (y < Z_ROOTEPS_F) {
        y
    } else {
        let g = y * y;

        /* Calculate the Taylor series. */
        let r = (((R[3] * g + R[2]) * g + R[1]) * g + R[0]) * g;

        /* Finally, compute the result. */
        y + y * r
    };

    res * (sgn as f32)
}
