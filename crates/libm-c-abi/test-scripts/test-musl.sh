#! /bin/bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

SCRIPT_DIR="$( cd "$(dirname "$0")" ; pwd -P )"
REPO_DIR="${SCRIPT_DIR}/libc-test/"
CRATE_RELEASE_DIR="${CARGO_TARGET_DIR:-${SCRIPT_DIR}/../../../target/}release/"

if [ ! -d "${REPO_DIR}" ]; then
  cd ${SCRIPT_DIR}
    git clone git://nsz.repo.hu:49100/repo/libc-test
    cd ${REPO_DIR}
      cat << EOF > config.mak
CFLAGS += -std=c99 -fno-builtin -frounding-math
LDLIBS += -L ${CRATE_RELEASE_DIR} -lrelibm -Wl,-rpath=${CRATE_RELEASE_DIR}
EOF
    cd -
  cd -
fi

# make sure we have a library test
cargo build --release
echo [+] Run musl test suite
cd ${REPO_DIR}/src/math
    make clean -s && make -s
    echo "[+] libc-test result for math"
    cat REPORT | grep -v exception | grep -v l\.h | grep X
cd -