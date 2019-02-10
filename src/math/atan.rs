/* origin: FreeBSD /usr/src/lib/msun/src/s_atan.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* atan(x)
 * Method
 *   1. Reduce x to positive by atan(x) = -atan(-x).
 *   2. According to the integer k=4t+0.25 chopped, t=x, the argument
 *      is further reduced to one of the following intervals and the
 *      arctangent of t is evaluated by the corresponding formula:
 *
 *      [0,7/16]      atan(x) = t-t^3*(a1+t^2*(a2+...(a10+t^2*a11)...)
 *      [7/16,11/16]  atan(x) = atan(1/2) + atan( (t-0.5)/(1+t/2) )
 *      [11/16.19/16] atan(x) = atan( 1 ) + atan( (t-1)/(1+t) )
 *      [19/16,39/16] atan(x) = atan(3/2) + atan( (t-1.5)/(1+1.5t) )
 *      [39/16,INF]   atan(x) = atan(INF) + atan( -1/t )
 *
 * Constants:
 * The hexadecimal values are the intended ones for the following
 * constants. The decimal values may be used, provided that the
 * compiler will convert from decimal to binary accurately enough
 * to produce the hexadecimal values shown.
 */

use super::fabs;
use core::f64;
use math::consts::*;

const ATANHI: [f64; 4] = [
    4.636_476_090_008_060_935_15_e-01, /* atan(0.5)hi 0x_3FDD_AC67, 0x_0561_BB4F */
    7.853_981_633_974_482_789_99_e-01, /* atan(1.0)hi 0x_3FE9_21FB, 0x_5444_2D18 */
    9.827_937_232_473_290_540_82_e-01, /* atan(1.5)hi 0x_3FEF_730B, 0x_D281_F69B */
    1.570_796_326_794_896_558,         /* atan(inf)hi 0x_3FF9_21FB, 0x_5444_2D18 */
];

const ATANLO: [f64; 4] = [
    2.269_877_745_296_168_709_24_e-17, /* atan(0.5)lo 0x_3C7A_2B7F, 0x_222F_65E2 */
    3.061_616_997_868_383_017_93_e-17, /* atan(1.0)lo 0x_3C81_A626, 0x_3314_5C07 */
    1.390_331_103_123_099_845_16_e-17, /* atan(1.5)lo 0x_3C70_0788, 0x_7AF0_CBBD */
    6.123_233_995_736_766_035_87_e-17, /* atan(inf)lo 0x_3C91_A626, 0x_3314_5C07 */
];

const AT: [f64; 11] = [
    3.333_333_333_333_293_180_27_e-01,  /* 0x_3FD5_5555, 0x_5555_550D */
    -1.999_999_999_987_648_324_76_e-01, /* 0x_BFC9_9999, 0x_9998_EBC4 */
    1.428_571_427_250_346_637_11_e-01,  /* 0x_3FC2_4924, 0x_9200_83FF */
    -1.111_111_040_546_235_578_8_e-01,  /* 0x_BFBC_71C6, 0x_FE23_1671 */
    9.090_887_133_436_506_561_96_e-02,  /* 0x_3FB7_45CD, 0x_C54C_206E */
    -7.691_876_205_044_829_994_95_e-02, /* 0x_BFB3_B0F2, 0x_AF74_9A6D */
    6.661_073_137_387_531_206_69_e-02,  /* 0x_3FB1_0D66, 0x_A0D0_3D51 */
    -5.833_570_133_790_573_486_45_e-02, /* 0x_BFAD_DE2D, 0x_52DE_FD9A */
    4.976_877_994_615_932_360_17_e-02,  /* 0x_3FA9_7B4B, 0x_2476_0DEB */
    -3.653_157_274_421_691_552_7_e-02,  /* 0x_BFA2_B444, 0x_2C6A_6C2F */
    1.628_582_011_536_578_236_23_e-02,  /* 0x_3F90_AD3A, 0x_E322_DA11 */
];

#[inline]
pub fn atan(x: f64) -> f64 {
    let mut x = x;
    let mut ix = (x.to_bits() >> 32) as u32;
    let sign = ix >> 31;
    ix &= UF_ABS;
    if ix >= 0x_4410_0000 {
        if x.is_nan() {
            return x;
        }

        let z = ATANHI[3] + f64::from_bits(0x_0380_0000); // 0x1p-120f
        return if sign != 0 { -z } else { z };
    }

    let id = if ix < 0x_3fdc_0000 {
        /* |x| < 0.4375 */
        if ix < 0x_3e40_0000 {
            /* |x| < 2^-27 */
            if ix < 0x_0010_0000 {
                /* raise underflow for subnormal x */
                force_eval!(x as f32);
            }

            return x;
        }

        -1
    } else {
        x = fabs(x);
        if ix < 0x_3ff3_0000 {
            /* |x| < 1.1875 */
            if ix < 0x_3fe6_0000 {
                /* 7/16 <= |x| < 11/16 */
                x = (2. * x - 1.) / (2. + x);
                0
            } else {
                /* 11/16 <= |x| < 19/16 */
                x = (x - 1.) / (x + 1.);
                1
            }
        } else if ix < 0x_4003_8000 {
            /* |x| < 2.4375 */
            x = (x - 1.5) / (1. + 1.5 * x);
            2
        } else {
            /* 2.4375 <= |x| < 2^66 */
            x = -1. / x;
            3
        }
    };

    let z = x * x;
    let w = z * z;
    /* break sum from i=0 to 10 AT[i]z**(i+1) into odd and even poly */
    let s1 = z * (AT[0] + w * (AT[2] + w * (AT[4] + w * (AT[6] + w * (AT[8] + w * AT[10])))));
    let s2 = w * (AT[1] + w * (AT[3] + w * (AT[5] + w * (AT[7] + w * AT[9]))));

    if id < 0 {
        return x - x * (s1 + s2);
    }

    let z = i!(ATANHI, id as usize) - (x * (s1 + s2) - i!(ATANLO, id as usize) - x);

    if sign != 0 {
        -z
    } else {
        z
    }
}

#[cfg(test)]
mod tests {
    use super::atan;
    use core::f64;

    #[test]
    fn sanity_check() {
        for (input, answer) in [
            (3.0_f64.sqrt() / 3.0, f64::consts::FRAC_PI_6),
            (1.0, f64::consts::FRAC_PI_4),
            (3.0_f64.sqrt(), f64::consts::FRAC_PI_3),
            (-3.0_f64.sqrt() / 3.0, -f64::consts::FRAC_PI_6),
            (-1.0, -f64::consts::FRAC_PI_4),
            (-3.0_f64.sqrt(), -f64::consts::FRAC_PI_3),
        ]
        .iter()
        {
            assert!(
                (atan(*input) - answer) / answer < 1e-5,
                "\natan({:.4}/16) = {:.4}, actual: {}",
                input * 16.0,
                answer,
                atan(*input)
            );
        }
    }

    #[test]
    fn zero() {
        assert_eq!(atan(0.0), 0.0);
    }

    #[test]
    fn infinity() {
        assert_eq!(atan(f64::INFINITY), f64::consts::FRAC_PI_2);
    }

    #[test]
    fn minus_infinity() {
        assert_eq!(atan(f64::NEG_INFINITY), -f64::consts::FRAC_PI_2);
    }

    #[test]
    fn nan() {
        assert!(atan(f64::NAN).is_nan());
    }
}
