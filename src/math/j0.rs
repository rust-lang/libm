/* origin: FreeBSD /usr/src/lib/msun/src/e_j0.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* j0(x), y0(x)
 * Bessel function of the first and second kinds of order zero.
 * Method -- j0(x):
 *      1. For tiny x, we use j0(x) = 1 - x^2/4 + x^4/64 - ...
 *      2. Reduce x to |x| since j0(x)=j0(-x),  and
 *         for x in (0,2)
 *              j0(x) = 1-z/4+ z^2*R0/S0,  where z = x*x;
 *         (precision:  |j0-1+z/4-z^2R0/S0 |<2**-63.67 )
 *         for x in (2,inf)
 *              j0(x) = sqrt(2/(pi*x))*(p0(x)*cos(x0)-q0(x)*sin(x0))
 *         where x0 = x-pi/4. It is better to compute sin(x0),cos(x0)
 *         as follow:
 *              cos(x0) = cos(x)cos(pi/4)+sin(x)sin(pi/4)
 *                      = 1/sqrt(2) * (cos(x) + sin(x))
 *              sin(x0) = sin(x)cos(pi/4)-cos(x)sin(pi/4)
 *                      = 1/sqrt(2) * (sin(x) - cos(x))
 *         (To avoid cancellation, use
 *              sin(x) +- cos(x) = -cos(2x)/(sin(x) -+ cos(x))
 *          to compute the worse one.)
 *
 *      3 Special cases
 *              j0(nan)= nan
 *              j0(0) = 1
 *              j0(inf) = 0
 *
 * Method -- y0(x):
 *      1. For x<2.
 *         Since
 *              y0(x) = 2/pi*(j0(x)*(ln(x/2)+Euler) + x^2/4 - ...)
 *         therefore y0(x)-2/pi*j0(x)*ln(x) is an even function.
 *         We use the following function to approximate y0,
 *              y0(x) = U(z)/V(z) + (2/pi)*(j0(x)*ln(x)), z= x^2
 *         where
 *              U(z) = u00 + u01*z + ... + u06*z^6
 *              V(z) = 1  + v01*z + ... + v04*z^4
 *         with absolute approximation error bounded by 2**-72.
 *         Note: For tiny x, U/V = u0 and j0(x)~1, hence
 *              y0(tiny) = u0 + (2/pi)*ln(tiny), (choose tiny<2**-27)
 *      2. For x>=2.
 *              y0(x) = sqrt(2/(pi*x))*(p0(x)*cos(x0)+q0(x)*sin(x0))
 *         where x0 = x-pi/4. It is better to compute sin(x0),cos(x0)
 *         by the method mentioned above.
 *      3. Special cases: y0(0)=-inf, y0(x<0)=NaN, y0(inf)=0.
 */

use super::consts::*;
use super::{cos, fabs, get_high_word, get_low_word, log, sin, sqrt};
use core::f64;

const INVSQRTPI: f64 = 5.641_895_835_477_562_792_80_e-01; /* 0x_3FE2_0DD7, 0x_5042_9B6D */
const TPI: f64 = 6.366_197_723_675_813_824_33_e-01; /* 0x_3FE4_5F30, 0x_6DC9_C883 */

/* common method when |x|>=2 */
fn common(ix: u32, x: f64, y0: bool) -> f64 {
    let s: f64;
    let mut c: f64;
    let mut ss: f64;
    let mut cc: f64;
    let z: f64;

    /*
     * j0(x) = sqrt(2/(pi*x))*(p0(x)*cos(x-pi/4)-q0(x)*sin(x-pi/4))
     * y0(x) = sqrt(2/(pi*x))*(p0(x)*sin(x-pi/4)+q0(x)*cos(x-pi/4))
     *
     * sin(x-pi/4) = (sin(x) - cos(x))/sqrt(2)
     * cos(x-pi/4) = (sin(x) + cos(x))/sqrt(2)
     * sin(x) +- cos(x) = -cos(2x)/(sin(x) -+ cos(x))
     */
    s = sin(x);
    c = cos(x);
    if y0 {
        c = -c;
    }
    cc = s + c;
    /* avoid overflow in 2*x, big ulp error when x>=0x1p1023 */
    if ix < 0x_7fe0_0000 {
        ss = s - c;
        z = -cos(2. * x);
        if s * c < 0. {
            cc = z / ss;
        } else {
            ss = z / cc;
        }
        if ix < 0x_4800_0000 {
            if y0 {
                ss = -ss;
            }
            cc = pzero(x) * cc - qzero(x) * ss;
        }
    }
    INVSQRTPI * cc / sqrt(x)
}

/* R0/S0 on [0, 2.00] */
const R02: f64 = 1.562_499_999_999_999_479_58_e-02; /* 0x_3F8F_FFFF, 0x_FFFF_FFFD */
const R03: f64 = -1.899_792_942_388_547_217_51_e-04; /* 0x_BF28_E6A5, 0x_B61A_C6E9 */
const R04: f64 = 1.829_540_495_327_006_656_70_e-06; /* 0x_3EBE_B1D1, 0x_0C50_3919 */
const R05: f64 = -4.618_326_885_321_031_891_99_e-09; /* 0x_BE33_D5E7, 0x_73D6_3FCE */
const S01: f64 = 1.561_910_294_648_900_104_92_e-02; /* 0x_3F8F_FCE8, 0x_82C8_C2A4 */
const S02: f64 = 1.169_267_846_633_374_502_60_e-04; /* 0x_3F1E_A6D2, 0x_DD57_DBF4 */
const S03: f64 = 5.135_465_502_073_181_114_46_e-07; /* 0x_3EA1_3B54, 0x_CE84_D5A9 */
const S04: f64 = 1.166_140_033_337_900_002_05_e-09; /* 0x_3E14_08BC, 0x_F474_5D8F */

pub fn j0(mut x: f64) -> f64 {
    let z: f64;
    let r: f64;
    let s: f64;
    let mut ix: u32;

    ix = get_high_word(x);
    ix &= UF_ABS;

    /* j0(+-inf)=0, j0(nan)=nan */
    if ix >= 0x_7ff0_0000 {
        return 1. / (x * x);
    }
    x = fabs(x);

    if ix >= 0x_4000_0000 {
        /* |x| >= 2 */
        /* large ulp error near zeros: 2.4, 5.52, 8.6537,.. */
        return common(ix, x, false);
    }

    /* 1 - x*x/4 + x*x*R(x^2)/S(x^2) */
    if ix >= 0x_3f20_0000 {
        /* |x| >= 2**-13 */
        /* up to 4ulp error close to 2 */
        z = x * x;
        r = z * (R02 + z * (R03 + z * (R04 + z * R05)));
        s = 1. + z * (S01 + z * (S02 + z * (S03 + z * S04)));
        return (1. + x / 2.) * (1. - x / 2.) + z * (r / s);
    }

    /* 1 - x*x/4 */
    /* prevent underflow */
    /* inexact should be raised when x!=0, this is not done correctly */
    if ix >= 0x_3800_0000 {
        /* |x| >= 2**-127 */
        x = 0.25 * x * x;
    }
    1. - x
}

const U00: f64 = -7.380_429_510_868_723_175_23_e-02; /* 0x_BFB2_E4D6, 0x_99CB_D01F */
const U01: f64 = 1.766_664_525_091_811_155_38_e-01; /* 0x_3FC6_9D01, 0x_9DE9_E3FC */
const U02: f64 = -1.381_856_719_455_968_988_96_e-02; /* 0x_BF8C_4CE8, 0x_B16C_FA97 */
const U03: f64 = 3.474_534_320_936_836_502_38_e-04; /* 0x_3F36_C54D, 0x_20B2_9B6B */
const U04: f64 = -3.814_070_537_243_641_611_25_e-06; /* 0x_BECF_FEA7, 0x_73D2_5CAD */
const U05: f64 = 1.955_901_370_350_229_202_06_e-08; /* 0x_3E55_0057, 0x_3B4E_ABD4 */
const U06: f64 = -3.982_051_941_321_033_984_53_e-11; /* 0x_BDC5_E43D, 0x_693F_B3C8 */
const V01: f64 = 1.273_048_348_341_236_993_28_e-02; /* 0x_3F8A_1270, 0x_91C9_C71A */
const V02: f64 = 7.600_686_273_503_532_537_02_e-05; /* 0x_3F13_ECBB, 0x_F578_C6C1 */
const V03: f64 = 2.591_508_518_404_578_054_67_e-07; /* 0x_3E91_642D, 0x_7FF2_02FD */
const V04: f64 = 4.411_103_113_326_754_674_03_e-10; /* 0x_3DFE_5018, 0x_3BD6_D9EF */

pub fn y0(x: f64) -> f64 {
    let z: f64;
    let u: f64;
    let v: f64;
    let ix: u32;
    let lx: u32;

    ix = get_high_word(x);
    lx = get_low_word(x);

    /* y0(nan)=nan, y0(<0)=nan, y0(0)=-inf, y0(inf)=0 */
    if ((ix << 1) | lx) == 0 {
        f64::NEG_INFINITY
    } else if (ix >> 31) != 0 {
        f64::NAN
    } else if ix >= 0x_7ff0_0000 {
        1. / x
    } else if ix >= 0x_4000_0000 {
        /* x >= 2 */
        /* large ulp errors near zeros: 3.958, 7.086,.. */
        common(ix, x, true)
    } else if ix >= 0x_3e40_0000
    /* U(x^2)/V(x^2) + (2/pi)*j0(x)*log(x) */
    {
        /* x >= 2**-27 */
        /* large ulp error near the first zero, x ~= 0.89 */
        z = x * x;
        u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06)))));
        v = 1. + z * (V01 + z * (V02 + z * (V03 + z * V04)));
        u / v + TPI * (j0(x) * log(x))
    } else {
        U00 + TPI * log(x)
    }
}

/* The asymptotic expansions of pzero is
 *      1 - 9/128 s^2 + 11025/98304 s^4 - ...,  where s = 1/x.
 * For x >= 2, We approximate pzero by
 *      pzero(x) = 1 + (R/S)
 * where  R = pR0 + pR1*s^2 + pR2*s^4 + ... + pR5*s^10
 *        S = 1 + pS0*s^2 + ... + pS4*s^10
 * and
 *      | pzero(x)-1-R/S | <= 2  ** ( -60.26)
 */
const PR8: [f64; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.,                                 /* 0x_0000_0000, 0x_0000_0000 */
    -7.031_249_999_999_003_574_84_e-02, /* 0x_BFB1_FFFF, 0x_FFFF_FD32 */
    -8.081_670_412_753_497_956_26,      /* 0x_C020_29D0, 0x_B44F_A779 */
    -2.570_631_056_797_048_472_62_e+02, /* 0x_C070_1102, 0x_7B19_E863 */
    -2.485_216_410_094_288_221_44_e+03, /* 0x_C0A3_6A6E, 0x_CD4D_CAFC */
    -5.253_043_804_907_295_452_72_e+03, /* 0x_C0B4_850B, 0x_36CC_643D */
];
const PS8: [f64; 5] = [
    1.165_343_646_196_681_817_17_e+02, /* 0x_405D_2233, 0x_07A9_6751 */
    3.833_744_753_641_218_267_15_e+03, /* 0x_40AD_F37D, 0x_5059_6938 */
    4.059_785_726_484_725_455_52_e+04, /* 0x_40E3_D2BB, 0x_6EB6_B05F */
    1.167_529_725_643_759_156_81_e+05, /* 0x_40FC_810F, 0x_8F9F_A9BD */
    4.762_772_841_467_309_626_75_e+04, /* 0x_40E7_4177, 0x_4F2C_49DC */
];

const PR5: [f64; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    -1.141_254_646_918_945_025_84_e-11, /* 0x_BDA9_18B1, 0x_47E4_95CC */
    -7.031_249_408_735_992_800_78_e-02, /* 0x_BFB1_FFFF, 0x_E69A_FBC6 */
    -4.159_610_644_705_877_824_38,      /* 0x_C010_A370, 0x_F90C_6BBF */
    -6.767_476_522_651_672_610_21_e+01, /* 0x_C050_EB2F, 0x_5A7D_1783 */
    -3.312_312_996_491_729_677_47_e+02, /* 0x_C074_B3B3, 0x_6742_CC63 */
    -3.464_333_883_656_049_124_51_e+02, /* 0x_C075_A6EF, 0x_28A3_8BD7 */
];
const PS5: [f64; 5] = [
    6.075_393_826_923_003_359_75_e+01, /* 0x_404E_6081, 0x_0C98_C5DE */
    1.051_252_305_957_045_791_73_e+03, /* 0x_4090_6D02, 0x_5C7E_2864 */
    5.978_970_943_338_557_844_98_e+03, /* 0x_40B7_5AF8, 0x_8FBE_1D60 */
    9.625_445_143_577_744_602_23_e+03, /* 0x_40C2_CCB8, 0x_FA76_FA38 */
    2.406_058_159_229_391_094_41_e+03, /* 0x_40A2_CC1D, 0x_C70B_E864 */
];

const PR3: [f64; 6] = [
    /* for x in [4.547,2.8571]=1/[0.2199,0.35001] */
    -2.547_046_017_719_519_156_20_e-09, /* 0x_BE25_E103, 0x_6FE1_AA86 */
    -7.031_196_163_814_816_546_54_e-02, /* 0x_BFB1_FFF6, 0x_F7C0_E24B */
    -2.409_032_215_495_296_114_23,      /* 0x_C003_45B2, 0x_AEA4_8074 */
    -2.196_597_747_348_830_864_67_e+01, /* 0x_C035_F74A, 0x_4CB9_4E14 */
    -5.807_917_047_017_375_722_36_e+01, /* 0x_C04D_0A22, 0x_420A_1A45 */
    -3.144_794_705_948_885_038_54_e+01, /* 0x_C03F_72AC, 0x_A892_D80F */
];
const PS3: [f64; 5] = [
    3.585_603_380_552_097_263_49_e+01, /* 0x_4041_ED92, 0x_8407_7DD3 */
    3.615_139_830_503_038_638_20_e+02, /* 0x_4076_9839, 0x_464A_7C0E */
    1.193_607_837_921_115_333_30_e+03, /* 0x_4092_A66E, 0x_6D10_61D6 */
    1.127_996_798_569_074_144_32_e+03, /* 0x_4091_9FFC, 0x_B8C3_9B7E */
    1.735_809_308_133_357_546_92_e+02, /* 0x_4065_B296, 0x_FC37_9081 */
];

const PR2: [f64; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    -8.875_343_330_325_264_112_54_e-08, /* 0x_BE77_D316, 0x_E927_026D */
    -7.030_309_954_836_247_432_47_e-02, /* 0x_BFB1_FF62, 0x_495E_1E42 */
    -1.450_738_467_809_529_863_57,      /* 0x_BFF7_3639, 0x_8A24_A843 */
    -7.635_696_138_235_277_707_91,      /* 0x_C01E_8AF3, 0x_EDAF_A7F3 */
    -1.119_316_688_603_567_477_86_e+01, /* 0x_C026_62E6, 0x_C524_6303 */
    -3.233_645_793_513_353_350_33,      /* 0x_C009_DE81, 0x_AF8F_E70F */
];
const PS2: [f64; 5] = [
    2.222_029_975_320_888_084_41_e+01, /* 0x_4036_3865, 0x_908B_5959 */
    1.362_067_942_182_152_080_48_e+02, /* 0x_4061_069E, 0x_0EE8_878F */
    2.704_702_786_580_834_867_89_e+02, /* 0x_4070_E786, 0x_42EA_079B */
    1.538_753_942_083_203_298_81_e+02, /* 0x_4063_3C03, 0x_3AB6_FAFF */
    1.465_761_769_482_561_938_10_e+01, /* 0x_402D_50B3, 0x_4439_1809 */
];

fn pzero(x: f64) -> f64 {
    let p: &[f64; 6];
    let q: &[f64; 5];
    let z: f64;
    let r: f64;
    let s: f64;
    let mut ix: u32;

    ix = get_high_word(x);
    ix &= UF_ABS;
    if ix >= 0x_4020_0000 {
        p = &PR8;
        q = &PS8;
    } else if ix >= 0x_4012_2E8B {
        p = &PR5;
        q = &PS5;
    } else if ix >= 0x_4006_DB6D {
        p = &PR3;
        q = &PS3;
    } else {
        /*ix >= 0x_4000_0000*/
        p = &PR2;
        q = &PS2;
    }
    z = 1. / (x * x);
    r = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    s = 1. + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * q[4]))));
    1. + r / s
}

/* For x >= 8, the asymptotic expansions of qzero is
 *      -1/8 s + 75/1024 s^3 - ..., where s = 1/x.
 * We approximate pzero by
 *      qzero(x) = s*(-1.25 + (R/S))
 * where  R = qR0 + qR1*s^2 + qR2*s^4 + ... + qR5*s^10
 *        S = 1 + qS0*s^2 + ... + qS5*s^12
 * and
 *      | qzero(x)/s +1.25-R/S | <= 2  ** ( -61.22)
 */
const QR8: [f64; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.,                                /* 0x_0000_0000, 0x_0000_0000 */
    7.324_218_749_999_350_519_53_e-02, /* 0x_3FB2_BFFF, 0x_FFFF_FE2C */
    1.176_820_646_822_526_938_99_e+01, /* 0x_4027_8952, 0x_5BB3_34D6 */
    5.576_733_802_564_018_560_59_e+02, /* 0x_4081_6D63, 0x_1530_1825 */
    8.859_197_207_564_686_323_17_e+03, /* 0x_40C1_4D99, 0x_3E18_F46D */
    3.701_462_677_768_878_347_71_e+04, /* 0x_40E2_12D4, 0x_0E90_1566 */
];
const QS8: [f64; 6] = [
    1.637_760_268_956_898_244_14_e+02,  /* 0x_4064_78D5, 0x_365B_39BC */
    8.098_344_946_564_498_059_16_e+03,  /* 0x_40BF_A258, 0x_4E6B_0563 */
    1.425_382_914_191_204_763_48_e+05,  /* 0x_4101_6652, 0x_54D3_8C3F */
    8.033_092_571_195_143_973_45_e+05,  /* 0x_4128_83DA, 0x_83A5_2B43 */
    8.405_015_798_190_605_128_18_e+05,  /* 0x_4129_A66B, 0x_28DE_0B3D */
    -3.438_992_935_378_666_152_25_e+05, /* 0x_C114_FD6D, 0x_2C95_30C5 */
];

const QR5: [f64; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    1.840_859_635_945_155_313_81_e-11, /* 0x_3DB4_3D8F, 0x_29CC_8CD9 */
    7.324_217_666_126_847_658_96_e-02, /* 0x_3FB2_BFFF, 0x_D172_B04C */
    5.835_635_089_620_569_537_77,      /* 0x_4017_57B0, 0x_B995_3DD3 */
    1.351_115_772_864_498_296_71_e+02, /* 0x_4060_E392, 0x_0A87_88E9 */
    1.027_243_765_961_640_974_64_e+03, /* 0x_4090_0CF9, 0x_9DC8_C481 */
    1.989_977_858_646_053_846_31_e+03, /* 0x_409F_17E9, 0x_53C6_E3A6 */
];
const QS5: [f64; 6] = [
    8.277_661_022_365_377_618_83_e+01,  /* 0x_4054_B1B3, 0x_FB5E_1543 */
    2.077_814_164_213_929_871_04_e+03,  /* 0x_40A0_3BA0, 0x_DA21_C0CE */
    1.884_728_877_857_180_850_70_e+04,  /* 0x_40D2_67D2, 0x_7B59_1E6D */
    5.675_111_228_949_473_297_69_e+04,  /* 0x_40EB_B5E3, 0x_97E0_2372 */
    3.597_675_384_251_144_714_65_e+04,  /* 0x_40E1_9118, 0x_1F7A_54A0 */
    -5.354_342_756_019_447_733_71_e+03, /* 0x_C0B4_EA57, 0x_BEDB_C609 */
];

const QR3: [f64; 6] = [
    /* for x in [4.547,2.8571]=1/[0.2199,0.35001] */
    4.377_410_140_897_386_209_06_e-09, /* 0x_3E32_CD03, 0x_6ADE_CB82 */
    7.324_111_800_429_114_471_63_e-02, /* 0x_3FB2_BFEE, 0x_0E8D_0842 */
    3.344_231_375_161_707_209_29,      /* 0x_400A_C0FC, 0x_6114_9CF5 */
    4.262_184_407_454_126_500_17_e+01, /* 0x_4045_4F98, 0x_962D_AEDD */
    1.708_080_913_405_655_962_83_e+02, /* 0x_4065_59DB, 0x_E25E_FD1F */
    1.667_339_486_966_511_685_75_e+02, /* 0x_4064_D77C, 0x_81FA_21E0 */
];
const QS3: [f64; 6] = [
    4.875_887_297_245_871_820_91_e+01,  /* 0x_4048_6122, 0x_BFE3_43A6 */
    7.096_892_210_566_060_157_36_e+02,  /* 0x_4086_2D83, 0x_8654_4EB3 */
    3.704_148_226_201_113_629_94_e+03,  /* 0x_40AC_F04B, 0x_E44D_FC63 */
    6.460_425_167_525_689_175_82_e+03,  /* 0x_40B9_3C6C, 0x_D7C7_6A28 */
    2.516_333_689_203_689_573_33_e+03,  /* 0x_40A3_A8AA, 0x_D94F_B1C0 */
    -1.492_474_518_361_563_866_62_e+02, /* 0x_C062_A7EB, 0x_201C_F40F */
];

const QR2: [f64; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    1.504_444_448_869_832_723_79_e-07, /* 0x_3E84_313B, 0x_54F7_6BDB */
    7.322_342_659_630_792_782_72_e-02, /* 0x_3FB2_BEC5, 0x_3E88_3E34 */
    1.998_191_740_938_159_988_16,      /* 0x_3FFF_F897, 0x_E727_779C */
    1.449_560_293_478_857_353_48_e+01, /* 0x_402C_FDBF, 0x_AAF9_6FE5 */
    3.166_623_175_047_815_408_33_e+01, /* 0x_403F_AA8E, 0x_29FB_DC4A */
    1.625_270_757_109_292_674_16_e+01, /* 0x_4030_40B1, 0x_7181_4BB4 */
];
const QS2: [f64; 6] = [
    3.036_558_483_552_191_844_98_e+01, /* 0x_403E_5D96, 0x_F7C0_7AED */
    2.693_481_186_080_498_446_24_e+02, /* 0x_4070_D591, 0x_E4D1_4B40 */
    8.447_837_575_953_201_394_44_e+02, /* 0x_408A_6645, 0x_22B3_BF22 */
    8.829_358_451_124_885_505_12_e+02, /* 0x_408B_977C, 0x_9C5C_C214 */
    2.126_663_885_117_988_286_31_e+02, /* 0x_406A_9553, 0x_0E00_1365 */
    -5.310_954_938_826_669_469_17,     /* 0x_C015_3E6A, 0x_F8B3_2931 */
];

fn qzero(x: f64) -> f64 {
    let p: &[f64; 6];
    let q: &[f64; 6];
    let s: f64;
    let r: f64;
    let z: f64;
    let mut ix: u32;

    ix = get_high_word(x);
    ix &= UF_ABS;
    if ix >= 0x_4020_0000 {
        p = &QR8;
        q = &QS8;
    } else if ix >= 0x_4012_2E8B {
        p = &QR5;
        q = &QS5;
    } else if ix >= 0x_4006_DB6D {
        p = &QR3;
        q = &QS3;
    } else {
        /*ix >= 0x_4000_0000*/
        p = &QR2;
        q = &QS2;
    }
    z = 1. / (x * x);
    r = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    s = 1. + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * (q[4] + z * q[5])))));
    (-0.125 + r / s) / x
}
