use super::{exp, fabs};
use crate::math::{get_high_word, with_set_low_word};
/* origin: FreeBSD /usr/src/lib/msun/src/s_erf.c */
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
/* double erf(double x)
 * double erfc(double x)
 *                           x
 *                    2      |\
 *     erf(x)  =  ---------  | exp(-t*t)dt
 *                 sqrt(pi) \|
 *                           0
 *
 *     erfc(x) =  1-erf(x)
 *  Note that
 *              erf(-x) = -erf(x)
 *              erfc(-x) = 2 - erfc(x)
 *
 * Method:
 *      1. For |x| in [0, 0.84375]
 *          erf(x)  = x + x*R(x^2)
 *          erfc(x) = 1 - erf(x)           if x in [-.84375,0.25]
 *                  = 0.5 + ((0.5-x)-x*R)  if x in [0.25,0.84375]
 *         where R = P/Q where P is an odd poly of degree 8 and
 *         Q is an odd poly of degree 10.
 *                                               -57.90
 *                      | R - (erf(x)-x)/x | <= 2
 *
 *
 *         Remark. The formula is derived by noting
 *          erf(x) = (2/sqrt(pi))*(x - x^3/3 + x^5/10 - x^7/42 + ....)
 *         and that
 *          2/sqrt(pi) = 1.128_379_167_095_512_573896_158_903_121_545_171688
 *         is close to one. The interval is chosen because the fix
 *         point of erf(x) is near 0.6174 (i.e., erf(x)=x when x is
 *         near 0.6174), and by some experiment, 0.84375 is chosen to
 *         guarantee the error is less than one ulp for erf.
 *
 *      2. For |x| in [0.84375,1.25], let s = |x| - 1, and
 *         c = 0.845_062_91151 rounded to single (24 bits)
 *              erf(x)  = sign(x) * (c  + P1(s)/Q1(s))
 *              erfc(x) = (1-c)  - P1(s)/Q1(s) if x > 0
 *                        1+(c+P1(s)/Q1(s))    if x < 0
 *              |P1/Q1 - (erf(|x|)-c)| <= 2**-59.06
 *         Remark: here we use the taylor series expansion at x=1.
 *              erf(1+s) = erf(1) + s*Poly(s)
 *                       = 0.845.. + P1(s)/Q1(s)
 *         That is, we use rational approximation to approximate
 *                      erf(1+s) - (c = (single)0.845_062_91151)
 *         Note that |P1/Q1|< 0.078 for x in [0.84375,1.25]
 *         where
 *              P1(s) = degree 6 poly in s
 *              Q1(s) = degree 6 poly in s
 *
 *      3. For x in [1.25,1/0.35(~2.857143)],
 *              erfc(x) = (1/x)*exp(-x*x-0.5625+R1/S1)
 *              erf(x)  = 1 - erfc(x)
 *         where
 *              R1(z) = degree 7 poly in z, (z=1/x^2)
 *              S1(z) = degree 8 poly in z
 *
 *      4. For x in [1/0.35,28]
 *              erfc(x) = (1/x)*exp(-x*x-0.5625+R2/S2) if x > 0
 *                      = 2.0 - (1/x)*exp(-x*x-0.5625+R2/S2) if -6<x<0
 *                      = 2.0 - tiny            (if x <= -6)
 *              erf(x)  = sign(x)*(1.0 - erfc(x)) if x < 6, else
 *              erf(x)  = sign(x)*(1.0 - tiny)
 *         where
 *              R2(z) = degree 6 poly in z, (z=1/x^2)
 *              S2(z) = degree 7 poly in z
 *
 *      Note1:
 *         To compute exp(-x*x-0.5625+R/S), let s be a single
 *         precision number and s := x; then
 *              -x*x = -s*s + (s-x)*(s+x)
 *              exp(-x*x-0.5626+R/S) =
 *                      exp(-s*s-0.5625)*exp((s-x)*(s+x)+R/S);
 *      Note2:
 *         Here 4 and 5 make use of the asymptotic series
 *                        exp(-x*x)
 *              erfc(x) ~ ---------- * ( 1 + Poly(1/x^2) )
 *                        x*sqrt(pi)
 *         We use rational approximation to approximate
 *              g(s)=f(1/x^2) = log(erfc(x)*x) - x*x + 0.5625
 *         Here is the error bound for R1/S1 and R2/S2
 *              |R1/S1 - f(x)|  < 2**(-62.57)
 *              |R2/S2 - f(x)|  < 2**(-61.52)
 *
 *      5. For inf > x >= 28
 *              erf(x)  = sign(x) *(1 - tiny)  (raise inexact)
 *              erfc(x) = tiny*tiny (raise underflow) if x > 0
 *                      = 2 - tiny if x<0
 *
 *      7. Special case:
 *              erf(0)  = 0, erf(inf)  = 1, erf(-inf) = -1,
 *              erfc(0) = 1, erfc(inf) = 0, erfc(-inf) = 2,
 *              erfc/erf(NaN) is NaN
 */

use crate::math::consts::*;

const ERX: f64 = 8.450_629_115_104_675_292_97_e-01; /* 0x_3FEB_0AC1, 0x_6000_0000 */
/*
 * Coefficients for approximation to  erf on [0,0.84375]
 */
const EFX8: f64 = 1.027_033_336_764_100_690_53; /* 0x_3FF0_6EBA, 0x_8214_DB69 */
const PP0: f64 = 1.283_791_670_955_125_585_61_e-01; /* 0x_3FC0_6EBA, 0x_8214_DB68 */
const PP1: f64 = -3.250_421_072_470_014_993_70_e-01; /* 0x_BFD4_CD7D, 0x_691C_B913 */
const PP2: f64 = -2.848_174_957_559_851_047_66_e-02; /* 0x_BF9D_2A51, 0x_DBD7_194F */
const PP3: f64 = -5.770_270_296_489_441_591_57_e-03; /* 0x_BF77_A291, 0x_2366_68E4 */
const PP4: f64 = -2.376_301_665_665_016_260_84_e-05; /* 0x_BEF8_EAD6, 0x_1200_16AC */
const QQ1: f64 = 3.979_172_239_591_553_528_19_e-01; /* 0x_3FD9_7779, 0x_CDDA_DC09 */
const QQ2: f64 = 6.502_224_998_876_729_444_85_e-02; /* 0x_3FB0_A54C, 0x_5536_CEBA */
const QQ3: f64 = 5.081_306_281_875_765_627_76_e-03; /* 0x_3F74_D022, 0x_C4D3_6B0F */
const QQ4: f64 = 1.324_947_380_043_216_445_26_e-04; /* 0x_3F21_5DC9, 0x_221C_1A10 */
const QQ5: f64 = -3.960_228_278_775_368_123_20_e-06; /* 0x_BED0_9C43, 0x_42A2_6120 */
/*
 * Coefficients for approximation to  erf  in [0.84375,1.25]
 */
const PA0: f64 = -2.362_118_560_752_659_440_77_e-03; /* 0x_BF63_59B8, 0x_BEF7_7538 */
const PA1: f64 = 4.148_561_186_837_483_316_66_e-01; /* 0x_3FDA_8D00, 0x_AD92_B34D */
const PA2: f64 = -3.722_078_760_357_013_238_47_e-01; /* 0x_BFD7_D240, 0x_FBB8_C3F1 */
const PA3: f64 = 3.183_466_199_011_617_536_74_e-01; /* 0x_3FD4_5FCA, 0x_8051_20E4 */
const PA4: f64 = -1.108_946_942_823_966_774_76_e-01; /* 0x_BFBC_6398, 0x_3D3E_28EC */
const PA5: f64 = 3.547_830_432_561_823_593_71_e-02; /* 0x_3FA2_2A36, 0x_5997_95EB */
const PA6: f64 = -2.166_375_594_868_790_843_00_e-03; /* 0x_BF61_BF38, 0x_0A96_073F */
const QA1: f64 = 1.064_208_804_008_442_282_86_e-01; /* 0x_3FBB_3E66, 0x_18EE_E323 */
const QA2: f64 = 5.403_979_177_021_710_489_37_e-01; /* 0x_3FE1_4AF0, 0x_92EB_6F33 */
const QA3: f64 = 7.182_865_441_419_626_628_68_e-02; /* 0x_3FB2_635C, 0x_D99F_E9A7 */
const QA4: f64 = 1.261_712_198_087_616_421_12_e-01; /* 0x_3FC0_2660, 0x_E763_351F */
const QA5: f64 = 1.363_708_391_202_905_073_62_e-02; /* 0x_3F8B_EDC2, 0x_6B51_DD1C */
const QA6: f64 = 1.198_449_984_679_910_741_70_e-02; /* 0x_3F88_8B54, 0x_5735_151D */
/*
 * Coefficients for approximation to  erfc in [1.25,1/0.35]
 */
const RA0: f64 = -9.864_944_034_847_148_227_05_e-03; /* 0x_BF84_3412, 0x_600D_6435 */
const RA1: f64 = -6.938_585_727_071_817_643_72_e-01; /* 0x_BFE6_3416, 0x_E4BA_7360 */
const RA2: f64 = -1.055_862_622_532_329_098_14_e+01; /* 0x_C025_1E04, 0x_41B0_E726 */
const RA3: f64 = -6.237_533_245_032_600_603_96_e+01; /* 0x_C04F_300A, 0x_E4CB_A38D */
const RA4: f64 = -1.623_966_694_625_734_703_55_e+02; /* 0x_C064_4CB1, 0x_8428_2266 */
const RA5: f64 = -1.846_050_929_067_110_359_94_e+02; /* 0x_C067_135C, 0x_EBCC_ABB2 */
const RA6: f64 = -8.128_743_550_630_659_342_46_e+01; /* 0x_C054_5265, 0x_57E4_D2F2 */
const RA7: f64 = -9.814_329_344_169_145_485_92; /* 0x_C023_A0EF, 0x_C69A_C25C */
const SA1: f64 = 1.965_127_166_743_925_712_92_e+01; /* 0x_4033_A6B9, 0x_BD70_7687 */
const SA2: f64 = 1.376_577_541_435_190_426_00_e+02; /* 0x_4061_350C, 0x_526A_E721 */
const SA3: f64 = 4.345_658_774_752_292_288_21_e+02; /* 0x_407B_290D, 0x_D58A_1A71 */
const SA4: f64 = 6.453_872_717_332_678_803_36_e+02; /* 0x_4084_2B19, 0x_21EC_2868 */
const SA5: f64 = 4.290_081_400_275_678_333_86_e+02; /* 0x_407A_D021, 0x_5770_0314 */
const SA6: f64 = 1.086_350_055_417_794_351_34_e+02; /* 0x_405B_28A3, 0x_EE48_AE2C */
const SA7: f64 = 6.570_249_770_319_281_701_35; /* 0x_401A_47EF, 0x_8E48_4A93 */
const SA8: f64 = -6.042_441_521_485_809_874_38_e-02; /* 0x_BFAE_EFF2, 0x_EE74_9A62 */
/*
 * Coefficients for approximation to  erfc in [1/.35,28]
 */
const RB0: f64 = -9.864_942_924_700_099_285_97_e-03; /* 0x_BF84_3412, 0x_39E8_6F4A */
const RB1: f64 = -7.992_832_376_805_230_065_74_e-01; /* 0x_BFE9_93BA, 0x_70C2_85DE */
const RB2: f64 = -1.775_795_491_775_475_198_89_e+01; /* 0x_C031_C209, 0x_555F_995A */
const RB3: f64 = -1.606_363_848_558_219_160_62_e+02; /* 0x_C064_145D, 0x_43C5_ED98 */
const RB4: f64 = -6.375_664_433_683_896_277_22_e+02; /* 0x_C083_EC88, 0x_1375_F228 */
const RB5: f64 = -1.025_095_131_611_077_249_54_e+03; /* 0x_C090_0461, 0x_6A2E_5992 */
const RB6: f64 = -4.835_191_916_086_513_970_19_e+02; /* 0x_C07E_384E, 0x_9BDC_383F */
const SB1: f64 = 3.033_806_074_348_245_829_24_e+01; /* 0x_403E_568B, 0x_261D_5190 */
const SB2: f64 = 3.257_925_129_965_739_188_26_e+02; /* 0x_4074_5CAE, 0x_221B_9F0A */
const SB3: f64 = 1.536_729_586_084_436_959_94_e+03; /* 0x_4098_02EB, 0x_189D_5118 */
const SB4: f64 = 3.199_858_219_508_595_539_08_e+03; /* 0x_40A8_FFB7, 0x_688C_246A */
const SB5: f64 = 2.553_050_406_433_164_425_83_e+03; /* 0x_40A3_F219, 0x_CEDF_3BE6 */
const SB6: f64 = 4.745_285_412_069_553_672_15_e+02; /* 0x_407D_A874, 0x_E79F_E763 */
const SB7: f64 = -2.244_095_244_658_581_833_62_e+01; /* 0x_C036_70E2, 0x_4271_2D62 */

fn erfc1(x: f64) -> f64 {
    let s: f64;
    let p: f64;
    let q: f64;

    s = fabs(x) - 1.;
    p = PA0 + s * (PA1 + s * (PA2 + s * (PA3 + s * (PA4 + s * (PA5 + s * PA6)))));
    q = 1. + s * (QA1 + s * (QA2 + s * (QA3 + s * (QA4 + s * (QA5 + s * QA6)))));

    1. - ERX - p / q
}

fn erfc2(ix: u32, mut x: f64) -> f64 {
    let s: f64;
    let r: f64;
    let big_s: f64;
    let z: f64;

    if ix < 0x_3ff4_0000 {
        /* |x| < 1.25 */
        return erfc1(x);
    }

    x = fabs(x);
    s = 1. / (x * x);
    if ix < 0x_4006_db6d {
        /* |x| < 1/.35 ~ 2.85714 */
        r = RA0 + s * (RA1 + s * (RA2 + s * (RA3 + s * (RA4 + s * (RA5 + s * (RA6 + s * RA7))))));
        big_s = 1.
            + s * (SA1
                + s * (SA2 + s * (SA3 + s * (SA4 + s * (SA5 + s * (SA6 + s * (SA7 + s * SA8)))))));
    } else {
        /* |x| > 1/.35 */
        r = RB0 + s * (RB1 + s * (RB2 + s * (RB3 + s * (RB4 + s * (RB5 + s * RB6)))));
        big_s =
            1. + s * (SB1 + s * (SB2 + s * (SB3 + s * (SB4 + s * (SB5 + s * (SB6 + s * SB7))))));
    }
    z = with_set_low_word(x, 0);

    exp(-z * z - 0.5625) * exp((z - x) * (z + x) + r / big_s) / x
}

pub fn erf(x: f64) -> f64 {
    let r: f64;
    let s: f64;
    let z: f64;
    let y: f64;
    let mut ix: u32;
    let sign: usize;

    ix = get_high_word(x);
    sign = (ix >> 31) as usize;
    ix &= UF_ABS;
    if ix >= 0x_7ff0_0000 {
        /* erf(nan)=nan, erf(+-inf)=+-1 */
        return 1. - 2. * (sign as f64) + 1. / x;
    }
    if ix < 0x_3feb_0000 {
        /* |x| < 0.84375 */
        if ix < 0x_3e30_0000 {
            /* |x| < 2**-28 */
            /* avoid underflow */
            return 0.125 * (8. * x + EFX8 * x);
        }
        z = x * x;
        r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4)));
        s = 1. + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5))));
        y = r / s;
        return x + x * y;
    }
    if ix < 0x_4018_0000 {
        /* 0.84375 <= |x| < 6 */
        y = 1. - erfc2(ix, x);
    } else {
        let x1p_1022 = f64::from_bits(0x_0010_0000_0000_0000);
        y = 1. - x1p_1022;
    }

    if sign != 0 {
        -y
    } else {
        y
    }
}

pub fn erfc(x: f64) -> f64 {
    let r: f64;
    let s: f64;
    let z: f64;
    let y: f64;
    let mut ix: u32;
    let sign: usize;

    ix = get_high_word(x);
    sign = (ix >> 31) as usize;
    ix &= UF_ABS;
    if ix >= 0x_7ff0_0000 {
        /* erfc(nan)=nan, erfc(+-inf)=0,2 */
        return 2. * (sign as f64) + 1. / x;
    }
    if ix < 0x_3feb_0000 {
        /* |x| < 0.84375 */
        if ix < 0x_3c70_0000 {
            /* |x| < 2**-56 */
            return 1. - x;
        }
        z = x * x;
        r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4)));
        s = 1. + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5))));
        y = r / s;
        if sign != 0 || ix < 0x_3fd0_0000 {
            /* x < 1/4 */
            return 1. - (x + x * y);
        }
        return 0.5 - (x - 0.5 + x * y);
    }
    if ix < 0x_403c_0000 {
        /* 0.84375 <= |x| < 28 */
        if sign != 0 {
            return 2. - erfc2(ix, x);
        } else {
            return erfc2(ix, x);
        }
    }

    let x1p_1022 = f64::from_bits(0x_0010_0000_0000_0000);
    if sign != 0 {
        2. - x1p_1022
    } else {
        x1p_1022 * x1p_1022
    }
}
