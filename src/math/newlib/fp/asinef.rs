/******************************************************************
 * The following routines are coded directly from the algorithms
 * and coefficients given in "Software Manual for the Elementary
 * Functions" by William J. Cody, Jr. and William Waite, Prentice
 * Hall, 1980.
 ******************************************************************/

use core::f32;

use crate::math::fabsf;
use crate::math::sqrt;

use super::consts::*;
use super::{numtestf, NumState};

const P: [f32; 2] = [0.933_935_835, -0.504_400_557];
const Q: [f32; 2] = [0.560_363_004_e+1, -0.554_846_723_e+1];
const A: [f32; 2] = [0., f32::consts::FRAC_PI_4];
const B: [f32; 2] = [f32::consts::FRAC_PI_2, f32::consts::FRAC_PI_4];

/// Arcsine
///
///  Input:
///    x - floating point value
///    acosine - indicates acos calculation
///
///  Output:
///    Arcsine of x.
///
///  Description:
///    This routine calculates arcsine / arccosine.
#[inline]
pub fn asinef(x: f32, acosine: bool) -> f32 {
    let mut branch = 0;
    let mut res: f32 = 0.; // fix possibly uninitialized
    let mut g: f32 = 0.; // fix possibly uninitialized
                         /* Check for special values. */
    match numtestf(x) {
        //errno = EDOM;
        NumState::Nan => {
            return x;
        }
        NumState::Inf => {
            return f32::INFINITY;
        }
        _ => {}
    };
    let mut y = fabsf(x);
    let flag = acosine;
    let i;
    if y > 0.5 {
        i = !flag;

        /* Check for range error. */
        if y > 1. {
            //errno = ERANGE;
            return f32::from_bits(Z_NOTANUM_F);
        }

        g = (1. - y) / 2.;
        y = (-2. * sqrt(g as f64)) as f32;
        branch = 1;
    } else {
        i = flag;
        if y < Z_ROOTEPS_F {
            res = y;
        } else {
            g = y * y;
        }
    }
    let i = i as usize;

    if (y >= Z_ROOTEPS_F) || (branch == 1) {
        /* Calculate the Taylor series. */
        let p = (P[1] * g + P[0]) * g;
        let q = (g + Q[1]) * g + Q[0];
        let r = p / q;

        res = y + y * r;
    }

    /* Calculate asine or acose. */
    if !flag {
        res = (A[i] + res) + A[i];
        if x < 0. {
            res = -res;
        }
    } else if x < 0. {
        res = (B[i] + res) + B[i];
    } else {
        res = (A[i] - res) + A[i];
    }

    res
}
