/*
"A Precision Approximation of the Gamma Function" - Cornelius Lanczos (1964)
"Lanczos Implementation of the Gamma Function" - Paul Godfrey (2001)
"An Analysis of the Lanczos Gamma Approximation" - Glendon Ralph Pugh (2004)

approximation method:

                        (x - 0.5)         S(x)
Gamma(x) = (x + g - 0.5)         *  ----------------
                                    exp(x + g - 0.5)

with
                 a1      a2      a3            aN
S(x) ~= [ a0 + ----- + ----- + ----- + ... + ----- ]
               x + 1   x + 2   x + 3         x + N

with a0, a1, a2, a3,.. aN constants which depend on g.

for x < 0 the following reflection formula is used:

Gamma(x)*Gamma(-x) = -pi/(x sin(pi x))

most ideas and constants are from boost and python
*/

use core::f64;
use math::consts::*;

use super::{exp, floor, k_cos, k_sin, pow};

const PI: f64 = 3.141_592_653_589_793_238_462_643_383_279_502_884;

/* sin(pi x) with x > 0x1p-100, if sin(pi*x)==0 the sign is arbitrary */
fn sinpi(mut x: f64) -> f64
{
    let mut n: isize;

    /* argument reduction: x = |x| mod 2 */
    /* spurious inexact when x is odd int */
    x *= 0.5;
    x = 2. * (x - floor(x));

    /* reduce x into [-.25,.25] */
    n = (4.0 * x) as isize;
    n = (n+1)/2;
    x -= (n as f64) * 0.5;

    x *= PI;
    match n {
        1   => k_cos(x, 0.),
        2   => k_sin(-x, 0., 0),
        3   => -k_cos(x, 0.),
        0|_ => k_sin(x, 0., 0),
    }
}

const N: usize = 12;
//static const double g = 6.024_680_040_776_729_583_740_234_375;
const GMHALF: f64 = 5.524_680_040_776_729_583_740_234_375;
const SNUM: [f64; N+1] = [
    23_531_376_880.410_759_688_572_007_674_451_636_754_734_846_804_940,
    42_919_803_642.649_098_768_957_899_047_001_988_850_926_355_848_959,
    35_711_959_237.355_668_049_440_185_451_547_166_705_960_488_635_843,
    17_921_034_426.037_209_699_919_755_754_458_931_112_671_403_265_390,
    6_039_542_586.352_028_005_064_291_644_307_297_921_069_938_842_070_8,
    1_439_720_407.311_721_673_663_223_072_794_912_393_971_548_578_677_2,
    248_874_557.862_054_156_511_460_386_413_229_423_216_321_251_278_01,
    31_426_415.585_400_194_380_614_231_628_318_205_362_874_684_987_640,
    2_876_370.628_935_372_441_225_409_051_620_849_613_599_114_537_876_8,
    186_056.265_395_223_495_040_294_989_716_045_699_282_207_842_363_28,
    8_071.672_002_365_816_210_638_002_902_272_250_613_821_851_632_502_4,
    210.824_277_751_579_345_872_509_733_920_713_362_711_669_695_802_91,
    2.506_628_274_631_000_270_164_908_177_133_837_338_626_431_079_340_8,
];
const SDEN: [f64; N+1] = [
    0., 39_916_800., 120_543_840., 150_917_976., 105_258_076.,
    45_995_730., 13_339_535., 2_637_558., 357_423., 32_670., 1_925., 66., 1.,
];
/* n! for small integer n */
const FACT: [f64; 23] = [
    1., 1., 2., 6., 24., 120., 720., 5040., 40_320., 362_880., 3_628_800.,
    39_916_800., 479_001_600., 6_227_020_800., 87_178_291_200., 1_307_674_368_000.,
    20_922_789_888_000., 355_687_428_096_000., 6_402_373_705_728_000., 121_645_100_408_832_000.,
    2_432_902_008_176_640_000., 51_090_942_171_709_440_000., 1_124_000_727_777_607_680_000.,
];

/* S(x) rational function for positive x */
fn s(x: f64) -> f64
{
    let mut num: f64 = 0.;
    let mut den: f64 = 0.;

    /* to avoid overflow handle large x differently */
    if x < 8. {
        for i in (0..=N).rev() {
            num = num * x + SNUM[i];
            den = den * x + SDEN[i];
        }
    } else {
        for i in 0..=N {
            num = num / x + SNUM[i];
            den = den / x + SDEN[i];
        }
    }
    num/den
}

pub fn tgamma(mut x: f64) -> f64
{
    let u: u64 = x.to_bits();
    let absx: f64;
    let y: f64;
    let mut dy: f64;
    let mut z: f64;
    let mut r: f64;
    let ix: u32 = ((u >> 32) as u32) & UF_ABS;
    let sign: bool = (u>>63) != 0;

    /* special cases */
    if ix >= 0x_7ff0_0000 {
        /* tgamma(nan)=nan, tgamma(inf)=inf, tgamma(-inf)=nan with invalid */
        return x + f64::INFINITY;
    }
    if ix < ((0x3ff-54)<<20) {
        /* |x| < 2^-54: tgamma(x) ~ 1/x, +-0 raises div-by-zero */
        return 1./x;
    }

    /* integer arguments */
    /* raise inexact when non-integer */
    if x == floor(x) {
        if sign {
            return f64::NAN;
        }
        if x <= FACT.len() as f64 {
            return FACT[(x as usize) - 1];
        }
    }

    /* x >= 172: tgamma(x)=inf with overflow */
    /* x =< -184: tgamma(x)=+-0 with underflow */
    if ix >= 0x_4067_0000 { /* |x| >= 184 */
        if sign {
            let x1p_126 = f64::from_bits(0x_3810_0000_0000_0000); // 0x1p-126 == 2^-126
            force_eval!((x1p_126/x) as f32);
            if floor(x) * 0.5 == floor(x * 0.5) {
                return 0.0;
            } else {
                return -0.0;
            }
        }
        let x1p1023 = f64::from_bits(0x_7fe0_0000_0000_0000); // 0x1p1023 == 2^1023
        x *= x1p1023;
        return x;
    }

    absx = if sign { -x } else { x };

    /* handle the error of x + g - 0.5 */
    y = absx + GMHALF;
    if absx > GMHALF {
        dy = y - absx;
        dy -= GMHALF;
    } else {
        dy = y - GMHALF;
        dy -= absx;
    }

    z = absx - 0.5;
    r = s(absx) * exp(-y);
    if x < 0.0 {
        /* reflection formula for negative x */
        /* sinpi(absx) is not 0, integers are already handled */
        r = -PI / (sinpi(absx) * absx * r);
        dy = -dy;
        z = -z;
    }
    r += dy * (GMHALF+0.5) * r / y;
    z = pow(y, 0.5*z);
    r * z * z
}
