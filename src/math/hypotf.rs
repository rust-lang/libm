use core::f32;

use super::sqrtf;

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn hypotf(x: f32, y: f32) -> f32 {
    let mut ha = f32::to_bits(x) as i32;
    ha &= 0x7fffffff;

    let mut hb = f32::to_bits(y) as i32;
    hb &= 0x7fffffff;

    if hb > ha {
        let temp = ha;
        ha = hb;
        hb = temp;
    }

    let mut a = f32::from_bits(ha as u32); /* a <- |a| */
    let mut b = f32::from_bits(hb as u32); /* b <- |b| */
    if (ha - hb) > 0xf000000 {
        return a + b;
    } /* x/y > 2**30 */
    let mut k: u32 = 0;
    if ha > 0x58800000 {
        /* a>2**50 */
        if ha >= 0x7f800000 {
            /* Inf or NaN */
            let mut w = a + b; /* for sNaN */
            if ha == 0x7f800000 {
                w = a;
            }
            if hb == 0x7f800000 {
                w = b;
            }
            return w;
        }
        /* scale a and b by 2**-60 */
        ha -= 0x5d800000;
        hb -= 0x5d800000;
        k += 60;
        a = f32::from_bits(ha as u32);
        b = f32::from_bits(hb as u32);
    }
    if hb < 0x26800000 {
        /* b < 2**-50 */
        if hb <= 0x007fffff {
            /* subnormal b or 0 */
            if hb == 0 {
                return a;
            };
            let t1 = f32::from_bits(0x3f000000); /* t1=2^126 */
            b *= t1;
            a *= t1;
            k -= 126;
        } else {
            /* scale a and b by 2^60 */
            ha += 0x5d800000; /* a *= 2^60 */
            hb += 0x5d800000; /* b *= 2^60 */
            k -= 60;
            a = f32::from_bits(ha as u32);
            b = f32::from_bits(hb as u32);
        }
    }
    /* medium size a and b */
    let mut w = a - b;
    if w > b {
        let t1 = f32::from_bits((ha as u32) & 0xfffff000);
        let t2 = a - t1;
        w = sqrtf(t1 * t1 - (b * (-b) - t2 * (a + t1)));
    } else {
        a = a + a;
        let y1 = f32::from_bits((hb as u32) & 0xfffff000);
        let y2 = b - y1;
        let t1 = f32::from_bits((ha as u32) + 0x00800000);
        let t2 = a - t1;
        w = sqrtf(t1 * y1 - (w * (-w) - (t1 * y2 + t2 * b)));
    }
    if k != 0 {
        let t1 = f32::from_bits(0x3f800000 + (k << 23));
        return t1 * w;
    } else {
        return w;
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::*;

    #[test]
    fn sanity_check() {
        assert_eq!(hypotf(0.0, 0.0), 0.0);
        assert_eq!(hypotf(0.0, -10.0), 10.0);
        assert_eq!(hypotf(3.0, 4.0), 5.0);
        assert_eq!(hypotf(-3.0, 4.0), 5.0);
        assert_eq!(hypotf(4.0, 3.0), 5.0);
        assert_eq!(hypotf(9.0, 10.0), 13.453624);
        assert_eq!(hypotf(1.0, 1.0), sqrtf(2.0));
    }

    /// The spec: https://en.cppreference.com/w/c/numeric/math/hypot
    #[test]
    fn spec_tests() {
        assert!(hypotf(0.0, NAN).is_nan());
        assert!(hypotf(NAN, 0.0).is_nan());
        assert!(hypotf(NAN, NAN).is_nan());
        assert_eq!(hypotf(INFINITY, NAN), INFINITY);
        assert_eq!(hypotf(INFINITY, 0.0), INFINITY);
    }
}

