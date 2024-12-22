//! Exhaustive tests for `f16` and `f32`, high-iteration for `f64` and `f128`.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use libm_test::gen::extensive::{self, ExtensiveInput};
use libm_test::mpfloat::MpOp;
use libm_test::{
    CheckBasis, CheckCtx, CheckOutput, EXTENSIVE_ENV, GeneratorKind, MathOp, TestAction,
    TestResult, TupleCall, get_iterations,
};
use libtest_mimic::{Arguments, Completion, Trial};
use rayon::prelude::*;

const PB_TEMPLATE: &str = "[{elapsed:3} {percent:3}%] {bar:20.cyan/blue} NAME \
    {human_pos:>13}/{human_len:13} {per_sec:18} eta {eta:8} {msg}";
const PB_TEMPLATE_FINAL: &str = "[{elapsed:3} {percent:3}%] {bar:20.cyan/blue} NAME \
    {human_pos:>13}/{human_len:13} {per_sec:18} done in {elapsed_precise}";

pub fn run() {
    let mut args = Arguments::from_args();
    // We parallelize internally
    args.test_threads = Some(1);
    let tests = register_tests();

    // With default parallelism, the CPU doesn't saturate. We don't need to be nice to
    // other processes, so do 1.5x to make sure we use all available resources.
    let threads = std::thread::available_parallelism().map(Into::into).unwrap_or(0) * 3 / 2;
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();

    libtest_mimic::run(&args, tests).exit();
}

macro_rules! mp_extensive_tests {
    (
        fn_name: $fn_name:ident,
        extra: [$push_to:ident],
    ) => {
        register_one::<libm_test::op::$fn_name::Routine>(&mut $push_to, stringify!($fn_name));
    };
}

fn register_one<Op>(all: &mut Vec<Trial>, name: &'static str)
where
    Op: MathOp + MpOp,
    Op::RustArgs: ExtensiveInput<Op> + Send,
{
    let test_name = format!("mp_extensive_{name}");
    all.push(Trial::skippable_test(test_name, move || {
        let ctx = CheckCtx::new(Op::IDENTIFIER, CheckBasis::Mpfr);
        let action = get_iterations(&ctx, GeneratorKind::Extensive, 0);
        match action {
            TestAction::Run => (),
            TestAction::Iterations(_) => panic!("extensive tests disregard iteration counts"),
            TestAction::Skip => {
                return Ok(Completion::Ignored {
                    reason: format!("extensive tests are only run if specified in {EXTENSIVE_ENV}"),
                });
            }
        };

        if !cfg!(optimizations_enabled) {
            panic!("exhaustive tests should be run with --release");
        }

        test_one::<Op>(name).map(|()| Completion::Completed).map_err(Into::into)
    }));
}

fn test_one<Op>(name: &str) -> TestResult
where
    Op: MathOp + MpOp,
    Op::RustArgs: ExtensiveInput<Op> + Send,
{
    static COMPLETED: AtomicU64 = AtomicU64::new(0);
    let ctx = CheckCtx::new(Op::IDENTIFIER, CheckBasis::Mpfr);

    let expected_checks = Op::RustArgs::count();
    let wait = Duration::from_millis(500);
    let name_padded = format!("{name:9}");
    let pb_style = ProgressStyle::with_template(&PB_TEMPLATE.replace("NAME", &name_padded))
        .unwrap()
        .progress_chars("##-");

    // Just delay a bit before printing anything so other output (ignored tests) has a chance
    // to flush.
    std::thread::sleep(wait);

    eprintln!("starting extensive tests for `{name}`");
    let pb = ProgressBar::new(expected_checks);
    COMPLETED.store(0, Ordering::Relaxed);
    pb.set_style(pb_style);

    let run_single_input = |mp_vals: &mut Op::MpTy, input: Op::RustArgs| -> TestResult {
        let mp_res = Op::run(mp_vals, input);
        let crate_res = input.call(Op::ROUTINE);

        crate_res.validate(mp_res, input, &ctx)?;
        let completed = COMPLETED.fetch_add(1, Ordering::Relaxed) + 1;
        if completed % 20_000 == 0 {
            pb.set_position(completed);
        }
        if completed % 500_000 == 0 {
            pb.set_message(format!("input: {input:?}"));
        }
        Ok(())
    };

    let cases = &mut extensive::get_test_cases::<Op>(&ctx);

    // Chunk the cases so Rayon doesn't switch threads between each iterator item. 50k seems to be
    // a performance sweet spot.
    let chunk_size = 50_000;
    let chunks = std::iter::from_fn(move || {
        let v = Vec::from_iter(cases.take(chunk_size));
        (!v.is_empty()).then_some(v)
    });

    let res = chunks.par_bridge().try_for_each_init(
        || Op::new_mp(),
        |mp_vals, input_vec| -> TestResult {
            for x in input_vec {
                run_single_input(mp_vals, x)?;
            }
            Ok(())
        },
    );

    // let res = cases.par_bridge().try_for_each_init(|| Op::new_mp(), run_single_input);
    let total_run = COMPLETED.load(Ordering::Relaxed);

    let pb_style = ProgressStyle::with_template(&PB_TEMPLATE_FINAL.replace("NAME", &name_padded))
        .unwrap()
        .progress_chars("##-");
    pb.set_style(pb_style);
    pb.set_position(total_run);
    pb.abandon();

    if total_run != expected_checks {
        eprintln!("ERROR: total run {total_run} does not match expected {expected_checks}");
    }

    eprintln!();
    res
}

fn register_tests() -> Vec<Trial> {
    let mut all_tests = Vec::new();
    libm_macros::for_each_function! {
        callback: mp_extensive_tests,
        extra: [all_tests],
        skip: [
            // TODO
            jn,
            jnf,

            // FIXME: MPFR tests needed
            frexp,
            frexpf,
            ilogb,
            ilogbf,
            ldexp,
            ldexpf,
            modf,
            modff,
            remquo,
            remquof,
            scalbn,
            scalbnf,

            // FIXME: test needed, see
            // https://github.com/rust-lang/libm/pull/311#discussion_r1818273392
            nextafter,
            nextafterf,
        ],
    }
    all_tests
}
