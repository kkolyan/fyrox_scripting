#!/usr/bin/env bash

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
