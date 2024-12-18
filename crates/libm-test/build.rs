use std::fmt::Write;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let cfg = Config::from_env();

    emit_optimization_cfg(&cfg);
    emit_cfg_shorthands(&cfg);
    list_all_tests(&cfg);

    #[cfg(feature = "test-musl-serialized")]
    musl_serialized_tests::generate();
}

#[allow(dead_code)]
struct Config {
    manifest_dir: PathBuf,
    out_dir: PathBuf,
    opt_level: u8,
    target_arch: String,
    target_env: String,
    target_family: Option<String>,
    target_os: String,
    target_string: String,
    target_vendor: String,
    target_features: Vec<String>,
}

impl Config {
    fn from_env() -> Self {
        let target_features = env::var("CARGO_CFG_TARGET_FEATURE")
            .map(|feats| feats.split(',').map(ToOwned::to_owned).collect())
            .unwrap_or_default();

        Self {
            manifest_dir: PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()),
            out_dir: PathBuf::from(env::var("OUT_DIR").unwrap()),
            opt_level: env::var("OPT_LEVEL").unwrap().parse().unwrap(),
            target_arch: env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            target_env: env::var("CARGO_CFG_TARGET_ENV").unwrap(),
            target_family: env::var("CARGO_CFG_TARGET_FAMILY").ok(),
            target_os: env::var("CARGO_CFG_TARGET_OS").unwrap(),
            target_string: env::var("TARGET").unwrap(),
            target_vendor: env::var("CARGO_CFG_TARGET_VENDOR").unwrap(),
            target_features,
        }
    }
}

/// Some tests are extremely slow. Emit a config option based on optimization level.
fn emit_optimization_cfg(cfg: &Config) {
    println!("cargo::rustc-check-cfg=cfg(optimizations_enabled)");

    if cfg.opt_level >= 2 {
        println!("cargo::rustc-cfg=optimizations_enabled");
    }
}

/// Provide an alias for common longer config combinations.
fn emit_cfg_shorthands(cfg: &Config) {
    println!("cargo::rustc-check-cfg=cfg(x86_no_sse)");
    if cfg.target_arch == "x86" && !cfg.target_features.iter().any(|f| f == "sse") {
        // Shorthand to detect i586 targets
        println!("cargo::rustc-cfg=x86_no_sse");
    }
}

/// Create a list of all source files in an array. This can be used for making sure that
/// all functions are tested or otherwise covered in some way.
// FIXME: it would probably be better to use rustdoc JSON output to get public functions.
fn list_all_tests(cfg: &Config) {
    let math_src = cfg.manifest_dir.join("../../src/math");

    let mut files = fs::read_dir(math_src)
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|entry| entry.is_file())
        .map(|f| f.file_stem().unwrap().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    files.sort();

    let mut s = "pub const ALL_FUNCTIONS: &[&str] = &[".to_owned();
    for f in files {
        if f == "mod" {
            // skip mod.rs
            continue;
        }
        write!(s, "\"{f}\",").unwrap();
    }
    write!(s, "];").unwrap();

    let outfile = cfg.out_dir.join("all_files.rs");
    fs::write(outfile, s).unwrap();
}

/// At build time, generate the output of what the corresponding `*musl` target does with a range
/// of inputs.
///
/// Serialize that target's output, run the same thing with our symbols, then load and compare
/// the resulting values.
#[cfg(feature = "test-musl-serialized")]
mod musl_serialized_tests {
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, fs};

    use rand::Rng;
    use rand::seq::SliceRandom;

    // Number of tests to generate for each function
    const NTESTS: usize = 500;

    // These files are all internal functions or otherwise miscellaneous, not
    // defining a function we want to test.
    const IGNORED_FILES: &[&str] = &[
        "fenv.rs",
        // These are giving slightly different results compared to musl
        "lgamma.rs",
        "lgammaf.rs",
        "tgamma.rs",
        "j0.rs",
        "j0f.rs",
        "jn.rs",
        "jnf.rs",
        "j1.rs",
        "j1f.rs",
    ];

    struct Function {
        name: String,
        args: Vec<Ty>,
        ret: Vec<Ty>,
        tests: Vec<Test>,
    }

    enum Ty {
        F32,
        F64,
        I32,
        Bool,
    }

    struct Test {
        inputs: Vec<i64>,
        outputs: Vec<i64>,
    }

    pub fn generate() {
        // PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let libm_test = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let math_src = libm_test.join("../../src/math");

        if target_arch == "powerpc64" {
            return;
        }

        let files = fs::read_dir(math_src)
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(file_needs_test)
            .collect::<Vec<_>>();

        let mut math = Vec::new();
        for file in files {
            if IGNORED_FILES.iter().any(|f| file.ends_with(f)) {
                continue;
            }

            println!("generating musl reference tests in {:?}", file);

            let contents = fs::read_to_string(file).unwrap();
            let mut functions = contents.lines().filter(|f| f.starts_with("pub fn"));
            while let Some(function_to_test) = functions.next() {
                math.push(parse(function_to_test));
            }
        }

        // Generate a bunch of random inputs for each function. This will
        // attempt to generate a good set of uniform test cases for exercising
        // all the various functionality.
        generate_random_tests(&mut math, &mut rand::thread_rng());

        // After we have all our inputs, use the x86_64-unknown-linux-musl
        // target to generate the expected output.
        generate_test_outputs(&mut math);
        //panic!("Boo");
        // ... and now that we have both inputs and expected outputs, do a bunch
        // of codegen to create the unit tests which we'll actually execute.
        generate_unit_tests(&math);
    }

    /// Check whether a path within `src/math` should get tests generated.
    fn file_needs_test(path: &PathBuf) -> bool {
        // Skip directories
        if path.is_dir() {
            return false;
        }

        let fname = path.file_name().unwrap().to_str().unwrap();

        // Musl doesn't support `f16` or `f128`
        !(fname.contains("f16") || fname.contains("f128"))
    }

    /// A "poor man's" parser for the signature of a function
    fn parse(s: &str) -> Function {
        let s = eat(s, "pub fn ");
        let pos = s.find('(').unwrap();
        let name = &s[..pos];
        let s = &s[pos + 1..];
        let end = s.find(')').unwrap();
        let args = s[..end]
            .split(',')
            .map(|arg| {
                let colon = arg.find(':').unwrap();
                parse_ty(arg[colon + 1..].trim())
            })
            .collect::<Vec<_>>();
        let tail = &s[end + 1..];
        let tail = eat(tail, " -> ");
        let ret = parse_retty(tail.replace("{", "").trim());

        return Function { name: name.to_string(), args, ret, tests: Vec::new() };

        fn parse_ty(s: &str) -> Ty {
            match s {
                "f32" => Ty::F32,
                "f64" => Ty::F64,
                "i32" => Ty::I32,
                "bool" => Ty::Bool,
                other => panic!("unknown type `{}`", other),
            }
        }

        fn parse_retty(s: &str) -> Vec<Ty> {
            match s {
                "(f32, f32)" => vec![Ty::F32, Ty::F32],
                "(f32, i32)" => vec![Ty::F32, Ty::I32],
                "(f64, f64)" => vec![Ty::F64, Ty::F64],
                "(f64, i32)" => vec![Ty::F64, Ty::I32],
                other => vec![parse_ty(other)],
            }
        }

        fn eat<'a>(s: &'a str, prefix: &str) -> &'a str {
            if s.starts_with(prefix) {
                &s[prefix.len()..]
            } else {
                panic!("{:?} didn't start with {:?}", s, prefix)
            }
        }
    }

    fn generate_random_tests<R: Rng>(functions: &mut [Function], rng: &mut R) {
        for function in functions {
            for _ in 0..NTESTS {
                function.tests.push(generate_test(function, rng));
            }
        }

        fn generate_test<R: Rng>(function: &Function, rng: &mut R) -> Test {
            let mut inputs = function.args.iter().map(|ty| ty.gen_i64(rng)).collect::<Vec<_>>();

            // First argument to this function appears to be a number of
            // iterations, so passing in massive random numbers causes it to
            // take forever to execute, so make sure we're not running random
            // math code until the heat death of the universe.
            if function.name == "jn" || function.name == "jnf" {
                inputs[0] &= 0xffff;
            }

            Test {
                inputs,
                // zero output for now since we'll generate it later
                outputs: vec![],
            }
        }
    }

    impl Ty {
        fn gen_i64<R: Rng>(&self, r: &mut R) -> i64 {
            use std::{f32, f64};

            return match self {
                Ty::F32 => {
                    if r.gen_range(0..20) < 1 {
                        let i = *[f32::NAN, f32::INFINITY, f32::NEG_INFINITY].choose(r).unwrap();
                        i.to_bits().into()
                    } else {
                        r.gen::<f32>().to_bits().into()
                    }
                }
                Ty::F64 => {
                    if r.gen_range(0..20) < 1 {
                        let i = *[f64::NAN, f64::INFINITY, f64::NEG_INFINITY].choose(r).unwrap();
                        i.to_bits() as i64
                    } else {
                        r.gen::<f64>().to_bits() as i64
                    }
                }
                Ty::I32 => {
                    if r.gen_range(0..10) < 1 {
                        let i = *[i32::max_value(), 0, i32::min_value()].choose(r).unwrap();
                        i.into()
                    } else {
                        r.gen::<i32>().into()
                    }
                }
                Ty::Bool => r.gen::<bool>() as i64,
            };
        }

        fn libc_ty(&self) -> &'static str {
            match self {
                Ty::F32 => "f32",
                Ty::F64 => "f64",
                Ty::I32 => "i32",
                Ty::Bool => "i32",
            }
        }

        fn libc_pty(&self) -> &'static str {
            match self {
                Ty::F32 => "*mut f32",
                Ty::F64 => "*mut f64",
                Ty::I32 => "*mut i32",
                Ty::Bool => "*mut i32",
            }
        }

        fn default(&self) -> &'static str {
            match self {
                Ty::F32 => "0_f32",
                Ty::F64 => "0_f64",
                Ty::I32 => "0_i32",
                Ty::Bool => "false",
            }
        }

        fn to_i64(&self) -> &'static str {
            match self {
                Ty::F32 => ".to_bits() as i64",
                Ty::F64 => ".to_bits() as i64",
                Ty::I32 => " as i64",
                Ty::Bool => " as i64",
            }
        }
    }

    fn generate_test_outputs(functions: &mut [Function]) {
        let mut src = String::new();
        let dst = std::env::var("OUT_DIR").unwrap();

        // Generate a program which will run all tests with all inputs in
        // `functions`. This program will write all outputs to stdout (in a
        // binary format).
        src.push_str("use std::io::Write;");
        src.push_str("fn main() {");
        src.push_str("let mut result = Vec::new();");
        for function in functions.iter_mut() {
            src.push_str("unsafe {");
            src.push_str("extern { fn ");
            src.push_str(&function.name);
            src.push_str("(");

            let (ret, retptr) = match function.name.as_str() {
                "sincos" | "sincosf" => (None, &function.ret[..]),
                _ => (Some(&function.ret[0]), &function.ret[1..]),
            };
            for (i, arg) in function.args.iter().enumerate() {
                src.push_str(&format!("arg{}: {},", i, arg.libc_ty()));
            }
            for (i, ret) in retptr.iter().enumerate() {
                src.push_str(&format!("argret{}: {},", i, ret.libc_pty()));
            }
            src.push_str(")");
            if let Some(ty) = ret {
                src.push_str(" -> ");
                src.push_str(ty.libc_ty());
            }
            src.push_str("; }");

            src.push_str(&format!("static TESTS: &[[i64; {}]]", function.args.len()));
            src.push_str(" = &[");
            for test in function.tests.iter() {
                src.push_str("[");
                for val in test.inputs.iter() {
                    src.push_str(&val.to_string());
                    src.push_str(",");
                }
                src.push_str("],");
            }
            src.push_str("];");

            src.push_str("for test in TESTS {");
            for (i, arg) in retptr.iter().enumerate() {
                src.push_str(&format!("let mut argret{} = {};", i, arg.default()));
            }
            src.push_str("let output = ");
            src.push_str(&function.name);
            src.push_str("(");
            for (i, arg) in function.args.iter().enumerate() {
                src.push_str(&match arg {
                    Ty::F32 => format!("f32::from_bits(test[{}] as u32)", i),
                    Ty::F64 => format!("f64::from_bits(test[{}] as u64)", i),
                    Ty::I32 => format!("test[{}] as i32", i),
                    Ty::Bool => format!("test[{}] as i32", i),
                });
                src.push_str(",");
            }
            for (i, _) in retptr.iter().enumerate() {
                src.push_str(&format!("&mut argret{},", i));
            }
            src.push_str(");");
            if let Some(ty) = &ret {
                src.push_str(&format!("let output = output{};", ty.to_i64()));
                src.push_str("result.extend_from_slice(&output.to_le_bytes());");
            }

            for (i, ret) in retptr.iter().enumerate() {
                src.push_str(&format!(
                    "result.extend_from_slice(&(argret{}{}).to_le_bytes());",
                    i,
                    ret.to_i64(),
                ));
            }
            src.push_str("}");

            src.push_str("}");
        }

        src.push_str("std::io::stdout().write_all(&result).unwrap();");

        src.push_str("}");

        let path = format!("{}/gen.rs", dst);
        fs::write(&path, src).unwrap();

        // Make it somewhat pretty if something goes wrong
        drop(Command::new("rustfmt").arg(&path).status());

        // Compile and execute this tests for the musl target, assuming we're an
        // x86_64 host effectively.
        let status = Command::new("rustc")
            .current_dir(&dst)
            .arg(&path)
            .arg("--target=x86_64-unknown-linux-musl")
            .status()
            .unwrap();
        assert!(status.success());
        let output = Command::new("./gen").current_dir(&dst).output().unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());

        // Map all the output bytes back to an `i64` and then shove it all into
        // the expected results.
        let mut results = output.stdout.chunks_exact(8).map(|buf| {
            let mut exact = [0; 8];
            exact.copy_from_slice(buf);
            i64::from_le_bytes(exact)
        });

        for f in functions.iter_mut() {
            for test in f.tests.iter_mut() {
                test.outputs = (0..f.ret.len()).map(|_| results.next().unwrap()).collect();
            }
        }
        assert!(results.next().is_none());
    }

    /// Codegens a file which has a ton of `#[test]` annotations for all the
    /// tests that we generated above.
    fn generate_unit_tests(functions: &[Function]) {
        let mut src = String::new();
        let dst = std::env::var("OUT_DIR").unwrap();

        for function in functions {
            src.push_str("#[test]");
            src.push_str("fn ");
            src.push_str(&function.name);
            src.push_str("_matches_musl() {");
            src.push_str(&format!(
                "static TESTS: &[([i64; {}], [i64; {}])]",
                function.args.len(),
                function.ret.len(),
            ));
            src.push_str(" = &[");
            for test in function.tests.iter() {
                src.push_str("([");
                for val in test.inputs.iter() {
                    src.push_str(&val.to_string());
                    src.push_str(",");
                }
                src.push_str("],");
                src.push_str("[");
                for val in test.outputs.iter() {
                    src.push_str(&val.to_string());
                    src.push_str(",");
                }
                src.push_str("],");
                src.push_str("),");
            }
            src.push_str("];");

            src.push_str("for (test, expected) in TESTS {");
            src.push_str("let output = libm::");
            src.push_str(&function.name);
            src.push_str("(");
            for (i, arg) in function.args.iter().enumerate() {
                src.push_str(&match arg {
                    Ty::F32 => format!("f32::from_bits(test[{}] as u32)", i),
                    Ty::F64 => format!("f64::from_bits(test[{}] as u64)", i),
                    Ty::I32 => format!("test[{}] as i32", i),
                    Ty::Bool => format!("test[{}] as i32", i),
                });
                src.push_str(",");
            }
            src.push_str(");");

            for (i, ret) in function.ret.iter().enumerate() {
                let get = if function.ret.len() == 1 { String::new() } else { format!(".{}", i) };
                src.push_str(&(match ret {
                    Ty::F32 => format!("if libm::_eqf(output{}, f32::from_bits(expected[{}] as u32)).is_ok() {{ continue }}", get, i),
                    Ty::F64 => format!("if libm::_eq(output{}, f64::from_bits(expected[{}] as u64)).is_ok() {{ continue }}", get, i),
                    Ty::I32 => format!("if output{} as i64 == expected[{}] {{ continue }}", get, i),
                    Ty::Bool => unreachable!(),
                }));
            }

            src.push_str(
                r#"
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", test, expected, output);
            "#,
            );
            src.push_str("}");

            src.push_str("}");
        }

        let path = format!("{}/musl-tests.rs", dst);
        fs::write(&path, src).unwrap();

        // Try to make it somewhat pretty
        drop(Command::new("rustfmt").arg(&path).status());
    }
}
