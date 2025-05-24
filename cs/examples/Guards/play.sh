#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

# Go to project root

cd ../../..

# cleanup to detect regression asap
rm -rf cs/examples/Guards/bin/Debug/net8.0/*.dll
rm -rf cs/examples/Guards/bin/Debug/net8.0/*.exe
rm -rf cs/examples/Guards/bin/Debug/net8.0/*.runtimeconfig.json

# build C# parts of Fyrox Lite

cd cs/FyroxLite
dotnet build
cd ../..

# build the C# game code

cd cs/examples/Guards
dotnet build
cd ../../..

cp cs/FyroxLite/FyroxLite/bin/Debug/net8.0/FyroxLite.dll cs/examples/Guards/bin/Debug/net8.0/

# build Rust parts of Fyrox Lite

# fyroxed-cs is built too to avoid rebuilds between edit and play (not sure it works that way, need to check)
RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p fyrox-lite-cs -p fyroxed-cs
cp target/debug/fyrox_lite_cs.dll cs/examples/Guards/bin/Debug/net8.0/
cp target/debug/deps/fyrox_dylib*.dll cs/examples/Guards/bin/Debug/net8.0/
./get_rust_std.sh cs/examples/Guards/bin/Debug/net8.0/

# Return to game folder

cd cs/examples/Guards

RUST_BACKTRACE=1 ../../../cs/examples/Guards/bin/Debug/net8.0/Guards.exe
