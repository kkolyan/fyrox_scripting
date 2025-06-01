#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

INSTALL_DIR=$(realpath target/fyrox_cs_sdk)

# remove installation to detect possible regression asap
rm -rf $INSTALL_DIR
mkdir -p $INSTALL_DIR

# build Rust parts of Fyrox C# SDK

## editor runtime native library
./langs/cs/fyroxed-cs/install.sh $INSTALL_DIR

## game runtime editor library
./langs/cs/fyrox-lite-cs/install.sh $INSTALL_DIR

# build C# parts of Fyrox C# SDK

## NetCore library to be used in game by default (from IDE or standalone build)
./langs/cs/fyrox-lite-sln/fyrox_lite_cs_netcore/install.sh $INSTALL_DIR

## NetCore library to be used in editor mode (linked to fyroxed_cs instead of fyrox_lite_cs)
./langs/cs/fyrox-lite-sln/fyrox_lite_cs_netcore/install_for_editor.sh $INSTALL_DIR

## NetCore executable of editor
./langs/cs/fyrox-lite-sln/fyroxed_cs_netcore/install.sh $INSTALL_DIR

RUST_BACKTRACE=1 $INSTALL_DIR/fyroxed_cs_netcore.exe
