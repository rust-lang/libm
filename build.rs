use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if !cfg!(feature = "checked") {
        let lvl = env::var("OPT_LEVEL").unwrap();
        if lvl != "0" {
            println!("cargo:rustc-cfg=assert_no_panic");
        }
    }
}
