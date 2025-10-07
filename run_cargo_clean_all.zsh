#!/bin/zsh

# Find all subdirectories containing a Cargo.toml and run 'cargo clean' in each
for dir in **/Cargo.toml(.N:h); do
  echo "Running 'cargo clean' in $dir"
  (cd "$dir" && cargo clean)
done