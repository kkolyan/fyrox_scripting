#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

dotnet build
cp bin/Debug/net8.0/fyroxed_cs_netcore.dll $INSTALL_DIR
cp bin/Debug/net8.0/fyroxed_cs_netcore.pdb $INSTALL_DIR
cp bin/Debug/net8.0/fyroxed_cs_netcore.exe $INSTALL_DIR
cp bin/Debug/net8.0/fyroxed_cs_netcore.runtimeconfig.json $INSTALL_DIR
