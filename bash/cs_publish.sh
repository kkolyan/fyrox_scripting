#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e
cd ..

./bash/utils/do_publish.sh 'C#' ./bash/cs_install_sdk.sh fyrox_csharp
