#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

./build_for_editor.sh

cp bin/Debug/net8.0/FyroxLiteCs_FyroxEdCs.dll $INSTALL_DIR
cp bin/Debug/net8.0/FyroxLiteCs_FyroxEdCs.pdb $INSTALL_DIR
