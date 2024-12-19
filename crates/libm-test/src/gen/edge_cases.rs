//! A generator that checks a handful of cases near infinities, zeros, asymptotes, and NaNs.

use libm::support::Float;

use crate::domain::{Domain, HasDomain};
use crate::{FloatExt, MathOp};

/// Number of values near an interesting point to check.
// FIXME(ntests): replace this with a more logical algorithm
const AROUND: usize = 100;

/// Some functions have infinite asymptotes, limit how many we check.
const MAX_CHECK_POINTS: usize = 10;

/// Create a test case iterator.
pub fn get_test_cases<Op>() -> impl Iterator<Item = (Op::FTy,)>
where
    Op: MathOp + HasDomain<Op::FTy>,
{
    // Create a vector full of values near interesting (bounds, asymptotes, etc).
    let mut values = Vec::new();
    populate_values::<Op::FTy>(&mut values, Op::D);
    values.sort_by_key(|x| x.to_bits());
    values.dedup_by_key(|x| x.to_bits());
    values.into_iter().map(|v| (v,))
}

fn populate_values<F: Float>(values: &mut Vec<F>, domain: Domain<F>) {
    values.push(F::MIN);
    values.push(F::MAX);

    // Values around min and max are interesting
    count_up(F::MIN, values);
    count_down(F::MAX, values);
    values.push(F::INFINITY);
    values.push(F::NEG_INFINITY);
    values.push(F::NEG_ZERO);

    // Check some special values that aren't included in the above ranges
    values.push(F::NAN);
    values.extend(F::consts().iter());

    count_up(F::ONE, values);
    count_up(F::ZERO, values);
    count_up(F::NEG_ONE, values);
    count_down(F::ONE, values);
    count_down(F::ZERO, values);
    count_down(F::NEG_ONE, values);

    // Check around asymptotest
    if let Some(f) = domain.check_points {
        let iter = f();
        for x in iter.take(MAX_CHECK_POINTS) {
            count_up(x, values);
            count_down(x, values);
        }
    }
}

/// Iterator that increments ULP up.
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

/// Iterator that increments ULP down.
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
