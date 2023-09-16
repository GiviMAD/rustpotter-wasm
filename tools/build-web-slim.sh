#!/bin/sh
set -e
rm -rf pkg
wasm-pack build --release --target web --scope web --no-default-features
(cd pkg && npm pkg set type='module')
sed -i 's/@web\/rustpotter-wasm/rustpotter-web-slim/g' pkg/package.json
