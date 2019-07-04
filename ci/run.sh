#!/usr/bin/env sh

if [ -z "$1" ]; then
    echo "This script takes the $TARGET triple as its argument"
    exit 1
fi

set -ex

TARGET=$1

CMD="cargo test \
  --manifest-path=crates/libm-test/Cargo.toml --all \
  --no-default-features \
  --target $TARGET "

$CMD
$CMD --release

$CMD --features 'stable'
$CMD --release --features 'stable'

if [ "$TARGET" = "x86_64-unknown-linux-gnu" ]; then
    export TARGET=x86_64-unknown-linux-musl
    
    $CMD --features 'stable checked system_libm'
    $CMD --release --features  'stable checked system_libm'
fi

