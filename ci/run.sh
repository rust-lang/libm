#!/usr/bin/env sh

set -ex
TARGET=$1

CMD="cargo test --all --no-default-features --target $TARGET"

$CMD
$CMD --release

$CMD --features 'stable'
$CMD --release --features 'stable'

cargo test -p libm-test --features 'stable checked'
cargo test -p libm-test --release --features  'stable checked'
