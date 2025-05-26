#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

# Go to project root

cd ../..

# cleanup to detect regression asap
rm -rf langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/*.dll
rm -rf langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/*.exe
rm -rf langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/*.runtimeconfig.json

# build Rust parts of Fyrox Lite

mkdir -p langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/

cargo build -p fyroxed-cs
cp target/debug/fyroxed_cs.dll langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/
cp target/debug/fyroxed_cs.pdb langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/

# build C# parts of Fyrox Lite

cd langs/cs/FyroxLiteCs
dotnet build -p:FyroxEditorSymbol=FYROX_EDITOR
cd ../../..
#cp langs/cs/FyroxLiteCs/FyroxLiteCs/bin/Debug/net8.0/FyroxLiteCs.dll langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/

RUST_BACKTRACE=1 langs/cs/FyroxLiteCs/FyroxEdCs/bin/Debug/net8.0/FyroxEdCs.exe
