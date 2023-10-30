#!/usr/bin/env sh

cargo +nightly fmt
cargo build --release
cargo doc --no-deps
