#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ../../..
cargo build -p fyrox-c
mkdir -p cs/examples/Guards/bin/Debug/net8.0
cp target/debug/*fyrox_c.* cs/examples/Guards/bin/Debug/net8.0
cd cs/FyroxLite
dotnet build
cd ../examples/Guards
RUST_BACKTRACE=full dotnet run
