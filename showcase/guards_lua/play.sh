#!/bin/bash
cd "$(dirname "$0")"
RUST_BACKTRACE=1 cargo run -p fyrox-lite-lua --manifest-path ../../../../Cargo.toml
