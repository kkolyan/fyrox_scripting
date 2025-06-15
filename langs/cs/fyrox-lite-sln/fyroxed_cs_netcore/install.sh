#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$1

dotnet build
cp bin/Debug/net9.0/fyroxed_cs_netcore.dll $INSTALL_DIR
cp bin/Debug/net9.0/fyroxed_cs_netcore.pdb $INSTALL_DIR
cp bin/Debug/net9.0/fyroxed_cs_netcore.runtimeconfig.json $INSTALL_DIR

os=$(./../../../../bash/utils/os.sh)
if [[ "$os" == "Windows" ]]; then
    cp bin/Debug/net9.0/fyroxed_cs_netcore.exe $INSTALL_DIR
elif [[ "$os" == "Macos" ]]; then
    cp bin/Debug/net9.0/fyroxed_cs_netcore $INSTALL_DIR
elif [[ "$os" == "Linux" ]]; then
    cp bin/Debug/net9.0/fyroxed_cs_netcore $INSTALL_DIR
else
    echo "Unknown OS: $os"
    exit -1
fi

