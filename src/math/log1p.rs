/* origin: FreeBSD /usr/src/lib/msun/src/s_log1p.c */

use core::f64;

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
pub fn log1p(x: f64) -> f64 {
    let mut ui: u64 = x.to_bits();
    let hfsq: f64;
    let mut f: f64 = 0.;
    let mut c: f64 = 0.;
    let s: f64;
    let z: f64;
    let r: f64;
    let w: f64;
    let t1: f64;
    let t2: f64;
    let dk: f64;
    let hx: u32;
    let mut hu: u32;
    let mut k: i32;

    hx = (ui >> 32) as u32;
    k = 1;
    if hx < 0x3fda827a || (hx >> 31) > 0 {
        /* 1+x < sqrt(2)+ */
        if hx >= 0xbff00000 {
            /* x <= -1.0 */
            if x == -1. {
                return x / 0.0; /* log1p(-1) = -inf */
            }
            return (x - x) / 0.0; /* log1p(x<-1) = NaN */
        }
        if hx << 1 < 0x3ca00000 << 1 {
            /* |x| < 2**-53 */
            /* underflow if subnormal */
            if (hx & 0x7ff00000) == 0 {
                force_eval!(x as f32);
            }
            return x;
        }
        if hx <= 0xbfd2bec4 {
            /* sqrt(2)/2- <= 1+x < sqrt(2)+ */
            k = 0;
            c = 0.;
            f = x;
        }
    } else if hx >= 0x7ff00000 {
        return x;
    }
    if k > 0 {
        ui = (1. + x).to_bits();
        hu = (ui >> 32) as u32;
        hu += 0x3ff00000 - 0x3fe6a09e;
        k = (hu >> 20) as i32 - 0x3ff;
        if k < 54 {
            c = if k >= 2 {
                1. - (f64::from_bits(ui) - x)
            } else {
                x - (f64::from_bits(ui) - 1.)
            };
            c /= f64::from_bits(ui);
        } else {
            c = 0.;
        }
        hu = (hu & 0x000fffff) + 0x3fe6a09e;
        ui = (hu as u64) << 32 | (ui & 0xffffffff);
        f = f64::from_bits(ui) - 1.;
    }
    hfsq = 0.5 * f * f;
    s = f / (2.0 + f);
    z = s * s;
    w = z * z;
    t1 = w * (LG2 + w * (LG4 + w * LG6));
    t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    r = t2 + t1;
    dk = k as f64;
    s * (hfsq + r) + (dk * LN2_LO + c) - hfsq + f + dk * LN2_HI
}
