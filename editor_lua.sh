#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

rm -rf target/sdk_lua && ./install_sdk_lua.sh target/sdk_lua

os=$(./os.sh)
if [[ "$os" == "Windows" ]]; then
    ./target/sdk_lua/fyroxed_lua.exe $*
elif [[ "$os" == "Macos" ]]; then
    ./target/sdk_lua/fyroxed_lua $*
elif [[ "$os" == "Linux" ]]; then
    ./target/sdk_lua/fyroxed_lua $*
else
    echo "Unknown OS: $os"
    exit -1
fi
