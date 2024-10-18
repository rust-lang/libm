#![allow(unused)]

mod parse;
use parse::{Invocation, StructuredInput};

use std::{collections::BTreeMap, sync::LazyLock};

use proc_macro as pm;
use proc_macro2::{self as pm2, Span};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Attribute, Expr, ExprArray, ExprPath, Ident, Meta, PatPath, Path, Token,
};

const ALL_FUNCTIONS: &[(Signature, Option<Signature>, &[&str])] = &[
    (
        // `fn(f32) -> f32`
        Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32],
        },
        None,
        &[
            "acosf", "acoshf", "asinf", "asinhf", "atanf", "atanhf", "cbrtf", "ceilf", "cosf",
            "coshf", "erff", "exp10f", "exp2f", "expf", "expm1f", "fabsf", "floorf", "j0f", "j1f",
            "lgammaf", "log10f", "log1pf", "log2f", "logf", "rintf", "roundf", "sinf", "sinhf",
            "sqrtf", "tanf", "tanhf", "tgammaf", "trunc",
        ],
    ),
    (
        // `(f64) -> f64`
        Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64],
        },
        None,
        &[
            "acos", "acosh", "asin", "asinh", "atan", "atanh", "cbrt", "ceil", "cos", "cosh",
            "erf", "exp10", "exp2", "exp", "expm1", "fabs", "floor", "j0", "j1", "lgamma", "log10",
            "log1p", "log2", "log", "rint", "round", "sin", "sinh", "sqrt", "tan", "tanh",
            "tgamma", "trunc",
        ],
    ),
    (
        // `(f32, f32) -> f32`
        Signature {
            args: &[Ty::F32, Ty::F32],
            returns: &[Ty::F32],
        },
        None,
        &[
            "atan2f",
            "copysignf",
            "fdimf",
            "fmaxf",
            "fminf",
            "fmodf",
            "hypotf",
            "nextafterf",
            "powf",
            "remainderf",
        ],
    ),
    (
        // `(f64, f64) -> f64`
        Signature {
            args: &[Ty::F64, Ty::F64],
            returns: &[Ty::F64],
        },
        None,
        &[
            "atan2",
            "copysign",
            "fdim",
            "fmax",
            "fmin",
            "fmod",
            "hypot",
            "nextafter",
            "pow",
            "remainder",
        ],
    ),
    (
        // `(f32, f32, f32) -> f32`
        Signature {
            args: &[Ty::F32, Ty::F32, Ty::F32],
            returns: &[Ty::F32],
        },
        None,
        &["fmaf"],
    ),
    (
        // `(f64, f64, f64) -> f64`
        Signature {
            args: &[Ty::F64, Ty::F64, Ty::F64],
            returns: &[Ty::F64],
        },
        None,
        &["fma"],
    ),
    (
        // `(f32) -> i32`
        Signature {
            args: &[Ty::F32],
            returns: &[Ty::I32],
        },
        None,
        &["ilogbf"],
    ),
    (
        // `(f64) -> i32`
        Signature {
            args: &[Ty::F64],
            returns: &[Ty::I32],
        },
        None,
        &["ilogb"],
    ),
    (
        // `(i32, f32) -> f32`
        Signature {
            args: &[Ty::I32, Ty::F32],
            returns: &[Ty::F32],
        },
        None,
        &["jnf"],
    ),
    (
        // `(i32, f64) -> f64`
        Signature {
            args: &[Ty::I32, Ty::F64],
            returns: &[Ty::F64],
        },
        None,
        &["jn"],
    ),
    (
        // `(f32, i32) -> f32`
        Signature {
            args: &[Ty::F32, Ty::I32],
            returns: &[Ty::F32],
        },
        None,
        &["scalbnf", "ldexpf"],
    ),
    (
        // `(f64, i64) -> f64`
        Signature {
            args: &[Ty::F64, Ty::I32],
            returns: &[Ty::F64],
        },
        None,
        &["scalbn", "ldexp"],
    ),
    (
        // `(f32, &mut f32) -> f32` as `(f32) -> (f32, f32)`
        Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32, Ty::F32],
        },
        Some(Signature {
            args: &[Ty::F32, Ty::MutF32],
            returns: &[Ty::F32],
        }),
        &["modff"],
    ),
    (
        // `(f64, &mut f64) -> f64` as  `(f64) -> (f64, f64)`
        Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64, Ty::F64],
        },
        Some(Signature {
            args: &[Ty::F64, Ty::MutF64],
            returns: &[Ty::F64],
        }),
        &["modf"],
    ),
    (
        // `(f32, &mut c_int) -> f32` as `(f32) -> (f32, i32)`
        Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32, Ty::I32],
        },
        Some(Signature {
            args: &[Ty::F32, Ty::MutCInt],
            returns: &[Ty::F32],
        }),
        &["frexpf", "lgammaf_r"],
    ),
    (
        // `(f64, &mut c_int) -> f64` as `(f64) -> (f64, i32)`
        Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64, Ty::I32],
        },
        Some(Signature {
            args: &[Ty::F64, Ty::MutCInt],
            returns: &[Ty::F64],
        }),
        &["frexp", "lgamma_r"],
    ),
    (
        // `(f32, f32, &mut c_int) -> f32` as `(f32, f32) -> (f32, i32)`
        Signature {
            args: &[Ty::F32, Ty::F32],
            returns: &[Ty::F32, Ty::I32],
        },
        Some(Signature {
            args: &[Ty::F32, Ty::F32, Ty::MutCInt],
            returns: &[Ty::F32],
        }),
        &["remquof"],
    ),
    (
        // `(f64, f64, &mut c_int) -> f64` as `(f64, f64) -> (f64, i32)`
        Signature {
            args: &[Ty::F64, Ty::F64],
            returns: &[Ty::F64, Ty::I32],
        },
        Some(Signature {
            args: &[Ty::F64, Ty::F64, Ty::MutCInt],
            returns: &[Ty::F64],
        }),
        &["remquo"],
    ),
    (
        // `(f32, &mut f32, &mut f32)` as `(f32) -> (f32, f32)`
        Signature {
            args: &[Ty::F32],
            returns: &[Ty::F32, Ty::F32],
        },
        Some(Signature {
            args: &[Ty::F32, Ty::MutF32, Ty::MutF32],
            returns: &[],
        }),
        &["sincosf"],
    ),
    (
        // `(f64, &mut f64, &mut f64)` as `(f64) -> (f64, f64)`
        Signature {
            args: &[Ty::F64],
            returns: &[Ty::F64, Ty::F64],
        },
        Some(Signature {
            args: &[Ty::F64, Ty::MutF64, Ty::MutF64],
            returns: &[],
        }),
        &["sincos"],
    ),
];

/// A type used in a function signature
#[derive(Debug, Clone)]
enum Ty {
    F16,
    F32,
    F64,
    F128,
    I32,
    CInt,
    MutF16,
    MutF32,
    MutF64,
    MutF128,
    MutI32,
    MutCInt,
}

/// Representation of e.g. `(f32, f32) -> f32`
#[derive(Debug, Clone)]
struct Signature {
    args: &'static [Ty],
    returns: &'static [Ty],
}

#[derive(Debug, Clone)]
struct ApiSignature {
    sys_sig: Signature,
    rust_sig: Signature,
    name: &'static str,
}

static ALL_FUNCTIONS_FLAT: LazyLock<Vec<ApiSignature>> = LazyLock::new(|| {
    let mut ret = Vec::new();

    for (rust_sig, c_sig, names) in ALL_FUNCTIONS {
        for name in *names {
            let api = ApiSignature {
                rust_sig: rust_sig.clone(),
                sys_sig: c_sig.clone().unwrap_or_else(|| rust_sig.clone()),
                name,
            };
            ret.push(api);
        }
    }

    ret
});

/*

Invoke as:

for_each_function! {
    callback: some_macro,
    skip: [foo, bar],
    attributes: [
        #[meta1]
        #[meta2]
        [baz, qux],
    ]
}

*/

#[proc_macro]
pub fn for_each_function(tokens: pm::TokenStream) -> pm::TokenStream {
    let input = syn::parse_macro_input!(tokens as Invocation);
    let structured = match StructuredInput::from_fields(input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    expand(structured).into()
}

fn expand(input: StructuredInput) -> pm2::TokenStream {
    let callback = input.callback;
    let mut out = pm2::TokenStream::new();

    for func in ALL_FUNCTIONS_FLAT.iter() {
        let fn_name = Ident::new(func.name, Span::call_site());

        // No output on functions that should be skipped
        if input.skip.contains(&fn_name) {
            continue;
        }

        let mut meta = input
            .attributes
            .iter()
            .filter(|map| map.names.contains(&fn_name))
            .flat_map(|map| &map.meta);

        let new = quote! {
            #callback! {
                fn_name: #fn_name,
                CArgsTuple: f32,
                RustArgsTuple: f32,
                CFnTy: f32,
                RustFnTy: f32,
                attrs: [
                    #( #meta )*
                ]
            }
        };

        out.extend(new);
    }

    out
}
