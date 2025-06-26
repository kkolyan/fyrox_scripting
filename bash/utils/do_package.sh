#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ../..

S_LANG=$1
SCRIPT=$2
NAME_BASE=$3

os=$(./bash/utils/os.sh)
if [[ "$os" == "Windows" ]]; then
  OS_SUFFIX=win
elif [[ "$os" == "Macos" ]]; then
  OS_SUFFIX=macos
elif [[ "$os" == "Linux" ]]; then
  OS_SUFFIX=linux
else
    echo "Unknown OS: $os"
    exit -1
fi

ARTIFACTS_DIR=$(mktemp -d)

rm -rf target/$FINAL_NAME.zip

$SCRIPT $ARTIFACTS_DIR

rm -rf $ARTIFACTS_DIR/*.pdb
if [[ "$os" == "Linux" ]]; then
  if [[ "$S_LANG" == "Lua" ]]; then
    strip --strip-unneeded $ARTIFACTS_DIR/fyrox_lite_lua
    strip --strip-unneeded $ARTIFACTS_DIR/fyroxed_lua
  elif [[ "$S_LANG" == "C#" ]]; then
    strip --strip-unneeded $ARTIFACTS_DIR/libfyrox_lite_cs.so
    strip --strip-unneeded $ARTIFACTS_DIR/libfyroxed_cs.so
  else
    echo "Unknown S_LANG: $S_LANG"
    exit -1
  fi
fi

if [[ "$os" == "Linux" ]]; then
  if [[ "$S_LANG" == "Lua" ]]; then
    LANG_TAG_SUFFIX=csharp
  elif [[ "$S_LANG" == "C#" ]]; then
    LANG_TAG_SUFFIX=lua
  else
    echo "Unknown S_LANG: $S_LANG"
    exit -1
  fi
fi

VERSION=$(cargo pkgid --manifest-path api/fyrox-lite/Cargo.toml | sed 's/.*#//')
ENGINE_VERSION=$(cargo pkgid --manifest-path engine/fyrox/Cargo.toml | sed 's/.*#//')
GIT_REVISION=$(git rev-parse --short HEAD)
FINAL_NAME=$NAME_BASE-$ENGINE_VERSION-$VERSION-$GIT_REVISION-$OS_SUFFIX

ARCH_DIR=$(mktemp -d)
mkdir $ARCH_DIR/$FINAL_NAME

cp -r $ARTIFACTS_DIR/* $ARCH_DIR/$FINAL_NAME

cargo run --bin zip_util -- $ARCH_DIR target/$FINAL_NAME.zip
