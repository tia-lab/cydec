//! High-performance compression library for numerical time series data.
//!
//! `cydec` provides specialized compression for time series data using:
//! - Delta encoding for temporal data
//! - Zigzag encoding for signed integers
//! - LZ4 compression for final output
//!
//! # Performance
//!
//! Achieves 2-4x compression ratios on typical time series data with:
//! - Sub-millisecond compression/decompression for 100K elements
//! - Zero-copy operations where possible
//! - Parallel processing support via Rayon
//!
//! # Example
//!
//! ```rust
//! use cydec::IntegerCodec;
//!
//! let codec = IntegerCodec::default();
//! let data: Vec<i64> = (0..10_000).collect();
//!
//! // Compress
//! let compressed = codec.compress_i64(&data).unwrap();
//!
//! // Decompress
//! let decompressed = codec.decompress_i64(&compressed).unwrap();
//! assert_eq!(data, decompressed);
//! ```
//!
//! # Supported Types
//!
//! - **Integers**: `i32`, `i64`, `u32`, `u64`
//! - **Floats**: `f32`, `f64` (with configurable precision)
//! - **Bytes**: Raw byte arrays

mod floating_codec;
mod integer_codec;

pub use floating_codec::FloatingCodec;
pub use integer_codec::IntegerCodec;
