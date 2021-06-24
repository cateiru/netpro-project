#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)

cd core
cargo build --release

cd ..
mv $SCRIPT_DIR/core/target/release/libcore.dylib $SCRIPT_DIR/repo_sync/core.so
