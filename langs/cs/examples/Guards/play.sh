#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

# Go to project root

cd ../../../..

# cleanup to detect regression asap
rm -rf langs/cs/examples/Guards/bin/Debug/net8.0/*.dll
rm -rf langs/cs/examples/Guards/bin/Debug/net8.0/*.exe
rm -rf langs/cs/examples/Guards/bin/Debug/net8.0/*.runtimeconfig.json

# build C# parts of Fyrox Lite

cd langs/cs/FyroxLiteCs
dotnet build
cd ../../..

# build the C# game code

cd langs/cs/examples/Guards
dotnet build
cd ../../../..

cp langs/cs/FyroxLiteCs/FyroxLiteCs/bin/Debug/net8.0/FyroxLiteCs.dll langs/cs/examples/Guards/bin/Debug/net8.0/

# build Rust parts of Fyrox Lite

# fyroxed-cs is built too to avoid rebuilds between edit and play (not sure it works that way, need to check)
RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p fyrox-lite-cs -p fyroxed-cs
cp target/debug/fyrox_lite_cs.dll langs/cs/examples/Guards/bin/Debug/net8.0/
cp target/debug/fyrox_lite_cs.pdb langs/cs/examples/Guards/bin/Debug/net8.0/
cp target/debug/deps/fyrox_dylib*.dll langs/cs/examples/Guards/bin/Debug/net8.0/
cp target/debug/deps/fyrox_dylib*.pdb langs/cs/examples/Guards/bin/Debug/net8.0/
./get_rust_std.sh langs/cs/examples/Guards/bin/Debug/net8.0/

# Return to game folder

cd langs/cs/examples/Guards

RUST_BACKTRACE=1 ../../../../langs/cs/examples/Guards/bin/Debug/net8.0/Guards.exe
