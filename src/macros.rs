/// Do something for each function present in this crate.
///
/// Takes a callback macro and invokes it multiple times, once for each function that
/// this crate exports. This makes it easy to create generic tests, benchmarks, or other checks
/// and apply it to each symbol.
#[macro_export]
#[cfg(feature = "_internal-features")]
macro_rules! for_each_function {
    // Main invocation
    //
    // Just calls back to this macro with a list of all functions in this crate.
    ($call_this:ident) => {
        $crate::for_each_function! {
            @implementation
            user_macro: $call_this;

            // Up to date list of all functions in this crate, grouped by signature.
            // Some signatures have a different signature in the system libraries and in Rust, e.g.
            // when multiple values are returned. These are the cases with multiple signatures
            // (`as` in between).
            (f32) -> f32 {
                acosf;
                acoshf;
                asinf;
                asinhf;
                atanf;
                atanhf;
                cbrtf;
                ceilf;
                cosf;
                coshf;
                erff;
                #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
                exp10f;
                #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
                exp2f;
                expf;
                expm1f;
                fabsf;
                floorf;
                j0f;
                j1f;
                lgammaf;
                log10f;
                log1pf;
                log2f;
                logf;
                rintf;
                roundf;
                sinf;
                sinhf;
                sqrtf;
                tanf;
                tanhf;
                tgammaf;
                truncf;
            };

            (f64) -> f64 {
                acos;
                acosh;
                asin;
                asinh;
                atan;
                atanh;
                cbrt;
                ceil;
                cos;
                cosh;
                erf;
                #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
                exp10;
                #[cfg_attr(x86_no_sse, ignore)] // FIXME(correctness): wrong result on i586
                exp2;
                exp;
                expm1;
                fabs;
                floor;
                j0;
                j1;
                lgamma;
                log10;
                log1p;
                log2;
                log;
                rint;
                round;
                sin;
                sinh;
                sqrt;
                tan;
                tanh;
                tgamma;
                trunc;
            };

            (f32, f32) -> f32 {
                atan2f;
                copysignf;
                fdimf;
                fmaxf;
                fminf;
                fmodf;
                hypotf;
                nextafterf;
                powf;
                remainderf;
            };

            (f64, f64) -> f64 {
                atan2;
                copysign;
                fdim;
                fmax;
                fmin;
                fmod;
                hypot;
                nextafter;
                pow;
                remainder;
            };

            (f32, f32, f32) -> f32 {
                fmaf;
            };

            (f64, f64, f64) -> f64 {
                fma;
            };

            (f32) -> i32 {
                ilogbf;
            };

            (f64) -> i32 {
                ilogb;
            };

            (i32, f32) -> f32 {
                jnf;
            };

            (f32, i32) -> f32 {
                scalbnf;
                ldexpf;
            };

            (i32, f64) -> f64 {
                jn;
            };

            (f64, i32) -> f64 {
                scalbn;
                ldexp;
            };

            (f32, &mut f32) -> f32 as (f32) -> (f32, f32) {
                modff;
            };

            (f64, &mut f64) -> f64 as (f64) -> (f64, f64) {
                modf;
            };

            (f32, &mut c_int) -> f32 as (f32) -> (f32, i32) {
                frexpf;
                lgammaf_r;
            };

            (f64, &mut c_int) -> f64 as (f64) -> (f64, i32) {
                frexp;
                lgamma_r;
            };

            (f32, f32, &mut c_int) -> f32 as (f32, f32) -> (f32, i32) {
                remquof;
            };

            (f64, f64, &mut c_int) -> f64 as (f64, f64) -> (f64, i32) {
                remquo;
            };

            (f32, &mut f32, &mut f32) -> () as (f32) -> (f32, f32) {
                sincosf;
            };

            (f64, &mut f64, &mut f64) -> () as (f64) -> (f64, f64) {
                sincos;
            };
        }
    };

    // This branch processes the function list and passes it to the user macro callback.
    (
        @implementation
        user_macro: $call_this:ident;
        $(
            // Main signature
            ($($sys_arg:ty),+) -> $sys_ret:ty
            // If the Rust signature is different from system, it is provided with `as`
            $(as ($($rust_arg:ty),+) -> $rust_ret:ty)? {
                $(
                    $(#[$fn_meta:meta])* // applied to the test
                    $name:ident;
                )*
            };
        )*
    ) => {
        // The user macro can have an `@all_items` pattern where it gets a list of the functions
        $call_this! {
            @all_items
            fn_names: [$($( $name ),*),*]
        }

        $(
            // Invoke the user macro once for each signature type.
            $call_this! {
                @each_signature
                // The input type, represented as a tuple. E.g. `(f32, f32)` for a
                // `fn(f32, f32) -> f32` signature.
                SysArgsTupleTy: ($($sys_arg),+ ,),
                // The tuple type to call the Rust function. So if the system signature is
                // `fn(f32, &mut f32) -> f32`, this type will only be `(f32, )`.
                RustArgsTupleTy: $crate::for_each_function!(
                    @coalesce [($($sys_arg),+ ,)] $( [($($rust_arg),+ ,)] )?
                ),
                // A function signature type for the system function.
                SysFnTy: fn($($sys_arg),+) -> $sys_ret,
                // A function signature type for the Rust function.
                RustFnTy: $crate::for_each_function!(
                    @coalesce [fn($($sys_arg),+) -> $sys_ret] $([fn($($rust_arg),+) -> $rust_ret])?
                ),
                // The list of all functions that have this signature.
                functions: [$( {
                    attrs: [$($fn_meta),*],
                    fn_name: $name,
                } ),*],
            }
        )*
    };

    // Macro helper to return the second item if two are provided, otherwise a default
    (@coalesce [$($tt1:tt)*]) => { $($tt1)* } ;
    (@coalesce [$($tt1:tt)*] [$($tt2:tt)*]) => { $($tt2)* } ;
}
