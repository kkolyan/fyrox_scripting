#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cd ..

INSTALL_DIR_CS=target/fyrox_cs_sdk
INSTALL_DIR_LUA=target/fyrox_lua_sdk

# remove installation to detect possible regression asap
rm -rf $INSTALL_DIR_CS
rm -rf $INSTALL_DIR_LUA

./bash/cs_install_sdk.sh $INSTALL_DIR_CS
./bash/lua_install_sdk.sh $INSTALL_DIR_LUA
