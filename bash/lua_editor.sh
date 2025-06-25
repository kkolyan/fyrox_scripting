#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cd ..

rm -rf target/fyrox_lua_sdk
./bash/lua_install_sdk.sh target/fyrox_lua_sdk

os=$(./bash/utils/os.sh)
if [[ "$os" == "Windows" ]]; then
    RUST_BACKTRACE=full ./target/fyrox_lua_sdk/fyroxed_lua.exe $*
elif [[ "$os" == "Macos" ]]; then
    RUST_BACKTRACE=full ./target/fyrox_lua_sdk/fyroxed_lua $*
elif [[ "$os" == "Linux" ]]; then
    RUST_BACKTRACE=full ./target/fyrox_lua_sdk/fyroxed_lua $*
else
    echo "Unknown OS: $os"
    exit -1
fi
