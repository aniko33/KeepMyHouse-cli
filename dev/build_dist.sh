#!/bin/bash

platforms=(
    "aarch64-apple-darwin"
    "i686-pc-windows-gnu"
    "i686-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "x86_64-pc-windows-gnu"
    "x86_64-unknown-linux-gnu"
)

if [[ ! -d "dist" ]]; then
    mkdir dist
fi

for platform in ${platforms[@]}
do
    echo -e "[ \033[0;32m$platform\033[0m ]"
    cargo build -r --target $platform

    if [ -f "target/$platform/release/kmh-cli" ]; then
        mv target/$platform/release/kmh-cli dist/kmh_$platform
    else 
        mv target/$platform/release/kmh-cli.exe dist/kmh_$platform.exe
    fi
done