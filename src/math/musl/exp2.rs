// origin: FreeBSD /usr/src/lib/msun/src/s_exp2.c */
//-
// Copyright (c) 2005 David Schultz <das@FreeBSD.ORG>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.

use super::scalbn;
use crate::math::consts::*;

#[cfg(all(target_os = "cuda", not(feature = "stable")))]
use super::cuda_intrinsics;

const TBLSIZE: usize = 256;

#[rustfmt::skip]
static TBL: [u64; TBLSIZE * 2] = [
    //  exp2(z + eps)          eps
    0x_3fe6_a09e_667f_3d5d, 0x_3d39_8800_0000_0000,
    0x_3fe6_b052_fa75_1744, 0x_3cd8_0000_0000_0000,
    0x_3fe6_c012_750b_d9fe, 0x_bd28_7800_0000_0000,
    0x_3fe6_cfdc_ddd4_76bf, 0x_3d1e_c000_0000_0000,
    0x_3fe6_dfb2_3c65_1a29, 0x_bcd8_0000_0000_0000,
    0x_3fe6_ef92_9859_3ae3, 0x_bcbc_0000_0000_0000,
    0x_3fe6_ff7d_f951_9386, 0x_bd2f_d800_0000_0000,
    0x_3fe7_0f74_66f4_2da3, 0x_bd2c_8800_0000_0000,
    0x_3fe7_1f75_e8ec_5fc3, 0x_3d13_c000_0000_0000,
    0x_3fe7_2f82_86ea_cf05, 0x_bd38_3000_0000_0000,
    0x_3fe7_3f9a_48a5_8152, 0x_bd00_c000_0000_0000,
    0x_3fe7_4fbd_35d7_ccfc, 0x_3d2f_8800_0000_0000,
    0x_3fe7_5feb_5642_67f1, 0x_3d03_e000_0000_0000,
    0x_3fe7_7024_b1ab_6d48, 0x_bd27_d000_0000_0000,
    0x_3fe7_8069_4fde_5d38, 0x_bcdd_0000_0000_0000,
    0x_3fe7_90b9_38ac_1d00, 0x_3ce3_0000_0000_0000,
    0x_3fe7_a114_73eb_0178, 0x_bced_0000_0000_0000,
    0x_3fe7_b17b_0976_d060, 0x_3d20_4000_0000_0000,
    0x_3fe7_c1ed_0130_c133, 0x_3ca0_0000_0000_0000,
    0x_3fe7_d26a_62ff_8636, 0x_bd26_9000_0000_0000,
    0x_3fe7_e2f3_36cf_4e3b, 0x_bd02_e000_0000_0000,
    0x_3fe7_f387_8491_c3e8, 0x_bd24_5800_0000_0000,
    0x_3fe8_0427_543e_1b4e, 0x_3d33_0000_0000_0000,
    0x_3fe8_14d2_add1_071a, 0x_3d0f_0000_0000_0000,
    0x_3fe8_2589_994c_cd7e, 0x_bd21_c000_0000_0000,
    0x_3fe8_364c_1eb9_42d0, 0x_3d29_d000_0000_0000,
    0x_3fe8_471a_4623_cab5, 0x_3d47_1000_0000_0000,
    0x_3fe8_57f4_179f_5bbc, 0x_3d22_6000_0000_0000,
    0x_3fe8_68d9_9b44_91af, 0x_bd32_c400_0000_0000,
    0x_3fe8_79ca_d931_a395, 0x_bd23_0000_0000_0000,
    0x_3fe8_8ac7_d98a_65b8, 0x_bd2a_8000_0000_0000,
    0x_3fe8_9bd0_a478_5800, 0x_bced_0000_0000_0000,
    0x_3fe8_ace5_422a_a223, 0x_3d33_2800_0000_0000,
    0x_3fe8_be05_bad6_19fa, 0x_3d42_b400_0000_0000,
    0x_3fe8_cf32_16b5_4383, 0x_bd2e_d000_0000_0000,
    0x_3fe8_e06a_5e08_664c, 0x_bd20_5000_0000_0000,
    0x_3fe8_f1ae_9915_7807, 0x_3d28_2800_0000_0000,
    0x_3fe9_02fe_d028_2c0e, 0x_bd1c_b000_0000_0000,
    0x_3fe9_145b_0b91_ff96, 0x_bd05_e000_0000_0000,
    0x_3fe9_25c3_53aa_2ff9, 0x_3cf5_4000_0000_0000,
    0x_3fe9_3737_b0cd_c64a, 0x_3d17_2000_0000_0000,
    0x_3fe9_48b8_2b5f_98ae, 0x_bd09_0000_0000_0000,
    0x_3fe9_5a44_cbc8_52cb, 0x_3d25_6800_0000_0000,
    0x_3fe9_6bdd_9a76_6f21, 0x_bd36_d000_0000_0000,
    0x_3fe9_7d82_9fde_4e2a, 0x_bd01_0000_0000_0000,
    0x_3fe9_8f33_e47a_23a3, 0x_3d2d_0000_0000_0000,
    0x_3fe9_a0f1_70ca_0604, 0x_bd38_a400_0000_0000,
    0x_3fe9_b2bb_4d53_ff89, 0x_3d35_5c00_0000_0000,
    0x_3fe9_c491_82a3_f15b, 0x_3d26_b800_0000_0000,
    0x_3fe9_d674_194b_b8c5, 0x_bcec_0000_0000_0000,
    0x_3fe9_e863_19e3_238e, 0x_3d17_d000_0000_0000,
    0x_3fe9_fa5e_8d07_f302, 0x_3d16_4000_0000_0000,
    0x_3fea_0c66_7b5d_e54d, 0x_bcf5_0000_0000_0000,
    0x_3fea_1e7a_ed8e_b8f6, 0x_3d09_e000_0000_0000,
    0x_3fea_309b_ec4a_2e27, 0x_3d2a_d800_0000_0000,
    0x_3fea_42c9_8046_0a5d, 0x_bd1a_f000_0000_0000,
    0x_3fea_5503_b23e_259b, 0x_3d0b_6000_0000_0000,
    0x_3fea_674a_8af4_6213, 0x_3d38_8800_0000_0000,
    0x_3fea_799e_1330_b3a7, 0x_3d11_2000_0000_0000,
    0x_3fea_8bfe_53c1_2e8d, 0x_3d06_c000_0000_0000,
    0x_3fea_9e6b_5579_fcd2, 0x_bd29_b800_0000_0000,
    0x_3fea_b0e5_2135_6fb8, 0x_3d2b_7000_0000_0000,
    0x_3fea_c36b_bfd3_f381, 0x_3cd9_0000_0000_0000,
    0x_3fea_d5ff_3a3c_2780, 0x_3ce4_0000_0000_0000,
    0x_3fea_e89f_995a_d2a3, 0x_bd2c_9000_0000_0000,
    0x_3fea_fb4c_e622_f367, 0x_3d16_5000_0000_0000,
    0x_3feb_0e07_298d_b790, 0x_3d2f_d400_0000_0000,
    0x_3feb_20ce_6c9a_89a9, 0x_3d12_7000_0000_0000,
    0x_3feb_33a2_b84f_1a4b, 0x_3d4d_4700_0000_0000,
    0x_3feb_4684_15b7_47e7, 0x_bd38_3800_0000_0000,
    0x_3feb_5972_8de5_593a, 0x_3c98_0000_0000_0000,
    0x_3feb_6c6e_29f1_c56a, 0x_3d0a_d000_0000_0000,
    0x_3feb_7f76_f2fb_5e50, 0x_3cde_8000_0000_0000,
    0x_3feb_928c_f227_49b2, 0x_bd04_c000_0000_0000,
    0x_3feb_a5b0_30a1_0603, 0x_bd0d_7000_0000_0000,
    0x_3feb_b8e0_b79a_6f66, 0x_3d0d_9000_0000_0000,
    0x_3feb_cc1e_904b_c1ff, 0x_3d02_a000_0000_0000,
    0x_3feb_df69_c3f3_a16f, 0x_bd1f_7800_0000_0000,
    0x_3feb_f2c2_5bd7_1db8, 0x_bd10_a000_0000_0000,
    0x_3fec_0628_6141_b2e9, 0x_bd11_4000_0000_0000,
    0x_3fec_199b_dd85_52e0, 0x_3d0b_e000_0000_0000,
    0x_3fec_2d1c_d9fa_64ee, 0x_bd09_4000_0000_0000,
    0x_3fec_40ab_5fff_d02f, 0x_bd0e_d000_0000_0000,
    0x_3fec_5447_78fa_fd15, 0x_3d39_6600_0000_0000,
    0x_3fec_67f1_2e57_d0cb, 0x_bd1a_1000_0000_0000,
    0x_3fec_7ba8_8988_c1b6, 0x_bd58_4580_0000_0000,
    0x_3fec_8f6d_9406_e733, 0x_bd1a_4800_0000_0000,
    0x_3fec_a340_5751_c4df, 0x_3ccb_0000_0000_0000,
    0x_3fec_b720_dcef_9094, 0x_3d01_4000_0000_0000,
    0x_3fec_cb0f_2e6d_1689, 0x_3cf0_2000_0000_0000,
    0x_3fec_df0b_555d_c412, 0x_3cf3_6000_0000_0000,
    0x_3fec_f315_5b5b_ab3b, 0x_bd06_9000_0000_0000,
    0x_3fed_072d_4a07_89bc, 0x_3d09_a000_0000_0000,
    0x_3fed_1b53_2b08_c8fa, 0x_bd15_e000_0000_0000,
    0x_3fed_2f87_080d_8a85, 0x_3d1d_2800_0000_0000,
    0x_3fed_43c8_eaca_a203, 0x_3d01_a000_0000_0000,
    0x_3fed_5818_dcfb_a491, 0x_3cdf_0000_0000_0000,
    0x_3fed_6c76_e862_e6a1, 0x_bd03_a000_0000_0000,
    0x_3fed_80e3_16c9_834e, 0x_bd0c_d800_0000_0000,
    0x_3fed_955d_71ff_6090, 0x_3cf4_c000_0000_0000,
    0x_3fed_a9e6_03db_32ae, 0x_3cff_9000_0000_0000,
    0x_3fed_be7c_d63a_8325, 0x_3ce9_8000_0000_0000,
    0x_3fed_d321_f301_b445, 0x_bcf5_2000_0000_0000,
    0x_3fed_e7d5_641c_05bf, 0x_bd1d_7000_0000_0000,
    0x_3fed_fc97_337b_9aec, 0x_bd16_1400_0000_0000,
    0x_3fee_1167_6b19_7d5e, 0x_3d0b_4800_0000_0000,
    0x_3fee_2646_14f5_a3e7, 0x_3d40_ce00_0000_0000,
    0x_3fee_3b33_3b16_ee5c, 0x_3d0c_6800_0000_0000,
    0x_3fee_502e_e78b_3fb4, 0x_bd09_3000_0000_0000,
    0x_3fee_6539_2467_6d68, 0x_bce5_0000_0000_0000,
    0x_3fee_7a51_fbc7_4c44, 0x_bd07_f800_0000_0000,
    0x_3fee_8f79_77cd_b726, 0x_bcf3_7000_0000_0000,
    0x_3fee_a4af_a2a4_90e8, 0x_3ce5_d000_0000_0000,
    0x_3fee_b9f4_867c_cae4, 0x_3d16_1a00_0000_0000,
    0x_3fee_cf48_2d8e_680d, 0x_3cf5_5000_0000_0000,
    0x_3fee_e4aa_a218_8514, 0x_3cc6_4000_0000_0000,
    0x_3fee_fa1b_ee61_5a13, 0x_bcee_8000_0000_0000,
    0x_3fef_0f9c_1cb6_4106, 0x_bcfa_8800_0000_0000,
    0x_3fef_252b_376b_b963, 0x_bd2c_9000_0000_0000,
    0x_3fef_3ac9_48dd_7275, 0x_3caa_0000_0000_0000,
    0x_3fef_5076_5b6e_4524, 0x_bcf4_f000_0000_0000,
    0x_3fef_6632_7988_44fd, 0x_3cca_8000_0000_0000,
    0x_3fef_7bfd_ad9c_be38, 0x_3cfa_bc00_0000_0000,
    0x_3fef_91d8_0224_3c82, 0x_bcd4_6000_0000_0000,
    0x_3fef_a7c1_819e_908e, 0x_bd0b_0c00_0000_0000,
    0x_3fef_bdba_3692_d511, 0x_bcc0_e000_0000_0000,
    0x_3fef_d3c2_2b8f_7194, 0x_bd10_de80_0000_0000,
    0x_3fef_e9d9_6b2a_23ee, 0x_3cee_4300_0000_0000,
    0x_3ff0_0000_0000_0000, 0x0,
    0x_3ff0_0b1a_fa5a_bcbe, 0x_bcb3_4000_0000_0000,
    0x_3ff0_163d_a9fb_3303, 0x_bd12_1700_0000_0000,
    0x_3ff0_2168_143b_0282, 0x_3cba_4000_0000_0000,
    0x_3ff0_2c9a_3e77_806c, 0x_3cef_9800_0000_0000,
    0x_3ff0_37d4_2e11_bbca, 0x_bcc7_4000_0000_0000,
    0x_3ff0_4315_e86e_7f89, 0x_3cd8_3000_0000_0000,
    0x_3ff0_4e5f_72f6_5467, 0x_bd1a_3f00_0000_0000,
    0x_3ff0_59b0_d315_855a, 0x_bd02_8400_0000_0000,
    0x_3ff0_650a_0e3c_1f95, 0x_3cf1_6000_0000_0000,
    0x_3ff0_706b_29dd_f71a, 0x_3d15_2400_0000_0000,
    0x_3ff0_7bd4_2b72_a82d, 0x_bce9_a000_0000_0000,
    0x_3ff0_8745_1875_9bd0, 0x_3ce6_4000_0000_0000,
    0x_3ff0_92bd_f666_07c8, 0x_bd00_7800_0000_0000,
    0x_3ff0_9e3e_cac6_f383, 0x_bc98_0000_0000_0000,
    0x_3ff0_a9c7_9b1f_3930, 0x_3cff_a000_0000_0000,
    0x_3ff0_b558_6cf9_88fc, 0x_bcfa_c800_0000_0000,
    0x_3ff0_c0f1_45e4_6c8a, 0x_3cd9_c000_0000_0000,
    0x_3ff0_cc92_2b72_4816, 0x_3d05_2000_0000_0000,
    0x_3ff0_d83b_2339_5dd8, 0x_bcfa_d000_0000_0000,
    0x_3ff0_e3ec_32d3_d1f3, 0x_3d1b_ac00_0000_0000,
    0x_3ff0_efa5_5fdf_a9a6, 0x_bd04_e800_0000_0000,
    0x_3ff0_fb66_affe_d2f0, 0x_bd0d_3000_0000_0000,
    0x_3ff1_0730_28d7_234b, 0x_3cf1_5000_0000_0000,
    0x_3ff1_1301_d012_5b5b, 0x_3cec_0000_0000_0000,
    0x_3ff1_1edb_ab5e_2af9, 0x_3d16_bc00_0000_0000,
    0x_3ff1_2abd_c06c_31d5, 0x_3ce8_4000_0000_0000,
    0x_3ff1_36a8_14f2_047d, 0x_bd0e_d000_0000_0000,
    0x_3ff1_429a_aea9_2de9, 0x_3ce8_e000_0000_0000,
    0x_3ff1_4e95_934f_3138, 0x_3ceb_4000_0000_0000,
    0x_3ff1_5a98_c8a5_8e71, 0x_3d05_3000_0000_0000,
    0x_3ff1_66a4_5471_c3df, 0x_3d03_3800_0000_0000,
    0x_3ff1_72b8_3c7d_5211, 0x_3d28_d400_0000_0000,
    0x_3ff1_7ed4_8695_bb9f, 0x_bd05_d000_0000_0000,
    0x_3ff1_8af9_388c_8d93, 0x_bd1c_8800_0000_0000,
    0x_3ff1_9726_5837_5d66, 0x_3d11_f000_0000_0000,
    0x_3ff1_a35b_eb6f_cba7, 0x_3d10_4800_0000_0000,
    0x_3ff1_af99_f813_87e3, 0x_bd47_3900_0000_0000,
    0x_3ff1_bbe0_8404_5d54, 0x_3d24_e400_0000_0000,
    0x_3ff1_c82f_9528_1c43, 0x_bd0a_2000_0000_0000,
    0x_3ff1_d487_3168_b9b2, 0x_3ce3_8000_0000_0000,
    0x_3ff1_e0e7_5eb4_4031, 0x_3cea_c000_0000_0000,
    0x_3ff1_ed50_22fc_d938, 0x_3d01_9000_0000_0000,
    0x_3ff1_f9c1_8438_cdf7, 0x_bd1b_7800_0000_0000,
    0x_3ff2_063b_8862_8d8f, 0x_3d2d_9400_0000_0000,
    0x_3ff2_12be_3578_a81e, 0x_3cd8_0000_0000_0000,
    0x_3ff2_1f49_917d_dd41, 0x_3d2b_3400_0000_0000,
    0x_3ff2_2bdd_a279_1323, 0x_3d19_f800_0000_0000,
    0x_3ff2_387a_6e75_61e7, 0x_bd19_c800_0000_0000,
    0x_3ff2_451f_fb82_1427, 0x_3d02_3000_0000_0000,
    0x_3ff2_51ce_4fb2_a602, 0x_bd13_4800_0000_0000,
    0x_3ff2_5e85_711e_ceb0, 0x_3d12_7000_0000_0000,
    0x_3ff2_6b45_65e2_7d16, 0x_3d11_d000_0000_0000,
    0x_3ff2_780e_341d_e00f, 0x_3d31_ee00_0000_0000,
    0x_3ff2_84df_e1f5_633e, 0x_bd14_c000_0000_0000,
    0x_3ff2_91ba_7591_bb30, 0x_bd13_d800_0000_0000,
    0x_3ff2_9e9d_f51f_df09, 0x_3d08_b000_0000_0000,
    0x_3ff2_ab8a_66d1_0e9b, 0x_bd22_7c00_0000_0000,
    0x_3ff2_b87f_d0da_da3a, 0x_3d2a_3400_0000_0000,
    0x_3ff2_c57e_3977_1af9, 0x_bd10_8000_0000_0000,
    0x_3ff2_d285_a6e4_02d9, 0x_bd0e_d000_0000_0000,
    0x_3ff2_df96_1f64_1579, 0x_bcf4_2000_0000_0000,
    0x_3ff2_ecaf_a93e_2ecf, 0x_bd24_9800_0000_0000,
    0x_3ff2_f9d2_4abd_8822, 0x_bd16_3000_0000_0000,
    0x_3ff3_06fe_0a31_b625, 0x_bd32_3600_0000_0000,
    0x_3ff3_1432_edee_a50b, 0x_bd70_df80_0000_0000,
    0x_3ff3_2170_fc4c_d7b8, 0x_bd22_4800_0000_0000,
    0x_3ff3_2eb8_3ba8_e9a2, 0x_bd25_9800_0000_0000,
    0x_3ff3_3c08_b264_1766, 0x_3d1e_d000_0000_0000,
    0x_3ff3_4962_66e3_fa27, 0x_bcdc_0000_0000_0000,
    0x_3ff3_56c5_5f92_9f0f, 0x_bd30_d800_0000_0000,
    0x_3ff3_6431_a2de_88b9, 0x_3d22_c800_0000_0000,
    0x_3ff3_71a7_373a_aa39, 0x_3d20_6000_0000_0000,
    0x_3ff3_7f26_231e_74fe, 0x_bd16_6000_0000_0000,
    0x_3ff3_8cae_6d05_d838, 0x_bd0a_e000_0000_0000,
    0x_3ff3_9a40_1b71_3ec3, 0x_bd44_7200_0000_0000,
    0x_3ff3_a7db_34e5_a020, 0x_3d08_2000_0000_0000,
    0x_3ff3_b57f_bfec_6e95, 0x_3d3e_8000_0000_0000,
    0x_3ff3_c32d_c313_a8f2, 0x_3cef_8000_0000_0000,
    0x_3ff3_d0e5_44ed_e122, 0x_bd17_a000_0000_0000,
    0x_3ff3_dea6_4c12_34bb, 0x_3d26_3000_0000_0000,
    0x_3ff3_ec70_df1c_4ecc, 0x_bd48_a600_0000_0000,
    0x_3ff3_fa45_04ac_7e8c, 0x_bd3c_dc00_0000_0000,
    0x_3ff4_0822_c367_a0bb, 0x_3d25_b800_0000_0000,
    0x_3ff4_160a_21f7_2e95, 0x_3d1e_c000_0000_0000,
    0x_3ff4_23fb_2709_4646, 0x_bd13_6000_0000_0000,
    0x_3ff4_31f5_d950_a920, 0x_3d23_9800_0000_0000,
    0x_3ff4_3ffa_3f84_b9eb, 0x_3cfa_0000_0000_0000,
    0x_3ff4_4e08_6061_8919, 0x_bcf6_c000_0000_0000,
    0x_3ff4_5c20_42a7_d201, 0x_bd0b_c000_0000_0000,
    0x_3ff4_6a41_ed1d_0016, 0x_bd12_8000_0000_0000,
    0x_3ff4_786d_668b_3326, 0x_3d30_e000_0000_0000,
    0x_3ff4_86a2_b5c1_3c00, 0x_bd2d_4000_0000_0000,
    0x_3ff4_94e1_e192_af04, 0x_3d0c_2000_0000_0000,
    0x_3ff4_a32a_f0d7_d372, 0x_bd1e_5000_0000_0000,
    0x_3ff4_b17d_ea6d_b801, 0x_3d07_8000_0000_0000,
    0x_3ff4_bfda_d536_29e1, 0x_bd13_8000_0000_0000,
    0x_3ff4_ce41_b817_c132, 0x_3d00_8000_0000_0000,
    0x_3ff4_dcb2_99fd_dddb, 0x_3d2c_7000_0000_0000,
    0x_3ff4_eb2d_81d8_ab96, 0x_bd1c_e000_0000_0000,
    0x_3ff4_f9b2_769d_2d02, 0x_3d19_2000_0000_0000,
    0x_3ff5_0841_7f45_31c1, 0x_bd08_c000_0000_0000,
    0x_3ff5_16da_a2cf_662a, 0x_bcfa_0000_0000_0000,
    0x_3ff5_257d_e83f_51ea, 0x_3d4a_0800_0000_0000,
    0x_3ff5_342b_569d_4eda, 0x_bd26_d800_0000_0000,
    0x_3ff5_42e2_f4f6_ac1a, 0x_bd32_4400_0000_0000,
    0x_3ff5_51a4_ca5d_94db, 0x_3d48_3c00_0000_0000,
    0x_3ff5_6070_dde9_116b, 0x_3d24_b000_0000_0000,
    0x_3ff5_6f47_36b5_29de, 0x_3d41_5a00_0000_0000,
    0x_3ff5_7e27_dbe2_c40e, 0x_bd29_e000_0000_0000,
    0x_3ff5_8d12_d497_c76f, 0x_bd23_0800_0000_0000,
    0x_3ff5_9c08_27ff_0b4c, 0x_3d4d_ec00_0000_0000,
    0x_3ff5_ab07_dd48_5427, 0x_bcc4_0000_0000_0000,
    0x_3ff5_ba11_fba8_7af4, 0x_3d30_0800_0000_0000,
    0x_3ff5_c926_8a59_460b, 0x_bd26_c800_0000_0000,
    0x_3ff5_d845_9099_8e3f, 0x_3d46_9a00_0000_0000,
    0x_3ff5_e76f_15ad_20e1, 0x_bd1b_4000_0000_0000,
    0x_3ff5_f6a3_20dc_ebca, 0x_3d17_7000_0000_0000,
    0x_3ff6_05e1_b976_dcb8, 0x_3d26_f800_0000_0000,
    0x_3ff6_152a_e6cd_f715, 0x_3d01_0000_0000_0000,
    0x_3ff6_247e_b03a_5531, 0x_bd15_d000_0000_0000,
    0x_3ff6_33dd_1d19_29b5, 0x_bd12_d000_0000_0000,
    0x_3ff6_4346_34cc_c313, 0x_bcea_8000_0000_0000,
    0x_3ff6_52b9_febc_8efa, 0x_bd28_6000_0000_0000,
    0x_3ff6_6238_8255_3397, 0x_3d71_fe00_0000_0000,
    0x_3ff6_71c1_c708_328e, 0x_bd37_2000_0000_0000,
    0x_3ff6_8155_d44c_a97e, 0x_3ce6_8000_0000_0000,
    0x_3ff6_90f4_b19e_9471, 0x_bd29_7800_0000_0000,
];

// exp2(x): compute the base 2 exponential of x
//
// Accuracy: Peak error < 0.503 ulp for normalized results.
//
// Method: (accurate tables)
//
//   Reduce x:
//     x = k + y, for integer k and |y| <= 1/2.
//     Thus we have exp2(x) = 2**k * exp2(y).
//
//   Reduce y:
//     y = i/TBLSIZE + z - eps[i] for integer i near y * TBLSIZE.
//     Thus we have exp2(y) = exp2(i/TBLSIZE) * exp2(z - eps[i]),
//     with |z - eps[i]| <= 2**-9 + 2**-39 for the table used.
//
//   We compute exp2(i/TBLSIZE) via table lookup and exp2(z - eps[i]) via
//   a degree-5 minimax polynomial with maximum error under 1.3 * 2**-61.
//   The values in exp2t[] and eps[] are chosen such that
//   exp2t[i] = exp2(i/TBLSIZE + eps[i]), and eps[i] is a small offset such
//   that exp2t[i] is accurate to 2**-64.
//
//   Note that the range of i is +-TBLSIZE/2, so we actually index the tables
//   by i0 = i + TBLSIZE/2.  For cache efficiency, exp2t[] and eps[] are
//   virtual tables, interleaved in the real table tbl[].
//
//   This method is due to Gal, with many details due to Gal and Bachelis:
//
//      Gal, S. and Bachelis, B.  An Accurate Elementary Mathematical Library
//      for the IEEE Floating Point Standard.  TOMS 17(1), 26-46 (1991).
#[inline]
pub fn exp2(mut x: f64) -> f64 {
    llvm_intrinsically_optimized! {
        #[cfg(target_os = "cuda")] {
            return unsafe { cuda_intrinsics::exp2_approx(x) }
        }
    }
    let redux = f64::from_bits(0x_4338_0000_0000_0000) / TBLSIZE as f64;
    let p1 = f64::from_bits(0x_3fe6_2e42_fefa_39ef);
    let p2 = f64::from_bits(0x_3fce_bfbd_ff82_c575);
    let p3 = f64::from_bits(0x_3fac_6b08_d704_a0a6);
    let p4 = f64::from_bits(0x_3f83_b2ab_88f7_0400);
    let p5 = f64::from_bits(0x_3f55_d880_0387_5c74);

    // double_t r, t, z;
    // uint32_t ix, i0;
    // union {double f; uint64_t i;} u = {x};
    // union {uint32_t u; int32_t i;} k;
    let x1p1023 = f64::from_bits(0x_7fe0_0000_0000_0000);
    let x1p52 = f64::from_bits(0x_4330_0000_0000_0000);
    let _0x1p_149 = f64::from_bits(0x_b6a0_0000_0000_0000);

    /* Filter out exceptional cases. */
    let ui = f64::to_bits(x);
    let ix = ui >> 32 & (UF_ABS as u64);
    if ix >= 0x_408f_f000 {
        /* |x| >= 1022 or nan */
        if ix >= 0x_4090_0000 && ui >> 63 == 0 {
            /* x >= 1024 or nan */
            /* overflow */
            x *= x1p1023;
            return x;
        }
        if ix >= 0x_7ff0_0000 {
            /* -inf or -nan */
            return -1. / x;
        }
        if ui >> 63 != 0 {
            /* x <= -1022 */
            /* underflow */
            if x <= -1075. || x - x1p52 + x1p52 != x {
                force_eval!((_0x1p_149 / x) as f32);
            }
            if x <= -1075. {
                return 0.;
            }
        }
    } else if ix < 0x_3c90_0000 {
        /* |x| < 0x1p-54 */
        return 1. + x;
    }

    /* Reduce x, computing z, i0, and k. */
    let ui = f64::to_bits(x + redux);
    let mut i0 = ui as u32;
    i0 += TBLSIZE as u32 / 2;
    let ku = i0 / TBLSIZE as u32 * TBLSIZE as u32;
    let ki = ku as i32 / TBLSIZE as i32;
    i0 %= TBLSIZE as u32;
    let uf = f64::from_bits(ui) - redux;
    let mut z = x - uf;

    /* Compute r = exp2(y) = exp2t[i0] * p(z - eps[i]). */
    let t = f64::from_bits(TBL[2 * i0 as usize]); /* exp2t[i0] */
    z -= f64::from_bits(TBL[2 * i0 as usize + 1]); /* eps[i0]   */
    let r = t + t * z * (p1 + z * (p2 + z * (p3 + z * (p4 + z * p5))));

    scalbn(r, ki)
}
