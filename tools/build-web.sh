#!/bin/sh
set -e
rm -rf pkg
wasm-pack build --release --target web --scope web
(cd pkg && npm pkg set type='module')
sed -i 's/@web\/rustpotter-wasm/rustpotter-web/g' pkg/package.json
