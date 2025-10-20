use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use cydec::{FloatingCodec, IntegerCodec};

// Heavy load: 10M elements
fn bench_heavy_i64_10m(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 10_000_000;
    let data: Vec<i64> = (0..size).collect();

    let mut group = c.benchmark_group("heavy_load");
    group.sample_size(10); // Reduce sample size for heavy loads
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("i64_compress_10M", |b| {
        b.iter(|| codec.compress_i64(black_box(&data)).unwrap());
    });

    let compressed = codec.compress_i64(&data).unwrap();

    group.bench_function("i64_decompress_10M", |b| {
        b.iter(|| codec.decompress_i64(black_box(&compressed)).unwrap());
    });

    println!("\n10M i64 elements:");
    println!("  Original size: {} MB", (size * 8) / 1_000_000);
    println!("  Compressed size: {} MB", compressed.len() / 1_000_000);
    println!(
        "  Compression ratio: {:.2}x",
        (size * 8) as f64 / compressed.len() as f64
    );

    group.finish();
}

// Heavy load: 100M elements (if memory allows)
#[cfg(not(debug_assertions))] // Only run in release mode
fn bench_heavy_i64_100m(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 100_000_000;
    let data: Vec<i64> = (0..size).collect();

    let mut group = c.benchmark_group("extreme_load");
    group.sample_size(10);
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("i64_compress_100M", |b| {
        b.iter(|| codec.compress_i64(black_box(&data)).unwrap());
    });

    let compressed = codec.compress_i64(&data).unwrap();

    group.bench_function("i64_decompress_100M", |b| {
        b.iter(|| codec.decompress_i64(black_box(&compressed)).unwrap());
    });

    println!("\n100M i64 elements:");
    println!("  Original size: {} MB", (size * 8) / 1_000_000);
    println!("  Compressed size: {} MB", compressed.len() / 1_000_000);
    println!(
        "  Compression ratio: {:.2}x",
        (size * 8) as f64 / compressed.len() as f64
    );

    group.finish();
}

// Sustained load: compress 1000 batches of 10K elements
fn bench_sustained_load(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let batch_size = 10_000;
    let num_batches = 1_000;

    let batches: Vec<Vec<i64>> = (0..num_batches)
        .map(|b| {
            (0..batch_size)
                .map(|i| (b * batch_size + i) as i64)
                .collect()
        })
        .collect();

    let mut group = c.benchmark_group("sustained_load");
    group.sample_size(10);
    group.throughput(Throughput::Bytes((num_batches * batch_size * 8) as u64));

    group.bench_function("i64_sustained_1000x10K", |b| {
        b.iter(|| {
            for batch in &batches {
                let _ = codec.compress_i64(black_box(batch)).unwrap();
            }
        });
    });

    group.finish();
}

// Parallel heavy load
fn bench_parallel_heavy_load(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let num_arrays = 100;
    let array_size = 100_000;

    let arrays: Vec<Vec<i64>> = (0..num_arrays)
        .map(|k| {
            (0..array_size)
                .map(|i| (k * array_size + i) as i64)
                .collect()
        })
        .collect();

    let mut group = c.benchmark_group("parallel_heavy");
    group.sample_size(10);
    group.throughput(Throughput::Bytes((num_arrays * array_size * 8) as u64));

    group.bench_function("parallel_compress_100x100K", |b| {
        b.iter(|| codec.compress_many_i64(black_box(&arrays)).unwrap());
    });

    let compressed = codec.compress_many_i64(&arrays).unwrap();

    group.bench_function("parallel_decompress_100x100K", |b| {
        b.iter(|| codec.decompress_many_i64(black_box(&compressed)).unwrap());
    });

    group.finish();
}

// Heavy f64 load
fn bench_heavy_f64_10m(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let size = 10_000_000;
    let data: Vec<f64> = (0..size).map(|i| i as f64 * 0.001).collect();

    let mut group = c.benchmark_group("heavy_f64");
    group.sample_size(10);
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("f64_compress_10M", |b| {
        b.iter(|| codec.compress_f64(black_box(&data), None).unwrap());
    });

    let compressed = codec.compress_f64(&data, None).unwrap();

    group.bench_function("f64_decompress_10M", |b| {
        b.iter(|| codec.decompress_f64(black_box(&compressed), None).unwrap());
    });

    println!("\n10M f64 elements:");
    println!("  Original size: {} MB", (size * 8) / 1_000_000);
    println!("  Compressed size: {} MB", compressed.len() / 1_000_000);
    println!(
        "  Compression ratio: {:.2}x",
        (size * 8) as f64 / compressed.len() as f64
    );

    group.finish();
}

// Memory usage stress test
fn bench_memory_stress(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 5_000_000;

    let mut group = c.benchmark_group("memory_stress");
    group.sample_size(10);

    // Test multiple simultaneous compressions
    group.bench_function("simultaneous_compress_5x5M", |b| {
        b.iter(|| {
            let data1: Vec<i64> = (0..size).collect();
            let data2: Vec<i64> = (0..size).map(|i| i + size).collect();
            let data3: Vec<i64> = (0..size).map(|i| i + size * 2).collect();
            let data4: Vec<i64> = (0..size).map(|i| i + size * 3).collect();
            let data5: Vec<i64> = (0..size).map(|i| i + size * 4).collect();

            let _ = codec.compress_i64(black_box(&data1)).unwrap();
            let _ = codec.compress_i64(black_box(&data2)).unwrap();
            let _ = codec.compress_i64(black_box(&data3)).unwrap();
            let _ = codec.compress_i64(black_box(&data4)).unwrap();
            let _ = codec.compress_i64(black_box(&data5)).unwrap();
        });
    });

    group.finish();
}

// Worst-case random data heavy load
fn bench_heavy_random(c: &mut Criterion) {
    use rand::{Rng, SeedableRng, rngs::StdRng};
    let mut rng = StdRng::seed_from_u64(12345);
    let codec = IntegerCodec::default();
    let size = 1_000_000;

    let random_data: Vec<i64> = (0..size).map(|_| rng.r#gen()).collect();

    let mut group = c.benchmark_group("heavy_random");
    group.sample_size(10);
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("random_compress_1M", |b| {
        b.iter(|| codec.compress_i64(black_box(&random_data)).unwrap());
    });

    let compressed = codec.compress_i64(&random_data).unwrap();

    group.bench_function("random_decompress_1M", |b| {
        b.iter(|| codec.decompress_i64(black_box(&compressed)).unwrap());
    });

    println!("\n1M random i64 elements (worst case):");
    println!("  Original size: {} MB", (size * 8) / 1_000_000);
    println!("  Compressed size: {} MB", compressed.len() / 1_000_000);
    println!(
        "  Compression ratio: {:.2}x",
        (size * 8) as f64 / compressed.len() as f64
    );

    group.finish();
}

// CPU utilization test
fn bench_cpu_utilization(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let num_arrays = 1000;
    let array_size = 10_000;

    let arrays: Vec<Vec<i64>> = (0..num_arrays)
        .map(|k| {
            (0..array_size)
                .map(|i| (k * array_size + i) as i64)
                .collect()
        })
        .collect();

    let mut group = c.benchmark_group("cpu_utilization");
    group.sample_size(10);

    group.bench_function("sequential_1000x10K", |b| {
        b.iter(|| {
            for array in &arrays {
                let _ = codec.compress_i64(black_box(array)).unwrap();
            }
        });
    });

    group.bench_function("parallel_1000x10K", |b| {
        b.iter(|| codec.compress_many_i64(black_box(&arrays)).unwrap());
    });

    group.finish();
}

// Throughput measurement
fn bench_throughput(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 10_000_000;
    let data: Vec<i64> = (0..size).collect();

    let mut group = c.benchmark_group("throughput");
    group.sample_size(10);
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress_throughput", |b| {
        b.iter(|| codec.compress_i64(black_box(&data)).unwrap());
    });

    let compressed = codec.compress_i64(&data).unwrap();

    group.bench_function("decompress_throughput", |b| {
        b.iter(|| codec.decompress_i64(black_box(&compressed)).unwrap());
    });

    // Calculate MB/s
    let iter_time = std::time::Instant::now();
    codec.compress_i64(&data).unwrap();
    let compress_time = iter_time.elapsed().as_secs_f64();
    let compress_mbps = ((size * 8) as f64 / 1_000_000.0) / compress_time;

    let iter_time = std::time::Instant::now();
    codec.decompress_i64(&compressed).unwrap();
    let decompress_time = iter_time.elapsed().as_secs_f64();
    let decompress_mbps = ((size * 8) as f64 / 1_000_000.0) / decompress_time;

    println!("\nThroughput (10M elements):");
    println!("  Compression:   {:.0} MB/s", compress_mbps);
    println!("  Decompression: {:.0} MB/s", decompress_mbps);

    group.finish();
}

#[cfg(not(debug_assertions))]
criterion_group!(
    benches,
    bench_heavy_i64_10m,
    bench_heavy_i64_100m,
    bench_sustained_load,
    bench_parallel_heavy_load,
    bench_heavy_f64_10m,
    bench_memory_stress,
    bench_heavy_random,
    bench_cpu_utilization,
    bench_throughput,
);

#[cfg(debug_assertions)]
criterion_group!(
    benches,
    bench_heavy_i64_10m,
    bench_sustained_load,
    bench_parallel_heavy_load,
    bench_heavy_f64_10m,
    bench_memory_stress,
    bench_heavy_random,
    bench_cpu_utilization,
    bench_throughput,
);

criterion_main!(benches);
