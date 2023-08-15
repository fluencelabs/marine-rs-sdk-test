#!/bin/sh

# This script builds all subprojects and puts all created Wasm modules in one dir
marine build --release

rm artifacts/* || true
mkdir -p artifacts

cp target/wasm32-wasi/release/consumer.wasm artifacts/
