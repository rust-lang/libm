mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))

ifndef CARGO_TARGET_DIR
override CARGO_TARGET_DIR = ${mkfile_dir}../../../target
endif

CRATE_RELEASE_DIR := ${CARGO_TARGET_DIR}/release
LDLIBS += -L $(CRATE_RELEASE_DIR) -lrelibm -Wl,-rpath=$(CRATE_RELEASE_DIR)

all: static

shared:
	${CC} -ansi --std=c99  simple.c $(LDLIBS)
static:
	${CC} -ansi --std=c99 simple.c $(CRATE_RELEASE_DIR)/librelibm.a