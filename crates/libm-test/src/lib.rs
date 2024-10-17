pub mod gen;
mod num_traits;
mod test_traits;

pub use num_traits::{Float, Hex, Int};
pub use test_traits::{CheckOutput, GenerateInput, TupleCall};

// List of all files present in libm's source
include!(concat!(env!("OUT_DIR"), "/all_files.rs"));

/// ULP allowed to differ from musl (note that musl itself may not be accurate).
const MUSL_DEFAULT_ULP: u32 = 2;

/// Certain functions have different allowed ULP (consider these xfail).
///
/// Currently this includes:
/// - gamma functions that have higher errors
/// - 32-bit functions fall back to a less precise algorithm.
pub fn musl_allowed_ulp(name: &str) -> u32 {
    match name {
        #[cfg(x86_no_sse)]
        "asinhf" => 6,
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
