#!/bin/bash

# list all the targets
rustc --print target-list

# Install the cross-compilation toolchain
brew tap osx-cross/arm
brew install arm-linux-gnueabihf-binutils
# cross-compilation toolchain for linux
sudo apt-get install gcc-arm-linux-gnueabihf

# Install the ARMv7l target:
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target=armv7-unknown-linux-gnueabihf

