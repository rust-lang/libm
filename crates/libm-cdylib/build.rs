use std::env;
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let profile = env::var("PROFILE").unwrap_or(String::new());
    if profile == "release" {
        println!("cargo:rustc-cfg=release_profile");
    }
}
