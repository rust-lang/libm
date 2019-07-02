use std::env;
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let profile = env::var("PROFILE").unwrap_or(String::new());
    if profile == "release" {
        println!("cargo:rustc-cfg=release_profile");
    }
    let nightly = {
        let mut cmd = std::process::Command::new("rustc");
        cmd.arg("--version");
        let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
        output.contains("nightly")
    };
    if nightly {
        println!("cargo:rustc-cfg=unstable_rust");
    }
}
