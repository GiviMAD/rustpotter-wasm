#!/bin/sh
set -e
rm -rf pkg
wasm-pack build --release --target web --scope web --no-default-features
(cd pkg && npm pkg set type='module')
(cd pkg && npm pkg set name='rustpotter-web-slim')
