#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

./build_for_editor.sh

cp bin/Debug/net8.0/fyrox_lite_cs_netcore_4editor.dll $INSTALL_DIR
cp bin/Debug/net8.0/fyrox_lite_cs_netcore_4editor.pdb $INSTALL_DIR
