//! Compare the results of the `libm` implementation against the system's libm.
#![cfg(test)]
#![cfg(feature = "system_libm")]

use libm_test::{assert_approx_eq, get_api_kind, Call, RandSeq, TupleVec};

// Number of tests to generate for each function
const NTESTS: usize = 10_000;

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
            // Type of the system libm fn:
            type FnTy
                = unsafe extern "C" fn ($($arg_ids: $arg_tys),*) -> $ret_ty;
            extern "C" {
                // The system's libm function:
                fn $id($($arg_ids: $arg_tys),*) -> $ret_ty;
            }

            let mut rng = rand::thread_rng();

            // Depending on the type of API, different ranges of values might be
            // allowed or interesting to test:
            let api_kind = get_api_kind!(fn: $id);

            // Generate a tuple of arguments containing random values:
            let mut args: ( $(Vec<$arg_tys>,)+ )
                = ( $(<$arg_tys as RandSeq>::rand_seq(&mut rng, api_kind, NTESTS),)+ );

            for i in 0..NTESTS {
                let args: ( $($arg_tys,)+ ) = args.get(i);
                let result = args.call(libm::$id as FnTy);
                let expected = args.call($id as FnTy);
                assert_approx_eq!(
                    result == expected,
                    id: $id, arg: args, ulp: ULP_TOL
                );
            }
        }
    }
}

libm_analyze::for_each_api!(system_libm);
