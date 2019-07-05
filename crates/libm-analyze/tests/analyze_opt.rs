//! Tests that the proc-macro accepts macros with
//! the following pattern:

macro_rules! nop {
    (
        id: $id:ident;
        api_kind: $api_kind:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) => {};
}

libm_analyze::for_each_api!(nop("j1f,jn"));
