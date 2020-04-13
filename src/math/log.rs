/* origin: FreeBSD /usr/src/lib/msun/src/e_log.c */

const LN2_HI: f64 = 6.93147180369123816490e-01; /* 3fe62e42 fee00000 */
const LN2_LO: f64 = 1.90821492927058770002e-10; /* 3dea39ef 35793c76 */
const LG1: f64 = 6.666666666666735130e-01; /* 3FE55555 55555593 */
const LG2: f64 = 3.999999999940941908e-01; /* 3FD99999 9997FA04 */
const LG3: f64 = 2.857142874366239149e-01; /* 3FD24924 94229359 */
const LG4: f64 = 2.222219843214978396e-01; /* 3FCC71C5 1D8E78AF */
const LG5: f64 = 1.818357216161805012e-01; /* 3FC74664 96CB03DE */
const LG6: f64 = 1.531383769920937332e-01; /* 3FC39A09 D078C69F */
const LG7: f64 = 1.479819860511658591e-01; /* 3FC2F112 DF3E5244 */

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log(mut x: f64) -> f64 {
    let x1p54 = f64::from_bits(0x4350000000000000); // 0x1p54 === 2 ^ 54

    let mut ui = x.to_bits();
    let mut hx: u32 = (ui >> 32) as u32;
    let mut k: i32 = 0;

    if (hx < 0x00100000) || ((hx >> 31) != 0) {
        /* x < 2**-126  */
        if ui << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if hx >> 31 != 0 {
            return (x - x) / 0.0; /* log(-#) = NaN */
        }
        /* subnormal number, scale x up */
        k -= 54;
        x *= x1p54;
        ui = x.to_bits();
        hx = (ui >> 32) as u32;
    } else if hx >= 0x7ff00000 {
        return x;
    } else if hx == 0x3ff00000 && ui << 32 == 0 {
        return 0.;
    }

    hx += 0x3ff00000 - 0x3fe6a09e;
    k += ((hx >> 20) as i32) - 0x3ff;
    hx = (hx & 0x000fffff) + 0x3fe6a09e;
    ui = ((hx as u64) << 32) | (ui & 0xffffffff);
    x = f64::from_bits(ui);

    let f: f64 = x - 1.0;
    let hfsq: f64 = 0.5 * f * f;
    let s: f64 = f / (2.0 + f);
    let z: f64 = s * s;
    let w: f64 = z * z;
    let t1: f64 = w * (LG2 + w * (LG4 + w * LG6));
    let t2: f64 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r: f64 = t2 + t1;
    let dk: f64 = k as f64;
    s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI
}
