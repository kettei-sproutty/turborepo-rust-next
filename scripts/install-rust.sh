#!/bin/bash

# Get the current working directory
CWD=$(pwd)

# Install Rust and Cargo in the CWD
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable --no-modify-path --prefix="$CWD"

# Add Cargo to PATH
export PATH="$CWD/bin:$PATH"

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f --prefix="$CWD/bin"

cargo --help
wasm-pack --help