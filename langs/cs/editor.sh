#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=target/fyrox_cs_sdk

# Go to project root
cd ../..

# remove installation to detect possible regression asap
rm -rf $INSTALL_DIR
mkdir -p $INSTALL_DIR

# build Rust parts of Fyrox C# SDK

cargo build -p fyroxed-cs
cp target/debug/fyroxed_cs.dll $INSTALL_DIR
cp target/debug/fyroxed_cs.pdb $INSTALL_DIR

# build C# parts of editor runtime

cd langs/cs/FyroxLiteCs/FyroxEdCs
#dotnet build -p:FyroxCsSdkInstallDir=$INSTALL_DIR
dotnet build
cd ../../../..
cp langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/FyroxEdCs.dll $INSTALL_DIR
cp langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/FyroxEdCs.pdb $INSTALL_DIR
cp langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/FyroxEdCs.exe $INSTALL_DIR
cp langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/FyroxEdCs.runtimeconfig.json $INSTALL_DIR

# build C# parts of game runtime

cd langs/cs/FyroxLiteCs/FyroxLiteCs
dotnet build
cd ../../../..
cp langs/cs/FyroxLiteCs/FyroxLiteCs/bin/Debug/net8.0/FyroxLiteCs.dll $INSTALL_DIR
cp langs/cs/FyroxLiteCs/FyroxLiteCs/bin/Debug/net8.0/FyroxLiteCs.pdb $INSTALL_DIR

RUST_BACKTRACE=1 $INSTALL_DIR/FyroxEdCs.exe
