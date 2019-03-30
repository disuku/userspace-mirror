#!/usr/bin/env bash

set -e

rm -rf build
cp -r src-server build

#cargo build --release
#cp target/release/librust_kmod_hello_world.a build/lib.o

(cd build; make)
