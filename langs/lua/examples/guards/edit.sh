#!/bin/bash
cd "$(dirname "$0")"
cd ../../../..
cargo build -p fyroxed-lua
./target/debug/fyroxed-lua.exe langs/lua/examples/guards