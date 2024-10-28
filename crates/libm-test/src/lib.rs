pub mod gen;
mod num_traits;
mod special_case;
mod test_traits;

pub use num_traits::{Float, Hex, Int};
pub use special_case::{MaybeOverride, SpecialCase};
pub use test_traits::{CheckBasis, CheckCtx, CheckOutput, GenerateInput, TupleCall};

/// Result type for tests is usually from `anyhow`. Most times there is no success value to
/// propagate.
pub type TestResult<T = (), E = anyhow::Error> = Result<T, E>;

// List of all files present in libm's source
include!(concat!(env!("OUT_DIR"), "/all_files.rs"));

/// ULP allowed to differ from musl (note that musl itself may not be accurate).
const MUSL_DEFAULT_ULP: u32 = 2;

/// Certain functions have different allowed ULP (consider these xfail).
///
/// Note that these results were obtained using 400,000,000 rounds of random inputs, which
/// is not a value used by default.
pub fn musl_allowed_ulp(name: &str) -> u32 {
    match name {
        #[cfg(x86_no_sse)]
        "asinh" | "asinhf" => 6,
        "lgamma" | "lgamma_r" | "lgammaf" | "lgammaf_r" => 400,
        "tanh" | "tanhf" => 4,
        "tgamma" => 20,
        "j0" | "j0f" | "j1" | "j1f" => {
            // Results seem very target-dependent
            if cfg!(target_arch = "x86_64") { 4000 } else { 800_000 }
        }
        "jn" | "jnf" => 1000,
        "sincosf" => 500,
        #[cfg(not(target_pointer_width = "64"))]
        "exp10" => 4,
        #[cfg(not(target_pointer_width = "64"))]
        "exp10f" => 4,
        _ => MUSL_DEFAULT_ULP,
    }
}

/// Return the unsuffixed version of a function name; e.g. `abs` and `absf` both return `abs`,
/// `lgamma_r` and `lgammaf_r` both return `lgamma_r`.
pub fn canonical_name(name: &str) -> &str {
    let known_mappings = &[
        ("erff", "erf"),
        ("erf", "erf"),
        ("lgammaf_r", "lgamma_r"),
        ("modff", "modf"),
        ("modf", "modf"),
    ];

    match known_mappings.iter().find(|known| known.0 == name) {
        Some(found) => found.1,
        None => name
            .strip_suffix("f")
            .or_else(|| name.strip_suffix("f16"))
            .or_else(|| name.strip_suffix("f128"))
            .unwrap_or(name),
    }
}
