#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cargo run --bin mdgen

rm -rf ./www/public
mdbook build ./www/hub && cp -r ./www/hub/book ./www/public
mdbook build ./www/sdk_cs && cp -r ./www/sdk_cs/book ./www/public/sdk_cs
mdbook build ./www/sdk_lua && cp -r ./www/sdk_lua/book ./www/public/sdk_lua

start ./www/public/index.html
