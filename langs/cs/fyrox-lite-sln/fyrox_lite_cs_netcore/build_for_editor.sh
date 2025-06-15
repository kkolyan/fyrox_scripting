#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

rm -rf bin/Debug/fyrox_lite_cs_netcore_4editor.*

dotnet build -c FyroxEdCs

for f in bin/FyroxEdCs/net9.0/fyrox_lite_cs_netcore.*; do
  ext="${f##*.}"
  mv "$f" "bin/Debug/net9.0/fyrox_lite_cs_netcore_4editor.$ext"
done
