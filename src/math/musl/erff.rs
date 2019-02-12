/* origin: FreeBSD /usr/src/lib/msun/src/s_erff.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 */
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

use super::{expf, fabsf};
use crate::math::consts::*;

const ERX: f32  =  8.450_629_115_1_e-01; /* 0x_3f58_560b */
/*
 * Coefficients for approximation to  erf on [0,0.84375]
 */
const EFX8: f32 =  1.027_033_329_0; /* 0x_3f83_75d4 */
const PP0: f32  =  1.283_791_661_3_e-01; /* 0x_3e03_75d4 */
const PP1: f32  = -3.250_420_987_6_e-01; /* 0x_bea6_6beb */
const PP2: f32  = -2.848_174_981_8_e-02; /* 0x_bce9_528f */
const PP3: f32  = -5.770_270_247_e-03; /* 0x_bbbd_1489 */
const PP4: f32  = -2.376_301_745_2_e-05; /* 0x_b7c7_56b1 */
const QQ1: f32  =  3.979_172_110_6_e-01; /* 0x_3ecb_bbce */
const QQ2: f32  =  6.502_225_25_e-02; /* 0x_3d85_2a63 */
const QQ3: f32  =  5.081_306_211_7_e-03; /* 0x_3ba6_8116 */
const QQ4: f32  =  1.324_947_370_4_e-04; /* 0x_390a_ee49 */
const QQ5: f32  = -3.960_228_241_3_e-06; /* 0x_b684_e21a */
/*
 * Coefficients for approximation to  erf  in [0.84375,1.25]
 */
const PA0: f32  = -2.362_118_568_3_e-03; /* 0x_bb1a_cdc6 */
const PA1: f32  =  4.148_561_060_4_e-01; /* 0x_3ed4_6805 */
const PA2: f32  = -3.722_078_800_2_e-01; /* 0x_bebe_9208 */
const PA3: f32  =  3.183_466_196_1_e-01; /* 0x_3ea2_fe54 */
const PA4: f32  = -1.108_946_949_2_e-01; /* 0x_bde3_1cc2 */
const PA5: f32  =  3.547_830_507_2_e-02; /* 0x_3d11_51b3 */
const PA6: f32  = -2.166_375_517_8_e-03; /* 0x_bb0d_f9c0 */
const QA1: f32  =  1.064_208_820_5_e-01; /* 0x_3dd9_f331 */
const QA2: f32  =  5.403_979_420_7_e-01; /* 0x_3f0a_5785 */
const QA3: f32  =  7.182_865_589_9_e-02; /* 0x_3d93_1ae7 */
const QA4: f32  =  1.261_712_163_7_e-01; /* 0x_3e01_3307 */
const QA5: f32  =  1.363_708_358_3_e-02; /* 0x_3c5f_6e13 */
const QA6: f32  =  1.198_450_010_3_e-02; /* 0x_3c44_5aa3 */
/*
 * Coefficients for approximation to  erfc in [1.25,1/0.35]
 */
const RA0: f32  = -9.864_944_033_3_e-03; /* 0x_bc21_a093 */
const RA1: f32  = -6.938_585_639_e-01; /* 0x_bf31_a0b7 */
const RA2: f32  = -1.055_862_617_5_e+01; /* 0x_c128_f022 */
const RA3: f32  = -6.237_533_187_9_e+01; /* 0x_c279_8057 */
const RA4: f32  = -1.623_966_674_8_e+02; /* 0x_c322_658c */
const RA5: f32  = -1.846_050_872_8_e+02; /* 0x_c338_9ae7 */
const RA6: f32  = -8.128_743_743_9_e+01; /* 0x_c2a2_932b */
const RA7: f32  = -9.814_329_147_3; /* 0x_c11d_077e */
const SA1: f32  =  1.965_127_182_e+01; /* 0x_419d_35ce */
const SA2: f32  =  1.376_577_606_2_e+02; /* 0x_4309_a863 */
const SA3: f32  =  4.345_658_874_5_e+02; /* 0x_43d9_486f */
const SA4: f32  =  6.453_872_680_7_e+02; /* 0x_4421_58c9 */
const SA5: f32  =  4.290_081_481_9_e+02; /* 0x_43d6_810b */
const SA6: f32  =  1.086_350_021_4_e+02; /* 0x_42d9_451f */
const SA7: f32  =  6.570_249_557_5; /* 0x_40d2_3f7c */
const SA8: f32  = -6.042_441_353_2_e-02; /* 0x_bd77_7f97 */
/*
 * Coefficients for approximation to  erfc in [1/.35,28]
 */
const RB0: f32  = -9.864_943_102_e-03; /* 0x_bc21_a092 */
const RB1: f32  = -7.992_832_660_7_e-01; /* 0x_bf4c_9dd4 */
const RB2: f32  = -1.775_795_555_1_e+01; /* 0x_c18e_104b */
const RB3: f32  = -1.606_363_830_6_e+02; /* 0x_c320_a2ea */
const RB4: f32  = -6.375_664_672_9_e+02; /* 0x_c41f_6441 */
const RB5: f32  = -1.025_095_092_8_e+03; /* 0x_c480_230b */
const RB6: f32  = -4.835_191_955_6_e+02; /* 0x_c3f1_c275 */
const SB1: f32  =  3.033_806_037_9_e+01; /* 0x_41f2_b459 */
const SB2: f32  =  3.257_925_109_9_e+02; /* 0x_43a2_e571 */
const SB3: f32  =  1.536_729_614_3_e+03; /* 0x_44c0_1759 */
const SB4: f32  =  3.199_858_154_3_e+03; /* 0x_4547_fdbb */
const SB5: f32  =  2.553_050_293_0_e+03; /* 0x_451f_90ce */
const SB6: f32  =  4.745_285_339_4_e+02; /* 0x_43ed_43a7 */
const SB7: f32  = -2.244_095_230_1_e+01; /* 0x_c1b3_8712 */

fn erfc1(x: f32) -> f32 {
    let s: f32;
    let p: f32;
    let q: f32;

    s = fabsf(x) - 1.;
    p = PA0+s*(PA1+s*(PA2+s*(PA3+s*(PA4+s*(PA5+s*PA6)))));
    q = 1.+s*(QA1+s*(QA2+s*(QA3+s*(QA4+s*(QA5+s*QA6)))));
    1. - ERX - p/q
}

fn erfc2(mut ix: u32, mut x: f32) -> f32 {
    let s: f32;
    let r: f32;
    let big_s: f32;
    let z: f32;

    if ix < 0x_3fa0_0000 { /* |x| < 1.25 */
        return erfc1(x);
    }

    x = fabsf(x);
    s = 1./(x*x);
    if ix < 0x_4036_db6d {   /* |x| < 1/0.35 */
        r = RA0+s*(RA1+s*(RA2+s*(RA3+s*(RA4+s*(
             RA5+s*(RA6+s*RA7))))));
        big_s = 1.+s*(SA1+s*(SA2+s*(SA3+s*(SA4+s*(
             SA5+s*(SA6+s*(SA7+s*SA8)))))));
    } else {                 /* |x| >= 1/0.35 */
        r = RB0+s*(RB1+s*(RB2+s*(RB3+s*(RB4+s*(
             RB5+s*RB6)))));
        big_s = 1.+s*(SB1+s*(SB2+s*(SB3+s*(SB4+s*(
             SB5+s*(SB6+s*SB7))))));
    }
    ix = x.to_bits();
    z = f32::from_bits(ix&0x_ffff_e000);

    expf(-z*z - 0.5625) * expf((z-x)*(z+x) + r/big_s)/x
}

pub fn erff(x: f32) -> f32
{
    let r: f32;
    let s: f32;
    let z: f32;
    let y: f32;
    let mut ix: u32;
    let sign: usize;

    ix = x.to_bits();
    sign = (ix>>31) as usize;
    ix &= UF_ABS;
    if ix >= UF_INF {
        /* erf(nan)=nan, erf(+-inf)=+-1 */
        return 1.-2.*(sign as f32) + 1./x;
    }
    if ix < 0x_3f58_0000 {  /* |x| < 0.84375 */
        if ix < 0x_3180_0000 {  /* |x| < 2**-28 */
            /*avoid underflow */
            return 0.125*(8.*x + EFX8*x);
        }
        z = x*x;
        r = PP0+z*(PP1+z*(PP2+z*(PP3+z*PP4)));
        s = 1.+z*(QQ1+z*(QQ2+z*(QQ3+z*(QQ4+z*QQ5))));
        y = r/s;
        return x + x*y;
    }
    if ix < 0x_40c0_0000 {  /* |x| < 6 */
        y = 1. - erfc2(ix,x);
    } else {
        let x1p_120 = f32::from_bits(0x_0380_0000);
        y = 1. - x1p_120;
    }

    if sign != 0 {
        -y
    } else {
        y
    }
}

pub fn erfcf(x: f32) -> f32 {
    let r: f32;
    let s: f32;
    let z: f32;
    let y: f32;
    let mut ix: u32;
    let sign: usize;

    ix = x.to_bits();
    sign = (ix>>31) as usize;
    ix &= UF_ABS;
    if ix >= UF_INF {
        /* erfc(nan)=nan, erfc(+-inf)=0,2 */
        return 2.*(sign as f32) + 1./x;
    }

    if ix < 0x_3f58_0000 {  /* |x| < 0.84375 */
        if ix < 0x_2380_0000 { /* |x| < 2**-56 */
            return 1. - x;
        }
        z = x*x;
        r = PP0+z*(PP1+z*(PP2+z*(PP3+z*PP4)));
        s = 1.+z*(QQ1+z*(QQ2+z*(QQ3+z*(QQ4+z*QQ5))));
        y = r/s;
        if sign != 0 || ix < 0x_3e80_0000 {  /* x < 1/4 */
            return 1. - (x+x*y);
        }
        return 0.5 - (x - 0.5 + x*y);
    }
    if ix < 0x_41e0_0000 {  /* |x| < 28 */
        if sign != 0 {
            return 2. - erfc2(ix, x);
        } else {
            return erfc2(ix, x);
        }
    }

    let x1p_120 = f32::from_bits(0x_0380_0000);
    if sign != 0 {
        2. - x1p_120
    } else {
        x1p_120*x1p_120
    }
}
