//! Exhaustively test unary APIs taking 32-bit wide arguments.
#![cfg(test)]
#![cfg(all(exhaustive32, feature = "exhaustive"))]
use libm_test::assert_approx_eq;

macro_rules! exhaustive32 {
    (
        id: $id:ident;
        api_kind: $api_kind:ident;
        arg_tys: f32;
        arg_ids: $arg_id:ident;
        ret_ty: $ret_ty:ty;
    ) => {
        #[test]
        #[allow(unused)]
        fn $id() {
            extern "C" {
                // The system's libm function:
                fn $id($arg_id: f32) -> $ret_ty;
            }

            for i in 0..=u32::max_value() {
                let arg: f32 = unsafe { std::mem::transmute(i) };
                let result = libm::$id(arg);
                let expected = unsafe { $id(arg) };
                assert_approx_eq!(
                    result == expected,
                    id: $id, arg: arg, ulp: 4
                );
            }
        }
    };
    (
        id: $id:ident;
        api_kind: $api_kind:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) => {};
}

libm_analyze::for_each_api!(exhaustive32(
    // jn is to expensive - the others have weird API:
    /*ignored:*/
    "j0f,j1f,y0f,y1f,exp10f,jn"
));
