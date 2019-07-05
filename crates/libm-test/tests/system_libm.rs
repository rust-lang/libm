//! Compare the results of the `libm` implementation against the system's libm.
#![cfg(test)]
//#![cfg(feature = "system_libm")]

use libm_test::{adjust_input, Call, WithinUlps};

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
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: j1f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: jnf;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: y0f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: y1f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: ynf;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: exp10;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};
    (
        id: exp10f;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) =>  {};

    // Generate random tests for all others:
    (
        id: $id:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
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

                // Some APIs need their inputs to be "adjusted" (see macro):
                // correct_input!(fn: $id, input: args);
                adjust_input!(fn: $id, input: args);

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
