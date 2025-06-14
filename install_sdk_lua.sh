#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

if [ -z "$1" ]; then
  echo "Error: First argument should be a path where to install Fyrox Lua SDK" >&2
  exit 1
fi

if [ -e "$1" ]; then
  echo "Error: specified path points to existing file or directory. Non-existing path required for Fyrox Lua SDK" >&2
  exit 1
fi

INSTALL_DIR=$1
mkdir -p $INSTALL_DIR
INSTALL_DIR=$(realpath $INSTALL_DIR)

cargo build -p fyrox_lite_lua
cargo build -p fyroxed_lua

os=$(./os.sh)
if [[ "$os" == "Windows" ]]; then
  cp target/debug/fyrox_lite_lua.exe $INSTALL_DIR
  cp target/debug/fyroxed_lua.exe $INSTALL_DIR
elif [[ "$os" == "Macos" ]]; then
  cp target/debug/fyrox_lite_lua $INSTALL_DIR
  cp target/debug/fyroxed_lua $INSTALL_DIR
elif [[ "$os" == "Linux" ]]; then
  cp target/debug/fyrox_lite_lua $INSTALL_DIR
  cp target/debug/fyroxed_lua $INSTALL_DIR
else
    echo "Unknown OS: $os"
    exit -1
fi

cd langs/lua/annotations
tar -czf $INSTALL_DIR/fyrox_lite_lua_annotations.tar.gz *
cd ../../..

echo "Fyrox Lua SDK has been installed to $INSTALL_DIR"
