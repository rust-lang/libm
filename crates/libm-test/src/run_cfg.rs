//! Configuration for how tests get run.

use std::env;
use std::sync::LazyLock;

use crate::{BaseName, FloatTy, Identifier, test_log};

/// The environment variable indicating which extensive tests should be run.
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

/// The different kinds of generators that provide test input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeneratorKind {
    Domain,
    Random,
}

/// A list of all functions that should get extensive tests.
///
/// This also supports the special test name `all` to run all tests, as well as `all_f16`,
/// `all_f32`, `all_f64`, and `all_f128` to run all tests for a specific float type.
static EXTENSIVE: LazyLock<Vec<Identifier>> = LazyLock::new(|| {
    let var = env::var(EXTENSIVE_ENV).unwrap_or_default();
    let list = var.split(",").filter(|s| !s.is_empty()).collect::<Vec<_>>();
    let mut ret = Vec::new();

    let append_ty_ops = |ret: &mut Vec<_>, fty: FloatTy| {
        let iter = Identifier::ALL.iter().filter(move |id| id.math_op().float_ty == fty).copied();
        ret.extend(iter);
    };

    for item in list {
        match item {
            "all" => ret = Identifier::ALL.to_owned(),
            "all_f16" => append_ty_ops(&mut ret, FloatTy::F16),
            "all_f32" => append_ty_ops(&mut ret, FloatTy::F32),
            "all_f64" => append_ty_ops(&mut ret, FloatTy::F64),
            "all_f128" => append_ty_ops(&mut ret, FloatTy::F128),
            s => {
                let id = Identifier::from_str(s)
                    .unwrap_or_else(|| panic!("unrecognized test name `{s}`"));
                ret.push(id);
            }
        }
    }

    ret
});

/// Information about the function to be tested.
#[derive(Debug)]
struct TestEnv {
    /// Tests should be reduced because the platform is slow. E.g. 32-bit or emulated.
    slow_platform: bool,
    /// The float cannot be tested exhaustively, `f64` or `f128`.
    large_float_ty: bool,
    /// Env indicates that an extensive test should be run.
    should_run_extensive: bool,
    /// Multiprecision tests will be run.
    mp_tests_enabled: bool,
    /// The number of inputs to the function.
    input_count: usize,
}

impl TestEnv {
    fn from_env(ctx: &CheckCtx) -> Self {
        let id = ctx.fn_ident;
        let op = id.math_op();

        let will_run_mp = cfg!(feature = "test-multiprecision");

        // Tests are pretty slow on non-64-bit targets, x86 MacOS, and targets that run in QEMU. Start
        // with a reduced number on these platforms.
        let slow_on_ci = crate::emulated()
            || usize::BITS < 64
            || cfg!(all(target_arch = "x86_64", target_vendor = "apple"));
        let slow_platform = slow_on_ci && crate::ci();

        let large_float_ty = match op.float_ty {
            FloatTy::F16 | FloatTy::F32 => false,
            FloatTy::F64 | FloatTy::F128 => true,
        };

        let will_run_extensive = EXTENSIVE.contains(&id);

        let input_count = op.rust_sig.args.len();

        Self {
            slow_platform,
            large_float_ty,
            should_run_extensive: will_run_extensive,
            mp_tests_enabled: will_run_mp,
            input_count,
        }
    }
}

/// The number of iterations to run for a given test.
pub fn iteration_count(ctx: &CheckCtx, gen_kind: GeneratorKind, argnum: usize) -> u64 {
    let t_env = TestEnv::from_env(ctx);

    // Ideally run 5M tests
    let mut domain_iter_count: u64 = 4_000_000;

    // Start with a reduced number of tests on slow platforms.
    if t_env.slow_platform {
        domain_iter_count = 100_000;
    }

    // Larger float types get more iterations.
    if t_env.large_float_ty {
        domain_iter_count *= 4;
    }

    // Functions with more arguments get more iterations.
    let arg_multiplier = 1 << (t_env.input_count - 1);
    domain_iter_count *= arg_multiplier;

    // If we will be running tests against MPFR, we don't need to test as much against musl.
    // However, there are some platforms where we have to test against musl since MPFR can't be
    // built.
    if t_env.mp_tests_enabled && ctx.basis == CheckBasis::Musl {
        domain_iter_count /= 100;
    }

    // Run fewer random tests than domain tests.
    let random_iter_count = domain_iter_count / 100;

    let mut total_iterations = match gen_kind {
        GeneratorKind::Domain => domain_iter_count,
        GeneratorKind::Random => random_iter_count,
    };

    if cfg!(optimizations_enabled) {
        // Always run at least 10,000 tests.
        total_iterations = total_iterations.max(10_000);
    } else {
        // Without optimizations, just run a quick check regardless of other parameters.
        total_iterations = 800;
    }

    // Adjust for the number of inputs
    let ntests = match t_env.input_count {
        1 => total_iterations,
        2 => (total_iterations as f64).sqrt().ceil() as u64,
        3 => (total_iterations as f64).cbrt().ceil() as u64,
        _ => panic!("test has more than three arguments"),
    };
    let total = ntests.pow(t_env.input_count.try_into().unwrap());

    test_log(&format!(
        "{gen_kind:?} {basis:?} {fn_ident} arg {arg}/{args}: {ntests} iterations \
         ({total} total)",
        basis = ctx.basis,
        fn_ident = ctx.fn_ident,
        arg = argnum + 1,
        args = t_env.input_count,
    ));

    ntests
}

/// For domain tests, limit how many asymptotes or specified check points we test.
pub fn check_point_count(ctx: &CheckCtx) -> usize {
    let t_env = TestEnv::from_env(ctx);
    if t_env.slow_platform || !cfg!(optimizations_enabled) { 4 } else { 10 }
}

/// When validating points of interest (e.g. asymptotes, inflection points, extremes), also check
/// this many surrounding values.
pub fn check_near_count(_ctx: &CheckCtx) -> u64 {
    if cfg!(optimizations_enabled) { 100 } else { 10 }
}

/// Check whether extensive actions should be run or skipped.
#[expect(dead_code, reason = "extensive tests have not yet been added")]
pub fn skip_extensive_test(ctx: &CheckCtx) -> bool {
    let t_env = TestEnv::from_env(ctx);
    !t_env.should_run_extensive
}
