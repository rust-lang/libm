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

#[derive(Debug)]
pub struct Invocation {
    fields: Punctuated<Mapping, Comma>,
}

impl Parse for Invocation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated(Mapping::parse, Token![,])?,
        })
    }
}

/// A `key: expression` mapping with nothing else. Basically a simplified `syn::Field`.
#[derive(Debug)]
struct Mapping {
    name: Ident,
    sep: Token![:],
    expr: Expr,
}

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            sep: input.parse()?,
            expr: input.parse()?,
        })
    }
}

/// The input provided to our proc macro.
#[derive(Debug)]
pub struct StructuredInput {
    pub callback: Ident,
    pub skip: Vec<Ident>,
    pub attributes: Vec<AttributeMap>,
}

impl StructuredInput {
    pub fn from_fields(input: Invocation) -> syn::Result<Self> {
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
fn expect_field(v: &mut Vec<Mapping>, name: &str) -> syn::Result<Expr> {
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
pub struct AttributeMap {
    pub meta: Vec<Meta>,
    pub names: Vec<Ident>,
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
