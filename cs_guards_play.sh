#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$(realpath target/fyrox_cs_sdk)

# remove installation to detect possible regression asap
rm -rf $INSTALL_DIR

./install_sdk_cs.sh $INSTALL_DIR

cd showcase/guards_cs
RUST_BACKTRACE=1 dotnet run
