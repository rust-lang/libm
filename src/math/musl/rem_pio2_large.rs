#![allow(unused_unsafe)]
/* origin: FreeBSD /usr/src/lib/msun/src/k_rem_pio2.c */
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

use super::floor;
use super::scalbn;

// initial value for jk
const INIT_JK: [usize; 4] = [3, 4, 4, 6];

// Table of constants for 2/pi, 396 Hex digits (476 decimal) of 2/pi
//
//              integer array, contains the (24*i)-th to (24*i+23)-th
//              bit of 2/pi after binary point. The corresponding
//              floating value is
//
//                      ipio2[i] * 2^(-24(i+1)).
//
// NB: This table must have at least (e0-3)/24 + jk terms.
//     For quad precision (e0 <= 16360, jk = 6), this is 686.
#[cfg(target_pointer_width = "32")]
#[rustfmt::skip]
const IPIO2: [i32; 66] = [
    0x_00A2_F983, 0x_006E_4E44, 0x_0015_29FC, 0x_0027_57D1,
    0x_00F5_34DD, 0x_00C0_DB62, 0x_0095_993C, 0x_0043_9041,
    0x_00FE_5163, 0x_00AB_DEBB, 0x_00C5_61B7, 0x_0024_6E3A,
    0x_0042_4DD2, 0x_00E0_0649, 0x_002E_EA09, 0x_00D1_921C,
    0x_00FE_1DEB, 0x_001C_B129, 0x_00A7_3EE8, 0x_0082_35F5,
    0x_002E_BB44, 0x_0084_E99C, 0x_0070_26B4, 0x_005F_7E41,
    0x_0039_91D6, 0x_0039_8353, 0x_0039_F49C, 0x_0084_5F8B,
    0x_00BD_F928, 0x_003B_1FF8, 0x_0097_FFDE, 0x_0005_980F,
    0x_00EF_2F11, 0x_008B_5A0A, 0x_006D_1F6D, 0x_0036_7ECF,
    0x_0027_CB09, 0x_00B7_4F46, 0x_003F_669E, 0x_005F_EA2D,
    0x_0075_27BA, 0x_00C7_EBE5, 0x_00F1_7B3D, 0x_0007_39F7,
    0x_008A_5292, 0x_00EA_6BFB, 0x_005F_B11F, 0x_008D_5D08,
    0x_0056_0330, 0x_0046_FC7B, 0x_006B_ABF0, 0x_00CF_BC20,
    0x_009A_F436, 0x_001D_A9E3, 0x_0091_615E, 0x_00E6_1B08,
    0x_0065_9985, 0x_005F_14A0, 0x_0068_408D, 0x_00FF_D880,
    0x_004D_7327, 0x_0031_0606, 0x_0015_56CA, 0x_0073_A8C9,
    0x_0060_E27B, 0x_00C0_8C6B,
];

#[cfg(target_pointer_width = "64")]
#[rustfmt::skip]
const IPIO2: [i32; 690] = [
    0x_00A2_F983, 0x_006E_4E44, 0x_0015_29FC, 0x_0027_57D1,
    0x_00F5_34DD, 0x_00C0_DB62, 0x_0095_993C, 0x_0043_9041,
    0x_00FE_5163, 0x_00AB_DEBB, 0x_00C5_61B7, 0x_0024_6E3A,
    0x_0042_4DD2, 0x_00E0_0649, 0x_002E_EA09, 0x_00D1_921C,
    0x_00FE_1DEB, 0x_001C_B129, 0x_00A7_3EE8, 0x_0082_35F5,
    0x_002E_BB44, 0x_0084_E99C, 0x_0070_26B4, 0x_005F_7E41,
    0x_0039_91D6, 0x_0039_8353, 0x_0039_F49C, 0x_0084_5F8B,
    0x_00BD_F928, 0x_003B_1FF8, 0x_0097_FFDE, 0x_0005_980F,
    0x_00EF_2F11, 0x_008B_5A0A, 0x_006D_1F6D, 0x_0036_7ECF,
    0x_0027_CB09, 0x_00B7_4F46, 0x_003F_669E, 0x_005F_EA2D,
    0x_0075_27BA, 0x_00C7_EBE5, 0x_00F1_7B3D, 0x_0007_39F7,
    0x_008A_5292, 0x_00EA_6BFB, 0x_005F_B11F, 0x_008D_5D08,
    0x_0056_0330, 0x_0046_FC7B, 0x_006B_ABF0, 0x_00CF_BC20,
    0x_009A_F436, 0x_001D_A9E3, 0x_0091_615E, 0x_00E6_1B08,
    0x_0065_9985, 0x_005F_14A0, 0x_0068_408D, 0x_00FF_D880,
    0x_004D_7327, 0x_0031_0606, 0x_0015_56CA, 0x_0073_A8C9,
    0x_0060_E27B, 0x_00C0_8C6B, 0x_0047_C419, 0x_00C3_67CD,
    0x_00DC_E809, 0x_002A_8359, 0x_00C4_768B, 0x_0096_1CA6,
    0x_00DD_AF44, 0x_00D1_5719, 0x_0005_3EA5, 0x_00FF_0705,
    0x_003F_7E33, 0x_00E8_32C2, 0x_00DE_4F98, 0x_0032_7DBB,
    0x_00C3_3D26, 0x_00EF_6B1E, 0x_005E_F89F, 0x_003A_1F35,
    0x_00CA_F27F, 0x_001D_87F1, 0x_0021_907C, 0x_007C_246A,
    0x_00FA_6ED5, 0x_0077_2D30, 0x_0043_3B15, 0x_00C6_14B5,
    0x_009D_19C3, 0x_00C2_C4AD, 0x_0041_4D2C, 0x_005D_000C,
    0x_0046_7D86, 0x_002D_71E3, 0x_009A_C69B, 0x_0000_6233,
    0x_007C_D2B4, 0x_0097_A7B4, 0x_00D5_5537, 0x_00F6_3ED7,
    0x_0018_10A3, 0x_00FC_764D, 0x_002A_9D64, 0x_00AB_D770,
    0x_00F8_7C63, 0x_0057_B07A, 0x_00E7_1517, 0x_0056_49C0,
    0x_00D9_D63B, 0x_0038_84A7, 0x_00CB_2324, 0x_0077_8AD6,
    0x_0023_545A, 0x_00B9_1F00, 0x_001B_0AF1, 0x_00DF_CE19,
    0x_00FF_319F, 0x_006A_1E66, 0x_0061_5799, 0x_0047_FBAC,
    0x_00D8_7F7E, 0x_00B7_6522, 0x_0089_E832, 0x_0060_BFE6,
    0x_00CD_C4EF, 0x_0009_366C, 0x_00D4_3F5D, 0x_00D7_DE16,
    0x_00DE_3B58, 0x_0092_9BDE, 0x_0028_22D2, 0x_00E8_8628,
    0x_004D_58E2, 0x_0032_CAC6, 0x_0016_E308, 0x_00CB_7DE0,
    0x_0050_C017, 0x_00A7_1DF3, 0x_005B_E018, 0x_0034_132E,
    0x_0062_1283, 0x_0001_4883, 0x_005B_8EF5, 0x_007F_B0AD,
    0x_00F2_E91E, 0x_0043_4A48, 0x_00D3_6710, 0x_00D8_DDAA,
    0x_0042_5FAE, 0x_00CE_616A, 0x_00A4_280A, 0x_00B4_99D3,
    0x_00F2_A606, 0x_007F_775C, 0x_0083_C2A3, 0x_0088_3C61,
    0x_0078_738A, 0x_005A_8CAF, 0x_00BD_D76F, 0x_0063_A62D,
    0x_00CB_BFF4, 0x_00EF_818D, 0x_0067_C126, 0x_0045_CA55,
    0x_0036_D9CA, 0x_00D2_A828, 0x_008D_61C2, 0x_0077_C912,
    0x_0014_2604, 0x_009B_4612, 0x_00C4_59C4, 0x_0044_C5C8,
    0x_0091_B24D, 0x_00F3_1700, 0x_00AD_43D4, 0x_00E5_4929,
    0x_0010_D5FD, 0x_00FC_BE00, 0x_00CC_941E, 0x_00EE_CE70,
    0x_00F5_3E13, 0x_0080_F1EC, 0x_00C3_E7B3, 0x_0028_F8C7,
    0x_0094_0593, 0x_003E_71C1, 0x_00B3_092E, 0x_00F3_450B,
    0x_009C_1288, 0x_007B_20AB, 0x_009F_B52E, 0x_00C2_9247,
    0x_002F_327B, 0x_006D_550C, 0x_0090_A772, 0x_001F_E76B,
    0x_0096_CB31, 0x_004A_1679, 0x_00E2_7941, 0x_0089_DFF4,
    0x_0097_94E8, 0x_0084_E6E2, 0x_0097_3199, 0x_006B_ED88,
    0x_0036_5F5F, 0x_000E_FDBB, 0x_00B4_9A48, 0x_006C_A467,
    0x_0042_7271, 0x_0032_5D8D, 0x_00B8_159F, 0x_0009_E5BC,
    0x_0025_318D, 0x_0039_74F7, 0x_001C_0530, 0x_0001_0C0D,
    0x_0068_084B, 0x_0058_EE2C, 0x_0090_AA47, 0x_0002_E774,
    0x_0024_D6BD, 0x_00A6_7DF7, 0x_0072_486E, 0x_00EF_169F,
    0x_00A6_948E, 0x_00F6_91B4, 0x_0051_53D1, 0x_00F2_0ACF,
    0x_0033_9820, 0x_007E_4BF5, 0x_0068_63B2, 0x_005F_3EDD,
    0x_0003_5D40, 0x_007F_8985, 0x_0029_5255, 0x_00C0_6437,
    0x_0010_D86D, 0x_0032_4832, 0x_0075_4C5B, 0x_00D4_714E,
    0x_006E_5445, 0x_00C1_090B, 0x_0069_F52A, 0x_00D5_6614,
    0x_009D_0727, 0x_0050_045D, 0x_00DB_3BB4, 0x_00C5_76EA,
    0x_0017_F987, 0x_007D_6B49, 0x_00BA_271D, 0x_0029_6996,
    0x_00AC_CCC6, 0x_0054_14AD, 0x_006A_E290, 0x_0089_D988,
    0x_0050_722C, 0x_00BE_A404, 0x_0094_0777, 0x_0070_30F3,
    0x_0027_FC00, 0x_00A8_71EA, 0x_0049_C266, 0x_003D_E064,
    0x_0083_DD97, 0x_0097_3FA3, 0x_00FD_9443, 0x_008C_860D,
    0x_00DE_4131, 0x_009D_3992, 0x_008C_70DD, 0x_00E7_B717,
    0x_003B_DF08, 0x_002B_3715, 0x_00A0_805C, 0x_0093_805A,
    0x_0092_1110, 0x_00D8_E80F, 0x_00AF_806C, 0x_004B_FFDB,
    0x_000F_9038, 0x_0076_1859, 0x_0015_A562, 0x_00BB_CB61,
    0x_00B9_89C7, 0x_00BD_4010, 0x_0004_F2D2, 0x_0027_7549,
    0x_00F6_B6EB, 0x_00BB_22DB, 0x_00AA_140A, 0x_002F_2689,
    0x_0076_8364, 0x_0033_3B09, 0x_001A_940E, 0x_00AA_3A51,
    0x_00C2_A31D, 0x_00AE_EDAF, 0x_0012_265C, 0x_004D_C26D,
    0x_009C_7A2D, 0x_0097_56C0, 0x_0083_3F03, 0x_00F6_F009,
    0x_008C_402B, 0x_0099_316D, 0x_0007_B439, 0x_0015_200C,
    0x_005B_C3D8, 0x_00C4_92F5, 0x_004B_ADC6, 0x_00A5_CA4E,
    0x_00CD_37A7, 0x_0036_A9E6, 0x_0094_92AB, 0x_0068_42DD,
    0x_00DE_6319, 0x_00EF_8C76, 0x_0052_8B68, 0x_0037_DBFC,
    0x_00AB_A1AE, 0x_0031_15DF, 0x_00A1_AE00, 0x_00DA_FB0C,
    0x_0066_4D64, 0x_00B7_05ED, 0x_0030_6529, 0x_00BF_5657,
    0x_003A_FF47, 0x_00B9_F96A, 0x_00F3_BE75, 0x_00DF_9328,
    0x_0030_80AB, 0x_00F6_8C66, 0x_0015_CB04, 0x_0006_22FA,
    0x_001D_E4D9, 0x_00A4_B33D, 0x_008F_1B57, 0x_0009_CD36,
    0x_00E9_424E, 0x_00A4_BE13, 0x_00B5_2333, 0x_001A_AAF0,
    0x_00A8_654F, 0x_00A5_C1D2, 0x_000F_3F0B, 0x_00CD_785B,
    0x_0076_F923, 0x_0004_8B7B, 0x_0072_1789, 0x_0053_A6C6,
    0x_00E2_6E6F, 0x_0000_EBEF, 0x_0058_4A9B, 0x_00B7_DAC4,
    0x_00BA_66AA, 0x_00CF_CF76, 0x_001D_02D1, 0x_002D_F1B1,
    0x_00C1_998C, 0x_0077_ADC3, 0x_00DA_4886, 0x_00A0_5DF7,
    0x_00F4_80C6, 0x_002F_F0AC, 0x_009A_ECDD, 0x_00BC_5C3F,
    0x_006D_DED0, 0x_001F_C790, 0x_00B6_DB2A, 0x_003A_25A3,
    0x_009A_AF00, 0x_0093_53AD, 0x_0004_57B6, 0x_00B4_2D29,
    0x_007E_804B, 0x_00A7_07DA, 0x_000E_AA76, 0x_00A1_597B,
    0x_002A_1216, 0x_002D_B7DC, 0x_00FD_E5FA, 0x_00FE_DB89,
    0x_00FD_BE89, 0x_006C_76E4, 0x_00FC_A906, 0x_0070_803E,
    0x_0015_6E85, 0x_00FF_87FD, 0x_0007_3E28, 0x_0033_6761,
    0x_0086_182A, 0x_00EA_BD4D, 0x_00AF_E7B3, 0x_006E_6D8F,
    0x_0039_6795, 0x_005B_BF31, 0x_0048_D784, 0x_0016_DF30,
    0x_0043_2DC7, 0x_0035_6125, 0x_00CE_70C9, 0x_00B8_CB30,
    0x_00FD_6CBF, 0x_00A2_00A4, 0x_00E4_6C05, 0x_00A0_DD5A,
    0x_0047_6F21, 0x_00D2_1262, 0x_0084_5CB9, 0x_0049_6170,
    0x_00E0_566B, 0x_0001_5299, 0x_0037_5550, 0x_00B7_D51E,
    0x_00C4_F133, 0x_005F_6E13, 0x_00E4_305D, 0x_00A9_2E85,
    0x_00C3_B21D, 0x_0036_32A1, 0x_00A4_B708, 0x_00D4_B1EA,
    0x_0021_F716, 0x_00E4_698F, 0x_0077_FF27, 0x_0080_030C,
    0x_002D_408D, 0x_00A0_CD4F, 0x_0099_A520, 0x_00D3_A2B3,
    0x_000A_5D2F, 0x_0042_F9B4, 0x_00CB_DA11, 0x_00D0_BE7D,
    0x_00C1_DB9B, 0x_00BD_17AB, 0x_0081_A2CA, 0x_005C_6A08,
    0x_0017_552E, 0x_0055_0027, 0x_00F0_147F, 0x_0086_07E1,
    0x_0064_0B14, 0x_008D_4196, 0x_00DE_BE87, 0x_002A_FDDA,
    0x_00B6_256B, 0x_0034_897B, 0x_00FE_F305, 0x_009E_BFB9,
    0x_004F_6A68, 0x_00A8_2A4A, 0x_005A_C44F, 0x_00BC_F82D,
    0x_0098_5AD7, 0x_0095_C7F4, 0x_008D_4D0D, 0x_00A6_3A20,
    0x_005F_57A4, 0x_00B1_3F14, 0x_0095_3880, 0x_0001_20CC,
    0x_0086_DD71, 0x_00B6_DEC9, 0x_00F5_60BF, 0x_0011_654D,
    0x_006B_0701, 0x_00AC_B08C, 0x_00D0_C0B2, 0x_0048_5551,
    0x_000E_FB1E, 0x_00C3_7295, 0x_003B_06A3, 0x_0035_40C0,
    0x_007B_DC06, 0x_00CC_45E0, 0x_00FA_294E, 0x_00C8_CAD6,
    0x_0041_F3E8, 0x_00DE_647C, 0x_00D8_649B, 0x_0031_BED9,
    0x_00C3_97A4, 0x_00D4_5877, 0x_00C5_E369, 0x_0013_DAF0,
    0x_003C_3ABA, 0x_0046_1846, 0x_005F_7555, 0x_00F5_BDD2,
    0x_00C6_926E, 0x_005D_2EAC, 0x_00ED_440E, 0x_0042_3E1C,
    0x_0087_C461, 0x_00E9_FD29, 0x_00F3_D6E7, 0x_00CA_7C22,
    0x_0035_916F, 0x_00C5_E008, 0x_008D_D7FF, 0x_00E2_6A6E,
    0x_00C6_FDB0, 0x_00C1_0893, 0x_0074_5D7C, 0x_00B2_AD6B,
    0x_009D_6ECD, 0x_007B_723E, 0x_006A_11C6, 0x_00A9_CFF7,
    0x_00DF_7329, 0x_00BA_C9B5, 0x_0051_00B7, 0x_000D_B2E2,
    0x_0024_BA74, 0x_0060_7DE5, 0x_008A_D874, 0x_002C_150D,
    0x_000C_1881, 0x_0094_667E, 0x_0016_2901, 0x_0076_7A9F,
    0x_00BE_FDFD, 0x_00EF_4556, 0x_0036_7ED9, 0x_0013_D9EC,
    0x_00B9_BA8B, 0x_00FC_97C4, 0x_0027_A831, 0x_00C3_6EF1,
    0x_0036_C594, 0x_0056_A8D8, 0x_00B5_A8B4, 0x_000E_CCCF,
    0x_002D_8912, 0x_0034_576F, 0x_0089_562C, 0x_00E3_CE99,
    0x_00B9_20D6, 0x_00AA_5E6B, 0x_009C_2A3E, 0x_00CC_5F11,
    0x_004A_0BFD, 0x_00FB_F4E1, 0x_006D_3B8E, 0x_002C_86E2,
    0x_0084_D4E9, 0x_00A9_B4FC, 0x_00D1_EEEF, 0x_00C9_352E,
    0x_0061_392F, 0x_0044_2138, 0x_00C8_D91B, 0x_000A_FC81,
    0x_006A_4AFB, 0x_00D8_1C2F, 0x_0084_B453, 0x_008C_994E,
    0x_00CC_2254, 0x_00DC_552A, 0x_00D6_C6C0, 0x_0096_190B,
    0x_00B8_701A, 0x_0064_9569, 0x_0060_5A26, 0x_00EE_523F,
    0x_000F_117F, 0x_0011_B5F4, 0x_00F5_CBFC, 0x_002D_BC34,
    0x_00EE_BC34, 0x_00CC_5DE8, 0x_0060_5EDD, 0x_009B_8E67,
    0x_00EF_3392, 0x_00B8_17C9, 0x_009B_5861, 0x_00BC_57E1,
    0x_00C6_8351, 0x_0010_3ED8, 0x_0048_71DD, 0x_00DD_1C2D,
    0x_00A1_18AF, 0x_0046_2C21, 0x_00D7_F359, 0x_0098_7AD9,
    0x_00C0_549E, 0x_00FA_864F, 0x_00FC_0656, 0x_00AE_79E5,
    0x_0036_2289, 0x_0022_AD38, 0x_00DC_9367, 0x_00AA_E855,
    0x_0038_2682, 0x_009B_E7CA, 0x_00A4_0D51, 0x_00B1_3399,
    0x_000E_D7A9, 0x_0048_0569, 0x_00F0_B265, 0x_00A7_887F,
    0x_0097_4C88, 0x_0036_D1F9, 0x_00B3_9221, 0x_004A_827B,
    0x_0021_CF98, 0x_00DC_9F40, 0x_0055_47DC, 0x_003A_74E1,
    0x_0042_EB67, 0x_00DF_9DFE, 0x_005F_D45E, 0x_00A4_677B,
    0x_007A_ACBA, 0x_00A2_F655, 0x_0023_882B, 0x_0055_BA41,
    0x_0008_6E59, 0x_0086_2A21, 0x_0083_4739, 0x_00E6_E389,
    0x_00D4_9EE5, 0x_0040_FB49, 0x_00E9_56FF, 0x_00CA_0F1C,
    0x_008A_59C5, 0x_002B_FA94, 0x_00C5_C1D3, 0x_00CF_C50F,
    0x_00AE_5ADB, 0x_0086_C547, 0x_0062_4385, 0x_003B_8621,
    0x_0094_792C, 0x_0087_6110, 0x_007B_4C2A, 0x_001A_2C80,
    0x_0012_BF43, 0x_0090_2688, 0x_0089_3C78, 0x_00E4_C4A8,
    0x_007B_DBE5, 0x_00C2_3AC4, 0x_00EA_F426, 0x_008A_67F7,
    0x_00BF_920D, 0x_002B_A365, 0x_00B1_933D, 0x_000B_7CBD,
    0x_00DC_51A4, 0x_0063_DD27, 0x_00DD_E169, 0x_0019_949A,
    0x_0095_29A8, 0x_0028_CE68, 0x_00B4_ED09, 0x_0020_9F44,
    0x_00CA_984E, 0x_0063_8270, 0x_0023_7C7E, 0x_0032_B90F,
    0x_008E_F5A7, 0x_00E7_5614, 0x_0008_F121, 0x_002A_9DB5,
    0x_004D_7E6F, 0x_0051_19A5, 0x_00AB_F9B5, 0x_00D6_DF82,
    0x_0061_DD96, 0x_0002_3616, 0x_009F_3AC4, 0x_00A1_A283,
    0x_006D_ED72, 0x_007A_8D39, 0x_00A9_B882, 0x_005C_326B,
    0x_005B_2746, 0x_00ED_3400, 0x_0077_00D2, 0x_0055_F4FC,
    0x_004D_5901, 0x_0080_71E0,
];

const PIO2: [f64; 8] = [
    1.570_796_251_296_997_070_31,      /* 0x_3FF9_21FB, 0x_4000_0000 */
    7.549_789_415_861_596_353_35_e-08, /* 0x_3E74_442D, 0x_0000_0000 */
    5.390_302_529_957_764_765_54_e-15, /* 0x_3CF8_4698, 0x_8000_0000 */
    3.282_003_415_807_912_941_23_e-22, /* 0x_3B78_CC51, 0x_6000_0000 */
    1.270_655_753_080_676_073_49_e-29, /* 0x_39F0_1B83, 0x_8000_0000 */
    1.229_333_089_811_113_289_32_e-36, /* 0x_387A_2520, 0x_4000_0000 */
    2.733_700_538_164_645_596_24_e-44, /* 0x_36E3_8222, 0x_8000_0000 */
    2.167_416_838_778_048_194_44_e-51, /* 0x_3569_F31D, 0x_0000_0000 */
];

// fn rem_pio2_large(x: &[f64], e0: i32, prec: usize) -> (i32, [f64; 3])
//
// Input parameters:
//      x[]     The input value (must be positive) is broken into nx
//              pieces of 24-bit integers in double precision format.
//              x[i] will be the i-th 24 bit of x. The scaled exponent
//              of x[0] is given in input parameter e0 (i.e., x[0]*2^e0
//              match x's up to 24 bits.
//
//              Example of breaking a double positive z into x[0]+x[1]+x[2]:
//                      e0 = ilogb(z)-23
//                      z  = scalbn(z,-e0)
//              for i = 0,1,2
//                      x[i] = floor(z)
//                      z    = (z-x[i])*2**24
//
//      y[]     ouput result in an array of double precision numbers.
//              The dimension of y[] is:
//                      24-bit  precision       1
//                      53-bit  precision       2
//                      64-bit  precision       2
//                      113-bit precision       3
//              The actual value is the sum of them. Thus for 113-bit
//              precison, one may have to do something like:
//
//              long double t,w,r_head, r_tail;
//              t = (long double)y[2] + (long double)y[1];
//              w = (long double)y[0];
//              r_head = t+w;
//              r_tail = w - (r_head - t);
//
//      e0      The exponent of x[0]. Must be <= 16360 or you need to
//              expand the ipio2 table.
//
//      prec    an integer indicating the precision:
//                      0       24  bits (single)
//                      1       53  bits (double)
//                      2       64  bits (extended)
//                      3       113 bits (quad)
//
// Here is the description of some local variables:
//
//      jk      jk+1 is the initial number of terms of ipio2[] needed
//              in the computation. The minimum and recommended value
//              for jk is 3,4,4,6 for single, double, extended, and quad.
//              jk+1 must be 2 larger than you might expect so that our
//              recomputation test works. (Up to 24 bits in the integer
//              part (the 24 bits of it that we compute) and 23 bits in
//              the fraction part may be lost to cancelation before we
//              recompute.)
//
//      jz      local integer variable indicating the number of
//              terms of ipio2[] used.
//
//      jx      nx - 1
//
//      jv      index for pointing to the suitable ipio2[] for the
//              computation. In general, we want
//                      ( 2^e0*x[0] * ipio2[jv-1]*2^(-24jv) )/8
//              is an integer. Thus
//                      e0-3-24*jv >= 0 or (e0-3)/24 >= jv
//              Hence jv = max(0,(e0-3)/24).
//
//      jp      jp+1 is the number of terms in PIo2[] needed, jp = jk.
//
//      q[]     double array with integral value, representing the
//              24-bits chunk of the product of x and 2/pi.
//
//      q0      the corresponding exponent of q[0]. Note that the
//              exponent for q[i] would be q0-24*i.
//
//      PIo2[]  double precision array, obtained by cutting pi/2
//              into 24 bits chunks.
//
//      f[]     ipio2[] in floating point
//
//      iq[]    integer array by breaking up q[] in 24-bits chunk.
//
//      fq[]    final product of x*(2/pi) in fq[0],..,fq[jk]
//
//      ih      integer. If >0 it indicates q[] is >= 0.5, hence
//              it also indicates the *sign* of the result.

/// Return the last three digits of N with y = x - N*pi/2
/// so that |y| < pi/2.
///
/// The method is to compute the integer (mod 8) and fraction parts of
/// (2/pi)*x without doing the full multiplication. In general we
/// skip the part of the product that are known to be a huge integer (
/// more accurately, = 0 mod 8 ). Thus the number of operations are
/// independent of the exponent of the input.

#[allow(clippy::cyclomatic_complexity)]
#[inline]
pub fn rem_pio2_large(x: &[f64], e0: i32, prec: usize) -> (i32, [f64; 3]) {
    let x1p24 = f64::from_bits(0x_4170_0000_0000_0000); // 0x1p24 === 2 ^ 24
    let x1p_24 = f64::from_bits(0x_3e70_0000_0000_0000); // 0x1p_24 === 2 ^ (-24)

    #[cfg(target_pointer_width = "64")]
    assert!(e0 <= 16360);

    let nx = x.len();

    let mut fw: f64;
    let mut n: i32;
    let mut ih: i32;
    let mut z: f64;
    let mut f: [f64; 20] = [0.; 20];
    let mut fq: [f64; 20] = [0.; 20];
    let mut q: [f64; 20] = [0.; 20];
    let mut iq: [i32; 20] = [0; 20];

    /* initialize jk*/
    let jk = INIT_JK[prec];
    let jp = jk;

    /* determine jx,jv,q0, note that 3>q0 */
    let jx = nx - 1;
    let mut jv = (e0 - 3) / 24;
    if jv < 0 {
        jv = 0;
    }
    let mut q0 = e0 - 24 * (jv + 1);
    let jv = jv as usize;

    /* set up f[0] to f[jx+jk] where f[jx+jk] = ipio2[jv+jk] */
    let mut j = (jv - jx) as i32;
    let m = jx + jk;
    for i in 0..=m {
        i!(f, i, =, if j < 0 {
            0.
        } else {
            i!(IPIO2, j as usize) as f64
        });
        j += 1;
    }

    /* compute q[0],q[1],...q[jk] */
    for i in 0..=jk {
        fw = 0.;
        for j in 0..=jx {
            fw += i!(x, j) * i!(f, jx + i - j);
        }
        i!(q, i, =, fw);
    }

    let mut jz = jk;

    'recompute: loop {
        /* distill q[] into iq[] reversingly */
        let mut i = 0_i32;
        z = i!(q, jz);
        for j in (1..=jz).rev() {
            fw = (x1p_24 * z) as i32 as f64;
            i!(iq, i as usize, =, (z - x1p24 * fw) as i32);
            z = i!(q, j - 1) + fw;
            i += 1;
        }

        /* compute n */
        z = scalbn(z, q0); /* actual value of z */
        z -= 8. * floor(z * 0.125); /* trim off integer >= 8 */
        n = z as i32;
        z -= n as f64;
        ih = 0;
        if q0 > 0 {
            /* need iq[jz-1] to determine n */
            i = i!(iq, jz - 1) >> (24 - q0);
            n += i;
            i!(iq, jz - 1, -=, i << (24 - q0));
            ih = i!(iq, jz - 1) >> (23 - q0);
        } else if q0 == 0 {
            ih = i!(iq, jz - 1) >> 23;
        } else if z >= 0.5 {
            ih = 2;
        }

        if ih > 0 {
            /* q > 0.5 */
            n += 1;
            let mut carry = 0_i32;
            for i in 0..jz {
                /* compute 1-q */
                let j = i!(iq, i);
                if carry == 0 {
                    if j != 0 {
                        carry = 1;
                        i!(iq, i, =, 0x_0100_0000 - j);
                    }
                } else {
                    i!(iq, i, =, 0x_00ff_ffff - j);
                }
            }
            if q0 > 0 {
                /* rare case: chance is 1 in 12 */
                match q0 {
                    1 => {
                        i!(iq, jz - 1, &=, 0x_007f_ffff);
                    }
                    2 => {
                        i!(iq, jz - 1, &=, 0x_003f_ffff);
                    }
                    _ => {}
                }
            }
            if ih == 2 {
                z = 1. - z;
                if carry != 0 {
                    z -= scalbn(1., q0);
                }
            }
        }

        /* check if recomputation is needed */
        if z == 0. {
            let mut j = 0;
            for i in (jk..jz).rev() {
                j |= i!(iq, i);
            }
            if j == 0 {
                /* need recomputation */
                let mut k = 1;
                while i!(iq, jk - k, ==, 0) {
                    k += 1; /* k = no. of terms needed */
                }

                for i in (jz + 1)..=(jz + k) {
                    /* add q[jz+1] to q[jz+k] */
                    i!(f, jx + i, =, i!(IPIO2, jv + i) as f64);
                    fw = 0.;
                    for j in 0..=jx {
                        fw += i!(x, j) * i!(f, jx + i - j);
                    }
                    i!(q, i, =, fw);
                }
                jz += k;
                continue 'recompute;
            }
        }

        break;
    }

    /* chop off zero terms */
    if z == 0. {
        jz -= 1;
        q0 -= 24;
        while i!(iq, jz) == 0 {
            jz -= 1;
            q0 -= 24;
        }
    } else {
        /* break z into 24-bit if necessary */
        z = scalbn(z, -q0);
        if z >= x1p24 {
            fw = (x1p_24 * z) as i32 as f64;
            i!(iq, jz, =, (z - x1p24 * fw) as i32);
            jz += 1;
            q0 += 24;
            i!(iq, jz, =, fw as i32);
        } else {
            i!(iq, jz, =, z as i32);
        }
    }

    /* convert integer "bit" chunk to floating-point value */
    fw = scalbn(1., q0);
    for i in (0..=jz).rev() {
        i!(q, i, =, fw * (i!(iq, i) as f64));
        fw *= x1p_24;
    }

    /* compute PIo2[0,...,jp]*q[jz,...,0] */
    for i in (0..=jz).rev() {
        fw = 0.;
        let mut k = 0;
        while (k <= jp) && (k <= jz - i) {
            fw += i!(PIO2, k) * i!(q, i + k);
            k += 1;
        }
        i!(fq, jz - i, =, fw);
    }

    /* compress fq[] into y[] */
    let y = match prec {
        0 => {
            fw = 0.;
            for i in (0..=jz).rev() {
                fw += i!(fq, i);
            }
            [if ih == 0 { fw } else { -fw }, 0., 0.]
        }
        1 | 2 => {
            fw = 0.;
            for i in (0..=jz).rev() {
                fw += i!(fq, i);
            }
            // TODO: drop excess precision here once double_t is used
            fw = fw as f64;
            let y0 = if ih == 0 { fw } else { -fw };
            fw = i!(fq, 0) - fw;
            for i in 1..=jz {
                fw += i!(fq, i);
            }
            [y0, if ih == 0 { fw } else { -fw }, 0.]
        }
        3 => {
            /* painful */
            for i in (1..=jz).rev() {
                fw = i!(fq, i - 1) + i!(fq, i);
                i!(fq, i, +=, i!(fq, i - 1) - fw);
                i!(fq, i - 1, =, fw);
            }
            for i in (2..=jz).rev() {
                fw = i!(fq, i - 1) + i!(fq, i);
                i!(fq, i, +=, i!(fq, i - 1) - fw);
                i!(fq, i - 1, =, fw);
            }
            fw = 0.;
            for i in (2..=jz).rev() {
                fw += i!(fq, i);
            }
            if ih == 0 {
                [i!(fq, 0), i!(fq, 1), fw]
            } else {
                [-i!(fq, 0), -i!(fq, 1), -fw]
            }
        }
        #[cfg(feature = "checked")]
        _ => unreachable!(),
        #[cfg(not(feature = "checked"))]
        _ => [0., 0., 0.],
    };
    (n & 7, y)
}
