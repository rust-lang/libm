macro_rules! basic {
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: $RustFn:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        attrs: [$($meta:meta)*]

    ) => {
        $(#[$meta])*
        mod $fn_name {
            #[allow(unused)]
            type CFnTy = $CFn;
            // type CArgsTy<'_> = $CArgs;
            // type CRetTy<'_> = $CRet;
            #[allow(unused)]
            type RustFnTy = $RustFn;
            #[allow(unused)]
            type RustArgsTy = $RustArgs;
            #[allow(unused)]
            type RustRetTy = $RustRet;
        }
    };
}

libm_macros::for_each_function! {
    callback: basic,
    skip: [sin, cos],
    attributes: [
        // just some random attributes
        #[allow(clippy::pedantic)]
        #[allow(dead_code)]
        [sinf, cosf]
    ],
}
