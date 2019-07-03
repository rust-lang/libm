use std::{fs, io, path::Path, process};

/// Writes the `content` string to a file at `path`.
pub(crate) fn write_to_file(path: &Path, content: &str) {
    use io::Write;
    let mut file = fs::File::create(&path).unwrap();
    write!(file, "{}", content).unwrap();
}

/// Compiles the libm-cdylib library as a C library.
///
/// This just compiles it once, all other times it just
/// succeeds. We compile it with --cfg link_test to
/// enable the tests.
pub(crate) fn compile_cdylib() {
    let mut cmd = process::Command::new("cargo");
    cmd.arg("build");
    if cfg!(release_profile) {
        cmd.arg("--release");
    }
    cmd.env("RUSTFLAGS", "--cfg=link_test");
    handle_err("lib_build", &cmd.output().unwrap());
}

/// Compiles the test C program with source at `src_path` into
/// an executable at `bin_path`.
pub(crate) fn compile_file(src_path: &Path, bin_path: &Path) {
    let cc = if std::env::var("CC").is_ok() {
        std::env::var("CC").unwrap().to_string()
    } else if cfg!(target_os = "linux") {
        "gcc".to_string()
    } else if cfg!(target_os = "macos") {
        "clang".to_string()
    } else {
        panic!("unknown platform - Ccompiler not found")
    };
    let mut cmd = process::Command::new(&cc);
    // We disable the usage of builtin functions, e.g., from libm.
    // This should ideally produce a link failure if libm is not dynamically
    // linked.
    //
    // On MacOSX libSystem is linked (for printf, etc.) and it links libSystem_m
    // transitively, so this doesn't help =/
    cmd.arg("-fno-builtin")
        .arg("-o")
        .arg(bin_path)
        .arg(src_path);

    // Link our libm
    let lib_path = cdylib_dir();
    cmd.arg(format!("-L{}", lib_path.display()));
    cmd.arg("-llibm");

    handle_err(
        &format!("compile file: {}", src_path.display()),
        &cmd.output().unwrap(),
    );
}

/// Run the program and verify that it prints the expected value.
pub(crate) fn check<T>(path: &Path, expected: T)
where
    T: PartialEq + std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut cmd = process::Command::new(path);

    if cfg!(target_os = "linux") {
        let ld_library_path = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
        let ld_library_path = format!("{}:{}", cdylib_dir().display(), ld_library_path);
        cmd.env("LD_LIBRARY_PATH", ld_library_path);
    }

    // Run the binary:
    let output = cmd.output().unwrap();
    handle_err(&format!("run file: {}", path.display()), &output);
    // Parse the result:
    let result = String::from_utf8(output.stdout.clone())
        .unwrap()
        .parse::<T>();

    if result.is_err() {
        panic!(format_output("check (parse failure)", &output));
    }
    // Check the result:
    let result = result.unwrap();
    assert_eq!(result, expected, "{}", format_output("check", &output));
}

pub(crate) fn handle_err(step: &str, output: &process::Output) {
    if !output.status.success() {
        panic!("{}", format_output(step, output));
    }
}

pub(crate) fn format_output(
    step: &str,
    process::Output {
        status,
        stdout,
        stderr,
    }: &process::Output,
) -> String {
    let mut s = format!("\nFAILED[{}]: exit code {:?}\n", step, status.code());
    s += &format!(
        "FAILED[{}]: stdout:\n\n{}\n\n",
        step,
        String::from_utf8(stdout.to_vec()).unwrap()
    );
    s += &format!(
        "FAILED[{}]: stderr:\n\n{}\n\n",
        step,
        String::from_utf8(stderr.to_vec()).unwrap()
    );
    s
}

/// For a given Rust type `x`, this prints the name of the type in C,
/// as well as the printf format specifier used to print values of that type.
pub(crate) fn ctype_and_printf_format_specifier(x: &str) -> (&str, &str) {
    match x {
        // Note: fprintf has no format specifier for floats, `%f`, converts
        // floats into a double, and prints that.
        //
        // For the linking tests, precision doesn't really matter. The only
        // thing that's tested is whether our implementation was properly called
        // or not. This is done by making our functions return an incorrect
        // magic value, different from the correct result. So as long as this is
        // precise enough for us to be able to parse `42.0` from stdout as
        // 42_f32/f64, everything works.
        "f32" => ("float", "%f"),
        "f64" => ("double", "%f"),
        "i32" => ("int32_t", "%d"),
        _ => panic!("unknown type: {}", x),
    }
}

pub(crate) fn target_dir() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        std::path::PathBuf::from(&dir)
    } else {
        Path::new("../../target").into()
    }
}

pub(crate) fn cdylib_dir() -> std::path::PathBuf {
    target_dir().join(if cfg!(release_profile) {
        "release"
    } else {
        "debug"
    })
}

pub(crate) fn cdylib_path() -> std::path::PathBuf {
    let libm_path = cdylib_dir();
    if cfg!(target_os = "macos") {
        libm_path.join("liblibm.dylib")
    } else if cfg!(target_os = "linux") {
        libm_path.join("liblibm.so")
    } else {
        panic!("unsupported target_os")
    }
}
