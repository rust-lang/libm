#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use self::proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn for_each_api(input: TokenStream) -> TokenStream {
    let files = get_libm_files();
    let functions = get_functions(files);
    let input = parse_macro_input!(input as syn::Ident);
    let mut tokens = proc_macro2::TokenStream::new();
    for function in functions {
        let id = function.ident;
        let ret_ty = function.ret_ty;
        let arg_tys = function.arg_tys;
        let arg_ids = get_arg_ids(arg_tys.len());
        let t = quote! {
            #input! {
                id: #id;
                arg_tys: #(#arg_tys),*;
                arg_ids: #(#arg_ids),*;
                ret: #ret_ty;
            }
        };
        tokens.extend(t);
    }
    tokens.into()
}

/// Traverses the libm crate directory, parsing all .rs files
fn get_libm_files() -> Vec<(syn::File, String)> {
    let root_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let libm_dir = root_dir
        .parent()
        .expect("couldn't access crates/ dir")
        .join("libm");
    let libm_src_dir = libm_dir.join("src");

    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(libm_src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        use std::io::Read;
        let file_path = entry.path();
        if file_path.is_dir()
            || !file_path
                .to_str()
                .expect("can't format file path")
                .ends_with(".rs")
        {
            continue;
        }

        let mut file_string = String::new();
        std::fs::File::open(&file_path)
            .unwrap_or_else(|_| panic!("can't open file at path: {}", file_path.display()))
            .read_to_string(&mut file_string)
            .expect("failed to read file to string");
        let file = syn::parse_file(&file_string).expect("failed to parse");
        files.push((file, file_path.to_str().unwrap().to_string()));
    }
    files
}

struct FnSig {
    ident: syn::Ident,
    unsafety: bool,
    c_abi: bool,
    ret_ty: Option<syn::Type>,
    arg_tys: Vec<syn::Type>,
}

impl FnSig {
    fn name(&self) -> String {
        self.ident.to_string()
    }
}

macro_rules! syn_to_str {
    ($e:expr) => {{
        let t = $e;
        let tokens = quote! {
            #t
        };
        format!("{}", tokens)
    }};
}

/// Extracts all public functions from the libm files.
fn get_functions(files: Vec<(syn::File, String)>) -> Vec<FnSig> {
    let mut error = false;
    let mut functions = Vec::new();
    for item in files.iter().flat_map(|f| f.0.items.iter()) {
        let mut e = false;
        match item {
            syn::Item::Fn(syn::ItemFn {
                vis: syn::Visibility::Public(_),
                ident,
                constness,
                asyncness,
                unsafety,
                attrs,
                abi,
                decl,
                block: _,
            }) => {
                let mut fn_sig = FnSig {
                    ident: ident.clone(),
                    unsafety: true,
                    c_abi: false,
                    arg_tys: Vec::new(),
                    ret_ty: None,
                };
                macro_rules! err {
                    ($msg:expr) => {{
                        #[cfg(feature = "analyze")]
                        {
                            eprintln!("[error]: Function \"{}\" {}", fn_sig.name(), $msg);
                        }
                        #[allow(unused_assignments)]
                        {
                            e = true;
                        }
                        ()
                    }};
                }
                if let Some(syn::Abi {
                    name: Some(l),
                    extern_token: _,
                }) = abi
                {
                    if l.value() == "C" {
                        fn_sig.c_abi = true;
                    }
                }
                if let Some(_) = constness {
                    err!("is const");
                }
                if let Some(_) = asyncness {
                    err!("is async");
                }
                if &None == unsafety {
                    fn_sig.unsafety = false;
                }
                let syn::FnDecl {
                    fn_token: _,
                    generics,
                    paren_token: _,
                    inputs,
                    variadic,
                    output,
                } = (**decl).clone();

                if variadic.is_some() {
                    err!(format!(
                        "contains variadic arguments \"{}\"",
                        syn_to_str!(variadic.unwrap())
                    ));
                }
                if generics.type_params().into_iter().count() != 0 {
                    err!(format!(
                        "contains generic parameters \"{}\"",
                        syn_to_str!(generics.clone())
                    ));
                }
                if generics.lifetimes().into_iter().count() != 0 {
                    err!(format!(
                        "contains lifetime parameters \"{}\"",
                        syn_to_str!(generics.clone())
                    ));
                }
                if generics.const_params().into_iter().count() != 0 {
                    err!(format!(
                        "contains const parameters \"{}\"",
                        syn_to_str!(generics.clone())
                    ));
                }
                if attrs.is_empty() {
                    err!(format!(
                        "missing `#[inline]` and `#[no_panic]` attributes {}",
                        attrs
                            .iter()
                            .map(|a| syn_to_str!(a))
                            .collect::<Vec<_>>()
                            .join(",")
                    ));
                } // TODO: might want to check other attributes as well
                if !fn_sig.c_abi {
                    // FIXME: do not disable test if this fails - otherwise no test passes
                    let e2 = e;
                    err!("not `extern \"C\"`");
                    e = e2;
                }
                match output {
                    syn::ReturnType::Default => (),
                    syn::ReturnType::Type(_, ref b) if valid_ty(&b) => {
                        fn_sig.ret_ty = Some(*b.clone())
                    }
                    other => err!(format!("returns unsupported type {}", syn_to_str!(other))),
                }
                for input in inputs {
                    match input {
                        syn::FnArg::Captured(ref c) if valid_ty(&c.ty) => {
                            fn_sig.arg_tys.push(c.ty.clone())
                        }
                        other => err!(format!(
                            "takes unsupported argument type {}",
                            syn_to_str!(other)
                        )),
                    }
                }
                if !e {
                    functions.push(fn_sig);
                } else {
                    error = true;
                }
            }
            _ => (),
        }
    }
    if error {
        // too many errors:
        //        panic!("errors found");
    }
    functions
}

/// Parses a type into a String - arg is true if the type is an argument, and
/// false if its a return value.
fn valid_ty(t: &syn::Type) -> bool {
    match t {
        syn::Type::Ptr(p) => {
            let c = p.const_token.is_some();
            let m = p.mutability.is_some();
            assert!(!(c && m));
            match &*p.elem {
                syn::Type::Path(_) => valid_ty(&p.elem),
                // Only one layer of pointers allowed:
                _ => false,
            }
        }
        syn::Type::Path(p) => {
            assert!(p.qself.is_none());
            assert_eq!(p.path.segments.len(), 1);
            let s = p
                .path
                .segments
                .first()
                .unwrap()
                .into_value()
                .ident
                .to_string();
            match s.as_str() {
                "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize"
                | "f32" | "f64" => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn get_arg_ids(len: usize) -> Vec<syn::Ident> {
    let mut ids = Vec::new();
    for i in 0..len {
        let x = format!("x{}", i);
        ids.push(syn::Ident::new(&x, proc_macro2::Span::call_site()));
    }
    ids
}
