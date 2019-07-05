#![cfg(test)]

use libm::*;
#[test]
fn remquo_q_overflow() {
    // 0xc000000000000001, 0x04c0000000000004
    let _ = remquo(-2.0000000000000004, 8.406091369059082e-286);
}

#[test]
fn ceil_sanity_check() {
    assert_eq!(ceil(1.1), 2.0);
    assert_eq!(ceil(2.9), 3.0);
}

#[test]
fn atan2_sanity_check() {
    use std::f64::consts::PI;
    assert_eq!(atan2(0.0, 1.0), 0.0);
    assert_eq!(atan2(0.0, -1.0), PI);
    assert_eq!(atan2(-0.0, -1.0), -PI);
    assert_eq!(atan2(3.0, 2.0), atan(3.0 / 2.0));
    assert_eq!(atan2(2.0, -1.0), atan(2.0 / -1.0) + PI);
    assert_eq!(atan2(-2.0, -1.0), atan(-2.0 / -1.0) - PI);
}

#[test]
fn floor_overflow() {
    assert_eq!(floorf(0.5), 0.0);
}

#[test]
fn trunc_sanity_check() {
    assert_eq!(trunc(1.1), 1.0);
}

mod pow {
    use libm::*;
    use std::f64::consts::{E, PI};
    use std::f64::{EPSILON, INFINITY, MAX, MIN, MIN_POSITIVE, NAN, NEG_INFINITY};

    const POS_ZERO: &[f64] = &[0.0];
    const NEG_ZERO: &[f64] = &[-0.0];
    const POS_ONE: &[f64] = &[1.0];
    const NEG_ONE: &[f64] = &[-1.0];
    const POS_FLOATS: &[f64] = &[99.0 / 70.0, E, PI];
    const NEG_FLOATS: &[f64] = &[-99.0 / 70.0, -E, -PI];
    const POS_SMALL_FLOATS: &[f64] = &[(1.0 / 2.0), MIN_POSITIVE, EPSILON];
    const NEG_SMALL_FLOATS: &[f64] = &[-(1.0 / 2.0), -MIN_POSITIVE, -EPSILON];
    const POS_EVENS: &[f64] = &[2.0, 6.0, 8.0, 10.0, 22.0, 100.0, MAX];
    const NEG_EVENS: &[f64] = &[MIN, -100.0, -22.0, -10.0, -8.0, -6.0, -2.0];
    const POS_ODDS: &[f64] = &[3.0, 7.0];
    const NEG_ODDS: &[f64] = &[-7.0, -3.0];
    const NANS: &[f64] = &[NAN];
    const POS_INF: &[f64] = &[INFINITY];
    const NEG_INF: &[f64] = &[NEG_INFINITY];

    const ALL: &[&[f64]] = &[
        POS_ZERO,
        NEG_ZERO,
        NANS,
        NEG_SMALL_FLOATS,
        POS_SMALL_FLOATS,
        NEG_FLOATS,
        POS_FLOATS,
        NEG_EVENS,
        POS_EVENS,
        NEG_ODDS,
        POS_ODDS,
        NEG_INF,
        POS_INF,
        NEG_ONE,
        POS_ONE,
    ];
    const POS: &[&[f64]] = &[POS_ZERO, POS_ODDS, POS_ONE, POS_FLOATS, POS_EVENS, POS_INF];
    const NEG: &[&[f64]] = &[NEG_ZERO, NEG_ODDS, NEG_ONE, NEG_FLOATS, NEG_EVENS, NEG_INF];

    fn pow_test(base: f64, exponent: f64, expected: f64) {
        let res = pow(base, exponent);
        assert!(
            if expected.is_nan() {
                res.is_nan()
            } else {
                pow(base, exponent) == expected
            },
            "{} ** {} was {} instead of {}",
            base,
            exponent,
            res,
            expected
        );
    }

    fn test_sets_as_base(sets: &[&[f64]], exponent: f64, expected: f64) {
        sets.iter()
            .for_each(|s| s.iter().for_each(|val| pow_test(*val, exponent, expected)));
    }

    fn test_sets_as_exponent(base: f64, sets: &[&[f64]], expected: f64) {
        sets.iter()
            .for_each(|s| s.iter().for_each(|val| pow_test(base, *val, expected)));
    }

    fn test_sets(sets: &[&[f64]], computed: &dyn Fn(f64) -> f64, expected: &dyn Fn(f64) -> f64) {
        sets.iter().for_each(|s| {
            s.iter().for_each(|val| {
                let exp = expected(*val);
                let res = computed(*val);

                assert!(
                    if exp.is_nan() {
                        res.is_nan()
                    } else {
                        exp == res
                    },
                    "test for {} was {} instead of {}",
                    val,
                    res,
                    exp
                );
            })
        });
    }

    #[test]
    fn zero_as_exponent() {
        test_sets_as_base(ALL, 0.0, 1.0);
        test_sets_as_base(ALL, -0.0, 1.0);
    }

    #[test]
    fn one_as_base() {
        test_sets_as_exponent(1.0, ALL, 1.0);
    }

    #[test]
    fn nan_inputs() {
        // NAN as the base:
        // (NAN ^ anything *but 0* should be NAN)
        test_sets_as_exponent(NAN, &ALL[2..], NAN);

        // NAN as the exponent:
        // (anything *but 1* ^ NAN should be NAN)
        test_sets_as_base(&ALL[..(ALL.len() - 2)], NAN, NAN);
    }

    #[test]
    fn infinity_as_base() {
        // Positive Infinity as the base:
        // (+Infinity ^ positive anything but 0 and NAN should be +Infinity)
        test_sets_as_exponent(INFINITY, &POS[1..], INFINITY);

        // (+Infinity ^ negative anything except 0 and NAN should be 0.0)
        test_sets_as_exponent(INFINITY, &NEG[1..], 0.0);

        // Negative Infinity as the base:
        // (-Infinity ^ positive odd ints should be -Infinity)
        test_sets_as_exponent(NEG_INFINITY, &[POS_ODDS], NEG_INFINITY);

        // (-Infinity ^ anything but odd ints should be == -0 ^ (-anything))
        // We can lump in pos/neg odd ints here because they don't seem to
        // cause panics (div by zero) in release mode (I think).
        test_sets(ALL, &|v: f64| pow(NEG_INFINITY, v), &|v: f64| pow(-0.0, -v));
    }

    #[test]
    fn infinity_as_exponent() {
        // Positive/Negative base greater than 1:
        // (pos/neg > 1 ^ Infinity should be Infinity - note this excludes NAN as the base)
        test_sets_as_base(&ALL[5..(ALL.len() - 2)], INFINITY, INFINITY);

        // (pos/neg > 1 ^ -Infinity should be 0.0)
        test_sets_as_base(&ALL[5..ALL.len() - 2], NEG_INFINITY, 0.0);

        // Positive/Negative base less than 1:
        let base_below_one = &[POS_ZERO, NEG_ZERO, NEG_SMALL_FLOATS, POS_SMALL_FLOATS];

        // (pos/neg < 1 ^ Infinity should be 0.0 - this also excludes NAN as the base)
        test_sets_as_base(base_below_one, INFINITY, 0.0);

        // (pos/neg < 1 ^ -Infinity should be Infinity)
        test_sets_as_base(base_below_one, NEG_INFINITY, INFINITY);

        // Positive/Negative 1 as the base:
        // (pos/neg 1 ^ Infinity should be 1)
        test_sets_as_base(&[NEG_ONE, POS_ONE], INFINITY, 1.0);

        // (pos/neg 1 ^ -Infinity should be 1)
        test_sets_as_base(&[NEG_ONE, POS_ONE], NEG_INFINITY, 1.0);
    }

    #[test]
    fn zero_as_base() {
        // Positive Zero as the base:
        // (+0 ^ anything positive but 0 and NAN should be +0)
        test_sets_as_exponent(0.0, &POS[1..], 0.0);

        // (+0 ^ anything negative but 0 and NAN should be Infinity)
        // (this should panic because we're dividing by zero)
        test_sets_as_exponent(0.0, &NEG[1..], INFINITY);

        // Negative Zero as the base:
        // (-0 ^ anything positive but 0, NAN, and odd ints should be +0)
        test_sets_as_exponent(-0.0, &POS[3..], 0.0);

        // (-0 ^ anything negative but 0, NAN, and odd ints should be Infinity)
        // (should panic because of divide by zero)
        test_sets_as_exponent(-0.0, &NEG[3..], INFINITY);

        // (-0 ^ positive odd ints should be -0)
        test_sets_as_exponent(-0.0, &[POS_ODDS], -0.0);

        // (-0 ^ negative odd ints should be -Infinity)
        // (should panic because of divide by zero)
        test_sets_as_exponent(-0.0, &[NEG_ODDS], NEG_INFINITY);
    }

    #[test]
    fn special_cases() {
        // One as the exponent:
        // (anything ^ 1 should be anything - i.e. the base)
        test_sets(ALL, &|v: f64| pow(v, 1.0), &|v: f64| v);

        // Negative One as the exponent:
        // (anything ^ -1 should be 1/anything)
        test_sets(ALL, &|v: f64| pow(v, -1.0), &|v: f64| 1.0 / v);

        // Factoring -1 out:
        // (negative anything ^ integer should be (-1 ^ integer) * (positive anything ^ integer))
        &[POS_ZERO, NEG_ZERO, POS_ONE, NEG_ONE, POS_EVENS, NEG_EVENS]
            .iter()
            .for_each(|int_set| {
                int_set.iter().for_each(|int| {
                    test_sets(ALL, &|v: f64| pow(-v, *int), &|v: f64| {
                        pow(-1.0, *int) * pow(v, *int)
                    });
                })
            });

        // Negative base (imaginary results):
        // (-anything except 0 and Infinity ^ non-integer should be NAN)
        &NEG[1..(NEG.len() - 1)].iter().for_each(|set| {
            set.iter().for_each(|val| {
                test_sets(&ALL[3..7], &|v: f64| pow(*val, v), &|_| NAN);
            })
        });
    }

    #[test]
    fn normal_cases() {
        assert_eq!(pow(2.0, 20.0), (1 << 20) as f64);
        assert_eq!(pow(-1.0, 9.0), -1.0);
        assert!(pow(-1.0, 2.2).is_nan());
        assert!(pow(-1.0, -1.14).is_nan());
    }
}

mod atan {
    use libm::atan;
    use std::f64;

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

#[test]
fn sin_near_pi() {
    let x = f64::from_bits(0x400921fb000FD5DD); // 3.141592026217707
    let sx = f64::from_bits(0x3ea50d15ced1a4a2); // 6.273720864039205e-7
    assert_eq!(sin(x), sx);
}

#[test]
fn truncf_sanity_check() {
    assert_eq!(truncf(1.1), 1.0);
}

#[test]
fn expm1_sanity_check() {
    assert_eq!(expm1(1.1), 2.0041660239464334);
}

#[test]
fn roundf_negative_zero() {
    assert_eq!(roundf(-0.0_f32).to_bits(), (-0.0_f32).to_bits());
}

#[test]
fn exp2_i0_wrap_test() {
    let x = -3.0 / 256.0;
    assert_eq!(exp2(x), f64::from_bits(0x3fefbdba3692d514));
}

#[test]
fn round_negative_zero() {
    assert_eq!(round(-0.0_f64).to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn j1f_2488() {
    // 0x401F3E49
    assert_eq!(j1f(2.4881766_f32), 0.49999475_f32);
}
#[test]
fn y1f_2002() {
    assert_eq!(y1f(2.0000002_f32), -0.10703229_f32);
}

#[test]
fn fma_segfault_bug() {
    // An attempt to substract with overflow was causing a segfault
    // on FMA for these inputs:
    assert_eq!(
        fma(
            -0.0000000000000002220446049250313,
            -0.0000000000000002220446049250313,
            -0.0000000000000002220446049250313
        ),
        -0.00000000000000022204460492503126
    );

    assert_eq!(fma(-0.992, -0.992, -0.992), -0.00793599999988632);
}
