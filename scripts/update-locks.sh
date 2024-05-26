#!/usr/bin/env sh
# Developing multiple flakes locally is kinda annoying: You need to update the lock file anytime you change one of its dependencies on disk.
# This just updates all the lock files for local dependencies in all of our 'subrepos'. Use it if you find things aren't refreshing properly.
set -ex

cd core/ && nix flake update auxlib && cd ..
cd python/ && nix flake update auxlib && cd ..
