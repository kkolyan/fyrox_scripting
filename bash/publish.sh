#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ..

VERSION=$(cargo pkgid --manifest-path api/fyrox-lite/Cargo.toml | sed 's/.*#//')

ROOT=$(realpath .)

do_publish() {
  SCRIPT=$1
  FINAL_NAME=$2

  cd $ROOT

  rm -rf target/$FINAL_NAME.zip
  rm -rf target/$FINAL_NAME

  $SCRIPT target/$FINAL_NAME
  rm -rf target/$FINAL_NAME/*.pdb

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

do_publish ./bash/cs_install_sdk.sh fyrox_csharp-$VERSION-$OS_SUFFIX
do_publish ./bash/lua_install_sdk.sh fyrox_lua-$VERSION-$OS_SUFFIX

cp target/*.zip ../fyrox_lite_bin/
