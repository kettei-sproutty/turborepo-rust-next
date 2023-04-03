#!/bin/bash

# Get the current working directory
PWD=$(pwd)

# Check if the .cargo/bin directory exists
if [ -d "$PWD/.cargo/bin" ]; then
  # Add Cargo to PATH
  export PATH="$PWD/.cargo/bin:$PATH"
fi

cargo --help
wasm-pack --help