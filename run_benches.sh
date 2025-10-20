#!/bin/bash
set -e

echo "================================"
echo "Running cydec Benchmark Suite"
echo "================================"
echo ""

# Build in release mode first
echo "Building in release mode..."
cargo build --release --benches
echo ""

# Run compression benchmarks
echo "================================"
echo "1. Compression Benchmarks"
echo "================================"
cargo bench --bench compression_benchmarks
echo ""

# Run heavy load benchmarks
echo "================================"
echo "2. Heavy Load Benchmarks"
echo "================================"
cargo bench --bench heavy_load
echo ""

# Run realistic data benchmarks
echo "================================"
echo "3. Realistic Data Benchmarks"
echo "================================"
cargo bench --bench realistic_data
echo ""

echo "================================"
echo "All Benchmarks Complete!"
echo "================================"
echo ""
echo "Results saved in: target/criterion/"
echo "View HTML reports: open target/criterion/report/index.html"
