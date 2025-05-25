#!/bin/bash
cd "$(dirname "$0")"
cargo run --release -p fyroxed-lua --manifest-path ../../../../Cargo.toml