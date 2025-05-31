#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

cd ../../..
cargo build -p fyroxed-cs
cp target/debug/fyroxed_cs.dll $INSTALL_DIR
cp target/debug/fyroxed_cs.pdb $INSTALL_DIR
