#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

if [[ "$OSTYPE" == "msys" ]]; then
    echo "Windows"
elif [[ "$(uname -s)" == "Darwin" ]]; then
    echo "Macos"
elif [[ "$(uname -s)" == "Linux" ]]; then
    echo "Linux"
else
    echo "Unknown OS (uname -a: $(uname -a))"
    exit -1
fi
