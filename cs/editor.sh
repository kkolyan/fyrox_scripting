#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

# Go to project root

cd ..

# cleanup to detect regression asap
rm -rf cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/*.dll
rm -rf cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/*.exe
rm -rf cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/*.runtimeconfig.json

# build C# parts of Fyrox Lite

cd cs/FyroxLite
dotnet build
cd ../..
cp cs/FyroxLite/FyroxLite/bin/Debug/net8.0/FyroxLite.dll cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/

# build Rust parts of Fyrox Lite

RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p fyrox-lite-cs -p fyroxed-cs
cp target/debug/fyrox_lite_cs.dll cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/
cp target/debug/fyrox_lite_cs.pdb cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/
cp target/debug/fyroxed_cs.dll cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/
cp target/debug/fyroxed_cs.pdb cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/
cp target/debug/deps/fyrox_dylib*.dll cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/
cp target/debug/deps/fyrox_dylib*.pdb cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/
./get_rust_std.sh cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/

RUST_BACKTRACE=1 cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/FyroxLiteEditor.exe
