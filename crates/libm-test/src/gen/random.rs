//! A simple generator that produces deterministic random input, caching to use the same
//! inputs for all functions.

use std::sync::LazyLock;

use crate::GenerateInput;

use super::CachedInput;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const SEED: [u8; 32] = *b"3.141592653589793238462643383279";

/// Number of tests to run.
const NTESTS: usize = {
    let mut ntests = if cfg!(optimizations_enabled) {
        5000
    } else {
        500
    };

    // Tests can be pretty slow on non-64-bit targets and, for some reason, ppc.
    if !cfg!(target_pointer_width = "64") || cfg!(target_arch = "powerpc64") {
        ntests /= 5;
    }

    ntests
};

/// Tested inputs.
static TEST_CASES: LazyLock<CachedInput> = LazyLock::new(|| make_test_cases(NTESTS));

/// The first argument to `jn` and `jnf` is the number of iterations. Make this a reasonable
/// value so tests don't run forever.
static TEST_CASES_JN: LazyLock<CachedInput> = LazyLock::new(|| {
    // It is easy to overflow the stack with these in debug mode
    let iterations = if cfg!(optimizations_enabled) && cfg!(target_pointer_width = "64") {
        0xffff
    } else if cfg!(windows) {
        0x00ff
    } else {
        0x0fff
    };

    let mut cases = (&*TEST_CASES).clone();
    for case in cases.inputs_i32.iter_mut() {
        case.0 = iterations;
    }
    for case in cases.inputs_i32.iter_mut() {
        case.0 = iterations;
    }
    cases
});

fn make_test_cases(ntests: usize) -> CachedInput {
    let mut rng = ChaCha8Rng::from_seed(SEED);

    let inputs_i32 = (0..ntests).map(|_| rng.gen::<(i32, i32, i32)>()).collect();
    let inputs_f32 = (0..ntests).map(|_| rng.gen::<(f32, f32, f32)>()).collect();
    let inputs_f64 = (0..ntests).map(|_| rng.gen::<(f64, f64, f64)>()).collect();

    CachedInput {
        inputs_f32,
        inputs_f64,
        inputs_i32,
    }
}

/// Create a test case iterator.
pub fn get_test_cases<RustArgs>(fname: &str) -> impl Iterator<Item = RustArgs>
where
    CachedInput: GenerateInput<RustArgs>,
{
    let inputs = if fname == "jn" || fname == "jnf" {
        &TEST_CASES_JN
    } else {
        &TEST_CASES
    };

    CachedInput::get_cases(inputs)
}
