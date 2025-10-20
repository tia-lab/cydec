# Testing Guide for cydec

This document describes how to run and interpret tests and benchmarks for the cydec compression library.

## Overview

The cydec test suite is comprehensive and organized into several categories:

- **Integration Tests**: Test public API functionality
- **Property Tests**: Property-based testing using `proptest`
- **Correctness Tests**: Mathematical correctness validation
- **Edge Case Tests**: Boundary conditions and error handling
- **Benchmarks**: Performance measurements for various workloads

## Running Tests

### Run All Tests

```bash
cargo test --all
```

### Run Specific Test Suites

```bash
# Integration tests only
cargo test --test integration_tests

# Property-based tests only
cargo test --test property_tests

# Correctness tests only
cargo test --test correctness_tests

# Edge case tests only
cargo test --test edge_cases
```

### Run Tests with Output

```bash
# Show println! output
cargo test -- --nocapture

# Show test names as they run
cargo test -- --nocapture --test-threads=1
```

### Run Tests in Release Mode

```bash
# Faster execution for large data tests
cargo test --release
```

## Test Suite Details

### Integration Tests (`tests/integration_tests.rs`)

Tests all public APIs with real-world usage patterns:

- Round-trip compression/decompression for all data types
- Custom scale factor handling for floating-point types
- Parallel compression operations
- Large dataset handling (100K+ elements)
- Negative number handling
- Time-series data patterns

**Expected Results**: All tests should pass with ZERO failures.

### Property Tests (`tests/property_tests.rs`)

Uses property-based testing to verify invariants:

- Compression → decompression is identity function
- Sorted data compresses better than random data
- Sequential data achieves >10x compression ratio
- Constant arrays achieve >20x compression ratio
- Empty input produces empty output
- Parallel and sequential compression produce identical results
- Scale factors affect precision as expected

**Expected Results**: All property tests should pass across thousands of randomly generated inputs.

### Correctness Tests (`tests/correctness_tests.rs`)

Validates mathematical correctness:

- Deterministic compression (same input → same output)
- Data integrity for extreme values (MIN/MAX)
- Compression ratio guarantees for known patterns
- Floating-point precision requirements
- Zigzag encoding correctness
- Scale factor preservation
- No data loss for large datasets

**Expected Results**: All correctness tests should pass, validating algorithmic soundness.

### Edge Case Tests (`tests/edge_cases.rs`)

Tests boundary conditions and error handling:

- Empty arrays
- Single-element arrays
- All identical values
- Maximum/minimum values
- Very large datasets (1M+ elements)
- Pathological worst-case compression
- Corrupted data handling
- Wrong type decompression
- Truncated blobs
- Parallel edge cases

**Expected Results**: All edge case tests should pass, including proper error handling for invalid inputs.

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Benchmark Suites

```bash
# Basic compression benchmarks
cargo bench --bench compression_benchmarks

# Heavy load stress tests
cargo bench --bench heavy_load

# Realistic data patterns
cargo bench --bench realistic_data
```

### Benchmark Output

Benchmarks generate HTML reports in `target/criterion/`. Open these files in a browser to see detailed performance graphs and statistics.

## Benchmark Suite Details

### Basic Benchmarks (`benches/compression_benchmarks.rs`)

Measures fundamental performance across data types and sizes:

- **Data Types**: i64, u64, i32, u32, f64, f32, bytes
- **Operations**: Compression and decompression
- **Sizes**: 100, 1K, 10K, 100K, 1M elements
- **Metrics**: Throughput (MB/s), latency, compression ratios

**Performance Targets**:
- Compression: 500-1000 MB/s
- Decompression: 1000-2000 MB/s
- Sequential data: >10x compression ratio
- Constant data: >50x compression ratio

### Heavy Load Benchmarks (`benches/heavy_load.rs`)

Stress tests with extreme workloads:

- 10M element compression/decompression
- 100M element compression (release mode only)
- Sustained load: 1000 batches of 10K elements
- Parallel compression: 100 arrays of 100K elements
- Memory stress: 5 simultaneous 5M compressions
- Random data worst-case
- CPU utilization comparison (sequential vs parallel)
- Throughput measurements

**Performance Expectations**:
- Should handle 10M+ elements without crashes
- Memory usage ~2x input size during compression
- Parallel compression should be faster than sequential for multiple arrays

### Realistic Data Benchmarks (`benches/realistic_data.rs`)

Real-world data patterns and use cases:

- **Stock Prices**: Gradual price movements with volatility
- **Sensor Readings**: IoT temperature data with diurnal patterns
- **Timestamps**: Monotonically increasing Unix timestamps
- **Sparse Data**: 95% zeros with occasional spikes
- **Database IDs**: Sequential IDs with gaps
- **Regime Indicators**: Financial analysis scores (-1 to 1)
- **Multi-Timeframe**: 1h, 4h, 12h, 1d data
- **Log Data**: Structured patterns with repetition
- **Network Metrics**: Cumulative byte counts

**Compression Ratio Expectations**:
- Time-series data: 5-15x
- Timestamps: 50-100x
- Sparse data: 20-50x
- Database IDs: 20-40x
- Regime indicators: 3-8x
- Log levels: 30-60x

## Interpreting Results

### Test Results

```bash
test result: ok. 150 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- **passed**: Number of successful tests (should be 150+)
- **failed**: Number of failed tests (MUST be 0)
- **ignored**: Tests skipped (should be 0)

### Benchmark Results

Example output:
```
i64_compression/1000    time:   [12.345 µs 12.567 µs 12.789 µs]
                        thrpt:  [600.12 MB/s 610.34 MB/s 620.56 MB/s]
```

- **time**: Median execution time with confidence interval
- **thrpt**: Throughput in MB/s (higher is better)
- **change**: Comparison to previous benchmark run

### Compression Ratios

Printed during benchmarks:
```
Sequential data (10,000 elements):
  Original size: 80000 B
  Compressed size: 1234 B
  Compression ratio: 64.82x
```

## Performance Regression Detection

Criterion automatically detects performance regressions. If you see:

```
Performance has regressed.
```

Investigate recent code changes that might have slowed down compression/decompression.

## Test Coverage

To generate test coverage report (requires `cargo-tarpaulin`):

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

Open `coverage/index.html` to view detailed coverage information.

## Continuous Integration

All tests and benchmarks should pass in CI before merging code. The CI pipeline runs:

1. `cargo test --all` - All test suites
2. `cargo test --release` - Release mode tests
3. `cargo bench --no-run` - Verify benchmarks compile

## Troubleshooting

### Tests Taking Too Long

```bash
# Run only fast tests
cargo test --lib

# Run with fewer threads
cargo test -- --test-threads=4
```

### Benchmarks Failing to Compile

```bash
# Check benchmark dependencies
cargo bench --no-run
```

### Out of Memory Errors

```bash
# Skip 100M element tests
cargo test --release -- --skip 100m
```

### Flaky Tests

All tests should be deterministic. If you see flaky tests, investigate:
- Random number generator seeds
- Floating-point precision tolerances
- Race conditions in parallel tests

## Adding New Tests

### Adding Integration Tests

Add test functions to `tests/integration_tests.rs`:

```rust
#[test]
fn test_my_feature() -> Result<()> {
    let codec = IntegerCodec::default();
    // Your test code
    Ok(())
}
```

### Adding Property Tests

Add to `tests/property_tests.rs`:

```rust
proptest! {
    #[test]
    fn prop_my_invariant(data in prop::collection::vec(any::<i64>(), 0..1000)) {
        // Your property test
    }
}
```

### Adding Benchmarks

Add to appropriate benchmark file:

```rust
fn bench_my_feature(c: &mut Criterion) {
    c.bench_function("my_feature", |b| {
        b.iter(|| {
            // Your benchmark code
        });
    });
}
```

## Performance Baselines

Expected performance on modern hardware (2020+ CPU):

| Operation | Size | Throughput | Compression Ratio |
|-----------|------|------------|-------------------|
| i64 compress (sequential) | 1M | 800+ MB/s | 50-100x |
| i64 decompress | 1M | 1500+ MB/s | - |
| f64 compress (time-series) | 1M | 600+ MB/s | 5-15x |
| f64 decompress | 1M | 1200+ MB/s | - |
| Random data compress | 1M | 400+ MB/s | 0.8-1.2x |

## Known Limitations

- Random data won't compress well (ratio ~1.0x)
- Very small datasets (< 100 elements) have header overhead
- Floating-point precision depends on scale factor
- Maximum single array size limited by available memory

## Support

For issues with tests or benchmarks:
1. Check this document first
2. Run `cargo test -- --nocapture` for detailed output
3. Run `cargo bench` to verify performance
4. File an issue with test output and system information
