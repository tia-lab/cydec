#!/bin/bash
set -e

echo "Running ALL benchmarks (combined)..."
cargo bench --all-benchmarks

echo ""
echo "Done! View results: open target/criterion/report/index.html"
