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

    // Tests seem to be pretty slow on non-64-bit targets, emulated ppc, and x86 MacOS
    if !cfg!(target_pointer_width = "64")
        || cfg!(target_arch = "powerpc64")
        || cfg!(all(target_arch = "x86_64", target_vendor = "apple"))
    {
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

    // make sure we include some basic cases
    let mut inputs_i32 = vec![(0, 0, 0), (1, 1, 1), (-1, -1, -1)];
    let mut inputs_f32 = vec![
        (0.0, 0.0, 0.0),
        (f32::EPSILON, f32::EPSILON, f32::EPSILON),
        (f32::INFINITY, f32::INFINITY, f32::INFINITY),
        (f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        (f32::MAX, f32::MAX, f32::MAX),
        (f32::MIN, f32::MIN, f32::MIN),
        (f32::MIN_POSITIVE, f32::MIN_POSITIVE, f32::MIN_POSITIVE),
        (f32::NAN, f32::NAN, f32::NAN),
    ];
    let mut inputs_f64 = vec![
        (0.0, 0.0, 0.0),
        (f64::EPSILON, f64::EPSILON, f64::EPSILON),
        (f64::INFINITY, f64::INFINITY, f64::INFINITY),
        (f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        (f64::MAX, f64::MAX, f64::MAX),
        (f64::MIN, f64::MIN, f64::MIN),
        (f64::MIN_POSITIVE, f64::MIN_POSITIVE, f64::MIN_POSITIVE),
        (f64::NAN, f64::NAN, f64::NAN),
    ];

    inputs_i32.extend((0..(ntests - inputs_i32.len())).map(|_| rng.gen::<(i32, i32, i32)>()));
    inputs_f32.extend((0..(ntests - inputs_f32.len())).map(|_| rng.gen::<(f32, f32, f32)>()));
    inputs_f64.extend((0..(ntests - inputs_f64.len())).map(|_| rng.gen::<(f64, f64, f64)>()));

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
