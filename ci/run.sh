#!/usr/bin/env sh

set -ex
TARGET=$1

CMD="cargo test --all --no-default-features --target $TARGET"

$CMD
$CMD --release

$CMD --features "stable"
$CMD --release --features "stable"

TEST_MUSL="musl-reference-tests"
if [ "$TARGET" = "x86_64-apple-darwin" ] || [ "$TARGET" = "i686-apple-darwin" ] ; then
    # FIXME: disable musl-reference-tests on OSX
    export TEST_MUSL=""
fi

$CMD --features "stable checked"
$CMD --release --features  "stable checked ${TEST_MUSL}"

if [ "$TARGET" = "x86_64-unknown-linux-gnu" ] || [ "${TARGET}" = "x86_64-apple-darwin" ]; then
    (
        cd crates/libm-cdylib
        cargo test
        cargo test --release
    )
fi
