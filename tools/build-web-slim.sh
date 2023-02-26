#!/bin/sh
set -e
rm -rf pkg
wasm-pack build --release --target web --scope web --no-default-features
sed -i 's/@web\/rustpotter-wasm/rustpotter-web-slim/g' pkg/package.json
