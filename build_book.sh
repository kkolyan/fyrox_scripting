#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cargo run --bin mdgen

mdbook build ./www/hub && cp ./www/hub/book ./www/public
mdbook build ./www/cs && cp ./www/cs/book ./www/public/cs
mdbook build ./www/lua && cp ./www/lua/book ./www/public/lua

open ./www/public/index.html
