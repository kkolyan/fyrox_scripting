#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ../..

S_LANG=$1
SCRIPT=$2
NAME_BASE=$3

VERSION=$(cargo pkgid --manifest-path api/fyrox-lite/Cargo.toml | sed 's/.*#//')
ENGINE_VERSION=$(cargo pkgid --manifest-path engine/fyrox/Cargo.toml | sed 's/.*#//')
GIT_REVISION=$(git rev-parse --short HEAD)

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

FINAL_NAME=$NAME_BASE-$ENGINE_VERSION-$VERSION-$GIT_REVISION-$OS_SUFFIX

rm -rf target/$FINAL_NAME.zip
rm -rf target/$FINAL_NAME

$SCRIPT target/$FINAL_NAME

rm -rf target/$FINAL_NAME/*.pdb
if [[ "$os" == "Linux" ]]; then
  if [[ "$S_LANG" == "Lua" ]]; then
    strip --strip-unneeded target/$FINAL_NAME/fyrox_lite_lua
    strip --strip-unneeded target/$FINAL_NAME/fyroxed_lua
  elif [[ "$S_LANG" == "C#" ]]; then
    strip --strip-unneeded target/$FINAL_NAME/libfyrox_lite_cs.so
    strip --strip-unneeded target/$FINAL_NAME/libfyroxed_cs.so
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

ARCH_DIR=$(mktemp -d)

cp -r target/$FINAL_NAME $ARCH_DIR

cargo run --bin zip_util -- $ARCH_DIR target/$FINAL_NAME.zip
rm -rf target/$FINAL_NAME

gh release create nightly-$LANG_TAG_SUFFIX-$OS_SUFFIX-$GIT_REVISION ./target/$FINAL_NAME.zip --title "$FINAL_NAME" --notes "Auto-upload"
