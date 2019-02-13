/* origin: FreeBSD /usr/src/lib/msun/src/e_lgamma_r.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 *
 */
/* lgamma_r(x, signgamp)
 * Reentrant version of the logarithm of the Gamma function
 * with user provide pointer for the sign of Gamma(x).
 *
 * Method:
 *   1. Argument Reduction for 0 < x <= 8
 *      Since gamma(1+s)=s*gamma(s), for x in [0,8], we may
 *      reduce x to a number in [1.5,2.5] by
 *              lgamma(1+s) = log(s) + lgamma(s)
 *      for example,
 *              lgamma(7.3) = log(6.3) + lgamma(6.3)
 *                          = log(6.3*5.3) + lgamma(5.3)
 *                          = log(6.3*5.3*4.3*3.3*2.3) + lgamma(2.3)
 *   2. Polynomial approximation of lgamma around its
 *      minimun ymin=1.461_632_144_968_362_245 to maintain monotonicity.
 *      On [ymin-0.23, ymin+0.27] (i.e., [1.23164,1.73163]), use
 *              Let z = x-ymin;
 *              lgamma(x) = -1.214_862_905_358_496_078218 + z^2*poly(z)
 *      where
 *              poly(z) is a 14 degree polynomial.
 *   2. Rational approximation in the primary interval [2,3]
 *      We use the following approximation:
 *              s = x-2.0;
 *              lgamma(x) = 0.5*s + s*P(s)/Q(s)
 *      with accuracy
 *              |P/Q - (lgamma(x)-0.5s)| < 2**-61.71
 *      Our algorithms are based on the following observation
 *
 *                             zeta(2)-1    2    zeta(3)-1    3
 * lgamma(2+s) = s*(1-Euler) + --------- * s  -  --------- * s  + ...
 *                                 2                 3
 *
 *      where Euler = 0.5771... is the Euler constant, which is very
 *      close to 0.5.
 *
 *   3. For x>=8, we have
 *      lgamma(x)~(x-0.5)log(x)-x+0.5*log(2pi)+1/(12x)-1/(360x**3)+....
 *      (better formula:
 *         lgamma(x)~(x-0.5)*(log(x)-1)-.5*(log(2pi)-1) + ...)
 *      Let z = 1/x, then we approximation
 *              f(z) = lgamma(x) - (x-0.5)(log(x)-1)
 *      by
 *                                  3       5             11
 *              w = w0 + w1*z + w2*z  + w3*z  + ... + w6*z
 *      where
 *              |w - f(z)| < 2**-58.74
 *
 *   4. For negative x, since (G is gamma function)
 *              -x*G(-x)*G(x) = PI/sin(PI*x),
 *      we have
 *              G(x) = PI/(sin(PI*x)*(-x)*G(-x))
 *      since G(-x) is positive, sign(G(x)) = sign(sin(PI*x)) for x<0
 *      Hence, for x<0, signgam = sign(sin(PI*x)) and
 *              lgamma(x) = log(|Gamma(x)|)
 *                        = log(PI/(|x*sin(PI*x)|)) - lgamma(-x);
 *      Note: one should avoid compute PI*(-x) directly in the
 *            computation of sin(PI*(-x)).
 *
 *   5. Special Cases
 *              lgamma(2+s) ~ s*(1-Euler) for tiny s
 *              lgamma(1) = lgamma(2) = 0
 *              lgamma(x) ~ -log(|x|) for tiny x
 *              lgamma(0) = lgamma(neg.integer) = inf and raise divide-by-zero
 *              lgamma(inf) = inf
 *              lgamma(-inf) = inf (bug for bug compatible with C99!?)
 *
 */

use super::{floor, k_cos, k_sin, log};
use crate::math::consts::*;
use core::f64;

const PI: f64 = f64::consts::PI; /* 0x_4009_21FB, 0x_5444_2D18 */
const A0: f64 = 7.721_566_490_153_286_554_94_e-02; /* 0x_3FB3_C467, 0x_E37D_B0C8 */
const A1: f64 = 3.224_670_334_241_135_916_11_e-01; /* 0x_3FD4_A34C, 0x_C4A6_0FAD */
const A2: f64 = 6.735_230_105_312_926_818_24_e-02; /* 0x_3FB1_3E00, 0x_1A55_62A7 */
const A3: f64 = 2.058_080_843_251_673_328_06_e-02; /* 0x_3F95_1322, 0x_AC92_547B */
const A4: f64 = 7.385_550_860_814_028_839_57_e-03; /* 0x_3F7E_404F, 0x_B68F_EFE8 */
const A5: f64 = 2.890_513_836_734_156_290_91_e-03; /* 0x_3F67_ADD8, 0x_CCB7_926B */
const A6: f64 = 1.192_707_631_833_620_678_45_e-03; /* 0x_3F53_8A94, 0x_116F_3F5D */
const A7: f64 = 5.100_697_921_535_113_366_08_e-04; /* 0x_3F40_B6C6, 0x_89B9_9C00 */
const A8: f64 = 2.208_627_907_139_083_855_57_e-04; /* 0x_3F2C_F2EC, 0x_ED10_E54D */
const A9: f64 = 1.080_115_672_475_839_399_54_e-04; /* 0x_3F1C_5088, 0x_987D_FB07 */
const A10: f64 = 2.521_445_654_512_573_269_39_e-05; /* 0x_3EFA_7074, 0x_428C_FA52 */
const A11: f64 = 4.486_409_496_189_151_601_50_e-05; /* 0x_3F07_858E, 0x_90A4_5837 */
const TC: f64 = 1.461_632_144_968_362_245_76; /* 0x_3FF7_62D8, 0x_6356_BE3F */
const TF: f64 = -1.214_862_905_358_496_114_61_e-01; /* 0x_BFBF_19B9, 0x_BCC3_8A42 */
/* tt = -(tail of TF) */
const TT: f64 = -3.638_676_997_039_505_365_41_e-18; /* 0x_BC50_C7CA, 0x_A48A_971F */
const T0: f64 = 4.838_361_227_238_100_470_42_e-01; /* 0x_3FDE_F72B, 0x_C8EE_38A2 */
const T1: f64 = -1.475_877_229_945_939_117_52_e-01; /* 0x_BFC2_E427, 0x_8DC6_C509 */
const T2: f64 = 6.462_494_023_913_338_547_78_e-02; /* 0x_3FB0_8B42, 0x_94D5_419B */
const T3: f64 = -3.278_854_107_598_596_495_65_e-02; /* 0x_BFA0_C9A8, 0x_DF35_B713 */
const T4: f64 = 1.797_067_508_118_203_871_26_e-02; /* 0x_3F92_66E7, 0x_970A_F9EC */
const T5: f64 = -1.031_422_412_983_414_374_50_e-02; /* 0x_BF85_1F9F, 0x_BA91_EC6A */
const T6: f64 = 6.100_538_702_462_913_326_35_e-03; /* 0x_3F78_FCE0, 0x_E370_E344 */
const T7: f64 = -3.684_520_167_811_382_567_60_e-03; /* 0x_BF6E_2EFF, 0x_B3E9_14D7 */
const T8: f64 = 2.259_647_809_006_124_722_50_e-03; /* 0x_3F62_82D3, 0x_2E15_C915 */
const T9: f64 = -1.403_464_699_892_328_438_13_e-03; /* 0x_BF56_FE8E, 0x_BF2D_1AF1 */
const T10: f64 = 8.810_818_824_376_540_113_82_e-04; /* 0x_3F4C_DF0C, 0x_EF61_A8E9 */
const T11: f64 = -5.385_953_053_567_405_467_15_e-04; /* 0x_BF41_A610, 0x_9C73_E0EC */
const T12: f64 = 3.156_320_709_036_259_503_61_e-04; /* 0x_3F34_AF6D, 0x_6C0E_BBF7 */
const T13: f64 = -3.127_541_683_751_208_605_18_e-04; /* 0x_BF34_7F24, 0x_ECC3_8C38 */
const T14: f64 = 3.355_291_926_355_190_735_43_e-04; /* 0x_3F35_FD3E, 0x_E8C2_D3F4 */
const U0: f64 = -7.721_566_490_153_286_554_94_e-02; /* 0x_BFB3_C467, 0x_E37D_B0C8 */
const U1: f64 = 6.328_270_640_250_933_665_17_e-01; /* 0x_3FE4_401E, 0x_8B00_5DFF */
const U2: f64 = 1.454_922_501_372_347_687_37; /* 0x_3FF7_475C, 0x_D119_BD6F */
const U3: f64 = 9.777_175_279_633_727_456_03_e-01; /* 0x_3FEF_4976, 0x_44EA_8450 */
const U4: f64 = 2.289_637_280_646_924_510_92_e-01; /* 0x_3FCD_4EAE, 0x_F601_0924 */
const U5: f64 = 1.338_109_185_367_876_603_77_e-02; /* 0x_3F8B_678B, 0x_BF2B_AB09 */
const V1: f64 = 2.455_977_937_130_411_348_22; /* 0x_4003_A5D7, 0x_C2BD_619C */
const V2: f64 = 2.128_489_763_798_933_953_61; /* 0x_4001_0725, 0x_A42B_18F5 */
const V3: f64 = 7.692_851_504_566_727_838_25_e-01; /* 0x_3FE8_9DFB, 0x_E450_50AF */
const V4: f64 = 1.042_226_455_933_691_342_54_e-01; /* 0x_3FBA_AE55, 0x_D653_7C88 */
const V5: f64 = 3.217_092_422_824_239_118_10_e-03; /* 0x_3F6A_5ABB, 0x_57D0_CF61 */
const S0: f64 = -7.721_566_490_153_286_554_94_e-02; /* 0x_BFB3_C467, 0x_E37D_B0C8 */
const S1: f64 = 2.149_824_159_606_088_525_01_e-01; /* 0x_3FCB_848B, 0x_36E2_0878 */
const S2: f64 = 3.257_787_964_089_309_817_87_e-01; /* 0x_3FD4_D98F, 0x_4F13_9F59 */
const S3: f64 = 1.463_504_726_524_644_528_05_e-01; /* 0x_3FC2_BB9C, 0x_BEE5_F2F7 */
const S4: f64 = 2.664_227_030_336_386_095_60_e-02; /* 0x_3F9B_481C, 0x_7E93_9961 */
const S5: f64 = 1.840_284_514_073_377_156_52_e-03; /* 0x_3F5E_26B6, 0x_7368_F239 */
const S6: f64 = 3.194_753_265_841_008_676_17_e-05; /* 0x_3F00_BFEC, 0x_DD17_E945 */
const R1: f64 = 1.392_005_334_676_210_459_58; /* 0x_3FF6_45A7, 0x_62C4_AB74 */
const R2: f64 = 7.219_355_475_671_380_695_25_e-01; /* 0x_3FE7_1A18, 0x_93D3_DCDC */
const R3: f64 = 1.719_338_656_328_030_789_93_e-01; /* 0x_3FC6_01ED, 0x_CCFB_DF27 */
const R4: f64 = 1.864_591_917_156_529_013_44_e-02; /* 0x_3F93_17EA, 0x_742E_D475 */
const R5: f64 = 7.779_424_963_818_935_964_34_e-04; /* 0x_3F49_7DDA, 0x_CA41_A95B */
const R6: f64 = 7.326_684_307_446_256_361_89_e-06; /* 0x_3EDE_BAF7, 0x_A5B3_8140 */
const W0: f64 = 4.189_385_332_046_727_250_52_e-01; /* 0x_3FDA_CFE3, 0x_90C9_7D69 */
const W1: f64 = 8.333_333_333_333_296_788_49_e-02; /* 0x_3FB5_5555, 0x_5555_553B */
const W2: f64 = -2.777_777_777_287_755_364_70_e-03; /* 0x_BF66_C16C, 0x_16B0_2E5C */
const W3: f64 = 7.936_505_586_430_195_585_00_e-04; /* 0x_3F4A_019F, 0x_98CF_38B6 */
const W4: f64 = -5.951_875_574_503_399_631_35_e-04; /* 0x_BF43_80CB, 0x_8C0F_E741 */
const W5: f64 = 8.363_399_189_962_821_391_26_e-04; /* 0x_3F4B_67BA, 0x_4CDA_D5D1 */
const W6: f64 = -1.630_929_340_965_752_739_89_e-03; /* 0x_BF5A_B89D, 0x_0B9E_43E4 */

/* sin(PI*x) assuming x > 2^-100, if sin(PI*x)==0 the sign is arbitrary */
fn sin_pi(mut x: f64) -> f64 {
    let mut n: isize;

    /* spurious inexact if odd int */
    x = 2. * (x * 0.5 - floor(x * 0.5)); /* x mod 2. */

    n = (x * 4.) as isize;
    n = (n + 1) / 2;
    x -= (n as f64) * 0.5;
    x *= PI;

    match n {
        1 => k_cos(x, 0.),
        2 => k_sin(-x, 0., 0),
        3 => -k_cos(x, 0.),
        0 | _ => k_sin(x, 0., 0),
    }
}

pub fn lgamma(x: f64) -> f64 {
    lgamma_r(x).0
}

pub fn lgamma_r(mut x: f64) -> (f64, isize) {
    let u: u64 = x.to_bits();
    let mut z: f64;
    let p: f64;
    let mut r: f64;

    /* purge off +-inf, NaN, +-0, tiny and negative arguments */
    let mut signgam = 1_isize;
    let sign = (u >> 63) != 0;
    let ix = ((u >> 32) as u32) & UF_ABS;
    if ix >= 0x_7ff0_0000 {
        return (x * x, signgam);
    }
    if ix < (0x3ff - 70) << 20 {
        /* |x|<2^-70, return -log(|x|) */
        if sign {
            x = -x;
            signgam = -1;
        }
        return (-log(x), signgam);
    }
    let nadj = if sign {
        x = -x;
        let mut t = sin_pi(x);
        if t == 0. {
            /* -integer */
            return (f64::INFINITY, signgam);
        }
        if t > 0. {
            signgam = -1;
        } else {
            t = -t;
        }
        log(PI / (t * x))
    } else {
        0.
    };

    /* purge off 1 and 2 */
    if (ix == 0x_3ff0_0000 || ix == 0x_4000_0000) && (u as u32) == 0 {
        r = 0.;
    } else if ix < 0x_4000_0000 {
        /* for x < 2. */
        let (y, i) = if ix <= 0x_3fec_cccc {
            /* lgamma(x) = lgamma(x+1)-log(x) */
            r = -log(x);
            if ix >= 0x_3fe7_6944 {
                (1. - x, 0)
            } else if ix >= 0x_3fcd_a661 {
                (x - (TC - 1.), 1)
            } else {
                (x, 2)
            }
        } else {
            r = 0.;
            if ix >= 0x_3ffb_b4c3 {
                /* [1.7316,2] */
                (2. - x, 0)
            } else if ix >= 0x_3ff3_b4c4 {
                /* [1.23,1.73] */
                (x - TC, 1)
            } else {
                (x - 1., 2)
            }
        };
        let p1: f64;
        let p2: f64;
        let p3: f64;
        match i {
            0 => {
                z = y * y;
                p1 = A0 + z * (A2 + z * (A4 + z * (A6 + z * (A8 + z * A10))));
                p2 = z * (A1 + z * (A3 + z * (A5 + z * (A7 + z * (A9 + z * A11)))));
                p = y * p1 + p2;
                r += p - 0.5 * y;
            }
            1 => {
                z = y * y;
                let w = z * y;
                p1 = T0 + w * (T3 + w * (T6 + w * (T9 + w * T12))); /* parallel comp */
                p2 = T1 + w * (T4 + w * (T7 + w * (T10 + w * T13)));
                p3 = T2 + w * (T5 + w * (T8 + w * (T11 + w * T14)));
                p = z * p1 - (TT - w * (p2 + y * p3));
                r += TF + p;
            }
            2 => {
                p1 = y * (U0 + y * (U1 + y * (U2 + y * (U3 + y * (U4 + y * U5)))));
                p2 = 1. + y * (V1 + y * (V2 + y * (V3 + y * (V4 + y * V5))));
                r += -0.5 * y + p1 / p2;
            }
            #[cfg(feature = "checked")]
            _ => unreachable!(),
            #[cfg(not(feature = "checked"))]
            _ => {}
        }
    } else if ix < 0x_4020_0000 {
        /* x < 8.0 */
        let i = x as isize;
        let y = x - (i as f64);
        p = y * (S0 + y * (S1 + y * (S2 + y * (S3 + y * (S4 + y * (S5 + y * S6))))));
        let q = 1. + y * (R1 + y * (R2 + y * (R3 + y * (R4 + y * (R5 + y * R6)))));
        r = 0.5 * y + p / q;
        z = 1.; /* lgamma(1+s) = log(s) + lgamma(s) */
        // TODO: In C, this was implemented using switch jumps with fallthrough.
        // Does this implementation have performance problems?
        if i >= 7 {
            z *= y + 6.;
        }
        if i >= 6 {
            z *= y + 5.;
        }
        if i >= 5 {
            z *= y + 4.;
        }
        if i >= 4 {
            z *= y + 3.;
        }
        if i >= 3 {
            z *= y + 2.;
            r += log(z);
        }
    } else if ix < 0x_4390_0000 {
        /* 8. <= x < 2^58 */
        let t = log(x);
        z = 1. / x;
        let y = z * z;
        let w = W0 + z * (W1 + y * (W2 + y * (W3 + y * (W4 + y * (W5 + y * W6)))));
        r = (x - 0.5) * (t - 1.) + w;
    } else {
        /* 2^58 <= x <= inf */
        r = x * (log(x) - 1.);
    }
    if sign {
        r = nadj - r;
    }
    (r, signgam)
}
