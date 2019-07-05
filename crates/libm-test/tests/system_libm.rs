//! Compare the results of the `libm` implementation against the system's libm.
#![cfg(test)]
#![cfg(feature = "system_libm")]

use libm_test::{assert_approx_eq, CallFn, RandSeq, TupleVec};

// Number of tests to generate for each function
const NTESTS: usize = 1_000_000;

const ULP_TOL: usize = 4;

macro_rules! system_libm {
    // Generate random tests for all others:
    (
        id: $id:ident;
        api_kind: $api_kind:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) => {
        #[test]
        #[allow(unused)]
        fn $id() {
            // Type of the system libm fn:
            type FnTy
                = unsafe extern "C" fn ($($arg_ids: $arg_tys),*) -> $ret_ty;
            extern "C" {
                // The system's libm function:
                fn $id($($arg_ids: $arg_tys),*) -> $ret_ty;
            }

            let mut rng = rand::thread_rng();

            // Generate a tuple of arguments containing random values:
            let mut args: ( $(Vec<$arg_tys>,)+ )
                = ( $(
                        <$arg_tys as RandSeq>::rand_seq(
                            &mut rng, libm_test::ApiKind::$api_kind, NTESTS
                         ),
                     )+
                );

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

libm_analyze::for_each_api!(system_libm(
    // FIXME: Some are not exposed by the system's musl,
    // others are incorrect. FMA is broken.
    /*ignored:*/
    "j0f,j1f,jnf,y0f,y1f,ynf,exp10,exp10f,fma"
));
