#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ../../..

cargo build -p fyrox-c
cp target/debug/*fyrox_c.* cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0

# debug editor is really slow
cargo build -p fyroxed-c --release
cp target/release/*fyroxed_c.* cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0

cd cs/FyroxLite
dotnet build
cd ../examples/Guards
RUST_BACKTRACE=1 dotnet run -p ../../FyroxLite/FyroxLiteEditor
