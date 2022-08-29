#!/bin/bash
set -e
cd "`dirname $0`"/../ft
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cp contract/target/wasm32-unknown-unknown/rlease/*.wasm ./res/