use criterion::{criterion_main, Criterion};
use libm_test::gen::random;
use libm_test::TupleCall;
use std::hint::black_box;
use std::time::Duration;

/// Benchmark with this many items to get a variety
const BENCH_ITER_ITEMS: usize = if cfg!(feature = "short-benchmarks") {
    50
} else {
    500
};

macro_rules! musl_rand_benches {
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: $RustFn:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $skip_on_i586:expr,
    ) => {
        paste::paste! {
            fn [< musl_bench_ $fn_name >](c: &mut Criterion) {
                let fname = stringify!($fn_name);
                let benchvec: Vec<_> = random::get_test_cases::<$RustArgs>(fname)
                    .take(BENCH_ITER_ITEMS)
                    .collect();

                // Perform a sanity check that we are benchmarking the same thing
                // Don't test against musl if it is not available
                #[cfg(feature = "build-musl")]
                for input in benchvec.iter().copied() {
                    use anyhow::Context;
                    use libm_test::{CheckBasis, CheckCtx, CheckOutput};

                    const QUICKCHECK_ULP: u32 = 10;

                    if cfg!(x86_no_sse) && $skip_on_i586 {
                        break;
                    }

                    let ctx = CheckCtx {
                        ulp: QUICKCHECK_ULP,
                        fname,
                        basis: CheckBasis::Musl
                    };

                    let musl_res = input.call(musl_math_sys::$fn_name as $CFn);
                    let crate_res = input.call(libm::$fn_name as $RustFn);

                    musl_res.validate(crate_res, input, &ctx).context(fname).unwrap();
                }

                let mut group = c.benchmark_group(fname);
                group.bench_function("crate", |b| b.iter(|| {
                    for input in benchvec.iter().copied() {
                        black_box(black_box(input).call(libm::$fn_name as $RustFn));
                    }
                }));

                // Don't test against musl if it is not available
                #[cfg(feature = "build-musl")]
                group.bench_function("musl", |b| b.iter(|| {
                    for input in benchvec.iter().copied() {
                        black_box(black_box(input).call(musl_math_sys::$fn_name as $CFn));
                    }
                }));
            }
        }
    };
}

libm_macros::for_each_function! {
    callback: musl_rand_benches,
    skip: [],
    fn_extra: match MACRO_FN_NAME {
        // FIXME(correctness): wrong result on i586
        exp10 | exp10f | exp2 | exp2f => true,
        _ => false
    }
}

macro_rules! run_callback {
    (
        fn_name: $fn_name:ident,
        CFn: $_CFn:ty,
        CArgs: $_CArgs:ty,
        CRet: $_CRet:ty,
        RustFn: $_RustFn:ty,
        RustArgs: $_RustArgs:ty,
        RustRet: $_RustRet:ty,
        extra: [$criterion:ident],
    ) => {
        paste::paste! {
            [< musl_bench_ $fn_name >](&mut $criterion)
        }
    };
}

pub fn musl_random() {
    let mut criterion = Criterion::default();

    // For CI, run a short 0.5s warmup and 1.0s tests. This makes benchmarks complete in
    // about the same time as other tests.
    if cfg!(feature = "short-benchmarks") {
        criterion = criterion
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_millis(1000));
    }

    criterion = criterion.configure_from_args();

    libm_macros::for_each_function! {
        callback: run_callback,
        extra: [criterion],
    };
}

criterion_main!(musl_random);
