#!/bin/bash

# list all the targets
rustc --print target-list

# Install the cross-compilation toolchain
brew tap osx-cross/arm
brew install arm-linux-gnueabihf-binutils
# cross-compilation toolchain for linux
sudo apt-get install gcc-arm-linux-gnueabihf # did not work for me

# Download the ARMv7l target:
# https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads
# wget https://developer.arm.com/-/media/Files/downloads/gnu/14.2.rel1/binrel/arm-gnu-toolchain-14.2.rel1-x86_64-arm-none-linux-gnueabihf.tar.xz

tar -xvf gcc-arm-10.2-2020.11-x86_64-arm-linux-gnueabihf.tar.xz
export PATH=$PATH:/path/to/gcc-arm-10.2-2020.11-x86_64-arm-linux-gnueabihf/bin
# verufy the installation
arm-none-linux-gnueabihf-gcc --version

# Add the linker to the .cargo/config file:
# .cargo/config
# [target.armv7-unknown-linux-gnueabihf]
# linker = "arm-none-linux-gnueabihf-gcc"

# Install the ARMv7l target:
rustup target add armv7-unknown-linux-gnueabihf
# Build the project
cargo build --target armv7-unknown-linux-gnueabihf --release

