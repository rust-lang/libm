/* @(#)z_numtestf.c 1.0 98/08/13 */
/******************************************************************
 * Numtest
 *
 * Input:
 *   x - pointer to a floating point value
 *
 * Output:
 *   An integer that indicates what kind of number was passed in:
 *     NUM = 3 - a finite value
 *     NAN = 2 - not a number
 *     INF = 1 - an infinite value
 *           0 - zero
 *
 * Description:
 *   This routine returns an integer that indicates the character-
 *   istics of the number that was passed in.
 *
 *****************************************************************/
use super::NumState;
use math::consts::*;

#[inline]
pub fn numtestf(x: f32) -> NumState {
    let wx = x.to_bits() as i32;
    let exp = (wx & IF_INF) >> 23;

    /* Check for a zero input. */
    if x == 0. {
        NumState::Zero
    }
    /* Check for not a number or infinity. */
    else if exp == 0xff {
        if (wx & 0x_007f_ffff) != 0 {
            NumState::Nan
        } else {
            NumState::Inf
        }
    }
    /* Otherwise it's a finite value. */
    else {
        NumState::Num
    }
}
