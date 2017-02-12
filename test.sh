#!/bin/sh
# Version 0.1

cargo fmt -- --write-mode checkstyle | grep -v '<'
cargo outdated
cargo doc && cargo deadlinks

# Run clippy for maximum pedantry
cargo +nightly clippy

# TODO: https://users.rust-lang.org/t/howto-sanitize-your-rust-code/9378
#      (And use clippy as a compiler plugin so we can save a pass)

# Test under stable so we don't accidentally depend on nightly-only stuff
cargo test
