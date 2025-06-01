#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

rm -rf target/sdk_lua && ./install_sdk_lua.sh target/sdk_lua &&  ./target/sdk_lua/fyroxed_lua.exe
