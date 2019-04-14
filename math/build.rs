extern crate cc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv") {
        let mut build = cc::Build::new();
        build.file("asm/syscall3.s");
        build.compile("asm");
    }
}