#![allow(unused)]

use std::iter;

use libm::support::Float;

use crate::domain::HasDomain;
use crate::num::ulp_between;
use crate::{CheckCtx, FloatExt, MathOp, logspace};

const MAX_ITERATIONS: u64 = u32::MAX as u64;

pub fn get_test_cases<Op>(ctx: &CheckCtx) -> impl Iterator<Item = Op::RustArgs> + Send
where
    Op: MathOp,
    Op::RustArgs: ExtensiveInput<Op>,
{
    Op::RustArgs::gen()
}

pub trait ExtensiveInput<Op> {
    fn gen() -> impl Iterator<Item = Self> + Send;
    fn count() -> u64 {
        MAX_ITERATIONS
    }
}

impl<Op> ExtensiveInput<Op> for (f32,)
where
    Op: MathOp<RustArgs = Self, FTy = f32>,
    Op: HasDomain<Op::FTy>,
{
    fn gen() -> impl Iterator<Item = Self> {
        let mut start = Op::D.range_start();
        let end = Op::D.range_end();
        iter::from_fn(move || {
            if start > end || start >= Op::FTy::INFINITY {
                return None;
            }
            let ret = start;
            start = FloatExt::next_up(start);
            Some((ret,))
        })
    }

    fn count() -> u64 {
        u64::from(ulp_between(Op::D.range_start(), Op::D.range_end()).unwrap()) + 1
    }
}

impl<Op> ExtensiveInput<Op> for (f64,)
where
    Op: MathOp<RustArgs = Self, FTy = f64>,
    Op: HasDomain<Op::FTy>,
{
    fn gen() -> impl Iterator<Item = Self> {
        let start = Op::D.range_start();
        let end = Op::D.range_end();
        let steps = <Op::FTy as Float>::Int::try_from(MAX_ITERATIONS)
            .unwrap_or(<Op::FTy as Float>::Int::MAX);
        logspace(start, end, steps).map(|v| (v,))
    }
}

impl<Op> ExtensiveInput<Op> for (f32, f32)
where
    Op: MathOp<RustArgs = Self, FTy = f32>,
{
    fn gen() -> impl Iterator<Item = Self> {
        let start = f32::NEG_INFINITY;
        let end = f32::INFINITY;
        let per_arg = MAX_ITERATIONS.isqrt().try_into().unwrap();
        logspace(start, end, per_arg)
            .flat_map(move |first| logspace(start, end, per_arg).map(move |second| (first, second)))
    }
}

impl<Op> ExtensiveInput<Op> for (f64, f64)
where
    Op: MathOp<RustArgs = Self, FTy = f64>,
{
    fn gen() -> impl Iterator<Item = Self> {
        let start = f64::NEG_INFINITY;
        let end = f64::INFINITY;
        let per_arg = MAX_ITERATIONS.isqrt();
        logspace(start, end, per_arg)
            .flat_map(move |first| logspace(start, end, per_arg).map(move |second| (first, second)))
    }
}

impl<Op> ExtensiveInput<Op> for (f32, f32, f32)
where
    Op: MathOp<RustArgs = Self, FTy = f32>,
{
    fn gen() -> impl Iterator<Item = Self> {
        let start = f32::NEG_INFINITY;
        let end = f32::INFINITY;
        let per_arg = (MAX_ITERATIONS as f32).cbrt() as u32;
        logspace(start, end, per_arg)
            .flat_map(move |first| logspace(start, end, per_arg).map(move |second| (first, second)))
            .flat_map(move |(first, second)| {
                logspace(start, end, per_arg).map(move |third| (first, second, third))
            })
    }
}

impl<Op> ExtensiveInput<Op> for (f64, f64, f64)
where
    Op: MathOp<RustArgs = Self, FTy = f64>,
{
    fn gen() -> impl Iterator<Item = Self> {
        let start = f64::NEG_INFINITY;
        let end = f64::INFINITY;
        let per_arg = (MAX_ITERATIONS as f32).cbrt() as u64;
        logspace(start, end, per_arg)
            .flat_map(move |first| logspace(start, end, per_arg).map(move |second| (first, second)))
            .flat_map(move |(first, second)| {
                logspace(start, end, per_arg).map(move |third| (first, second, third))
            })
    }
}
