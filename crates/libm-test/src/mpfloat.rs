//! Interfaces needed to support testing with multi-precision floating point numbers.
//!
//! Within this module, the macros create a submodule for each `libm` function. These contain
//! a struct named `Operation` that implements [`MpOp`].

use std::ops::RemAssign;

use az::Az;
use rug::ops::PowAssign;
use rug::Assign;
pub use rug::Float as MpFloat;

use crate::Float;

/// Create a multiple-precision float with the correct number of bits for a concrete float type.
fn new_mpfloat<F: Float>() -> MpFloat {
    MpFloat::new(F::SIGNIFICAND_BITS + 1)
}

/// Set subnormal emulation and convert to a concrete float type.
fn prep_retval<F: Float>(mp: &mut MpFloat) -> F
where
    for<'a> &'a MpFloat: az::Cast<F>,
{
    mp.subnormalize_ieee();
    (&*mp).az::<F>()
}

/// Structures that represent a float operation.
///
/// The struct itself should hold any context that can be reused among calls to `run` (allocated
/// `MpFloat`s).
pub trait MpOp {
    /// Inputs to the operation (concrete float types).
    type Input;

    /// Outputs from the operation (concrete float types).
    type Output;

    /// Create a new instance.
    fn new() -> Self;

    /// Perform the operation.
    ///
    /// Usually this means assigning inputs to cached floats, performing the operation, applying
    /// subnormal approximation, and converting the result back to concrete values.
    fn run(&mut self, input: Self::Input) -> Self::Output;
}

/// Implement `MpOp` for functions with a single return value.
macro_rules! impl_mp_op {
    // Matcher for unary functions
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: fn($fty:ty,) -> $_ret:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Operation(MpFloat);

                impl MpOp for Operation {
                    type Input = $RustArgs;
                    type Output = $RustRet;

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.0.[< $fn_name_normalized _mut >]();
                        prep_retval::<Self::Output>(&mut self.0)
                    }
                }
            }
        }
    };
    // Matcher for binary functions
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: fn($fty:ty, $_fty2:ty,) -> $_ret:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = $RustArgs;
                    type Output = $RustRet;

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>(), new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.[< $fn_name_normalized _mut >](&self.1);
                        prep_retval::<Self::Output>(&mut self.0)
                    }
                }
            }
        }
    };
    // Matcher for ternary functions
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: fn($fty:ty, $_fty2:ty, $_fty3:ty,) -> $_ret:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Operation(MpFloat, MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = $RustArgs;
                    type Output = $RustRet;

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>(), new_mpfloat::<$fty>(), new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.2.assign(input.2);
                        self.0.[< $fn_name_normalized _mut >](&self.1, &self.2);
                        prep_retval::<Self::Output>(&mut self.0)
                    }
                }
            }
        }
    };
}

libm_macros::for_each_function! {
    callback: impl_mp_op,
    skip: [
        // Most of these need a manual implementation
        fmod, fmodf, frexp, frexpf, ilogb, ilogbf, jn, jnf, ldexp, ldexpf,
        lgamma_r, lgammaf_r, modf, modff, nextafter, nextafterf, pow,powf,
        remquo, remquof, scalbn, scalbnf, sincos, sincosf,
    ],
    fn_extra: match MACRO_FN_NAME {
        // Remap function names that are different between mpfr and libm
        expm1 | expm1f => exp_m1,
        fabs | fabsf => abs,
        fdim | fdimf => positive_diff,
        fma | fmaf => mul_add,
        fmax | fmaxf => max,
        fmin | fminf => min,
        lgamma | lgammaf => ln_gamma,
        log | logf => ln,
        log1p | log1pf => ln_1p,
        rint | rintf => round,
        tgamma | tgammaf => gamma,
        _ => MACRO_FN_NAME_NORMALIZED
    }
}

/// Some functions are difficult to do in a generic way. Implement them here.
macro_rules! impl_op_for_ty {
    ($fty:ty, $suffix:literal) => {
        paste::paste! {
            pub mod [<nextafter $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty, $fty);
                    type Output = $fty;

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>(), new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.next_toward(&self.1);
                        prep_retval::<Self::Output>(&mut self.0)
                    }
                }
            }

            pub mod [<pow $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty, $fty);
                    type Output = $fty;

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>(), new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.pow_assign(&self.1);
                        prep_retval::<Self::Output>(&mut self.0)
                    }
                }
            }

            pub mod [<fmod $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty, $fty);
                    type Output = $fty;

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>(), new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.rem_assign(&self.1);
                        prep_retval::<Self::Output>(&mut self.0)
                    }
                }
            }

            pub mod [<lgamma_r $suffix>] {
                use super::*;
                pub struct Operation(MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty,);
                    type Output = ($fty, i32);

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        let ordering = self.0.ln_abs_gamma_mut();
                        let ret = prep_retval::<$fty>(&mut self.0);
                        (ret, ordering as i32)
                    }
                }
            }

            pub mod [<jn $suffix>] {
                use super::*;
                pub struct Operation(i32, MpFloat);

                impl MpOp for Operation {
                    type Input = (i32, $fty);
                    type Output = $fty;

                    fn new() -> Self {
                        Self(0, new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0 = input.0;
                        self.1.assign(input.1);
                        self.1.jn_mut(self.0);
                        prep_retval::<$fty>(&mut self.1)
                    }
                }
            }

            pub mod [<sincos $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty,);
                    type Output = ($fty, $fty);

                    fn new() -> Self {
                        Self(new_mpfloat::<$fty>(), new_mpfloat::<$fty>())
                    }

                    fn run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(0.0);
                        self.0.sin_cos_mut(&mut self.1);
                        (prep_retval::<$fty>(&mut self.0), prep_retval::<$fty>(&mut self.1))
                    }
                }
            }
        }
    };
}

impl_op_for_ty!(f32, "f");
impl_op_for_ty!(f64, "");

// Account for `lgamma_r` not having a simple `f` suffix
pub mod lgammaf_r {
    pub use super::lgamma_rf::*;
}
