#!/bin/sh
set -e
rm -rf pkg
wasm-pack build --target web --scope web
sed -i 's/@web\/rustpotter-wasm/rustpotter-web/g' pkg/package.json
