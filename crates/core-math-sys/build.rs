use std::env;
use std::path::PathBuf;

const SOURCES: &[&str] = &[
    "binary64/cos/cos.c",
    "binary64/erf/erf.c",
    // "binary64/exp/exp.c",
    "binary64/sin/sin.c",
    "binary64/tan/tan.c",
];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut cbuild = cc::Build::new();
    cbuild.files(SOURCES.iter().map(|s| manifest_dir.join("core-math/src").join(s)));
    cbuild.compile("core_math");
}
