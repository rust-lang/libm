pub mod gen;
#[cfg(feature = "multiprecision-tests")]
pub mod mpfloat;
mod num_traits;
mod test_traits;
mod xfail;

pub use num_traits::{Float, Hex, Int};
pub use test_traits::{CheckBasis, CheckCtx, CheckOutput, GenerateInput, TupleCall};
pub use xfail::{IgnoreCase, XFail};

// List of all files present in libm's source
include!(concat!(env!("OUT_DIR"), "/all_files.rs"));

/// Default ULP allowed to differ from musl (note that musl itself may not be accurate).
const MUSL_DEFAULT_ULP: u32 = 2;

/// Default ULP allowed to differ from multiprecision (i.e. infinite) results.
const MULTIPREC_DEFAULT_ULP: u32 = 1;

/// ULP allowed to differ from muls results.
///
/// Current overrides includes:
/// - gamma functions that have higher errors
/// - 32-bit functions fall back to a less precise algorithm.
pub fn musl_allowed_ulp(name: &str) -> u32 {
    // Consider overrides xfail
    match name {
        #[cfg(x86_no_sse)]
        "asinh" | "asinhf" => 6,
        "lgamma" | "lgamma_r" | "lgammaf" | "lgammaf_r" => 6,
        "tanh" => 4,
        "tgamma" => 8,
        #[cfg(not(target_pointer_width = "64"))]
        "exp10" => 4,
        #[cfg(not(target_pointer_width = "64"))]
        "exp10f" => 4,
        _ => MUSL_DEFAULT_ULP,
    }
}

/// ULP allowed to differ from multiprecision results.
pub fn multiprec_allowed_ulp(name: &str) -> u32 {
    // Consider overrides xfail
    match name {
        "asinh" | "asinhf" => 2,
        "atanh" | "atanhf" => 2,
        "exp10" | "exp10f" => 3,
        "j0" | "j0f" => 2,
        "lgamma" | "lgammaf" | "lgamma_r" | "lgammaf_r" => 2,
        "sinh" | "sinhf" => 2,
        "tanh" | "tanhf" => 2,
        "tgamma" => 6,
        _ => MULTIPREC_DEFAULT_ULP,
    }
}
