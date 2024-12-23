//! Configuration of

#![allow(unused)]

use std::collections::BTreeMap;
use std::env;
use std::sync::LazyLock;

use crate::{BaseName, FloatTy, Identifier, op};

pub const EXTENSIVE_ENV: &str = "LIBM_EXTENSIVE_TESTS";

/// Context passed to [`CheckOutput`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckCtx {
    /// Allowed ULP deviation
    pub ulp: u32,
    pub fn_ident: Identifier,
    pub base_name: BaseName,
    /// Function name.
    pub fn_name: &'static str,
    /// Return the unsuffixed version of the function name.
    pub base_name_str: &'static str,
    /// Source of truth for tests.
    pub basis: CheckBasis,
}

impl CheckCtx {
    /// Create a new check context, using the default ULP for the function.
    pub fn new(fn_ident: Identifier, basis: CheckBasis) -> Self {
        let mut ret = Self {
            ulp: 0,
            fn_ident,
            fn_name: fn_ident.as_str(),
            base_name: fn_ident.base_name(),
            base_name_str: fn_ident.base_name().as_str(),
            basis,
        };
        ret.ulp = crate::default_ulp(&ret);
        ret
    }
}

/// Possible items to test against
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckBasis {
    /// Check against Musl's math sources.
    Musl,
    /// Check against infinite precision (MPFR).
    Mpfr,
}

/// The different kinds of tests that we run
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeneratorKind {
    Extensive,
    Logspace,
    Random,
    EdgeCases,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TestAction {
    Run,
    Iterations(u64),
    Skip,
}

/// A list of all functions that should get extensive tests
static EXTENSIVE: LazyLock<Vec<Identifier>> = LazyLock::new(|| {
    let var = env::var(EXTENSIVE_ENV).unwrap_or_default();
    let list = var.split(",").filter(|s| !s.is_empty()).collect::<Vec<_>>();
    let mut ret = Vec::new();

    for item in list {
        match item {
            "all" => ret = Identifier::ALL.to_owned(),
            "all_f32" => ret.extend(
                Identifier::ALL
                    .iter()
                    .filter(|id| matches!(id.math_op().float_ty, FloatTy::F32))
                    .copied(),
            ),
            "all_f64" => ret.extend(
                Identifier::ALL
                    .iter()
                    .filter(|id| matches!(id.math_op().float_ty, FloatTy::F64))
                    .copied(),
            ),
            s => ret.push(
                Identifier::from_str(s).unwrap_or_else(|| panic!("unrecognized test name `{s}`")),
            ),
        }
    }

    ret
});

pub fn get_iterations(ctx: &CheckCtx, test_ty: GeneratorKind, argnum: usize) -> TestAction {
    // TODO: use argnum to figure out that the second arg of `jn` should be reduced

    let id = ctx.fn_ident;
    // Run more musl tests if we don't have mp
    let run_mp = cfg!(feature = "test-multiprecision");
    let run_musl = cfg!(feature = "build-musl");
    let run_extensive = EXTENSIVE.contains(&id);

    // Extensive tests handle their own iterations
    if matches!(test_ty, GeneratorKind::Extensive) {
        return if run_extensive { TestAction::Run } else { TestAction::Skip };
    }

    // Ideally run 5M tests
    let mut baseline = 5_000_000;

    // Tests are pretty slow on non-64-bit targets, x86 MacOS, and targets that run in QEMU.
    let slow_on_ci = crate::emulated()
        || !cfg!(target_pointer_width = "64")
        || cfg!(all(target_arch = "x86_64", target_vendor = "apple"));
    let slow_platform = slow_on_ci && crate::ci();
    if slow_platform {
        baseline = 100_000;
    }

    let op = id.math_op();

    let mut rand_tests = match op.float_ty {
        FloatTy::F16 | FloatTy::F32 => baseline,
        FloatTy::F64 | FloatTy::F128 => baseline * 4,
    };

    // Provide more space for functions with multiple arguments
    let arg_multiplier = 1 << (op.rust_sig.args.len() - 1);
    rand_tests *= arg_multiplier;

    // Idea: let main = `TestTy::...`, `main_test_iterations = big number`, `other_iterations = big / 100

    // let primary_test = TestTy::

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
        GeneratorKind::Extensive => unreachable!(),
        GeneratorKind::Logspace => logspace_tests.unwrap(),
        GeneratorKind::Random => rand_tests,
        GeneratorKind::EdgeCases => todo!(),
    };

    eprintln!("running {ntests:?} tests for {test_ty:?} `{id}`");
    TestAction::Iterations(ntests)
}
