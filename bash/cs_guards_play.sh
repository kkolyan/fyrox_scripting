#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cd ..

INSTALL_DIR=target/fyrox_cs_sdk
mkdir -p $INSTALL_DIR
INSTALL_DIR=$(realpath $INSTALL_DIR)

# remove installation to detect possible regression asap
rm -rf $INSTALL_DIR

./bash/cs_install_sdk.sh $INSTALL_DIR

cd showcase/guards_cs
RUST_BACKTRACE=1 dotnet run
