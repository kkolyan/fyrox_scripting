#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=target/fyrox_cs_sdk

# remove installation to detect possible regression asap
rm -rf $INSTALL_DIR

./install_sdk_cs.sh $INSTALL_DIR

os=$(./os.sh)
if [[ "$os" == "Windows" ]]; then
    RUST_BACKTRACE=1 $INSTALL_DIR/fyroxed_cs_netcore.exe $*
elif [[ "$os" == "Macos" ]]; then
    RUST_BACKTRACE=1 $INSTALL_DIR/fyroxed_cs_netcore $*
elif [[ "$os" == "Linux" ]]; then
    RUST_BACKTRACE=1 $INSTALL_DIR/fyroxed_cs_netcore $*
else
    echo "Unknown OS: $os"
    exit -1
fi
