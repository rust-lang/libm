// `STATUS_DLL_NOT_FOUND` on i686 MinGW, not worth looking into.
#![cfg(not(all(target_arch = "x86", target_os = "windows", target_env = "gnu")))]

macro_rules! basic {
    (
        fn_name: $fn_name:ident,
        extra: [$($extra_tt:tt)*],
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
            #[allow(unused)]
            const A: &[&str] = &[$($extra_tt)*];
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
    extra: ["foo", "bar"],
}
