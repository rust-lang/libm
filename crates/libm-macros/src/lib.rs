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
            "sqrtf", "tanf", "tanhf", "tgammaf", "truncf",
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

/// A type used in a function signature.
#[derive(Debug, Clone, Copy)]
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

impl ToTokens for Ty {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let ts = match self {
            Ty::F16 => quote! { f16  },
            Ty::F32 => quote! { f32  },
            Ty::F64 => quote! { f64  },
            Ty::F128 => quote! { f128  },
            Ty::I32 => quote! { i32  },
            Ty::CInt => quote! { ::core::ffi::c_int  },
            Ty::MutF16 => quote! { &mut f16  },
            Ty::MutF32 => quote! { &mut f32  },
            Ty::MutF64 => quote! { &mut f64  },
            Ty::MutF128 => quote! { &mut f128  },
            Ty::MutI32 => quote! { &mut i32  },
            Ty::MutCInt => quote! { &mut core::ffi::c_int },
        };

        tokens.extend(ts);
    }
}

/// Representation of e.g. `(f32, f32) -> f32`
#[derive(Debug, Clone)]
struct Signature {
    args: &'static [Ty],
    returns: &'static [Ty],
}

/// Combined information about a function implementation.
#[derive(Debug, Clone)]
struct FunctionInfo {
    name: &'static str,
    c_sig: Signature,
    rust_sig: Signature,
}

/// A flat representation of `ALL_FUNCTIONS`.
static ALL_FUNCTIONS_FLAT: LazyLock<Vec<FunctionInfo>> = LazyLock::new(|| {
    let mut ret = Vec::new();

    for (rust_sig, c_sig, names) in ALL_FUNCTIONS {
        for name in *names {
            let api = FunctionInfo {
                rust_sig: rust_sig.clone(),
                c_sig: c_sig.clone().unwrap_or_else(|| rust_sig.clone()),
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

    if let Some(e) = validate(&structured) {
        return e.into_compile_error().into();
    }

    expand(structured).into()
}

fn validate(input: &StructuredInput) -> Option<syn::Error> {
    let mentioned_functions = input.skip.iter().chain(
        input
            .attributes
            .iter()
            .flat_map(|attr_map| attr_map.names.iter()),
    );

    for mentioned in mentioned_functions {
        if !ALL_FUNCTIONS_FLAT.iter().any(|func| mentioned == func.name) {
            let e = syn::Error::new(
                mentioned.span(),
                format!("unrecognized function name `{mentioned}`"),
            );
            return Some(e);
        }
    }

    None
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

        let c_args = func.c_sig.args;
        let c_ret = func.c_sig.returns;
        let rust_args = func.rust_sig.args;
        let rust_ret = func.rust_sig.returns;

        let new = quote! {
            #callback! {
                fn_name: #fn_name,
                CFn: fn( #(#c_args),* ,) -> ( #(#c_ret),* ),
                CArgs: ( #(#c_args),* ,),
                CRet: ( #(#c_ret),* ),
                RustFn: fn( #(#rust_args),* ,) -> ( #(#rust_ret),* ),
                RustArgs: ( #(#rust_args),* ,),
                RustRet: ( #(#rust_ret),* ),
                attrs: [
                    #( #meta )*
                ]
            }
        };

        out.extend(new);
    }

    out
}
