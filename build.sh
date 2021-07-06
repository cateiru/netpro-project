#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)

cd core
cargo build --release

cd ..

if [ "$(uname)" == 'Darwin' ]; then
    # MacOS
    mv $SCRIPT_DIR/core/target/release/libcore.dylib $SCRIPT_DIR/repo_sync/core.so
elif [ "$(expr substr $(uname -s) 1 5)" == 'Linux' ]; then
    # Linux
    mv $SCRIPT_DIR/core/target/release/libcore.so $SCRIPT_DIR/repo_sync/core.so
else
  echo "Your platform ($(uname -a)) is not supported."
  exit 1
fi
