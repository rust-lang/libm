#!/bin/sh

set -eux

export RUST_BACKTRACE="${RUST_BACKTRACE:-full}"
# Needed for no-panic to correct detect a lack of panics
export RUSTFLAGS="${RUSTFLAGS:-} -Ccodegen-units=1"

target="${1:-}"

if [ -z "$target" ]; then
    host_target=$(rustc -vV | awk '/^host/ { print $2 }')
    echo "Defaulted to host target $host_target"
    target="$host_target"
fi

# We enumerate features manually.
extra_flags="--no-default-features"

# Enable arch-specific routines when available.
extra_flags="$extra_flags --features arch"

# Always enable `unstable-float` since it expands available API but does not
# change any implementations.
extra_flags="$extra_flags --features unstable-float"

# We need to specifically skip tests for musl-math-sys on systems that can't
# build musl since otherwise `--all` will activate it.
case "$target" in
    # Can't build at all on MSVC, WASM, or thumb
    *windows-msvc*) extra_flags="$extra_flags --exclude musl-math-sys" ;;
    *wasm*) extra_flags="$extra_flags --exclude musl-math-sys" ;;
    *thumb*) extra_flags="$extra_flags --exclude musl-math-sys" ;;

    # We can build musl on MinGW but running tests gets a stack overflow
    *windows-gnu*) ;;
    # FIXME(#309): LE PPC crashes calling the musl version of some functions. It
    # seems like a qemu bug but should be investigated further at some point.
    # See <https://github.com/rust-lang/libm/issues/309>.
    *powerpc64le*) ;;

    # Everything else gets musl enabled
    *) extra_flags="$extra_flags --features libm-test/build-musl" ;;
esac

# Configure which targets test against MPFR
case "$target" in
    # MSVC cannot link MPFR
    *windows-msvc*) ;;
    # FIXME: MinGW should be able to build MPFR, but setup in CI is nontrivial.
    *windows-gnu*) ;;
    # Targets that aren't cross compiled work fine
    # FIXME(ci): we should be able to enable aarch64 Linux here once GHA
    # support rolls out.
    x86_64*) extra_flags="$extra_flags --features libm-test/build-mpfr" ;;
    i686*) extra_flags="$extra_flags --features libm-test/build-mpfr" ;;
    i586*) extra_flags="$extra_flags --features libm-test/build-mpfr --features gmp-mpfr-sys/force-cross" ;;
    # Apple aarch64 is native
    aarch64*apple*) extra_flags="$extra_flags --features libm-test/build-mpfr" ;;
esac

# FIXME: `STATUS_DLL_NOT_FOUND` testing macros on CI.
# <https://github.com/rust-lang/rust/issues/128944>
case "$target" in
    *windows-gnu) extra_flags="$extra_flags --exclude libm-macros" ;;
esac

# Make sure we can build with overriding features.
cargo check -p libm --no-default-features

if [ "${BUILD_ONLY:-}" = "1" ]; then
    cmd="cargo build --target $target --package libm"
    $cmd
    $cmd --features unstable-intrinsics

    echo "can't run tests on $target; skipping"
else
    cmd="cargo test --all --target $target $extra_flags"

    # Test once without intrinsics
    $cmd

    # Exclude the macros and utile crates from the rest of the tests to save CI
    # runtime, they shouldn't have anything feature- or opt-level-dependent.
    cmd="$cmd --exclude util --exclude libm-macros"

    # Test once with intrinsics enabled
    $cmd --features unstable-intrinsics
    $cmd --features unstable-intrinsics --benches
    
    # Test the same in release mode, which also increases coverage. Also ensure
    # the soft float routines are checked.
    $cmd --profile release-checked 
    $cmd --profile release-checked --features force-soft-floats
    $cmd --profile release-checked --features unstable-intrinsics
    $cmd --profile release-checked --features unstable-intrinsics --benches

    # Ensure that the routines do not panic.
    ENSURE_NO_PANIC=1 cargo build -p libm --target "$target" --no-default-features --release
fi

