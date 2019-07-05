//! Trait for generating rand sequence testing relevant values for each API

use crate::ApiKind;

pub trait RandSeq: Sized {
    /// Generates a sequence containing exactly `len` values computed from the RNG `rng`
    /// according to the `api_kind`
    fn rand_seq<R: rand::Rng>(rng: &mut R, api_kind: ApiKind, len: usize) -> Vec<Self>;
}

macro_rules! impl_rand_seq_f {
    ($float_ty:ident) => {
        #[allow(clippy::use_self)]
        impl RandSeq for $float_ty {
            fn rand_seq<R: rand::Rng>(rng: &mut R, api_kind: ApiKind, len: usize) -> Vec<Self> {
                use crate::{Distribute, Toward};
                use rand::seq::SliceRandom;
                use std::$float_ty::*;

                let mut vec = Vec::with_capacity(len);
                let double = std::mem::size_of::<Self>() == 8;

                // These inputs are always tested
                const BOUNDS: [$float_ty; 9] = [
                    NAN,
                    INFINITY,
                    NEG_INFINITY,
                    EPSILON,
                    -EPSILON,
                    MAX,
                    MIN,
                    MIN_POSITIVE,
                    -MIN_POSITIVE,
                ];
                vec.extend(&BOUNDS);

                // The domain close to these inputs is also tested:
                const LIMIT_NSTEPS: usize = 10_001;
                vec.extend(INFINITY.toward(0., LIMIT_NSTEPS));
                vec.extend(NEG_INFINITY.toward(0., LIMIT_NSTEPS));
                vec.extend((0. as Self).toward(MIN_POSITIVE, LIMIT_NSTEPS));
                vec.extend((0. as Self).toward(-MIN_POSITIVE, LIMIT_NSTEPS));
                vec.extend((-1. as Self).distribute(1. as Self, 200_001));

                // These domains are extended with tests specific to each
                // type of API:
                {
                    macro_rules! dist {
                        ($from:expr, $to:expr, $steps:expr) => {
                            vec.extend(($from as Self).distribute($to as Self, $steps));
                        };
                    }
                    use crate::ApiKind::*;
                    match api_kind {
                        Sin | Cos | Tan => {
                            if double {
                                dist!(1e-300, 1e+8, 200_001);
                            } else {
                                dist!(1e-30, 1e+8, 200_001);
                            }
                        }
                        SinCos | SinCosPi => {
                            dist!(-1e-14, 1e+14, 200_001);
                        }
                        Log | Log2 | Log10 | Log1p => {
                            if double {
                                dist!(-1e-300, 1e+14, 200_001);
                            } else {
                                dist!(-1e-30, 1e+14, 200_001);
                            }
                        }

                        Exp | Exp2 | Exp10 | Expm1 => {
                            dist!(-1000., 1000., 200_001);
                        }
                        Pow => {
                            dist!(-100., 100., 200_001);
                        }
                        Cbrt => {
                            dist!(1e-14, 1e+14, 100_001);
                            dist!(-1e-14, -1e+14, 100_001);
                        }
                        Hypot => {
                            dist!(-1e-7, 1e7, 200_001);
                        }
                        Atan => {
                            dist!(1e-3, 1e+7, 100_001);
                            dist!(1e-2, 1e+8, 100_001);
                            dist!(1e-3, 1e+7, 100_001);
                            dist!(-1e-2, 1e+8, 100_001);
                        }
                        Atan2 => {
                            dist!(-10, 10, 200_001);
                        }
                        Sinh | Cosh | Tanh | Asinh | Atanh => {
                            if double {
                                dist!(-700, 700, 200_001);
                            } else {
                                dist!(-88, 88, 200_001)
                            }
                        }

                        Acosh => {
                            if double {
                                dist!(1, 700, 200_001);
                            } else {
                                dist!(1, 88, 200_001);
                            }
                        }
                        Lgamma => {
                            dist!(-5000, 5000, 200_001);
                        }
                        Tgamma => {
                            dist!(-10, 10, 200_001);
                        }
                        Erf => {
                            dist!(-100, 100, 200_001);
                        }
                        Erfc => {
                            dist!(-1, 100, 200_001);
                        }
                        Fabs => {
                            dist!(-100.5, 100.5, 200_001);
                        }
                        Copysign | Fmax | Fmin | Fmod | Nextafter => {
                            dist!(-1e+10, 1e+10, 200_001);
                        }
                        Modf => {
                            dist!(-1e+14, 1e+14, 200_001);
                        }
                        Trunc | Floor | Ceil | Round | Rint => {
                            dist!(-100, 100, 800);
                        }
                        _ => (),
                    }
                }

                // ~NSTEPS * 4
                let current_len = vec.len();
                assert!(len > current_len);
                let remaining_len = len.checked_sub(current_len).unwrap();

                for _ in 0..remaining_len {
                    let n = rng.gen::<Self>();
                    vec.push(n);
                }
                assert_eq!(vec.len(), len);

                // Duplicate the vector, randomly shuffle it, and
                // concatenate it. Otherwise for n-ary functions
                // all vectors might have the same values. But
                // testing with the same values is also worth doing.
                let mut vec2 = vec.clone();
                vec2.shuffle(rng);
                vec.extend(vec2);
                vec
            }
        }
    };
}

impl_rand_seq_f!(f32);
impl_rand_seq_f!(f64);

impl RandSeq for i32 {
    fn rand_seq<R: rand::Rng>(rng: &mut R, api_kind: ApiKind, len: usize) -> Vec<Self> {
        use rand::seq::SliceRandom;

        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            let mut r = rng.gen::<Self>();
            if let ApiKind::Jn = api_kind {
                // The integer argument of these APIs is a number of iterations.
                // Computational cost blows up if we pass huge values, so zero
                // their lower bits.
                r &= 0xffff;
            }
            v.push(r);
        }
        assert_eq!(v.len(), len);

        // Duplicate the vector, randomly shuffle it, and
        // concatenate it. Otherwise for n-ary functions
        // all vectors might have the same values. But
        // testing with the same values is also worth doing.
        let mut v2 = v.clone();
        v2.shuffle(rng);
        v.extend(v2);
        v
    }
}
