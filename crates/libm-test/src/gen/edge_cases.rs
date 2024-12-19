//! A generator that checks a handful of cases near infinities, zeros, asymptotes, and NaNs.

use libm::support::Float;

use crate::domain::{Domain, HasDomain};
use crate::{FloatExt, MathOp};

/// Number of values near an interesting point to check.
// FIXME(ntests): replace this with a more logical algorithm
const AROUND: usize = 100;

/// Functions have infinite asymptotes, limit how many we check.
const MAX_CHECK_POINTS: usize = 10;

/// Create a list of values around interesting points (infinities, zeroes, NaNs).
pub fn get_test_cases<Op>() -> impl Iterator<Item = (Op::FTy,)>
where
    Op: MathOp + HasDomain<Op::FTy>,
{
    let mut values = Vec::new();
    populate_values::<Op::FTy>(&mut values, Op::D);
    values.sort_by_key(|x| x.to_bits());
    values.dedup_by_key(|x| x.to_bits());
    values.into_iter().map(|v| (v,))
}

fn populate_values<F: Float>(values: &mut Vec<F>, domain: Domain<F>) {
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

    // Check around asymptotest
    if let Some(f) = domain.check_points {
        let iter = f();
        for x in iter.take(MAX_CHECK_POINTS) {
            count_up(x, values);
            count_down(x, values);
        }
    }
}

/// Add `AROUND` values starting at `x` and counting up, using the smallest possible increments
/// (1 ULP).
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

/// Add `AROUND` values starting at `x` and counting down, using the smallest possible increments
/// (1 ULP).
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
