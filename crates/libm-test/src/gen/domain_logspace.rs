//! A generator that produces logarithmically spaced values within domain bounds.

use libm::support::{IntTy, MinInt};

use crate::domain::HasDomain;
use crate::op::OpITy;
use crate::run_cfg::{TestAction, TestTy};
use crate::{MathOp, logspace};

/// Create a range of logarithmically spaced inputs within a function's domain.
///
/// This allows us to get reasonably thorough coverage without wasting time on values that are
/// NaN or out of range. Random tests will still cover values that are excluded here.
pub fn get_test_cases<Op>() -> impl Iterator<Item = (Op::FTy,)>
where
    Op: MathOp + HasDomain<Op::FTy>,
    IntTy<Op::FTy>: TryFrom<u64>,
{
    let domain = Op::DOMAIN;
    let action = crate::run_cfg::get_iterations(Op::IDENTIFIER, TestTy::Logspace, 0);
    let ntests = match action {
        TestAction::Iterations(n) => n,
        TestAction::Run => unimplemented!(),
        TestAction::Skip => unimplemented!(),
    };

    // We generate logspaced inputs within a specific range, excluding values that are out of
    // range in order to make iterations useful (random tests still cover the full range).
    let start = domain.range_start();
    let end = domain.range_end();
    let steps = OpITy::<Op>::try_from(ntests).unwrap_or(OpITy::<Op>::MAX);
    logspace(start, end, steps).map(|v| (v,))
}
