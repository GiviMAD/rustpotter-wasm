#!/bin/sh
set -e
./tools/build-web-slim.sh
(cd pkg && npm publish)
