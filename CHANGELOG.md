# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.12](https://github.com/rust-lang/libm/compare/libm-v0.2.11...libm-v0.2.12) - 2025-01-16

### Other

- Adjust precision and add xfails based on new tests
- Simplify and optimize `fdim` ([#442](https://github.com/rust-lang/libm/pull/442))
- Don't set `codegen-units=1` by default in CI
- Add `fdimf16` and `fdimf128`
- Add a generic version of `fdim`
- Add `truncf16` and `truncf128`
- Add a generic version of `trunc`
- Add a utility crate for quick evaluation
- Enable `build-mpfr` and `build-musl` by default
- Rename the `test-multiprecision` feature to `build-mpfr`
- Introduce arch::aarch64 and use it for rint{,f}
- Use wasm32 arch intrinsics for rint{,f}
- Expose C versions of `libm` functions in the `cb` crate
- Add `biteq` and `exp_unbiased` to `Float`
- Add a `release-checked` profile with debug and overflow assertions
- Remove `ExpInt` from `Float`, always use `i32` instead
- Split `cast` into `cast` and `cast_lossy`
- Use `core::arch::wasm` functions rather than intrinsics
- Account for optimization levels other than numbers
- Replace "intrinsic" config with "arch" config
- Don't use intrinsics abs for `f16` and `f128` on wasm32
- Remove an unused `feature = "force-soft-floats"` gate
- Switch from using `unstable-intrinsics` to `intrinsics_enabled`
- Add test infrastructure for `f16` and `f128`
- Add `fabsf16`, `fabsf128`, `copysignf16`, and `copysignf128`
- Enable `f16` and `f128` when creating the API change list
- Add more detailed definition output for `update-api-list.py`
- Rename `unstable-test-support` to `unstable-public-internals`
- Add a way for tests to log to a file
- Use intrinsics for `abs` and `copysign` when available
- Rename generic `abs` to `fabs`
- Use `rustdoc` output to create a list of public API
- Remove an `is_nan` workaround that is no longer needed
- Update and slightly refactor some of the `Float` trait
- Add `f16` and `f128` configuration from `compiler-builtins`
- Introduce generic `abs` and `copysign`
- Fix new `clippy::precedence` lints
- Introduce helper types for accessing trait items
- Fix a bug in `abs_diff`
- Remove tests against system musl
- Use `https:` links in `README.md`
- Move some numeric trait logic to default implementations
- Resolve clippy errors in `libm` tests and check this in CI
- Add some more basic docstrings ([#352](https://github.com/rust-lang/libm/pull/352))
- Introduce `hf32!` and `hf64!` macros for hex float support
- Fix errors reported by Clippy in `libm`
- Expose the `support` module publicly with a test feature
- Update libm `Float` and `Int` with functions from the test traits
- Change prefixes used by the `Float` trait
- Remove `libm-bench`
- Rename `canonical_name` to `base_name`
- Add float and integer traits from compiler-builtins
- Move architecture-specific code to `src/math/arch`
- Update `select_implementation` to accept arch configuration
- Add an "arch" Cargo feature that is on by default
- Vendor `cfg_if::cfg_if!`
- Make use of `select_implementation`
- Introduce a `select_implementation` macro
- Introduce `math::arch::intrinsics`
- Replace `feature = "unstable-intrinsics"` with `intrinsics_enabled`
- Move the existing "unstable" feature to "unstable-intrinsics"

## [0.2.11](https://github.com/rust-lang/libm/compare/libm-v0.2.10...libm-v0.2.11) - 2024-10-28

### Fixed

- fix type of constants in ported sincosf ([#331](https://github.com/rust-lang/libm/pull/331))

### Other

- Disable a unit test that is failing on i586
- Add a procedural macro for expanding all function signatures
- Introduce `musl-math-sys` for bindings to musl math symbols
- Add basic docstrings to some functions ([#337](https://github.com/rust-lang/libm/pull/337))

## [0.2.10](https://github.com/rust-lang/libm/compare/libm-v0.2.9...libm-v0.2.10) - 2024-10-28

### Other

- Set the MSRV to 1.63 and test this in CI

## [0.2.9](https://github.com/rust-lang/libm/compare/libm-v0.2.8...libm-v0.2.9) - 2024-10-26

### Fixed

- Update exponent calculations in nextafter to match musl

### Changed

- Update licensing to MIT AND (MIT OR Apache-2.0), as this is derivative from
  MIT-licensed musl.
- Set edition to 2021 for all crates
- Upgrade all dependencies

### Other

- Don't deny warnings in lib.rs
- Rename the `musl-bitwise-tests` feature to `test-musl-serialized`
- Rename the `musl-reference-tests` feature to `musl-bitwise-tests`
- Move `musl-reference-tests` to a new `libm-test` crate
- Add a `force-soft-floats` feature to prevent using any intrinsics or
  arch-specific code
- Deny warnings in CI
- Fix `clippy::deprecated_cfg_attr` on compiler_builtins
- Corrected English typos
- Remove unneeded `extern core` in `tgamma`
- Allow internal_features lint when building with "unstable"

## [v0.2.1] - 2019-11-22

### Fixed

- sincosf

## [v0.2.0] - 2019-10-18

### Added

- Benchmarks
- signum
- remainder
- remainderf
- nextafter
- nextafterf

### Fixed

- Rounding to negative zero
- Overflows in rem_pio2 and remquo
- Overflows in fma
- sincosf

### Removed

- F32Ext and F64Ext traits

## [v0.1.4] - 2019-06-12

### Fixed

- Restored compatibility with Rust 1.31.0

## [v0.1.3] - 2019-05-14

### Added

- minf
- fmin
- fmaxf
- fmax

## [v0.1.2] - 2018-07-18

### Added

- acosf
- asin
- asinf
- atan
- atan2
- atan2f
- atanf
- cos
- cosf
- cosh
- coshf
- exp2
- expm1
- expm1f
- expo2
- fmaf
- pow
- sin
- sinf
- sinh
- sinhf
- tan
- tanf
- tanh
- tanhf

## [v0.1.1] - 2018-07-14

### Added

- acos
- acosf
- asin
- asinf
- atanf
- cbrt
- cbrtf
- ceil
- ceilf
- cosf
- exp
- exp2
- exp2f
- expm1
- expm1f
- fdim
- fdimf
- floorf
- fma
- fmod
- log
- log2
- log10
- log10f
- log1p
- log1pf
- log2f
- roundf
- sinf
- tanf

## v0.1.0 - 2018-07-13

- Initial release

[Unreleased]: https://github.com/japaric/libm/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/japaric/libm/compare/0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/libm/compare/0.1.4...v0.2.0
[v0.1.4]: https://github.com/japaric/libm/compare/0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/libm/compare/v0.1.2...0.1.3
[v0.1.2]: https://github.com/japaric/libm/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/libm/compare/v0.1.0...v0.1.1
