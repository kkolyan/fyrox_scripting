#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

dotnet build
cp bin/Debug/net8.0/FyroxEdCs.dll $INSTALL_DIR
cp bin/Debug/net8.0/FyroxEdCs.pdb $INSTALL_DIR
cp bin/Debug/net8.0/FyroxEdCs.exe $INSTALL_DIR
cp bin/Debug/net8.0/FyroxEdCs.runtimeconfig.json $INSTALL_DIR
