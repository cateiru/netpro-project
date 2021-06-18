#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)

cd core
# create .cargo/config
if [ "$(uname)" == 'Darwin' ]; then
    mkdir .cargo
    printf "[target.x86_64-apple-darwin]\nrustflags = [\n  \"-C\", \"link-arg=-undefined\",\n  \"-C\", \"link-arg=dynamic_lookup\",\n]\n\n[target.aarch64-apple-darwin]\nrustflags = [\n  \"-C\", \"link-arg=-undefined\",\n  \"-C\", \"link-arg=dynamic_lookup\",\n]" $SHELL > .cargo/config
fi

cargo build --release

cd ..
mv $SCRIPT_DIR/core/target/release/libcore.dylib $SCRIPT_DIR/repo_sync/core.so
