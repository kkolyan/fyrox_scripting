#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cd ..

#cargo run --bin mdgen

rm -rf ./www/public
mdbook build ./www/hub && cp -r ./www/hub/book ./www/public
mdbook build ./www/fyrox_cs && cp -r ./www/fyrox_cs/book ./www/public/fyrox_cs
mdbook build ./www/fyrox_lua && cp -r ./www/fyrox_lua/book ./www/public/fyrox_lua

start ./www/public/index.html
