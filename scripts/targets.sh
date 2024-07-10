#!/usr/bin/env sh

TARGET_LIST=$(cat << EOF
nrf52840: thumbv7em-none-eabihf
EOF
)

echo $TARGET_LIST | awk -F': ' '$1 == key {print $2}' key="$1"
