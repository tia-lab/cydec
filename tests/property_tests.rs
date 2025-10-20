use anyhow::Result;
use cydec::{FloatingCodec, IntegerCodec};
use proptest::prelude::*;

// Property: Compression and decompression should be identity functions

proptest! {
    #[test]
    fn prop_i64_identity(data in prop::collection::vec(any::<i64>(), 0..1000)) {
        let codec = IntegerCodec::default();
        let compressed = codec.compress_i64(&data).unwrap();
        let decompressed = codec.decompress_i64(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_u64_identity(data in prop::collection::vec(any::<u64>(), 0..1000)) {
        let codec = IntegerCodec::default();
        let compressed = codec.compress_u64(&data).unwrap();
        let decompressed = codec.decompress_u64(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_i32_identity(data in prop::collection::vec(any::<i32>(), 0..1000)) {
        let codec = IntegerCodec::default();
        let compressed = codec.compress_i32(&data).unwrap();
        let decompressed = codec.decompress_i32(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_u32_identity(data in prop::collection::vec(any::<u32>(), 0..1000)) {
        let codec = IntegerCodec::default();
        let compressed = codec.compress_u32(&data).unwrap();
        let decompressed = codec.decompress_u32(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_bytes_identity(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let codec = IntegerCodec::default();
        let compressed = codec.compress_bytes(&data).unwrap();
        let decompressed = codec.decompress_bytes(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
    }
}

// Property: f64 identity with tolerance
proptest! {
    #[test]
    fn prop_f64_identity_with_tolerance(
        data in prop::collection::vec(-1e8f64..1e8f64, 0..1000)
    ) {
        let codec = FloatingCodec::default();
        let compressed = codec.compress_f64(&data, None).unwrap();
        let decompressed = codec.decompress_f64(&compressed, None).unwrap();

        prop_assert_eq!(data.len(), decompressed.len());
        for (original, decoded) in data.iter().zip(decompressed.iter()) {
            let diff = (original - decoded).abs();
            let tolerance = original.abs() * 1e-9 + 1e-9; // Relative + absolute tolerance
            prop_assert!(diff < tolerance, "f64 mismatch: {} vs {}, diff: {}", original, decoded, diff);
        }
    }

    #[test]
    fn prop_f32_identity_with_tolerance(
        data in prop::collection::vec(-2000.0f32..2000.0f32, 0..1000)
    ) {
        let codec = FloatingCodec::default();
        let compressed = codec.compress_f32(&data, None).unwrap();
        let decompressed = codec.decompress_f32(&compressed, None).unwrap();

        prop_assert_eq!(data.len(), decompressed.len());
        for (original, decoded) in data.iter().zip(decompressed.iter()) {
            let diff = (original - decoded).abs();
            let tolerance = original.abs() * 1e-6 + 1e-6; // Relative + absolute tolerance
            prop_assert!(diff < tolerance, "f32 mismatch: {} vs {}, diff: {}", original, decoded, diff);
        }
    }
}

// Property: Sorted data should compress better than random data
proptest! {
    #[test]
    fn prop_sorted_compresses_better(
        mut data in prop::collection::vec(0i64..1_000_000, 100..1000)
    ) {
        let codec = IntegerCodec::default();

        // Compress random data
        let random_compressed = codec.compress_i64(&data).unwrap();
        let random_size = random_compressed.len();

        // Sort and compress
        data.sort_unstable();
        let sorted_compressed = codec.compress_i64(&data).unwrap();
        let sorted_size = sorted_compressed.len();

        // Sorted data should compress better (or at least as well)
        prop_assert!(
            sorted_size <= random_size,
            "Sorted ({} bytes) should compress better than random ({} bytes)",
            sorted_size,
            random_size
        );
    }
}

// Property: Sequential data should have excellent compression
proptest! {
    #[test]
    fn prop_sequential_excellent_compression(
        start in any::<i64>(),
        len in 100usize..1000
    ) {
        let codec = IntegerCodec::default();
        let data: Vec<i64> = (0..len).map(|i| start.wrapping_add(i as i64)).collect();

        let compressed = codec.compress_i64(&data).unwrap();
        let decompressed = codec.decompress_i64(&compressed).unwrap();

        prop_assert_eq!(data.clone(), decompressed);

        // Sequential data should compress to less than 10% of original size
        let original_size = data.len() * 8;
        let compressed_size = compressed.len();
        let ratio = original_size as f64 / compressed_size as f64;

        prop_assert!(
            ratio > 10.0,
            "Sequential data should have compression ratio > 10x, got {:.2}x",
            ratio
        );
    }
}

// Property: Empty input produces empty output
proptest! {
    #[test]
    fn prop_empty_i64_produces_empty(codec_type in any::<u8>()) {
        let _ = codec_type; // Unused, just for variation
        let codec = IntegerCodec::default();
        let data: Vec<i64> = vec![];
        let compressed = codec.compress_i64(&data).unwrap();
        let decompressed = codec.decompress_i64(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
        prop_assert_eq!(compressed.len(), 0);
    }

    #[test]
    fn prop_empty_f64_produces_empty(codec_type in any::<u8>()) {
        let _ = codec_type;
        let codec = FloatingCodec::default();
        let data: Vec<f64> = vec![];
        let compressed = codec.compress_f64(&data, None).unwrap();
        let decompressed = codec.decompress_f64(&compressed, None).unwrap();
        prop_assert_eq!(data, decompressed);
        prop_assert_eq!(compressed.len(), 0);
    }
}

// Property: Single element should round-trip correctly
proptest! {
    #[test]
    fn prop_single_i64_roundtrip(value in any::<i64>()) {
        let codec = IntegerCodec::default();
        let data = vec![value];
        let compressed = codec.compress_i64(&data).unwrap();
        let decompressed = codec.decompress_i64(&compressed).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_single_f64_roundtrip(value in -1e8f64..1e8f64) {
        let codec = FloatingCodec::default();
        let data = vec![value];
        let compressed = codec.compress_f64(&data, None).unwrap();
        let decompressed = codec.decompress_f64(&compressed, None).unwrap();

        prop_assert_eq!(data.len(), decompressed.len());
        let diff = (data[0] - decompressed[0]).abs();
        let tolerance = value.abs() * 1e-9 + 1e-9;
        prop_assert!(diff < tolerance, "Single f64 mismatch: {} vs {}", value, decompressed[0]);
    }
}

// Property: Constant arrays should compress extremely well
proptest! {
    #[test]
    fn prop_constant_array_compression(
        value in any::<i64>(),
        len in 100usize..1000
    ) {
        let codec = IntegerCodec::default();
        let data = vec![value; len];

        let compressed = codec.compress_i64(&data).unwrap();
        let decompressed = codec.decompress_i64(&compressed).unwrap();

        prop_assert_eq!(data.clone(), decompressed);

        // Constant arrays should compress to less than 5% of original size
        let original_size = data.len() * 8;
        let compressed_size = compressed.len();
        let ratio = original_size as f64 / compressed_size as f64;

        prop_assert!(
            ratio > 19.0,
            "Constant array should have compression ratio > 19x, got {:.2}x",
            ratio
        );
    }
}

// Property: Parallel compression should produce same results as sequential
proptest! {
    #[test]
    fn prop_parallel_matches_sequential_i64(
        arrays in prop::collection::vec(
            prop::collection::vec(any::<i64>(), 10..100),
            2..10
        )
    ) {
        let codec = IntegerCodec::default();

        // Sequential compression
        let sequential: Result<Vec<Vec<u8>>> = arrays.iter()
            .map(|a| codec.compress_i64(a))
            .collect();
        let sequential = sequential.unwrap();

        // Parallel compression
        let parallel = codec.compress_many_i64(&arrays).unwrap();

        prop_assert_eq!(sequential.len(), parallel.len());

        // Results should be identical
        for (seq, par) in sequential.iter().zip(parallel.iter()) {
            prop_assert_eq!(seq, par);
        }
    }
}

// Property: Compression should never panic (error handling test)
proptest! {
    #[test]
    fn prop_compression_never_panics_i64(data in prop::collection::vec(any::<i64>(), 0..10000)) {
        let codec = IntegerCodec::default();
        let result = std::panic::catch_unwind(|| {
            codec.compress_i64(&data)
        });
        prop_assert!(result.is_ok(), "Compression panicked!");
    }

    #[test]
    fn prop_decompression_never_panics_on_valid_data(data in prop::collection::vec(any::<i64>(), 0..1000)) {
        let codec = IntegerCodec::default();
        if let Ok(compressed) = codec.compress_i64(&data) {
            let result = std::panic::catch_unwind(|| {
                codec.decompress_i64(&compressed)
            });
            prop_assert!(result.is_ok(), "Decompression panicked on valid data!");
        }
    }
}

// Property: Different scale factors should preserve data differently
proptest! {
    #[test]
    fn prop_f64_scale_affects_precision(
        data in prop::collection::vec(0f64..1000f64, 10..100),
        scale_exp in 0u32..9
    ) {
        let codec = FloatingCodec::default();
        let scale = 10f64.powi(scale_exp as i32);

        let compressed = codec.compress_f64(&data, Some(scale)).unwrap();
        let decompressed = codec.decompress_f64(&compressed, Some(scale)).unwrap();

        prop_assert_eq!(data.len(), decompressed.len());

        // Tolerance should scale with the scale factor
        let expected_precision = 1.0 / scale;
        for (original, decoded) in data.iter().zip(decompressed.iter()) {
            let diff = (original - decoded).abs();
            prop_assert!(
                diff <= expected_precision,
                "Precision error too large: {} vs {}, diff: {}, expected precision: {}",
                original, decoded, diff, expected_precision
            );
        }
    }
}
