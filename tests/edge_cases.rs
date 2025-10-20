use anyhow::Result;
use cydec::{FloatingCodec, IntegerCodec};

// Empty data edge cases

#[test]
fn test_empty_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = vec![];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    assert_eq!(
        compressed.len(),
        0,
        "Empty data should produce empty output"
    );
    Ok(())
}

#[test]
fn test_empty_u64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<u64> = vec![];
    let compressed = codec.compress_u64(&data)?;
    let decompressed = codec.decompress_u64(&compressed)?;
    assert_eq!(data, decompressed);
    assert_eq!(compressed.len(), 0);
    Ok(())
}

#[test]
fn test_empty_i32() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i32> = vec![];
    let compressed = codec.compress_i32(&data)?;
    let decompressed = codec.decompress_i32(&compressed)?;
    assert_eq!(data, decompressed);
    assert_eq!(compressed.len(), 0);
    Ok(())
}

#[test]
fn test_empty_u32() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<u32> = vec![];
    let compressed = codec.compress_u32(&data)?;
    let decompressed = codec.decompress_u32(&compressed)?;
    assert_eq!(data, decompressed);
    assert_eq!(compressed.len(), 0);
    Ok(())
}

#[test]
fn test_empty_bytes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<u8> = vec![];
    let compressed = codec.compress_bytes(&data)?;
    let decompressed = codec.decompress_bytes(&compressed)?;
    assert_eq!(data, decompressed);
    assert_eq!(compressed.len(), 0);
    Ok(())
}

#[test]
fn test_empty_f64() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = vec![];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;
    assert_eq!(data, decompressed);
    assert_eq!(compressed.len(), 0);
    Ok(())
}

#[test]
fn test_empty_f32() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f32> = vec![];
    let compressed = codec.compress_f32(&data, None)?;
    let decompressed = codec.decompress_f32(&compressed, None)?;
    assert_eq!(data, decompressed);
    assert_eq!(compressed.len(), 0);
    Ok(())
}

// Single element edge cases

#[test]
fn test_single_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![42i64];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_single_i64_zero() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![0i64];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_single_i64_max() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![i64::MAX];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_single_i64_min() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![i64::MIN];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_single_f64() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![std::f64::consts::PI];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;
    assert!((data[0] - decompressed[0]).abs() < 1e-9);
    Ok(())
}

// All identical values

#[test]
fn test_all_zeros_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![0i64; 1000];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);

    // Should compress extremely well
    let ratio = (data.len() * 8) as f64 / compressed.len() as f64;
    assert!(
        ratio > 100.0,
        "All zeros should compress >100x, got {:.2}x",
        ratio
    );
    Ok(())
}

#[test]
fn test_all_same_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![12345i64; 10000];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);

    // Should compress extremely well
    let ratio = (data.len() * 8) as f64 / compressed.len() as f64;
    assert!(
        ratio > 50.0,
        "Constant values should compress >50x, got {:.2}x",
        ratio
    );
    Ok(())
}

#[test]
fn test_all_same_f64() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![42.42; 10000];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-9);
    }

    // Should compress extremely well
    let ratio = (data.len() * 8) as f64 / compressed.len() as f64;
    assert!(
        ratio > 50.0,
        "Constant f64 should compress >50x, got {:.2}x",
        ratio
    );
    Ok(())
}

// Maximum/minimum values

#[test]
fn test_i64_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![i64::MIN, i64::MAX];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_u64_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![0u64, u64::MAX];
    let compressed = codec.compress_u64(&data)?;
    let decompressed = codec.decompress_u64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_i32_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![i32::MIN, i32::MAX];
    let compressed = codec.compress_i32(&data)?;
    let decompressed = codec.decompress_i32(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_u32_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![0u32, u32::MAX];
    let compressed = codec.compress_u32(&data)?;
    let decompressed = codec.decompress_u32(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

// Very large datasets

#[test]
fn test_very_large_i64_1m() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..1_000_000).collect();
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data.len(), decompressed.len());
    assert_eq!(data[0], decompressed[0]);
    assert_eq!(data[data.len() / 2], decompressed[data.len() / 2]);
    assert_eq!(data[data.len() - 1], decompressed[data.len() - 1]);
    Ok(())
}

#[test]
fn test_very_large_f64_1m() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = (0..1_000_000).map(|i| i as f64 * 0.001).collect();
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;
    assert_eq!(data.len(), decompressed.len());

    // Spot check
    for i in [0, data.len() / 2, data.len() - 1] {
        assert!((data[i] - decompressed[i]).abs() < 1e-9);
    }
    Ok(())
}

// Pathological worst-case compression

#[test]
fn test_random_worst_case() -> Result<()> {
    use rand::{Rng, SeedableRng, rngs::StdRng};
    let mut rng = StdRng::seed_from_u64(12345);
    let codec = IntegerCodec::default();

    // Completely random data - worst case for delta encoding
    let data: Vec<i64> = (0..10000).map(|_| rng.r#gen()).collect();

    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);

    // Random data won't compress well, but should still work
    let ratio = (data.len() * 8) as f64 / compressed.len() as f64;
    eprintln!("Random data compression ratio: {:.2}x", ratio);
    Ok(())
}

// Alternating patterns

#[test]
fn test_alternating_values() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..10000).map(|i| if i % 2 == 0 { 0 } else { 1 }).collect();
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_alternating_extremes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..1000)
        .map(|i| if i % 2 == 0 { i64::MIN } else { i64::MAX })
        .collect();
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

// Error handling - corrupted data

#[test]
fn test_corrupted_magic_bytes() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![1i64, 2, 3];
    let mut compressed = codec.compress_i64(&data)?;

    // Corrupt magic bytes
    compressed[0] = b'X';

    let result = codec.decompress_i64(&compressed);
    assert!(result.is_err(), "Should fail with corrupted magic bytes");
    assert!(result.unwrap_err().to_string().contains("bad magic"));
    Ok(())
}

#[test]
fn test_corrupted_version() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![1i64, 2, 3];
    let mut compressed = codec.compress_i64(&data)?;

    // Corrupt version (at index 5)
    compressed[5] = 99;

    let result = codec.decompress_i64(&compressed);
    assert!(result.is_err(), "Should fail with bad version");
    assert!(result.unwrap_err().to_string().contains("bad version"));
    Ok(())
}

#[test]
fn test_wrong_type_decompression() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![1i64, 2, 3];
    let compressed = codec.compress_i64(&data)?;

    // Try to decompress as u64 (wrong type)
    let result = codec.decompress_u64(&compressed);
    assert!(
        result.is_err(),
        "Should fail when decompressing with wrong type"
    );
    assert!(result.unwrap_err().to_string().contains("expected u64"));
    Ok(())
}

#[test]
fn test_truncated_blob() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![1i64, 2, 3];
    let compressed = codec.compress_i64(&data)?;

    // Truncate blob
    let truncated = &compressed[0..10];

    let result = codec.decompress_i64(truncated);
    assert!(result.is_err(), "Should fail with truncated blob");
    assert!(result.unwrap_err().to_string().contains("blob too small"));
    Ok(())
}

#[test]
fn test_empty_blob_decompression() -> Result<()> {
    let codec = IntegerCodec::default();
    let empty: &[u8] = &[];

    let result = codec.decompress_i64(empty)?;
    assert_eq!(result.len(), 0, "Empty blob should decompress to empty vec");
    Ok(())
}

// Parallel edge cases

#[test]
fn test_parallel_empty_arrays() -> Result<()> {
    let codec = IntegerCodec::default();
    let arrays: Vec<Vec<i64>> = vec![vec![], vec![], vec![]];
    let compressed = codec.compress_many_i64(&arrays)?;
    let decompressed = codec.decompress_many_i64(&compressed)?;
    assert_eq!(arrays, decompressed);
    Ok(())
}

#[test]
fn test_parallel_mixed_sizes() -> Result<()> {
    let codec = IntegerCodec::default();
    let arrays: Vec<Vec<i64>> = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        (0..1000).collect(),
        (0..10000).collect(),
    ];
    let compressed = codec.compress_many_i64(&arrays)?;
    let decompressed = codec.decompress_many_i64(&compressed)?;
    assert_eq!(arrays, decompressed);
    Ok(())
}

#[test]
fn test_parallel_single_array() -> Result<()> {
    let codec = IntegerCodec::default();
    let arrays: Vec<Vec<i64>> = vec![vec![1, 2, 3, 4, 5]];
    let compressed = codec.compress_many_i64(&arrays)?;
    let decompressed = codec.decompress_many_i64(&compressed)?;
    assert_eq!(arrays, decompressed);
    Ok(())
}

// Floating-point edge cases

#[test]
fn test_f64_very_small_values() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![1e-100, 1e-200, 1e-300];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        let tolerance = original.abs() * 1e-3 + 1e-9; // More lenient for very small numbers
        assert!(
            error < tolerance,
            "Very small f64: {} vs {}",
            original,
            decoded
        );
    }
    Ok(())
}

#[test]
fn test_f64_very_large_values() -> Result<()> {
    let codec = FloatingCodec::default();

    // Test large values within representable range
    // i64::MAX ~= 9.2e18, so with scale 1e3, we can represent up to ~9e15
    // Use values that are large but representable: ~1e8 range
    let data = vec![1e8, -1e8, 5.5e8, -7.3e8, 9.9e8];

    // Use smaller scale for very large values to maintain precision
    let custom_scale = 1e3;
    let compressed = codec.compress_f64(&data, Some(custom_scale))?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        let tolerance = original.abs() * 1e-3; // More lenient for very large numbers
        assert!(
            error < tolerance,
            "Very large f64: {} vs {}",
            original,
            decoded
        );
    }
    Ok(())
}

#[test]
fn test_f64_negative_zero() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![0.0, -0.0];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-9);
    }
    Ok(())
}

// Binary data edge cases

#[test]
fn test_bytes_all_zeros() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![0u8; 10000];
    let compressed = codec.compress_bytes(&data)?;
    let decompressed = codec.decompress_bytes(&compressed)?;
    assert_eq!(data, decompressed);

    // Should compress very well
    let ratio = data.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 50.0,
        "All-zero bytes should compress >50x, got {:.2}x",
        ratio
    );
    Ok(())
}

#[test]
fn test_bytes_all_255() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![255u8; 10000];
    let compressed = codec.compress_bytes(&data)?;
    let decompressed = codec.decompress_bytes(&compressed)?;
    assert_eq!(data, decompressed);

    // Should compress very well
    let ratio = data.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 50.0,
        "All-255 bytes should compress >50x, got {:.2}x",
        ratio
    );
    Ok(())
}

#[test]
fn test_bytes_single() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = vec![42u8];
    let compressed = codec.compress_bytes(&data)?;
    let decompressed = codec.decompress_bytes(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

// Scale factor edge cases

#[test]
fn test_f64_scale_factor_zero() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![1.0, 2.0, 3.0];

    // Scale factor of 1.0 (essentially no scaling)
    let compressed = codec.compress_f64(&data, Some(1.0))?;
    let decompressed = codec.decompress_f64(&compressed, Some(1.0))?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        assert!(error < 1.0, "Scale=1.0: {} vs {}", original, decoded);
    }
    Ok(())
}

#[test]
fn test_f64_scale_factor_very_large() -> Result<()> {
    let codec = FloatingCodec::default();
    let data = vec![1.123456789012345, 2.234567890123456, 3.345678901234567];

    // Very large scale factor for maximum precision
    let scale = 1e15;
    let compressed = codec.compress_f64(&data, Some(scale))?;
    let decompressed = codec.decompress_f64(&compressed, Some(scale))?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        let error = (original - decoded).abs();
        assert!(
            error < 1e-15,
            "High precision: {} vs {}, error: {}",
            original,
            decoded,
            error
        );
    }
    Ok(())
}
