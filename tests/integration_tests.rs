use anyhow::Result;
use cydec::{FloatingCodec, IntegerCodec};

// Integer types - comprehensive round-trip testing

#[test]
fn test_i64_roundtrip() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = vec![100, 102, 105, 110, 115, 120];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_u64_roundtrip() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<u64> = vec![1000, 1002, 1005, 1010, 1015, 1020];
    let compressed = codec.compress_u64(&data)?;
    let decompressed = codec.decompress_u64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_i32_roundtrip() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i32> = vec![100, 102, 105, 110, 115, 120];
    let compressed = codec.compress_i32(&data)?;
    let decompressed = codec.decompress_i32(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_u32_roundtrip() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<u32> = vec![1000, 1002, 1005, 1010, 1015, 1020];
    let compressed = codec.compress_u32(&data)?;
    let decompressed = codec.decompress_u32(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_bytes_roundtrip() -> Result<()> {
    let codec = IntegerCodec::default();
    let data = b"Hello, World! This is a comprehensive test of byte compression.".to_vec();
    let compressed = codec.compress_bytes(&data)?;
    let decompressed = codec.decompress_bytes(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

// Floating-point types - comprehensive round-trip testing

#[test]
fn test_f64_roundtrip() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = vec![1.0, 1.1, 1.2, 1.3, 1.4, 1.5];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-9, "f64 mismatch: {} vs {}", original, decoded);
    }
    Ok(())
}

#[test]
fn test_f32_roundtrip() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f32> = vec![1.0, 1.1, 1.2, 1.3, 1.4, 1.5];
    let compressed = codec.compress_f32(&data, None)?;
    let decompressed = codec.decompress_f32(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-6, "f32 mismatch: {} vs {}", original, decoded);
    }
    Ok(())
}

// Custom scale factor testing

#[test]
fn test_f64_custom_scale() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = vec![100.123456, 101.234567, 102.345678];
    let scale = Some(1_000_000.0); // 6 decimal places
    let compressed = codec.compress_f64(&data, scale)?;
    let decompressed = codec.decompress_f64(&compressed, scale)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-6, "f64 custom scale mismatch: {} vs {}", original, decoded);
    }
    Ok(())
}

#[test]
fn test_f32_custom_scale() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f32> = vec![100.123, 101.234, 102.345];
    let scale = Some(1_000.0); // 3 decimal places
    let compressed = codec.compress_f32(&data, scale)?;
    let decompressed = codec.decompress_f32(&compressed, scale)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-3, "f32 custom scale mismatch: {} vs {}", original, decoded);
    }
    Ok(())
}

// Parallel compression testing

#[test]
fn test_parallel_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let arrays: Vec<Vec<i64>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![10, 20, 30, 40, 50],
        vec![100, 200, 300, 400, 500],
    ];
    let compressed = codec.compress_many_i64(&arrays)?;
    let decompressed = codec.decompress_many_i64(&compressed)?;
    assert_eq!(arrays, decompressed);
    Ok(())
}

#[test]
fn test_parallel_u64() -> Result<()> {
    let codec = IntegerCodec::default();
    let arrays: Vec<Vec<u64>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![10, 20, 30, 40, 50],
        vec![100, 200, 300, 400, 500],
    ];
    let compressed = codec.compress_many_u64(&arrays)?;
    let decompressed = codec.decompress_many_u64(&compressed)?;
    assert_eq!(arrays, decompressed);
    Ok(())
}

#[test]
fn test_parallel_f64() -> Result<()> {
    let codec = FloatingCodec::default();
    let arrays: Vec<Vec<f64>> = vec![
        vec![1.1, 2.2, 3.3, 4.4, 5.5],
        vec![10.1, 20.2, 30.3, 40.4, 50.5],
        vec![100.1, 200.2, 300.3, 400.4, 500.5],
    ];
    let compressed = codec.compress_many_f64(&arrays, None)?;
    let decompressed = codec.decompress_many_f64(&compressed, None)?;

    for (original_array, decoded_array) in arrays.iter().zip(decompressed.iter()) {
        for (original, decoded) in original_array.iter().zip(decoded_array.iter()) {
            assert!((original - decoded).abs() < 1e-9, "parallel f64 mismatch: {} vs {}", original, decoded);
        }
    }
    Ok(())
}

#[test]
fn test_parallel_f32() -> Result<()> {
    let codec = FloatingCodec::default();
    let arrays: Vec<Vec<f32>> = vec![
        vec![1.1, 2.2, 3.3, 4.4, 5.5],
        vec![10.1, 20.2, 30.3, 40.4, 50.5],
        vec![100.1, 200.2, 300.3, 400.4, 500.5],
    ];
    let compressed = codec.compress_many_f32(&arrays, None)?;
    let decompressed = codec.decompress_many_f32(&compressed, None)?;

    for (original_array, decoded_array) in arrays.iter().zip(decompressed.iter()) {
        for (original, decoded) in original_array.iter().zip(decoded_array.iter()) {
            assert!((original - decoded).abs() < 1e-6, "parallel f32 mismatch: {} vs {}", original, decoded);
        }
    }
    Ok(())
}

// Large data testing

#[test]
fn test_large_i64_array() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = (0..100_000).map(|i| i * 2).collect();
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_large_f64_array() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = (0..100_000).map(|i| i as f64 * 0.1).collect();
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-9, "large f64 mismatch: {} vs {}", original, decoded);
    }
    Ok(())
}

// Negative numbers testing

#[test]
fn test_negative_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = vec![-100, -50, 0, 50, 100];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_negative_i32() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i32> = vec![-1000, -500, 0, 500, 1000];
    let compressed = codec.compress_i32(&data)?;
    let decompressed = codec.decompress_i32(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

#[test]
fn test_negative_f64() -> Result<()> {
    let codec = FloatingCodec::default();
    let data: Vec<f64> = vec![-100.5, -50.25, 0.0, 50.75, 100.123];
    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-9, "negative f64 mismatch: {} vs {}", original, decoded);
    }
    Ok(())
}

// Mixed positive/negative testing

#[test]
fn test_mixed_signs_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    let data: Vec<i64> = vec![
        i64::MIN,
        -1_000_000,
        -100,
        -1,
        0,
        1,
        100,
        1_000_000,
        i64::MAX,
    ];
    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);
    Ok(())
}

// Time-series like data (common use case)

#[test]
fn test_time_series_i64() -> Result<()> {
    let codec = IntegerCodec::default();
    // Simulates time-series data with gradual changes
    let mut data = Vec::new();
    let mut value = 1000i64;
    for _ in 0..1000 {
        data.push(value);
        value += (value / 100).max(1); // ~1% increase each time
    }

    let compressed = codec.compress_i64(&data)?;
    let decompressed = codec.decompress_i64(&compressed)?;
    assert_eq!(data, decompressed);

    // Verify compression is effective for time-series
    let original_size = data.len() * 8;
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;
    assert!(ratio > 2.0, "Expected compression ratio > 2.0x for time-series, got {:.2}x", ratio);
    Ok(())
}

#[test]
fn test_time_series_f64() -> Result<()> {
    let codec = FloatingCodec::default();
    // Simulates price data
    let mut data = Vec::new();
    let mut price = 100.0;
    for i in 0..1000 {
        data.push(price);
        price += (i as f64 * 0.01).sin() * 0.1; // Small oscillations
    }

    let compressed = codec.compress_f64(&data, None)?;
    let decompressed = codec.decompress_f64(&compressed, None)?;

    for (original, decoded) in data.iter().zip(decompressed.iter()) {
        assert!((original - decoded).abs() < 1e-9, "time-series f64 mismatch: {} vs {}", original, decoded);
    }

    // Verify compression is effective
    let original_size = data.len() * 8;
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;
    assert!(ratio > 1.8, "Expected compression ratio > 1.8x for time-series, got {:.2}x", ratio);
    Ok(())
}
