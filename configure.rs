// Configuration shared with both libm and libm-test

use std::env;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Config {
    pub manifest_dir: PathBuf,
    pub out_dir: PathBuf,
    pub opt_level: u8,
    pub target_arch: String,
    pub target_env: String,
    pub target_family: Option<String>,
    pub target_os: String,
    pub target_string: String,
    pub target_vendor: String,
    pub target_features: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
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

/// Libm gets most config options made available.
#[allow(dead_code)]
pub fn emit_libm_config(cfg: &Config) {
    emit_intrinsics_cfg();
    emit_arch_cfg();
    emit_optimization_cfg(&cfg);
    emit_cfg_shorthands(&cfg);
    emit_f16_f128_cfg(&cfg);
}

/// Tests don't need most feature-related config.
#[allow(dead_code)]
pub fn emit_test_config(cfg: &Config) {
    emit_optimization_cfg(&cfg);
    emit_cfg_shorthands(&cfg);
    emit_f16_f128_cfg(&cfg);
}

/// Simplify the feature logic for enabling intrinsics so code only needs to use
/// `cfg(intrinsics_enabled)`.
fn emit_intrinsics_cfg() {
    println!("cargo:rustc-check-cfg=cfg(intrinsics_enabled)");

    // Disabled by default; `unstable-intrinsics` enables again; `force-soft-floats` overrides
    // to disable.
    if cfg!(feature = "unstable-intrinsics") && !cfg!(feature = "force-soft-floats") {
        println!("cargo:rustc-cfg=intrinsics_enabled");
    }
}

/// Simplify the feature logic for enabling arch-specific features so code only needs to use
/// `cfg(arch_enabled)`.
fn emit_arch_cfg() {
    println!("cargo:rustc-check-cfg=cfg(arch_enabled)");

    // Enabled by default via the "arch" feature, `force-soft-floats` overrides to disable.
    if cfg!(feature = "arch") && !cfg!(feature = "force-soft-floats") {
        println!("cargo:rustc-cfg=arch_enabled");
    }
}

/// Some tests are extremely slow. Emit a config option based on optimization level.
fn emit_optimization_cfg(cfg: &Config) {
    println!("cargo:rustc-check-cfg=cfg(optimizations_enabled)");

    if cfg.opt_level >= 2 {
        println!("cargo:rustc-cfg=optimizations_enabled");
    }
}

/// Provide an alias for common longer config combinations.
fn emit_cfg_shorthands(cfg: &Config) {
    println!("cargo:rustc-check-cfg=cfg(x86_no_sse)");
    if cfg.target_arch == "x86" && !cfg.target_features.iter().any(|f| f == "sse") {
        // Shorthand to detect i586 targets
        println!("cargo:rustc-cfg=x86_no_sse");
    }
}

/// Configure whether or not `f16` and `f128` support should be enabled.
fn emit_f16_f128_cfg(cfg: &Config) {
    println!("cargo:rustc-check-cfg=cfg(f16_enabled)");
    println!("cargo:rustc-check-cfg=cfg(f128_enabled)");

    // `unstable-float` enables these features. Either `no-f16-f128` or `force-soft-floats`
    // will disable them.
    if !cfg!(feature = "unstable-float")
        || cfg!(feature = "no-f16-f128")
        || cfg!(feature = "force-soft-floats")
    {
        return;
    }

    // Set whether or not `f16` and `f128` are supported at a basic level by LLVM. This only means
    // that the backend will not crash when using these types. This does not mean that the
    // backend does the right thing, or that the platform doesn't have ABI bugs.
    //
    // We do this here rather than in `rust-lang/rust` because configuring via cargo features is
    // not straightforward.
    //
    // Original source of this list:
    // <https://github.com/rust-lang/compiler-builtins/pull/652#issuecomment-2266151350>
    let (f16_ok, f128_ok) = match cfg.target_arch.as_str() {
        // `f16` and `f128` both crash <https://github.com/llvm/llvm-project/issues/94434>
        "arm64ec" => (false, false),
        // `f16` crashes <https://github.com/llvm/llvm-project/issues/50374>
        "s390x" => (false, true),
        // `f128` crashes <https://github.com/llvm/llvm-project/issues/96432>
        "mips64" | "mips64r6" => (true, false),
        // `f128` crashes <https://github.com/llvm/llvm-project/issues/101545>
        "powerpc64" if &cfg.target_os == "aix" => (true, false),
        // `f128` crashes <https://github.com/llvm/llvm-project/issues/41838>
        "sparc" | "sparcv9" => (true, false),
        // `f16` miscompiles <https://github.com/llvm/llvm-project/issues/96438>
        "wasm32" | "wasm64" => (false, true),
        // Most everything else works as of LLVM 19
        _ => (true, true),
    };

    if f16_ok {
        println!("cargo:rustc-cfg=f16_enabled");
    }

    if f128_ok {
        println!("cargo:rustc-cfg=f128_enabled");
    }
}
