use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use cydec::{FloatingCodec, IntegerCodec};

// Benchmark i64 compression for various sizes
fn bench_i64_compression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("i64_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<i64> = (0..size).collect();
        let compressed = codec.compress_i64(&data).unwrap();
        let ratio = (size * 8) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 8) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_i64(black_box(data)).unwrap());
        });

        println!("  i64_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark i64 decompression for various sizes
fn bench_i64_decompression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("i64_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<i64> = (0..size).collect();
        let compressed = codec.compress_i64(&data).unwrap();
        let ratio = (size * 8) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 8) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_i64(black_box(compressed)).unwrap());
            },
        );

        println!("  i64_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark u64 compression
fn bench_u64_compression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("u64_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<u64> = (0..size).collect();
        let compressed = codec.compress_u64(&data).unwrap();
        let ratio = (size * 8) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes(size * 8));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_u64(black_box(data)).unwrap());
        });

        println!("  u64_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark u64 decompression
fn bench_u64_decompression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("u64_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<u64> = (0..size).collect();
        let compressed = codec.compress_u64(&data).unwrap();
        let ratio = (size * 8) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes(size * 8));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_u64(black_box(compressed)).unwrap());
            },
        );

        println!("  u64_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark i32 compression
fn bench_i32_compression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("i32_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<i32> = (0..size).collect();
        let compressed = codec.compress_i32(&data).unwrap();
        let ratio = (size * 4) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 4) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_i32(black_box(data)).unwrap());
        });

        println!("  i32_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark i32 decompression
fn bench_i32_decompression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("i32_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<i32> = (0..size).collect();
        let compressed = codec.compress_i32(&data).unwrap();
        let ratio = (size * 4) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 4) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_i32(black_box(compressed)).unwrap());
            },
        );

        println!("  i32_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark u32 compression
fn bench_u32_compression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("u32_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<u32> = (0..size as u32).collect();
        let compressed = codec.compress_u32(&data).unwrap();
        let ratio = (size * 4) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 4) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_u32(black_box(data)).unwrap());
        });

        println!("  u32_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark u32 decompression
fn bench_u32_decompression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("u32_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<u32> = (0..size as u32).collect();
        let compressed = codec.compress_u32(&data).unwrap();
        let ratio = (size * 4) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 4) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_u32(black_box(compressed)).unwrap());
            },
        );

        println!("  u32_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark f64 compression
fn bench_f64_compression(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let mut group = c.benchmark_group("f64_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<f64> = (0..size).map(|i| i as f64 * 0.1).collect();
        let compressed = codec.compress_f64(&data, None).unwrap();
        let ratio = (size * 8) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 8) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_f64(black_box(data), None).unwrap());
        });

        println!("  f64_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark f64 decompression
fn bench_f64_decompression(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let mut group = c.benchmark_group("f64_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<f64> = (0..size).map(|i| i as f64 * 0.1).collect();
        let compressed = codec.compress_f64(&data, None).unwrap();
        let ratio = (size * 8) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 8) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_f64(black_box(compressed), None).unwrap());
            },
        );

        println!("  f64_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark f32 compression
fn bench_f32_compression(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let mut group = c.benchmark_group("f32_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<f32> = (0..size).map(|i| i as f32 * 0.1).collect();
        let compressed = codec.compress_f32(&data, None).unwrap();
        let ratio = (size * 4) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 4) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_f32(black_box(data), None).unwrap());
        });

        println!("  f32_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark f32 decompression
fn bench_f32_decompression(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let mut group = c.benchmark_group("f32_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<f32> = (0..size).map(|i| i as f32 * 0.1).collect();
        let compressed = codec.compress_f32(&data, None).unwrap();
        let ratio = (size * 4) as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes((size * 4) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_f32(black_box(compressed), None).unwrap());
            },
        );

        println!("  f32_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark bytes compression
fn bench_bytes_compression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("bytes_compression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let compressed = codec.compress_bytes(&data).unwrap();
        let ratio = size as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| codec.compress_bytes(black_box(data)).unwrap());
        });

        println!("  bytes_compression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark bytes decompression
fn bench_bytes_decompression(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let mut group = c.benchmark_group("bytes_decompression");

    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let compressed = codec.compress_bytes(&data).unwrap();
        let ratio = size as f64 / compressed.len() as f64;

        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &compressed,
            |b, compressed| {
                b.iter(|| codec.decompress_bytes(black_box(compressed)).unwrap());
            },
        );

        println!("  bytes_decompression/{}: ratio: {:.2}x", size, ratio);
    }

    group.finish();
}

// Benchmark compression ratios
fn bench_compression_ratios(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let group = c.benchmark_group("compression_ratios");

    let size = 10_000;

    // Sequential data (best case)
    let sequential: Vec<i64> = (0..size).collect();
    let seq_compressed = codec.compress_i64(&sequential).unwrap();
    let seq_ratio = (size * 8) as f64 / seq_compressed.len() as f64;

    // Constant data (excellent case)
    let constant = vec![42i64; size as usize];
    let const_compressed = codec.compress_i64(&constant).unwrap();
    let const_ratio = (size * 8) as f64 / const_compressed.len() as f64;

    // Random data (worst case)
    use rand::{Rng, SeedableRng, rngs::StdRng};
    let mut rng = StdRng::seed_from_u64(12345);
    let random: Vec<i64> = (0..size).map(|_| rng.r#gen()).collect();
    let rand_compressed = codec.compress_i64(&random).unwrap();
    let rand_ratio = (size * 8) as f64 / rand_compressed.len() as f64;

    println!("\nCompression Ratios (10,000 elements):");
    println!("  Sequential: {:.2}x", seq_ratio);
    println!("  Constant:   {:.2}x", const_ratio);
    println!("  Random:     {:.2}x", rand_ratio);

    group.finish();
}

criterion_group!(
    benches,
    bench_i64_compression,
    bench_i64_decompression,
    bench_u64_compression,
    bench_u64_decompression,
    bench_i32_compression,
    bench_i32_decompression,
    bench_u32_compression,
    bench_u32_decompression,
    bench_f64_compression,
    bench_f64_decompression,
    bench_f32_compression,
    bench_f32_decompression,
    bench_bytes_compression,
    bench_bytes_decompression,
    bench_compression_ratios,
);

criterion_main!(benches);
