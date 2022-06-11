#!/bin/sh
set -e
rm -rf pkg
wasm-pack build --release --target web --no-default-features --scope web
sed -i 's/@web\/rustpotter-wasm/rustpotter-web-slim/g' pkg/package.json
