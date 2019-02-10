/* origin: FreeBSD /usr/src/lib/msun/src/k_sinf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Optimized by Bruce D. Evans.
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

/* |sin(x)/x - s(x)| < 2**-37.5 (~[-4.89e-12, 4.824e-12]). */
const S1: f64 = -0.166_666_666_416_265_235_595; /* -0x_1555_5554cbac77.0p-55 */
const S2: f64 = 0.008_333_329_385_889_463_175_6; /*  0x_1111_10896efbb2.0p-59 */
const S3: f64 = -0.000_198_393_348_360_966_317_347; /* -0x_1a00_f9e2cae774.0p-65 */
const S4: f64 = 0.000_002_718_311_493_989_821_906_4; /*  0x_16cd_878c3b46a7.0p-71 */

#[inline]
pub fn k_sinf(x: f64) -> f32 {
    let z = x * x;
    let w = z * z;
    let r = S3 + z * S4;
    let s = z * x;
    ((x + s * (S1 + z * S2)) + s * w * r) as f32
}
