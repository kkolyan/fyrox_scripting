#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

if [ -z "$1" ]; then
  echo "Error: First argument should be a path where to install Fyrox C# SDK" >&2
  exit 1
fi

if [ -e "$1" ]; then
  echo "Error: specified path points to existing file or directory. Non-existing path required" >&2
  exit 1
fi

INSTALL_DIR=$(realpath $1)

mkdir -p "$(dirname "$INSTALL_DIR")"

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

echo "Fyrox C# SDK has been installed to $INSTALL_DIR"