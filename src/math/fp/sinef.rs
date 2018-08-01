/******************************************************************
 * The following routines are coded directly from the algorithms
 * and coefficients given in "Software Manual for the Elementary
 * Functions" by William J. Cody, Jr. and William Waite, Prentice
 * Hall, 1980.
 ******************************************************************/

use math::fabsf;

use super::consts::*;
use super::{numtestf, NumState};

const HALF_PI: f32 = 1.570796326;
const ONE_OVER_PI: f32 = 0.318309886;
const R: [f32; 4] = [
    -0.1666665668,
    0.8333025139e-02,
    -0.1980741872e-03,
    0.2601903036e-5,
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
    const YMAX: f32 = 210828714.0;

    match numtestf(x) {
        NumState::Nan => {
            //errno = EDOM;
            return x;
        }
        NumState::Inf => {
            //errno = EDOM;
            return f32::from_bits(Z_INFINITY_F);
        }
        _ => {}
    }
    let mut sgn: i32;
    /* Use sin and cos properties to ease computations. */
    let mut y = if cosine {
        sgn = 1;
        fabsf(x) + HALF_PI
    } else {
        if x < 0. {
            sgn = -1;
            -x
        } else {
            sgn = 1;
            x
        }
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
    let mut res: f32;
    if (-Z_ROOTEPS_F < y) && (y < Z_ROOTEPS_F) {
        res = y;
    } else {
        let g = y * y;

        /* Calculate the Taylor series. */
        let r = (((R[3] * g + R[2]) * g + R[1]) * g + R[0]) * g;

        /* Finally, compute the result. */
        res = y + y * r;
    }

    res *= sgn as f32;

    res
}
