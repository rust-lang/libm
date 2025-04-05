/// The square root of `x` (f64).
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sqrt(x: f64) -> f64 {
    select_implementation! {
        name: sqrt,
        use_arch: any(
            all(target_arch = "wasm32", intrinsics_enabled),
            // Codegen backends (e.g. rustc_codegen_gcc) that implement intrinsics like simd_fsqrt
            // by calling sqrt on every element of the vector ends up with an infinite recursion
            // without the force-soft-floats feature because sqrt would call simd_fsqrt, which in
            // turn calls sqrt on those codegen backends.
            all(target_feature = "sse2", not(feature = "force-soft-floats"))
        ),
        args: x,
    }

    super::generic::sqrt(x)
}
