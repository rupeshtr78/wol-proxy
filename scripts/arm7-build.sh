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

# for raspberry pi aaarch64
wget https://developer.arm.com/-/media/Files/downloads/gnu/14.2.rel1/binrel/arm-gnu-toolchain-14.2.rel1-x86_64-aarch64-none-linux-gnu.tar.xz
tar -xvf arm-gnu-toolchain-14.2.rel1-x86_64-aarch64-none-linux-gnu.tar.xz
export PATH=$PATH:/path/to/arm-gnu-toolchain-14.2.rel1-x86_64-aarch64-none-linux-gnu/bin
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-none-linux-gnu-gcc"

# Add the linker to the .cargo/config file:
# .cargo/config
# [target.armv7-unknown-linux-gnueabihf]
# linker = "arm-none-linux-gnueabihf-gcc"

# Install the ARMv7l target:
rustup target add armv7-unknown-linux-gnueabihf
# Build the project
cargo build --target armv7-unknown-linux-gnueabihf --release

# using cross option
# cross it will launch a docker container, spawn
# the build inside that, and cross compile your code
cargo install cross
cross build --target armv7-unknown-linux-gnueabihf

# soft float works for ddwrt routers
cross build --target armv7-unknown-linux-gnueabi --release

# for raspberry pi 4
cross build --target aarch64-unknown-linux-gnu --release