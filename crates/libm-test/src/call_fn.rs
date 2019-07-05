//! Trait to do function dispatch based on tuples:

/// This implements function dispatch for tuples of arguments used in the tests
/// above, so that we can: (f32, 32).call(fn(f32, f32) -> f32) generically.
///
/// We need the input parameter F to support dispatching, e.g., (f32,f32) with
/// functions that return both f32 or i32. Those are two different types, so we
/// need to be parametric over them.
pub trait CallFn<F> {
    type Ret;
    fn call(self, f: F) -> Self::Ret;
}

macro_rules! impl_call {
    (($($arg_tys:ty),*) -> $ret_ty:ty: $self_:ident: $($xs:expr),*)  => {
        impl CallFn<unsafe extern"C" fn($($arg_tys),*) -> $ret_ty> for ($($arg_tys,)+) {
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
impl_call!((f32, f64) -> f32: x: x.0, x.1);
impl_call!((f64, f64) -> f64: x: x.0, x.1);
impl_call!((f64, i32) -> f64: x: x.0, x.1);
impl_call!((f32, i32) -> f32: x: x.0, x.1);
impl_call!((i32, f64) -> f64: x: x.0, x.1);
impl_call!((i32, f32) -> f32: x: x.0, x.1);
impl_call!((f32, f32, f32) -> f32: x: x.0, x.1, x.2);
impl_call!((f64, f64, f64) -> f64: x: x.0, x.1, x.2);
