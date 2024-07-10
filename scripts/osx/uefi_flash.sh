#!/bin/sh

# Build in release
cargo build --release

# Unmount disk
sudo diskutil unmountDisk "$1"

# Flash the UEFI image
sudo gdd bs=2M if=target/release/build/os-67e8133262280158/out/uefi.img of="$1" status=progress

# Unmount disk
sudo diskutil unmountDisk "$1"

echo "Done!"