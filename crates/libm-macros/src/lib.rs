#![allow(unused)]

use std::{collections::BTreeMap, sync::LazyLock};

use proc_macro as pm;
use proc_macro2::{self as pm2, Span};
use quote::ToTokens;
use syn::{
    bracketed,
    parse::{self, Parse, ParseStream, Parser},
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
    let structured = match Structured::from_fields(input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    panic!("{structured:#?}");

    todo!();
    tokens
}

// fn inner(input: Invocation) -> syn::Result<pm::TokenStream> {

// }

#[derive(Debug)]
struct Invocation {
    fields: Punctuated<Field, Comma>,
}

impl Parse for Invocation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

#[derive(Debug)]
struct Field {
    name: Ident,
    sep: Token![:],
    expr: Expr,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            sep: input.parse()?,
            expr: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct Structured {
    callback: Ident,
    skip: Vec<Ident>,
    attributes: Vec<AttributeMap>,
}

impl Structured {
    fn from_fields(input: Invocation) -> syn::Result<Self> {
        let mut map: Vec<_> = input.fields.into_iter().collect();
        let cb_expr = expect_field(&mut map, "callback")?;
        let skip_expr = expect_field(&mut map, "skip")?;
        let attr_expr = expect_field(&mut map, "attributes")?;

        if !map.is_empty() {
            Err(syn::Error::new(
                map.first().unwrap().name.span(),
                format!("unexpected fields {map:?}"),
            ))?
        }

        let skip = Parser::parse2(parse_ident_array, skip_expr.into_token_stream())?;
        let attr_exprs = Parser::parse2(parse_expr_array, attr_expr.into_token_stream())?;
        let mut attributes = Vec::new();

        for attr in attr_exprs {
            attributes.push(syn::parse2(attr.into_token_stream())?);
        }

        Ok(Self {
            callback: expect_ident(cb_expr)?,
            skip,
            attributes,
        })
    }
}

/// Extract a named field from a map, raising an error if it doesn't exist.
fn expect_field(v: &mut Vec<Field>, name: &str) -> syn::Result<Expr> {
    let pos = v.iter().position(|v| v.name == name).ok_or_else(|| {
        syn::Error::new(
            Span::call_site(),
            format!("missing expected field `{name}`"),
        )
    })?;

    Ok(v.remove(pos).expr)
}

/// Coerce an expression into a simple identifier.
fn expect_ident(expr: Expr) -> syn::Result<Ident> {
    syn::parse2(expr.into_token_stream())
}

/// Parse an array of expressions.
fn parse_expr_array(input: ParseStream) -> syn::Result<Vec<Expr>> {
    let content;
    let _ = bracketed!(content in input);
    let fields = content.parse_terminated(Expr::parse, Token![,])?;
    Ok(fields.into_iter().collect())
}

/// Parse an array of idents, e.g. `[foo, bar, baz]`.
fn parse_ident_array(input: ParseStream) -> syn::Result<Vec<Ident>> {
    let content;
    let _ = bracketed!(content in input);
    let fields = content.parse_terminated(Ident::parse, Token![,])?;
    Ok(fields.into_iter().collect())
}

/// A mapping of attributes to identifiers (just a simplified `Expr`).
///
/// Expressed as:
///
/// ```ignore
/// #[meta1]
/// #[meta2]
/// [foo, bar, baz]
/// ```
#[derive(Debug)]
struct AttributeMap {
    meta: Vec<Meta>,
    names: Vec<Ident>,
}

impl Parse for AttributeMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        Ok(Self {
            meta: attrs.into_iter().map(|a| a.meta).collect(),
            names: parse_ident_array(input)?,
        })
    }
}
