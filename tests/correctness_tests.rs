use anyhow::Result;
use cydec::{FloatingCodec, IntegerCodec};

// Test determinism: same input produces same output

#[test]
fn test_i64_determinism() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..1000).collect();

    let compressed1 = codec.compress_i64(&data)?;
    let compressed2 = codec.compress_i64(&data)?;
    let compressed3 = codec.compress_i64(&data)?;

    assert_eq!(
        compressed1, compressed2,
        "Compression should be deterministic"
    );
    assert_eq!(
        compressed2, compressed3,
        "Compression should be deterministic"
    );
    Ok(())
}

#[test]
fn test_f64_determinism() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = (0..1000).map(|i| i as f64 * 0.1).collect();

    let compressed1 = codec.compress_f64(&data, None)?;
    let compressed2 = codec.compress_f64(&data, None)?;
    let compressed3 = codec.compress_f64(&data, None)?;

    assert_eq!(
        compressed1, compressed2,
        "Compression should be deterministic"
    );
    assert_eq!(
        compressed2, compressed3,
        "Compression should be deterministic"
    );
    Ok(())
}

// Test data integrity: no corruption

#[test]
fn test_i64_data_integrity_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![
        i64::MIN,
        i64::MIN + 1,
        -1000000,
        -1,
        0,
        1,
        1000000,
        i64::MAX - 1,
        i64::MAX,
    ];

    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;

    assert_eq!(
        data, decompressed,
        "Extreme values must be preserved exactly"
    );
    Ok(())
}

#[test]
fn test_u64_data_integrity_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![0, 1, 1000000, u64::MAX / 2, u64::MAX - 1, u64::MAX];

    let compressed = codec.compress_u64(&data)?;
    let decompressed = codec.decompress_u64(&compressed)?;

    assert_eq!(
        data, decompressed,
        "Extreme values must be preserved exactly"
    );
    Ok(())
}

// Test compression ratio expectations

#[test]
fn test_sequential_compression_ratio() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..10_000).collect();

    let compressed = codec.compress_i64(&data)?;

    let original_size = data.len() * 8;
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;

    assert!(
        ratio > 10.0,
        "Sequential data should compress > 10x, got {:.2}x",
        ratio
    );
    Ok(())
}

#[test]
fn test_constant_compression_ratio() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![42i64; 10_000];

    let compressed = codec.compress_i64(&data)?;

    let original_size = data.len() * 8;
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;

    assert!(
        ratio > 50.0,
        "Constant data should compress > 50x, got {:.2}x",
        ratio
    );
    Ok(())
}

#[test]
fn test_time_series_compression_ratio() -> Result<()> {
    let codec = IntegerCodec::default();

    // Simulate time-series: slowly changing values
    let mut data = Vec::new();
    let mut val = 1000i64;
    for _ in 0..10_000 {
        data.push(val);
        val += 1; // Small delta
    }

    let compressed = codec.compress_i64(&data)?;

    let original_size = data.len() * 8;
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;

    assert!(
        ratio > 10.0,
        "Time-series data should compress > 10x, got {:.2}x",
        ratio
    );
    Ok(())
}

// Test floating-point precision

#[test]
fn test_f64_precision_default_scale() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![
        1.123456789,
        2.234567890,
        3.345678901,
        4.456789012,
        5.567890123,
    ];

    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        assert!(
            error < 1e-9,
            "f64 precision error too large: {} vs {}, error: {}",
            original,
            decoded,
            error
        );
    }
    Ok(())
}

#[test]
fn test_f32_precision_default_scale() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![1.123456, 2.234567, 3.345678, 4.456789, 5.567_89];

    let compressed = codec.compress_f32(&data, None)?;
    let decompressed = codec.decompress_f32(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        assert!(
            error < 1e-6,
            "f32 precision error too large: {} vs {}, error: {}",
            original,
            decoded,
            error
        );
    }
    Ok(())
}

// Test known good outputs (regression tests)

#[test]
fn test_i64_known_output_format() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![100i64, 101, 102, 103, 104];

    let compressed = codec.compress_i64(&data)?;

    // Verify header
    assert_eq!(&compressed[0..5], b"CYDEC", "Magic bytes should be CYDEC");
    assert_eq!(compressed[5], 1, "Version should be 1");
    assert_eq!(compressed[6], 1, "Codec should be 1 (LZ4)");
    assert_eq!(compressed[7], 0, "Type should be 0 (i64)");

    // Verify length field
    let len = u64::from_le_bytes(compressed[8..16].try_into().unwrap());
    assert_eq!(len, 5, "Length should be 5");

    Ok(())
}

#[test]
fn test_f64_known_output_format() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let compressed = codec.compress_f64(&data, None)?;

    // Verify header
    assert_eq!(&compressed[0..5], b"CYDEC", "Magic bytes should be CYDEC");
    assert_eq!(compressed[5], 1, "Version should be 1");
    assert_eq!(compressed[6], 1, "Codec should be 1 (LZ4)");
    assert_eq!(compressed[7], 4, "Type should be 4 (f64)");

    // Verify length field
    let len = u64::from_le_bytes(compressed[8..16].try_into().unwrap());
    assert_eq!(len, 5, "Length should be 5");

    // Verify scale factor field
    let scale = f64::from_le_bytes(compressed[16..24].try_into().unwrap());
    assert_eq!(
        scale,
        FloatingCodec::DEFAULT_F64_SCALE,
        "Scale should be default"
    );

    Ok(())
}

// Test zigzag encoding correctness

#[test]
fn test_zigzag_encoding_coverage() -> Result<()> {
    let codec = IntegerCodec::default();

    // Test various patterns that stress zigzag encoding
    let test_cases = vec![
        vec![0, 1, -1, 2, -2, 3, -3],     // Alternating signs
        vec![-10, -9, -8, -7, -6],        // Negative sequence
        vec![10, 9, 8, 7, 6],             // Descending
        vec![-100, 0, 100, -100, 0, 100], // Oscillating
        vec![i64::MIN, 0, i64::MAX],      // Extreme jumps
    ];

    for data in test_cases {
        let compressed = codec.compress_i64(&data)?;
        let decompressed = codec.decompress_i64(&compressed)?;
        assert_eq!(
            data, decompressed,
            "Zigzag encoding failed for pattern: {:?}",
            data
        );
    }

    Ok(())
}

// Test parallel consistency

#[test]
fn test_parallel_sequential_consistency() -> Result<()> {
    let codec = IntegerCodec::default();

    let arrays: Vec<Vec<i64>> = (0..100)
        .map(|k| (0..1000).map(|i| i * k).collect())
        .collect();

    // Sequential compression
    let sequential: Vec<Vec<u8>> = arrays
        .iter()
        .map(|a| codec.compress_i64(a).unwrap())
        .collect();

    // Parallel compression
    let parallel = codec.compress_many_i64(&arrays)?;

    assert_eq!(
        sequential, parallel,
        "Parallel and sequential compression should produce identical results"
    );

    Ok(())
}

// Test scale factor preservation

#[test]
fn test_scale_factor_embedded_in_blob() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![1.1, 2.2, 3.3];
    let custom_scale = 12345.6789;

    let compressed = codec.compress_f64(&data, Some(custom_scale))?;

    // Decompress without providing scale (should use embedded scale)
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        let tolerance = 1.0 / custom_scale + 1e-15;
        assert!(
            error < tolerance,
            "Scale factor not properly embedded/extracted: {} vs {}, error: {}",
            original,
            decoded,
            error
        );
    }

    Ok(())
}

// Test compression doesn't lose data

#[test]
fn test_no_data_loss_large_dataset() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..100_000).map(|i| i * 7 + 13).collect();

    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;

    // Verify every single element
    assert_eq!(data.len(), decompressed.len(), "Length mismatch");
    for (i, (original, decoded)) in data.iter().zip(decompressed.iter()).enumerate() {
        assert_eq!(
            original, decoded,
            "Data loss at index {}: {} vs {}",
            i, original, decoded
        );
    }

    Ok(())
}

// Test compression of special float values

#[test]
fn test_f64_special_values() -> Result<()> {
    let codec = FloatingCodec::default();

    // Test zeros, very small, very large
    // Note: Use smaller scale (1e6) for very large values to avoid i64 overflow
    // since 1e10 * 1e9 (default scale) > i64::MAX
    let data = vec![
        0.0,
        -0.0,
        1e-10,
        -1e-10,
        1e10,
        -1e10,
        std::f64::consts::PI,
        std::f64::consts::E,
    ];

    // Use custom scale to avoid overflow on large values
    let custom_scale = 1e6;
    let compressed = codec.compress_f64(&data, Some(custom_scale))?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (i, (original, decoded)) in data.iter().zip(decompressed.iter()).enumerate() {
        let error = (original - decoded).abs();
        let tolerance = original.abs() * (1.0 / custom_scale) + (1.0 / custom_scale);
        assert!(
            error < tolerance,
            "Special value mismatch at index {}: {} vs {}, error: {}",
            i,
            original,
            decoded,
            error
        );
    }

    Ok(())
}

// Test byte compression preserves all byte values

#[test]
fn test_bytes_all_values() -> Result<()> {
    let codec = IntegerCodec::default();

    // Test all possible byte values
    let data: Vec<u8> = (0..=255).collect();

    let compressed = codec.compress_bytes(&data)?;
    let decompressed = codec.decompress_bytes(&compressed)?;

    assert_eq!(data, decompressed, "All byte values should be preserved");

    Ok(())
}

// Test large deltas

#[test]
fn test_i64_large_deltas() -> Result<()> {
    let codec = IntegerCodec::default();

    // Create data with very large deltas
    let data = vec![0, i64::MAX / 2, i64::MIN / 2, 0, i64::MAX, i64::MIN, 0];

    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;

    assert_eq!(
        data, decompressed,
        "Large deltas should be handled correctly"
    );

    Ok(())
}

// Test wrapping arithmetic correctness

#[test]
fn test_wrapping_arithmetic() -> Result<()> {
    let codec = IntegerCodec::default();

    // Test that wrapping is handled correctly
    let data = vec![
        i64::MAX - 5,
        i64::MAX - 4,
        i64::MAX - 3,
        i64::MAX - 2,
        i64::MAX - 1,
        i64::MAX,
    ];

    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;

    assert_eq!(data, decompressed, "Near-max values should wrap correctly");

    Ok(())
}
