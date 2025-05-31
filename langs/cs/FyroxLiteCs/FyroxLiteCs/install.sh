#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

./build.sh

cp bin/Debug/net8.0/FyroxLiteCs.dll $INSTALL_DIR
cp bin/Debug/net8.0/FyroxLiteCs.pdb $INSTALL_DIR
