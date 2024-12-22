//! Configuration of

#![allow(unused)]

use std::collections::BTreeMap;
use std::env;
use std::sync::LazyLock;

use crate::{FloatTy, op};

pub const EXTENSIVE_ENV: &str = "LIBM_EXTENSIVE_TESTS";

#[derive(Debug)]
pub enum TestTy {
    Extensive,
    Logspace,
    Random,
    EdgeCases,
}

#[derive(Debug)]
pub enum TestAction {
    Run,
    Iterations(u64),
    Skip,
}

/// A list of all functions that should get extensive tests
static EXTENSIVE: LazyLock<Vec<op::Identifier>> = LazyLock::new(|| {
    let var = env::var(EXTENSIVE_ENV).unwrap_or_default();
    let list = var.split(",").filter(|s| !s.is_empty()).collect::<Vec<_>>();
    let mut ret = Vec::new();

    for item in list {
        match item {
            "all" => ret = op::Identifier::ALL.to_owned(),
            "all_f32" => ret.extend(
                op::Identifier::ALL
                    .iter()
                    .filter(|id| matches!(id.math_op().float_ty, FloatTy::F32))
                    .copied(),
            ),
            "all_f64" => ret.extend(
                op::Identifier::ALL
                    .iter()
                    .filter(|id| matches!(id.math_op().float_ty, FloatTy::F64))
                    .copied(),
            ),
            s => ret.push(
                op::Identifier::from_str(s)
                    .unwrap_or_else(|| panic!("unrecognized test name `{s}`")),
            ),
        }
    }

    ret
});

pub fn get_iterations(id: op::Identifier, test_ty: TestTy, argnum: usize) -> TestAction {
    // Run more musl tests if we don't have mp
    let run_mp = cfg!(feature = "test-multiprecision");
    let run_musl = cfg!(feature = "build-musl");
    let run_extensive = EXTENSIVE.contains(&id);
    // Tests are pretty slow on non-64-bit targets, x86 MacOS, and targets that run in QEMU.
    let ci_slow_platform = crate::emulated()
        || !cfg!(target_pointer_width = "64")
        || cfg!(all(target_arch = "x86_64", target_vendor = "apple"));
    let slow_platform = ci_slow_platform && crate::ci();

    // Extensive tests handle their own iterations
    if matches!(test_ty, TestTy::Extensive) {
        return if run_extensive { TestAction::Run } else { TestAction::Skip };
    }

    // Ideally run 5M tests
    let mut baseline = 5_000_000;

    if slow_platform {
        baseline = 100_000;
    }

    let op = id.math_op();

    let mut rand_tests = match op.float_ty {
        FloatTy::F16 | FloatTy::F32 => baseline,
        FloatTy::F64 | FloatTy::F128 => baseline * 4,
    };

    rand_tests *= match op.rust_sig.args.len() {
        1 => 1,
        2 => 2,
        3 => 4,
        _ => unimplemented!(),
    };

    // Idea: let main = `TestTy::...`, `main_test_iterations = big number`, `other_iterations = big / 100

    // TODO
    let has_logspace_test = true;
    let mut logspace_tests = None;
    if has_logspace_test {
        // Still run some random tests for NaN and similar, but most of the tests should be
        // logspace.
        logspace_tests = Some(rand_tests);
        rand_tests /= 100;
    }

    if run_extensive {
        // Keep some checks
        match logspace_tests.as_mut() {
            Some(v) => *v /= 100,
            None => rand_tests /= 100,
        }
    }

    // Without optimizations just run a quick check
    if !cfg!(optimizations_enabled) {
        baseline = 800;
    }

    let ntests = match test_ty {
        TestTy::Extensive => unreachable!(),
        TestTy::Logspace => logspace_tests.unwrap(),
        TestTy::Random => rand_tests,
        TestTy::EdgeCases => todo!(),
    };

    eprintln!("running {ntests:?} tests for {test_ty:?} `{id}`");
    TestAction::Iterations(ntests)
}
