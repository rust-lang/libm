// origin: FreeBSD /usr/src/lib/msun/src/s_sin.c */
use super::{k_cos, k_sin, rem_pio2};

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sin(x: f64) -> f64 {
    let x1p120 = f64::from_bits(0x4770000000000000); // 0x1p120f === 2 ^ 120

    let ix = (f64::to_bits(x) >> 32) as u32 & 0x7fffffff;

    if ix <= 0x3fe921fb {
        if ix < 0x3e500000 {
            /* |x| < 2**-26 */
            if ix < 0x00100000 {
                force_eval!(x / x1p120);
            } else {
                force_eval!(x + x1p120);
            }
            return x;
        }
        return k_sin(x, 0.0, 0);
    }

    if ix >= 0x7ff00000 {
        return x - x;
    }

    let (n, y0, y1) = rem_pio2(x);
    match n & 3 {
        0 => k_sin(y0, y1, 1),
        1 => k_cos(y0, y1),
        2 => -k_sin(y0, y1, 1),
        _ => -k_cos(y0, y1),
    }
}

#[test]
fn test_near_pi() {
    let x = f64::from_bits(0x400921fb000FD5DD); // 3.141592026217707
    let sx = f64::from_bits(0x3ea50d15ced1a4a2); // 6.273720864039205e-7
    assert_eq!(sin(x), sx);
}
