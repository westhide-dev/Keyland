#!/usr/bin/env bash

set -ex

CURRENT_DIR=$(realpath $(dirname $0))
KEYLAND_DIR=$(realpath ${CURRENT_DIR}/../../)

cd ${KEYLAND_DIR}

# Deps
cargo tree

# Check
cargo check  --verbose    --color always --all-targets --all-features

# Format
cargo fmt    --verbose -- --color always

# Lint
cargo clippy --verbose    --color always --all-targets --all-features

# Test
cargo xtest               --color always --all-targets --all-features
