#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

cd ..

cd showcase/guards_vanilla
cargo run -p guards-vanilla-editor --manifest-path ../../Cargo.toml
