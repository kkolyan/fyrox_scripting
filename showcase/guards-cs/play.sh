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

cargo build -p fyrox-lite-cs -p
cp target/debug/fyrox_lite_cs.dll langs/cs/examples/Guards/bin/Debug/net8.0/
cp target/debug/fyrox_lite_cs.pdb langs/cs/examples/Guards/bin/Debug/net8.0/

# Return to game folder

cd langs/cs/examples/Guards

RUST_BACKTRACE=1 ../../../../langs/cs/examples/Guards/bin/Debug/net8.0/Guards.exe
