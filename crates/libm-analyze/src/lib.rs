#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use self::proc_macro::TokenStream;

#[proc_macro]
pub fn for_each_api(input: TokenStream) -> TokenStream {
    let files = get_libm_files();
    let functions = get_functions(files);
    input
}

/// Traverses the libm crate directory, parsing all .rs files
fn get_libm_files() -> Vec<(syn::File, String)> {
    let root_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    dbg!(&root_dir);
    let libm_dir = root_dir
        .parent()
        .expect("couldn't access crates/ dir")
        .join("libm");
    dbg!(&libm_dir);
    let libm_src_dir = libm_dir.join("src");
    dbg!(&libm_src_dir);

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

        eprintln!("{}", file_path.display());
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
    constness: bool,
    asyncness: bool,
    attrs: Vec<syn::Attribute>,
    abi: &'static str,

}

/// Extracts all public functions from the libm files.
fn get_functions(files: Vec<(syn::File, String)>) {
    //let mut functions = Vec::new();
    for item in files.iter().flat_map(|f| f.0.items.iter()) {
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
                if let Some(syn::Abi { name: Some(l), extern_token: _ }) = abi {
                    println!("{:#?}", l);
                    if l.value() != "C" {
                        l.span().unwrap().warning(
                            "public libm function is not `extern \"C\""
                        ).emit();
                    }
                } else {
                    ident.span().unwrap().warning(
                        "public libm function is not `extern \"C\""
                    ).emit();
                }
                println!("{:#?}", ident);
            }
            _ => (),
        }
    }
}
