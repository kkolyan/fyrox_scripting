#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cd showcase/guards-vanilla
cargo run -p guards-vanilla-executor --manifest-path ../../Cargo.toml
