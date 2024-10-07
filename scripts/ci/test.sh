#!/bin/bash
set -e

echo "Running unit tests..."
cargo test --lib

echo "Running integration tests..."
cargo test --test '*'

echo "Running doc tests..."
cargo test --doc

echo "All tests completed successfully!"