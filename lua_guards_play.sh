#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
# cargo run --bin luagen
cd showcase/guards_lua
cargo run -p fyrox_lite_lua --manifest-path ../../Cargo.toml
