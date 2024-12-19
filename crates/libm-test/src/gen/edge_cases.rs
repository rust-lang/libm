//! A generator that checks a handful of cases near infinities, zeros, asymptotes, and NaNs.

use libm::support::Float;

use crate::domain::HasDomain;
use crate::{FloatExt, MathOp};

/// Number of values near an interesting point to check.
// FIXME(ntests): replace this with a more logical algorithm
const AROUND: usize = 100;

/// Functions have infinite asymptotes, limit how many we check.
// FIXME(ntests): replace this with a more logical algorithm
const MAX_CHECK_POINTS: usize = 10;

/// Create a list of values around interesting points (infinities, zeroes, NaNs).
pub fn get_test_cases<Op, F>() -> impl Iterator<Item = (F,)>
where
    Op: MathOp<FTy = F> + HasDomain<F>,
    F: Float,
{
    let mut ret = Vec::new();
    let values = &mut ret;
    let domain = Op::DOMAIN;

    // Check near some notable constants
    count_up(F::ONE, values);
    count_up(F::ZERO, values);
    count_up(F::NEG_ONE, values);
    count_down(F::ONE, values);
    count_down(F::ZERO, values);
    count_down(F::NEG_ONE, values);
    values.push(F::NEG_ZERO);

    // Check values near the extremes
    values.push(F::INFINITY);
    values.push(F::NEG_INFINITY);
    values.push(F::MIN);
    values.push(F::MAX);
    count_up(F::MIN, values);
    count_down(F::MAX, values);

    // Check some special values that aren't included in the above ranges
    values.push(F::NAN);
    values.extend(F::consts().iter());

    // Check around asymptotes
    if let Some(f) = domain.check_points {
        let iter = f();
        for x in iter.take(MAX_CHECK_POINTS) {
            count_up(x, values);
            count_down(x, values);
        }
    }

    // Some results may overlap so deduplicate the vector to save test cycles.
    values.dedup_by_key(|x| x.to_bits());

    ret.into_iter().map(|v| (v,))
}

/// Add `AROUND` values starting at and including `x` and counting up. Uses the smallest possible
/// increments (1 ULP).
fn count_up<F: Float>(mut x: F, values: &mut Vec<F>) {
    assert!(!x.is_nan());
    assert!(!x.is_infinite());

    let mut count = 0;
    while !x.is_infinite() && count < AROUND {
        values.push(x);
        x = x.next_up();
        count += 1;
    }
}

/// Add `AROUND` values starting at and including `x` and counting down. Uses the smallest possible
/// increments (1 ULP).
fn count_down<F: Float>(mut x: F, values: &mut Vec<F>) {
    assert!(!x.is_nan());
    assert!(!x.is_infinite());

    let mut count = 0;
    while !x.is_infinite() && count < AROUND {
        values.push(x);
        x = x.next_down();
        count += 1;
    }
}
