#!/bin/bash

# Get the current working directory
PWD=$(pwd)

export CARGO_HOME=$PWD/.cargo
export RUSTUP_HOME=$PWD/.rustup

# Install Rust and Cargo in the CWD
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable --no-modify-path

# Add Cargo to PATH
export PATH=$PATH:$PWD/.cargo/bin

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f --prefix="$CWD/bin"

cargo --help
wasm-pack --help