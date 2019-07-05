//! Testing utilities required by most tests.

pub trait WithinUlps {
    /// Returns true if two numbers are closer than `ulp_tol` to each other.
    fn within_ulps(self, other: Self, ulp_tol: usize) -> bool;
}

// Stamp the impls for floats:
macro_rules! impl_within_ulps_f {
    ($f_ty:ty, $i_ty:ty) => {
        impl WithinUlps for $f_ty {
            #[allow(clippy::cast_possible_truncation)]
            fn within_ulps(self, y: Self, ulp_tol: usize) -> bool {
                let x = self;
                if x.is_nan() != y.is_nan() {
                    // one is nan but the other is not
                    return false;
                }
                if x.is_nan() && y.is_nan() {
                    return true;
                }
                if x.is_infinite() != y.is_infinite() {
                    // one is inf but the other is not
                    return false;
                }

                let xi: $i_ty = unsafe { core::intrinsics::transmute(x) };
                let yi: $i_ty = unsafe { core::intrinsics::transmute(y) };
                if (xi < 0) != (yi < 0) {
                    // different sign, e.g., -0.0 != +0.0:
                    return false;
                }
                let ulps = (xi - yi).abs();
                ulps <= ulp_tol as _
            }
        }
    };
}

impl_within_ulps_f!(f32, i32);
impl_within_ulps_f!(f64, i64);

impl WithinUlps for i32 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn within_ulps(self, y: Self, ulp_tol: usize) -> bool {
        let x = self;
        let ulps = (x - y).abs();
        ulps <= ulp_tol as _
    }
}

/// This implements function dispatch for tuples of arguments used in the tests
/// above, so that we can: (f32, 32).call(fn(f32, f32) -> f32) generically.
///
/// We need the input parameter F to support dispatching, e.g., (f32,f32) with
/// functions that return both f32 or i32. Those are two different types, so we
/// need to be parametric over them.
pub trait Call<F> {
    type Ret;
    fn call(self, f: F) -> Self::Ret;
}

macro_rules! impl_call {
    (($($arg_tys:ty),*) -> $ret_ty:ty: $self_:ident: $($xs:expr),*)  => {
        impl Call<unsafe extern"C" fn($($arg_tys),*) -> $ret_ty> for ($($arg_tys,)+) {
            type Ret = $ret_ty;
            fn call(self, f: unsafe extern "C" fn($($arg_tys),*) -> $ret_ty) -> Self::Ret {
                let $self_ = self;
                unsafe { f($($xs),*) }
            }
        }
    };
}

impl_call!((f32) -> f32: x: x.0);
impl_call!((f64) -> f64: x: x.0);
impl_call!((f64) -> i32: x: x.0);
impl_call!((f32) -> i32: x: x.0);
impl_call!((f32, f32) -> f32: x: x.0, x.1);
impl_call!((f64, f64) -> f64: x: x.0, x.1);
impl_call!((f64, i32) -> f64: x: x.0, x.1);
impl_call!((f32, i32) -> f32: x: x.0, x.1);
impl_call!((i32, f64) -> f64: x: x.0, x.1);
impl_call!((i32, f32) -> f32: x: x.0, x.1);
impl_call!((f32, f32, f32) -> f32: x: x.0, x.1, x.2);
impl_call!((f64, f64, f64) -> f64: x: x.0, x.1, x.2);

// Adjust the input of a function.
#[macro_export]
macro_rules! adjust_input {
    (fn: j1, input: $arg:ident) => {
        adjust_input!(adjust: $arg)
    };
    (fn: jn, input: $arg:ident) => {
        adjust_input!(adjust: $arg)
    };
    (fn: $id:ident, input: $args:ident) => {};
    (adjust: $arg:ident) => {
        // First argument to these functions are a number of
        // iterations and passing large random numbers takes forever
        // to execute, so check if their higher bits are set and
        // zero them:
        let p = &mut $arg as *mut _ as *mut i32;
        unsafe { p.write(p.read() & 0xffff) }
    };
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($result:ident == $expected:ident,
     id: $id:ident, arg: $arg:ident, ulp: $ulps:expr) => {
        if !$crate::WithinUlps::within_ulps($result, $expected, $ulps) {
            let f = format!(
                "{}{:?} returns = {:?} != {:?} (expected)",
                stringify!($id),
                $arg,
                $result,
                $expected
            );
            panic!(f);
        }
    };
}
