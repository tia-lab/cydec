# cydec

A straightforward compression library for numeric data in Rust, designed with database and time-series applications in mind.

## What it does

cydec provides efficient compression for numeric arrays (integers and floating-point numbers) using a combination of well-established techniques:

- **Delta encoding** - Stores differences between consecutive values instead of absolute values
- **Zigzag encoding** - Efficiently handles negative numbers in variable-length encoding
- **Variable-length integers** - Smaller values use fewer bytes
- **LZ4 compression** - Fast final compression step

The result is typically 2-10x compression ratios for sorted or time-series numeric data, with very fast compression and decompression speeds.

## When to use it

This library works best for:

- Time-series data where values change gradually
- Sorted or semi-sorted numeric arrays
- Database column storage
- Scientific data with regular patterns
- Any application where you need to compress large arrays of numbers

It may not be the best choice for:

- Random, unsorted data with no patterns
- Very small datasets (the header overhead isn't worth it)
- Applications requiring extreme compression ratios (consider zstd instead)

## Basic usage

```rust
use cydec::{IntegerCodec, FloatingCodec};
use anyhow::Result;

fn main() -> Result<()> {
    // Compress integers
    let codec = IntegerCodec::default();
    let data: Vec<i64> = vec![100, 102, 105, 110, 115, 120];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);

    // Compress floating-point numbers
    let float_codec = FloatingCodec::default();
    let float_data: Vec<f64> = vec![1.0, 1.1, 1.2, 1.3, 1.4];
    let compressed = float_codec.compress_f64(&float_data, None)?;
    let decompressed = float_codec.decompress_f64(&compressed, None)?;

    Ok(())
}
```

## Supported types

### Integer types

- `i64` / `u64` - 64-bit integers
- `i32` / `u32` - 32-bit integers
- `i16` / `u16` - 16-bit integers
- `i8` / `u8` - 8-bit integers
- Raw bytes - Generic byte arrays

### Floating-point types

- `f64` - 64-bit floats (9 decimal places precision by default)
- `f32` - 32-bit floats (6 decimal places precision by default)

You can adjust the precision/scale factor for floating-point compression based on your needs.

## Parallel processing

The library includes parallel compression variants for large datasets:

```rust
let codec = IntegerCodec::default();
let large_data: Vec<i64> = (0..1_000_000).collect();

// Compress in parallel chunks
let compressed = codec.par_compress_i64(&large_data, 10_000)?;
let decompressed = codec.par_decompress_i64(&compressed)?;
```

## How it works internally

1. **Delta encoding**: For a sequence [100, 102, 105, 110], we store [100, 2, 3, 5]
2. **Zigzag encoding**: Negative deltas are encoded to positive integers for efficient varint encoding
3. **Variable-length encoding**: Small numbers use fewer bytes (e.g., 127 uses 1 byte, 128 uses 2 bytes)
4. **LZ4 compression**: The final encoded bytes are compressed with LZ4 for additional space savings

The compressed format includes a small header (15-23 bytes) containing:

- Magic bytes ("CYDEC")
- Version number
- Codec type
- Data type identifier
- Original array length
- Scale factor (for floating-point types)

## Performance notes

- Compression speed: ~500-1000 MB/s on modern hardware
- Decompression speed: ~1000-2000 MB/s
- Memory usage: Approximately 2x the input size during compression
- Best compression ratios on sorted or time-series data

The library prioritizes speed over maximum compression. If you need better compression ratios and can accept slower speeds, consider using zstd or similar algorithms.

## Development status

This library is functional but still evolving. The API may change in future versions. Currently tested on:

- Linux x86_64
- macOS ARM64 and x86_64

Contributions, bug reports, and feature requests are welcome.

## License

Dual licensed under MIT OR Apache-2.0. Choose whichever license works best for your project.

## Acknowledgments

Built on top of excellent Rust crates:

- `integer-encoding` for variable-length integers
- `lz4_flex` for LZ4 compression
- `rayon` for parallel processing
- `anyhow` for error handling

The compression techniques used here are industry-standard approaches, not novel inventions. This library simply packages them in a convenient, Rust-native way for numeric data compression.

## Release Process

This project uses automated releases via [release-plz](https://release-plz.ieni.dev/).

### How it works

1. **Conventional Commits**: All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) format:
   - `feat:` - New features (triggers minor version bump)
   - `fix:` - Bug fixes (triggers patch version bump)
   - `docs:` - Documentation changes
   - `style:` - Code style changes
   - `refactor:` - Code refactoring
   - `perf:` - Performance improvements
   - `test:` - Test additions or updates
   - `build:` - Build system changes
   - `ci:` - CI/CD changes
   - `chore:` - Maintenance tasks

2. **Automatic Release PRs**: When changes are pushed to `main`, release-plz automatically:
   - Analyzes conventional commits since last release
   - Determines appropriate version bump (major/minor/patch)
   - Creates or updates a Release PR with:
     - Updated version in `Cargo.toml`
     - Generated CHANGELOG.md entries
     - All changes since last release

3. **Publishing**: When the Release PR is merged:
   - Creates a GitHub Release with tag (e.g., `v0.1.1`)
   - Publishes to crates.io automatically
   - Updates documentation

### Making a release

1. Write code and commit using conventional commit format
2. Push to `main` branch
3. Wait for release-plz to create/update the Release PR
4. Review the Release PR (check version bump and changelog)
5. Merge the Release PR
6. Automated workflow publishes to crates.io and GitHub Releases

### Git hooks

This project uses `cargo-husky` for Git hooks:

- **pre-commit**: Automatically formats code with `cargo fmt` and runs `cargo clippy`
- **commit-msg**: Validates conventional commit format

Install hooks by running any `cargo` command (they're installed automatically via `cargo-husky`).

### Requirements for maintainers

To enable automated publishing, repository secrets must be configured:

- `GITHUB_TOKEN` - Automatically provided by GitHub Actions
- `CARGO_REGISTRY_TOKEN` - Your crates.io API token ([get one here](https://crates.io/settings/tokens))
