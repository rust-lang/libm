// origin: FreeBSD /usr/src/lib/msun/src/s_tan.c */
use super::{k_tan, rem_pio2};

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn tan(x: f64) -> f64 {
    let x1p120 = f32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let ix = (f64::to_bits(x) >> 32) as u32 & 0x7fffffff;
    /* |x| ~< pi/4 */
    if ix <= 0x3fe921fb {
        if ix < 0x3e400000 {
            /* |x| < 2**-27 */
            force_eval!(if ix < 0x00100000 {
                x / x1p120 as f64
            } else {
                x + x1p120 as f64
            });
            return x;
        }
        return k_tan(x, 0.0, 0);
    }

    /* tan(Inf or NaN) is NaN */
    if ix >= 0x7ff00000 {
        return x - x;
    }

    let (n, y0, y1) = rem_pio2(x);
    k_tan(y0, y1, n & 1)
}
