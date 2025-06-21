#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ..

VERSION=$(cargo pkgid --manifest-path api/fyrox-lite/Cargo.toml | sed 's/.*#//')

ROOT=$(realpath .)

do_publish() {
  S_LANG=$1
  SCRIPT=$2
  FINAL_NAME=$3

  cd $ROOT

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

  ARCH_DIR=$(mktemp -d)

  cp -r target/$FINAL_NAME $ARCH_DIR

  cargo run --bin zip_util -- $ARCH_DIR target/$FINAL_NAME.zip
  rm -rf target/$FINAL_NAME
}


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

do_publish 'C#' ./bash/cs_install_sdk.sh fyrox_csharp-$VERSION-$OS_SUFFIX
do_publish 'Lua' ./bash/lua_install_sdk.sh fyrox_lua-$VERSION-$OS_SUFFIX

cp target/*.zip ../fyrox_lite_bin/
