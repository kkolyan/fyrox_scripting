#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

rm -rf *.dll
rm -rf *.exe
rm -rf *.runtimeconfig.json

cd ../../..

./get_std_dll.sh cs/examples/Guards

RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p fyrox-c
cp target/debug/*fyrox_c.dll cs/examples/Guards

# debug editor is really slow
#cargo build -p fyroxed-c --release
#cp target/release/*fyroxed_c.dll cs/examples/Guards

# when we still need to debug editor
RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p fyroxed-c
cp target/debug/*fyroxed_c.dll cs/examples/Guards

RUSTFLAGS='-C prefer-dynamic=yes' cargo build -p fake_cs
cp target/debug/fake_cs.exe cs/examples/Guards

cp target/debug/*fyrox_dylib.dll cs/examples/Guards

cp target/debug/deps/fyrox_*.dll cs/examples/Guards

cd cs/FyroxLite
dotnet build
cd ../..
cp cs/FyroxLite/FyroxLite/bin/Debug/net8.0/FyroxLite.dll cs/examples/Guards
cp cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/FyroxLiteEditor.dll cs/examples/Guards
cp cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/FyroxLiteEditor.exe cs/examples/Guards
cp cs/FyroxLite/FyroxLiteEditor/bin/Debug/net8.0/FyroxLiteEditor.runtimeconfig.json cs/examples/Guards

cd cs/examples/Guards
dotnet build

#RUST_BACKTRACE=1 ./fake_cs.exe
RUST_BACKTRACE=1 ./FyroxLiteEditor.exe
