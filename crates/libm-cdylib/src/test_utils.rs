use std::{fs, io, path::Path, process};

pub(crate) fn write_to_file(path: &Path, content: &str) {
    use io::Write;
    let mut file = fs::File::create(&path).unwrap();
    write!(file, "{}", content).unwrap();
}

pub(crate) fn compile_lib() {
    let mut cmd = process::Command::new("cargo");
    cmd.arg("build");
    if cfg!(release_profile) {
        cmd.arg("--release");
    }
    cmd.env("RUSTFLAGS", "--cfg=link_test");
    handle_err("lib_build", &cmd.output().unwrap());
}

pub(crate) fn compile_file(src_path: &Path, bin_path: &Path) {
    let mut cmd = process::Command::new("CC");
    cmd.arg("-fno-builtin")
        .arg("-o")
        .arg(bin_path)
        .arg(src_path);
    handle_err(
        &format!("compile file: {}", src_path.display()),
        &cmd.output().unwrap(),
    );
}

pub(crate) fn check<T>(path: &Path, expected: T)
where
    T: PartialEq + std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut cmd = process::Command::new(path);

    let libm_path = format!(
        "../../target/{}/liblibm.",
        if cfg!(release_profile) {
            "release"
        } else {
            "debug"
        },
    );

    // Replace libm at runtime
    if cfg!(target_os = "macos") {
        // cmd.env("DYLD_PRINT_LIBRARIES", "1");
        // cmd.env("X", "1");
        cmd.env("DYLD_FORCE_FLAT_NAMESPACE", "1");
        cmd.env(
            "DYLD_INSERT_LIBRARIES",
            format!("{}.{}", libm_path, "dylib"),
        );
    } else if cfg!(target_os = "linux") {
        cmd.env("LD_PRELOAD", format!("{}.{}", libm_path, "so"))
    }
    let output = cmd.output().unwrap();
    handle_err(&format!("run file: {}", path.display()), &output);
    let result = String::from_utf8(output.stdout.clone())
        .unwrap()
        .parse::<T>();

    if result.is_err() {
        panic!(format_output("check (parse failure)", &output));
    }
    let result = result.unwrap();
    assert_eq!(result, expected, "{}", format_output("check", &output));
}

pub(crate) fn handle_err(step: &str, output: &process::Output) {
    if !output.status.success() {
        eprintln!("{}", format_output(step, output));
        panic!();
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

pub(crate) fn ctype_and_cformat(x: &str) -> (&str, &str) {
    match x {
        "f32" => ("float", "%f"),
        "f64" => ("double", "%f"),
        "i32" => ("int32_t", "%d"),
        _ => panic!("unknown type: {}", x),
    }
}
