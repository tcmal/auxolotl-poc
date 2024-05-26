#!/usr/bin/env sh
# Runs the resolver, updating `deps.nix` and `deps.svg`
# You can also do this less reproducibly by installing graphviz and using `cargo run --manifest-path resolver/Cargo.toml`
set -ex

./scripts/update-locks.sh
nix run ./resolver#
