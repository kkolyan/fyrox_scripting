#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

cd ../../..
cargo build -p fyrox-lite-cs
cp target/debug/fyrox_lite_cs.dll $INSTALL_DIR
cp target/debug/fyrox_lite_cs.pdb $INSTALL_DIR
