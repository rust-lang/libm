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

if [ "${NO_STD:-}" = "1" ]; then
    cargo build --target "$target"
    cargo build --target "$target" --features 'unstable'

    echo "no tests to run for no_std"
else
    cmd="cargo test --all --target $target"

    # # We nceed to specif
    # case "$target" in
    #   *msvc*) cmd="$cmd --exclude musl-math-sys" ;;
    #   *wasm*) cmd="$cmd --exclude musl-math-sys" ;;
    # esac

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
