//! A generator that makes use of domain bounds.

use std::ops::Bound;

use libm::support::{Float, MinInt};

use crate::domain::{Domain, HasDomain};
use crate::{MathOp, logspace};

/// Number of tests to run.
// FIXME(ntests): replace this with a more logical algorithm
const NTESTS: usize = {
    if cfg!(optimizations_enabled) {
        if crate::emulated()
            || !cfg!(target_pointer_width = "64")
            || cfg!(all(target_arch = "x86_64", target_vendor = "apple"))
        {
            // Tests are pretty slow on non-64-bit targets, x86 MacOS, and targets that run
            // in QEMU.
            100_000
        } else {
            5_000_000
        }
    } else {
        // Without optimizations just run a quick check
        800
    }
};

/// Create a range of logarithmically spaced inputs within a function's domain.
///
/// This allows us to get reasonably thorough coverage while avoiding values that are NaN or out
/// of range. Random tests will still cover these values that are not included here.
pub fn get_test_cases<Op>() -> impl Iterator<Item = (Op::FTy,)>
where
    Op: MathOp + HasDomain<Op::FTy>,
    <Op::FTy as Float>::Int: TryFrom<usize>,
{
    get_test_cases_inner::<Op::FTy>(Op::D)
}

pub fn get_test_cases_inner<F>(domain: Domain<F>) -> impl Iterator<Item = (F,)>
where
    F: Float<Int: TryFrom<usize>>,
{
    // We generate logspaced inputs within a specific range, excluding values that are out of
    // range in order to make iterations useful (random tests still cover the full range).
    let range_start = match domain.start {
        Bound::Included(v) | Bound::Excluded(v) => v,
        Bound::Unbounded => F::NEG_INFINITY,
    };
    let range_end = match domain.end {
        Bound::Included(v) | Bound::Excluded(v) => v,
        Bound::Unbounded => F::INFINITY,
    };

    let steps = F::Int::try_from(NTESTS).unwrap_or(F::Int::MAX);
    logspace(range_start, range_end, steps).map(|v| (v,))
}
