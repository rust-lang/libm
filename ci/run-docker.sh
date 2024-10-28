#!/bin/bash

# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -euxo pipefail

run() {
    local target=$1

    echo "testing target: $target"

    # This directory needs to exist before calling docker, otherwise docker will create it but it
    # will be owned by root
    mkdir -p target

    docker build -t "$target" "ci/docker/$target"
    docker run \
           --rm \
           --user "$(id -u):$(id -g)" \
           -e RUSTFLAGS \
           -e CARGO_HOME=/cargo \
           -e CARGO_TARGET_DIR=/target \
           -e EMULATED=1 \
           -v "${HOME}/.cargo:/cargo" \
           -v "$(pwd)/target:/target" \
           -v "$(pwd):/checkout:ro" \
           -v "$(rustc --print sysroot):/rust:ro" \
           --init \
           -w /checkout \
           "$target" \
           sh -c "HOME=/tmp PATH=\$PATH:/rust/bin exec ci/run.sh $target"
}

if [ -z "$1" ]; then
  echo "running tests for all targets"

  for d in ci/docker/*; do
    run $d
  done
else
  run $1
fi
