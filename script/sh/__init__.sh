#!/usr/bin/env bash

set -ex

CURRENT_DIR=$(dirname $0)


function fn_init()
{
    # pipenv intall

    cargo install cargo-insta
    cargo install cargo-nextest --locked
}

