//! Program to write all inputs from a generator to a file, then invoke a Julia script
//! to plot them. Requires Julia with the `CairoMakie` dependency.
//!
//! Note that running in release mode by default generates a _lot_ more datapoints, which
//! causes plotting to be extremely slow (some simplification to be done in the script).

use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use libm_test::domain::Domain;
use libm_test::gen::domain_logspace;

const JL_PLOT: &str = "examples/plot_file.jl";

fn main() {
    let out_dir = Path::new("build");
    if !out_dir.exists() {
        fs::create_dir(out_dir).unwrap();
    }

    let jl_script = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(JL_PLOT);
    let mut j_args = Vec::new();

    // Plot a few domains with some functions that use them.
    plot_one(out_dir, "sqrt", Domain::SQRT, &mut j_args);
    plot_one(out_dir, "cos", Domain::TRIG, &mut j_args);
    plot_one(out_dir, "cbrt", Domain::UNBOUNDED, &mut j_args);

    println!("launching script");
    let mut cmd = Command::new("julia");
    if !cfg!(debug_assertions) {
        cmd.arg("-O3");
    }
    cmd.arg(jl_script).args(j_args).status().unwrap();
}

/// Plot a single domain.
fn plot_one(out_dir: &Path, name: &str, domain: Domain<f32>, j_args: &mut Vec<String>) {
    let base_name = out_dir.join(format!("domain-inputs-{name}"));
    let text_file = base_name.with_extension("txt");

    {
        // Scope for file and writer
        let f = fs::File::create(&text_file).unwrap();
        let mut w = BufWriter::new(f);

        for input in domain_logspace::get_test_cases_inner::<f32>(domain) {
            writeln!(w, "{:e}", input.0).unwrap();
        }
        w.flush().unwrap();
    }

    // The julia script expects `name1 path1 name2 path2...` args
    j_args.push(name.to_owned());
    j_args.push(base_name.to_str().unwrap().to_owned());
}
