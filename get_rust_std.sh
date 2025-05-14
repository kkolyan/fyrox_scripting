#!/bin/sh

set -e

TARGET_DIR="$1"

if [ -z "$TARGET_DIR" ]; then
  echo "Usage: $0 <target-directory>"
  exit 1
fi

mkdir -p "$TARGET_DIR"

# Detect host triple
TARGET_TRIPLE=$(rustc -vV | grep 'host:' | awk '{print $2}')

# Detect toolchain channel (stable/nightly)
TOOLCHAIN=$(rustc -vV | grep 'release:' | awk '{print $2}' | grep -q nightly && echo "nightly" || echo "stable")

# Construct rustlib path
RUST_LIB_DIR="$HOME/.rustup/toolchains/${TOOLCHAIN}-${TARGET_TRIPLE}/lib/rustlib/${TARGET_TRIPLE}/lib"

# Copy relevant DLLs
find "$RUST_LIB_DIR" -maxdepth 1 -name '*-*.dll' -exec cp -v {} "$TARGET_DIR" \;