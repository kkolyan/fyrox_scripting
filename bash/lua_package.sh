#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ..

./bash/utils/do_package.sh 'Lua' ./bash/lua_install_sdk.sh fyrox_lua
