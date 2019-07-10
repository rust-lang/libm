#!/usr/bin/env sh

# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -ex

run() {
    # This directory needs to exist before calling docker, otherwise docker will create it but it
    # will be owned by root
    mkdir -p target

    docker build -t "${1}" -f "ci/docker/${1}/Dockerfile" ci/
    docker run \
           --rm \
           --user "$(id -u)":"$(id -g)" \
           --env CARGO_HOME=/cargo \
           --env CARGO_TARGET_DIR=/checkout/target \
           --volume "$(dirname "$(dirname "$(command -v cargo)")")":/cargo \
           --volume "$(rustc --print sysroot)":/rust:ro \
           --volume "$(pwd)":/checkout:ro \
           --volume "$(pwd)"/target:/checkout/target \
           --init \
           --workdir /checkout \
           --privileged \
           "${1}" \
           sh -c "HOME=/tmp PATH=\$PATH:/rust/bin exec ci/run.sh ${1}"
}

if [ -z "$1" ]; then
    for d in ci/docker/*; do
      run "${d}"
  done
else
  run "${1}"
fi
