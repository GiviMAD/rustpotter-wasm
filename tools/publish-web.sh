#!/bin/sh
set -e
./tools/build-web.sh
(cd pkg && npm publish)
