use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = env::var("TARGET").expect("TARGET was not set");
    let profile = env::var("PROFILE").unwrap_or(String::new());
    let opt_level: i32 = env::var("OPT_LEVEL").unwrap().parse().unwrap();
    if !cfg!(feature = "checked") {
        if opt_level != 0 {
            println!("cargo:rustc-cfg=assert_no_panic");
        }
    }

    if profile == "release" || opt_level > 0 {
        match target.as_str() {
            "x86_64-unknown-linux-gnu" | "x86_64-apple-darwin" | "x86_64-pc-windows-msvc" => {
                println!("cargo:rustc-cfg=exhaustive32");
            }
            _ => (),
        }
    }
}
