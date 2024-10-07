#!/bin/bash
set -e

echo "Running Rust linter..."
cargo clippy -- -D warnings

echo "Running rustfmt..."
cargo fmt -- --check

echo "Checking for outdated dependencies..."
cargo outdated

echo "Linting complete!"