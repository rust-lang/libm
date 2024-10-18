#!/bin/sh

set -eux

export RUST_BACKTRACE="${RUST_BACKTRACE:-full}"
# Needed for no-panic to correct detect a lack of panics
export RUSTFLAGS="$RUSTFLAGS -Ccodegen-units=1"

target="${1:-}"

if [ -z "$target" ]; then
    host_target=$(rustc -vV | awk '/^host/ { print $2 }')
    echo "Defaulted to host target $host_target"
    target="$host_target"
fi


# We nceed to specifically skip tests for this crate on systems that can't
# build musl since otherwise `--all` will activate it.
case "$target" in
    *msvc*) exclude_flag="--exclude musl-math-sys" ;;
    *wasm*) exclude_flag="--exclude musl-math-sys" ;;
    *thumb*) exclude_flag="--exclude musl-math-sys" ;;
    # `STATUS_DLL_NOT_FOUND` on CI for some reason
    # <https://github.com/rust-lang/rust/issues/128944>
    *windows-gnu) exclude_flag="--exclude libm-macros" ;;
    *) exclude_flag="" ;;
esac

if [ "${BUILD_ONLY:-}" = "1" ]; then
    cmd="cargo build --target $target --package libm"
    $cmd
    $cmd --features 'unstable'

    echo "no tests to run for no_std"
else
    cmd="cargo test --all --target $target $exclude_flag"


    # stable by default
    $cmd
    $cmd --release

    # unstable with a feature
    $cmd --features 'unstable'
    $cmd --release --features 'unstable'

    if [ "$(uname -a)" = "Linux" ]; then
        # also run the reference tests when we can. requires a Linux host.
        $cmd --features 'unstable libm-test/musl-bitwise-tests'
        $cmd --release --features 'unstable libm-test/musl-bitwise-tests'
    fi
fi
