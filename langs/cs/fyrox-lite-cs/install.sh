#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

cd ../../..
cargo build -p fyrox-lite-cs

os=$(./bash/utils/os.sh)
if [[ "$os" == "Windows" ]]; then
    cp target/debug/fyrox_lite_cs.dll $INSTALL_DIR
    cp target/debug/fyrox_lite_cs.pdb $INSTALL_DIR
elif [[ "$os" == "Macos" ]]; then
    cp target/debug/libfyrox_lite_cs.dylib $INSTALL_DIR
elif [[ "$os" == "Linux" ]]; then
    cp target/debug/libfyrox_lite_cs.so $INSTALL_DIR
else
    echo "Unknown OS: $os"
    exit -1
fi
