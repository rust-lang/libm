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

extra_flags=""

# We need to specifically skip tests for musl-math-sys on systems that can't
# build musl since otherwise `--all` will activate it.
case "$target" in
    *windows-msvc*) extra_flags="$extra_flags --exclude musl-math-sys" ;;
    *wasm*) extra_flags="$extra_flags --exclude musl-math-sys" ;;
    *thumb*) extra_flags="$extra_flags --exclude musl-math-sys" ;;
esac

# FIXME: `STATUS_DLL_NOT_FOUND` testing macros on CI.
# <https://github.com/rust-lang/rust/issues/128944>
case "$target" in
    *windows-gnu) extra_flags="$extra_flags --exclude libm-macros" ;;
esac

if [ "$(uname -a)" = "Linux" ]; then
    # also run the reference tests when we can. requires a Linux host.
    extra_flags="$extra_flags --features libm-test/musl-bitwise-tests"
fi

if [ "${BUILD_ONLY:-}" = "1" ]; then
    cmd="cargo build --target $target --package libm"
    $cmd
    $cmd --features 'unstable'

    echo "no tests to run for no_std"
else
    cmd="cargo test --all --target $target $extra_flags"

    # stable by default
    $cmd
    $cmd --release

    # unstable with a feature
    $cmd --features 'unstable'
    $cmd --release --features 'unstable'
fi
