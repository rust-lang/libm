macro_rules! basic {
    (
        fn_name: $fn_name:ident,
        CArgsTuple: $CArgsTuple:ty,
        RustArgsTuple: $RustArgsTuple:ty,
        CFnTy: $CFnTy:ty,
        RustFnTy: $RustFnTy:ty,
        attrs: [$($meta:meta)*]
        
    ) => {
        
    };
}

libm_macros::for_each_function! {
    callback: basic,
    skip: [foo, bar],
    attributes: [
        #[meta1]
        #[meta2]
        [baz, corge]
    ],
}
