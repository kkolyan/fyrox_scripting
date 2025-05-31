#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

rm -rf bin/Debug/FyroxLiteCs_FyroxEdCs.*

dotnet build -c FyroxEdCs

for f in bin/FyroxEdCs/net8.0/FyroxLiteCs.*; do
  ext="${f##*.}"
  mv "$f" "bin/Debug/net8.0/FyroxLiteCs_FyroxEdCs.$ext"
done
