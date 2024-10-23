//! A generator that makes use of the `Domain` trait
//!
//! Does the following:
//!
//! - Check 100 values near each of the bounds (default)
//! - If there are defined asymptotes, check those
//! - Figure out a number of tests to do within the domain. If exhaustive, check all (but skip
//!   NaNs?)
//! - Check near zero if defined
//! - If unbounded, ensure that real inputs do not produce any NaNs
//! - If periodic, check that results are identical for a few periods (?)

use std::iter;
use std::ops::Bound;

use libm::support::{Float, MinInt};

use crate::domain::Domain;
use crate::{FloatExt, logspace};

/// Number of values near an interesting point to check.
const AROUND: usize = 100;

/// Number of tests to run.
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

/// Some functions have infinite asymptotes, limit how many we check.
const MAX_ASYMPTOTES: usize = 10;

/// Create a test case iterator.
pub fn get_test_cases<F: Float, D: Domain<F>>() -> impl Iterator<Item = (F,)>
where
    F::Int: TryFrom<usize>,
{
    // We generate logspaced inputs within a specific range. Use the function domain
    // by default but if the function is periodic, check only within that period.
    let (start, end) = D::PERIODIC.unwrap_or(D::DEFINED);
    let range_start = match start {
        Bound::Included(v) => v,
        Bound::Excluded(v) => v.next_up(),
        Bound::Unbounded => F::NEG_INFINITY,
    };
    let range_end = match end {
        Bound::Included(v) => v,
        Bound::Excluded(v) => v.next_down(),
        Bound::Unbounded => F::INFINITY,
    };

    let steps = F::Int::try_from(NTESTS).unwrap_or(F::Int::MAX);

    // Always check near bounds in addition to the logspace
    near_bounds::<F, D>().into_iter().chain(logspace(range_start, range_end, steps)).map(|v| (v,))
}

/// Create a vector full of values near interesting (bounds, asymptotes, etc).
fn near_bounds<F: Float, D: Domain<F>>() -> Vec<F> {
    let mut values = Vec::new();

    let lower = D::DEFINED.0;
    let upper = D::DEFINED.1;
    let one = F::ONE;
    let two = one + one;
    let three = two + one;

    if let (Bound::Included(l) | Bound::Excluded(l), Bound::Included(u) | Bound::Excluded(u)) =
        (lower, upper)
    {
        assert!(l < u, "lower bound must be less than upper bound");
    }

    // Check that the bound provided in the trait is correct
    let validate_bound = |b: Bound<F>| match b {
        Bound::Included(v) | Bound::Excluded(v) => {
            assert!(!v.is_nan(), "bounds cannot be NaN");
            assert!(!v.is_infinite(), "use Unbounded rather than infinity");
        }
        Bound::Unbounded => (),
    };

    validate_bound(lower);
    validate_bound(upper);

    /* Test near interesting values */

    // Bounds are interesting
    around_bound(lower, &mut values);
    around_bound(upper, &mut values);

    // One, negative one, and zero are interesting
    for x in [F::ONE, F::NEG_ONE, F::ZERO] {
        around(x, &mut values);
    }

    // Values around min and max are interesting
    values.extend(count_up(F::MIN).take(AROUND));
    values.extend(count_down(F::MAX).take(AROUND));

    // Check some special values that aren't included in the above ranges
    values.push(F::NAN);
    values.push(F::MIN);
    values.push(F::MAX);
    values.push(F::INFINITY);
    values.push(F::NEG_INFINITY);
    values.push(F::NEG_ZERO);

    // Check period endpoints (as we define them) if available
    if let Some((start, end)) = D::PERIODIC {
        validate_bound(start);
        validate_bound(end);

        around_bound(start, &mut values);
        around_bound(end, &mut values);

        let p = D::period().unwrap();

        // Check the same points for a few period to make sure there is no drift
        for mul in [one / two, one, three / two, two, three] {
            let back = D::period_start() - mul * p;
            let forward = D::period_end() + mul * p;

            around(back, &mut values);
            around(forward, &mut values);
        }
    }

    // Check around asymptotest
    for (from, _to) in D::defined_asymptotes().take(MAX_ASYMPTOTES) {
        around(from, &mut values);
    }

    for x in D::check_points().take(MAX_ASYMPTOTES) {
        around(x, &mut values);
    }

    values.sort_by_key(|x| x.to_bits());
    values.dedup_by_key(|x| x.to_bits());

    values
}

/// Push `AROUND` values up and `AROUND` values down from a value exhaustively (increments ULP).
fn around<F: Float>(x: F, values: &mut Vec<F>) {
    values.push(x);
    values.extend(count_up(x).take(AROUND));
    values.extend(count_down(x).take(AROUND));
}

/// Helper to call `around` on set of bounds. Does nothing if the bounds are infinite, since
/// infinities should already be included somewhere else.
fn around_bound<F: Float>(bound: Bound<F>, values: &mut Vec<F>) {
    let (Bound::Included(x) | Bound::Excluded(x)) = bound else {
        return;
    };

    around(x, values);
}

/// Iterator that increments ULP up.
fn count_up<F: FloatExt>(mut x: F) -> impl Iterator<Item = F> {
    assert!(!x.is_nan());
    assert!(!x.is_infinite());

    iter::from_fn(move || {
        (!x.is_infinite()).then(|| {
            x = x.next_up();
            x
        })
    })
}

/// Iterator that increments ULP down.
fn count_down<F: FloatExt>(mut x: F) -> impl Iterator<Item = F> {
    assert!(!x.is_nan());
    assert!(!x.is_infinite());

    iter::from_fn(move || {
        (!x.is_infinite()).then(|| {
            x = x.next_down();
            x
        })
    })
}
