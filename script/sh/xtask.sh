#!/usr/bin/env bash

set -ex

CURRENT_DIR=$(dirname $0)
KEYLAND_DIR=$(cd ${CURRENT_DIR}/../../ & pwd)

# Check
cargo check  --verbose    --color always

# Format
cargo fmt    --verbose -- --color always

# Lint
cargo clippy --verbose    --color always

# Test
cargo xtest               --color always
