/* origin: FreeBSD /usr/src/lib/msun/src/k_tan.c */

/* |tan(x)/x - t(x)| < 2**-25.5 (~[-2e-08, 2e-08]). */
const T: [f64; 6] = [
    0.333331395030791399758,   /* 0x15554d3418c99f.0p-54 */
    0.133392002712976742718,   /* 0x1112fd38999f72.0p-55 */
    0.0533812378445670393523,  /* 0x1b54c91d865afe.0p-57 */
    0.0245283181166547278873,  /* 0x191df3908c33ce.0p-58 */
    0.00297435743359967304927, /* 0x185dadfcecf44e.0p-61 */
    0.00946564784943673166728, /* 0x1362b9bf971bcd.0p-59 */
];

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_tanf(x: f64, odd: bool) -> f32 {
    let z = x * x;
    let mut r = T[4] + z * T[5];
    let t = T[2] + z * T[3];
    let w = z * z;
    let s = z * x;
    let u = T[0] + z * T[1];
    r = (x + s * u) + (s * w) * (t + w * r);
    (if odd { -1. / r } else { r }) as f32
}
