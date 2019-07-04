//! Compare the results of the `libm` implementation against the system's libm.
#![cfg(test)]
//#![cfg(feature = "system_libm")]

use libm_test::WithinUlps;

// Number of tests to generate for each function
const NTESTS: usize = 500;

const ULP_TOL: usize = 4;

macro_rules! system_libm {
    // Skip those parts of the API that are not
    // exposed by the system libm library:
    //
    // FIXME: maybe we can skip them as part of libm-analyze?
    (
        id: j0f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: j1f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: jnf;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: y0f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: y1f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: ynf;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: exp10;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};
    (
        id: exp10f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) =>  {};

    // Generate random tests for all others:
    (
        id: $id:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) => {
        #[test]
        #[allow(unused)]
        fn $id() {
            use crate::Call;
            let mut rng = rand::thread_rng();
            for _ in 0..NTESTS {
                // Type of the system libm fn:
                type FnTy = unsafe extern "C" fn ($($arg_ids: $arg_tys),*) -> $ret_ty;
                // FIXME: extern "C" wrapper over our libm functions
                // Shouldn't be needed once they are all extern "C"
                extern "C" fn libm_fn($($arg_ids: $arg_tys),*) -> $ret_ty {
                    libm::$id($($arg_ids),*)
                }
                extern "C" {
                    // The system's libm function:
                    fn $id($($arg_ids: $arg_tys),*) -> $ret_ty;
                }

                // Generate a tuple of arguments containing random values:
                let mut args: ( $($arg_tys,)+ ) = ( $(<$arg_tys as Rand>::gen(&mut rng),)+ );

                // HACK
                if let "j1" | "jn" = stringify!($id) {
                    // First argument to these functions are a number of
                    // iterations and passing large random numbers takes forever
                    // to execute, so check if their higher bits are set and
                    // zero them:
                    let p = &mut args as *mut _ as *mut i32;
                    unsafe { p.write(p.read() & 0xffff) }
                }

                let result = args.call(libm_fn as FnTy);
                let expected = args.call($id as FnTy);
                if !result.within_ulps(expected, ULP_TOL) {
                    eprintln!("{}{:?} returns = {:?} != {:?} (expected)", stringify!($id), args, result, expected);
                    panic!();
                }
            }
        }
    }
}

libm_analyze::for_each_api!(system_libm);

// This implements function dispatch for tuples of arguments used in the tests
// above, so that we can: (f32, 32).call(fn(f32, f32) -> f32) generically.
//
// We need the input parameter F to support dispatching, e.g., (f32,f32) with
// functions that return both f32 or i32. Those are two different types, so we
// need to be parametric over them.
trait Call<F> {
    type Ret;
    fn call(self, f: F) -> Self::Ret;
}

macro_rules! impl_call {
    (($($arg_tys:ty),*) -> $ret_ty:ty: $self_:ident: $($xs:expr),*)  => {
        // We only care about unsafe extern "C" functions here, safe functions coerce to them:
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

// We need to be able to generate random numbers for the types involved.
//
// Rand does this well, but we want to also test some other specific values.
trait Rand {
    fn gen(rng: &mut rand::rngs::ThreadRng) -> Self;
}

macro_rules! impl_rand {
    ($id:ident: [$($e:expr),*]) => {
        impl Rand for $id {
            fn gen(r: &mut rand::rngs::ThreadRng) -> Self {
                use rand::{Rng, seq::SliceRandom};
                // 1/20 probability of picking a non-random value
                if r.gen_range(0, 20) < 1 {
                    *[$($e),*].choose(r).unwrap()
                } else {
                    r.gen::<$id>()
                }
            }
        }
    }
}

// Some interesting random values
impl_rand!(f32: [std::f32::NAN, std::f32::INFINITY, std::f32::NEG_INFINITY]);
impl_rand!(f64: [std::f64::NAN, std::f64::INFINITY, std::f64::NEG_INFINITY]);
impl_rand!(i32: [i32::max_value(), 0_i32, i32::min_value()]);
