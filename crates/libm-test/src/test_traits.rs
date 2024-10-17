//! Traits related to testing.
//!
//! There are three main traits in this module:
//!
//! - `GenerateInput`: implemented on any types that create test cases.
//! - `TupleCall`: implemented on tuples to allow calling them as function arguments.
//! - `CheckOutput`: implemented on anything that is an output type for validation against an
//!   expected value.

use crate::{Float, Hex, Int};
use std::ffi::c_int;
use std::fmt;

/// Implement this on types that can generate a sequence of tuples for test input.
pub trait GenerateInput<TupleArgs> {
    fn get_cases(&self) -> impl ExactSizeIterator<Item = TupleArgs>;
}

/// Trait for calling a function with a tuple as arguments.
///
/// Implemented on the tuple with the function signature as the generic (so we can use the same
/// tuple for multiple signatures).
pub trait TupleCall<Func>: fmt::Debug {
    type Output;
    fn call(self, f: Func) -> Self::Output;
}

/// A trait to implement on any output type so we can verify it in a generic way.
pub trait CheckOutput<Input>: Sized {
    /// Assert that `self` and `expected` are the same.
    ///
    /// `input` is only used here for error messages.
    fn validate(self, expected: Self, input: Input, allowed_ulp: u32);
}

/// Implement `TupleCall` for signatures with no `&mut`.
macro_rules! impl_tupl_call {
    ($( ($($argty:ty),*) -> $ret:ty; )+) => {
        $(
            impl TupleCall<fn( $($argty),* ) -> $ret> for ( $($argty,)* ) {
                type Output = $ret;

                fn call(self, f: fn($($argty),*) -> $ret) -> Self::Output {
                    impl_tupl_call!(@call f, self, $($argty),*)
                }
            }
        )*
    };

    (@call $f:ident, $this:ident, $a1:ty, $a2:ty, $a3:ty) => {
        $f($this.0, $this.1, $this.2)
    };
    (@call $f:ident, $this:ident, $a1:ty, $a2:ty) => {
        $f($this.0, $this.1)
    };
    (@call $f:ident, $this:ident, $a1:ty) => {
        $f($this.0)
    };
}

impl_tupl_call! {
    (f32) -> f32;
    (f64) -> f64;
    (f32) -> i32;
    (f64) -> i32;
    (f32, f32) -> f32;
    (f64, f64) -> f64;
    (f32, i32) -> f32;
    (f64, i32) -> f64;
    (i32, f32) -> f32;
    (i32, f64) -> f64;
    (f32, f32, f32) -> f32;
    (f64, f64, f64) -> f64;
    (f32) -> (f32, f32);
    (f64) -> (f64, f64);
    (f32) -> (f32, c_int);
    (f64) -> (f64, c_int);
    (f32, f32) -> (f32, c_int);
    (f64, f64) -> (f64, c_int);
}

/* Implement `TupleCall` for signatures that use `&mut` (i.e. system symbols that return
 * more than one value) */

impl TupleCall<fn(f32, &mut c_int) -> f32> for (f32,) {
    type Output = (f32, c_int);

    fn call(self, f: fn(f32, &mut c_int) -> f32) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, &mut iret);
        (fret, iret)
    }
}

impl TupleCall<fn(f64, &mut c_int) -> f64> for (f64,) {
    type Output = (f64, c_int);

    fn call(self, f: fn(f64, &mut c_int) -> f64) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, &mut iret);
        (fret, iret)
    }
}

impl TupleCall<fn(f32, &mut f32) -> f32> for (f32,) {
    type Output = (f32, f32);

    fn call(self, f: fn(f32, &mut f32) -> f32) -> Self::Output {
        let mut ret2 = 0.0;
        let ret1 = f(self.0, &mut ret2);
        (ret1, ret2)
    }
}

impl TupleCall<fn(f64, &mut f64) -> f64> for (f64,) {
    type Output = (f64, f64);

    fn call(self, f: fn(f64, &mut f64) -> f64) -> Self::Output {
        let mut ret2 = 0.0;
        let ret1 = f(self.0, &mut ret2);
        (ret1, ret2)
    }
}

impl TupleCall<fn(f32, f32, &mut c_int) -> f32> for (f32, f32) {
    type Output = (f32, c_int);

    fn call(self, f: fn(f32, f32, &mut c_int) -> f32) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, self.1, &mut iret);
        (fret, iret)
    }
}

impl TupleCall<fn(f64, f64, &mut c_int) -> f64> for (f64, f64) {
    type Output = (f64, c_int);

    fn call(self, f: fn(f64, f64, &mut c_int) -> f64) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, self.1, &mut iret);
        (fret, iret)
    }
}

impl TupleCall<fn(f32, &mut f32, &mut f32)> for (f32,) {
    type Output = (f32, f32);

    fn call(self, f: fn(f32, &mut f32, &mut f32)) -> Self::Output {
        let mut ret1 = 0.0;
        let mut ret2 = 0.0;
        f(self.0, &mut ret1, &mut ret2);
        (ret1, ret2)
    }
}

impl TupleCall<fn(f64, &mut f64, &mut f64)> for (f64,) {
    type Output = (f64, f64);

    fn call(self, f: fn(f64, &mut f64, &mut f64)) -> Self::Output {
        let mut ret1 = 0.0;
        let mut ret2 = 0.0;
        f(self.0, &mut ret1, &mut ret2);
        (ret1, ret2)
    }
}

// Implement for floats
impl<F, Input> CheckOutput<Input> for F
where
    F: Float + Hex,
    Input: Hex + fmt::Debug,
    u32: TryFrom<F::SignedInt, Error: fmt::Debug>,
{
    fn validate(self, expected: Self, input: Input, allowed_ulp: u32) {
        let make_msg = || {
            format!(
                "expected {expected:?} crate {self:?} ({expbits}, {actbits}) input {input:?} ({ibits})",
                expbits = expected.hex(),
                actbits = self.hex(),
                ibits = input.hex()
           )
        };

        // Check when both are NaN
        if self.is_nan() && expected.is_nan() {
            assert_eq!(
                self.to_bits(),
                expected.to_bits(),
                "NaN have different bitpatterns: {}",
                make_msg()
            );
            // Nothing else to check
            return;
        } else if self.is_nan() || expected.is_nan() {
            panic!("mismatched NaN: {}", make_msg());
        }

        // Make sure that the signs are the same before checing ULP
        assert_eq!(
            self.signum(),
            expected.signum(),
            "mismatched signs: {}",
            make_msg()
        );

        let ulp_diff = self
            .to_bits()
            .signed()
            .checked_sub(expected.to_bits().signed())
            .unwrap()
            .abs();

        let ulp_u32 = u32::try_from(ulp_diff).unwrap_or_else(|e| {
            panic!("{e:?}: ulp of {ulp_diff} exceeds u32::MAX: {}", make_msg())
        });

        assert!(
            ulp_u32 <= allowed_ulp,
            "ulp {ulp_diff} > {allowed_ulp}: {}",
            make_msg()
        );
    }
}

/// Implement `CheckOutput` for combinations of types.
macro_rules! impl_tuples {
    ($(($a:ty, $b:ty);)*) => {
        $(
            impl<Input: Hex + fmt::Debug> CheckOutput<Input> for ($a, $b) {
                fn validate(self, expected: Self, input: Input, allowed_ulp: u32)
                {
                    self.0.validate(expected.0, input, allowed_ulp);
                    self.1.validate(expected.1, input, allowed_ulp);
                }
            }
        )*
    };
}

impl_tuples!(
    (f32, i32);
    (f64, i32);
    (f32, f32);
    (f64, f64);
);
