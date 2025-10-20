use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use cydec::{FloatingCodec, IntegerCodec};

// Time-series data: stock prices
fn bench_stock_prices(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let size = 100_000; // ~100K price points (years of minute data)

    // Simulate realistic stock price movement
    let mut prices = Vec::with_capacity(size);
    let mut price = 100.0;
    for i in 0..size {
        let t = i as f64;
        // Add trend, daily cycle, and noise
        let trend = 0.0001 * t; // Slow upward trend
        let daily = (t / 390.0 * std::f64::consts::PI * 2.0).sin() * 0.5; // Daily cycle
        let noise = ((t * 0.1).sin() * (t * 0.03).cos()) * 0.1; // Market noise
        price += trend + daily + noise;
        prices.push(price);
    }

    let mut group = c.benchmark_group("realistic_stock_prices");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_f64(black_box(&prices), None).unwrap()
        });
    });

    let compressed = codec.compress_f64(&prices, None).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_f64(black_box(&compressed), None).unwrap()
        });
    });

    println!("\nStock prices (100K points):");
    println!("  Original size: {} KB", (size * 8) / 1000);
    println!("  Compressed size: {} KB", compressed.len() / 1000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

// Time-series data: sensor readings (IoT)
fn bench_sensor_readings(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let size = 1_000_000; // 1M sensor readings

    // Simulate temperature sensor with gradual changes
    let mut readings = Vec::with_capacity(size);
    for i in 0..size {
        let t = i as f64;
        // Diurnal variation + slow drift + sensor noise
        let diurnal = (t / 3600.0 * std::f64::consts::PI * 2.0).sin() * 5.0; // ±5°C daily
        let drift = (t / 86400.0).sin() * 0.5; // Slow drift
        let noise = ((t * 0.1).sin() * 137.0) % 0.1; // Sensor noise ±0.1°C
        let temp = 20.0 + diurnal + drift + noise;
        readings.push(temp);
    }

    let mut group = c.benchmark_group("realistic_sensor_data");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_f64(black_box(&readings), None).unwrap()
        });
    });

    let compressed = codec.compress_f64(&readings, None).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_f64(black_box(&compressed), None).unwrap()
        });
    });

    println!("\nSensor readings (1M points):");
    println!("  Original size: {} MB", (size * 8) / 1_000_000);
    println!("  Compressed size: {} MB", compressed.len() / 1_000_000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

// Time-series: monotonically increasing timestamps
fn bench_timestamps(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 1_000_000;

    // Unix timestamps in milliseconds, incrementing by ~1 second
    let base_time = 1_600_000_000_000i64;
    let timestamps: Vec<i64> = (0..size).map(|i| base_time + (i as i64 * 1000)).collect();

    let mut group = c.benchmark_group("realistic_timestamps");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_i64(black_box(&timestamps)).unwrap()
        });
    });

    let compressed = codec.compress_i64(&timestamps).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_i64(black_box(&compressed)).unwrap()
        });
    });

    println!("\nTimestamps (1M points):");
    println!("  Original size: {} MB", (size * 8) / 1_000_000);
    println!("  Compressed size: {} KB", compressed.len() / 1000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

// Sparse data: mostly zeros with occasional spikes
fn bench_sparse_data(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 100_000;

    // 95% zeros, 5% non-zero values
    let mut data = vec![0i64; size];
    for i in (0..size).step_by(20) {
        data[i] = (i as i64 * 13) % 1000; // Occasional spike
    }

    let mut group = c.benchmark_group("realistic_sparse");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_i64(black_box(&data)).unwrap()
        });
    });

    let compressed = codec.compress_i64(&data).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_i64(black_box(&compressed)).unwrap()
        });
    });

    println!("\nSparse data (100K points, 95% zeros):");
    println!("  Original size: {} KB", (size * 8) / 1000);
    println!("  Compressed size: {} KB", compressed.len() / 1000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

// Database IDs: sequential with gaps
fn bench_database_ids(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 100_000;

    // Simulate database IDs: mostly sequential with some gaps (deleted records)
    let mut ids = Vec::with_capacity(size);
    let mut id = 1000i64;
    for i in 0..size {
        ids.push(id);
        // Occasionally skip an ID (deleted record)
        if i % 100 == 0 {
            id += (i % 10) as i64; // Gap
        }
        id += 1;
    }

    let mut group = c.benchmark_group("realistic_db_ids");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_i64(black_box(&ids)).unwrap()
        });
    });

    let compressed = codec.compress_i64(&ids).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_i64(black_box(&compressed)).unwrap()
        });
    });

    println!("\nDatabase IDs (100K IDs with gaps):");
    println!("  Original size: {} KB", (size * 8) / 1000);
    println!("  Compressed size: {} KB", compressed.len() / 1000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

// Financial data: regime indicators (the actual use case!)
fn bench_regime_indicators(c: &mut Criterion) {
    let codec = FloatingCodec::default();
    let size = 10_000; // 10K candles

    // Simulate regime indicators: values typically between -1.0 and 1.0
    let mut trend_scores = Vec::with_capacity(size);
    let mut volatility_scores = Vec::with_capacity(size);
    let mut momentum_scores = Vec::with_capacity(size);

    for i in 0..size {
        let t = i as f64;
        // Trend: slow-moving between -1 and 1
        trend_scores.push((t / 100.0).sin() * 0.8);
        // Volatility: 0 to 1, with spikes
        volatility_scores.push(((t / 50.0).sin().abs() + (t / 13.0).cos() * 0.2).clamp(0.0, 1.0));
        // Momentum: -1 to 1, oscillating
        momentum_scores.push(((t / 30.0).sin() * 0.7 + (t / 7.0).cos() * 0.3).clamp(-1.0, 1.0));
    }

    let mut group = c.benchmark_group("realistic_regime_indicators");
    group.throughput(Throughput::Bytes((size * 3 * 8) as u64)); // 3 indicators

    group.bench_function("compress_trend", |b| {
        b.iter(|| {
            codec.compress_f64(black_box(&trend_scores), None).unwrap()
        });
    });

    group.bench_function("compress_volatility", |b| {
        b.iter(|| {
            codec.compress_f64(black_box(&volatility_scores), None).unwrap()
        });
    });

    group.bench_function("compress_momentum", |b| {
        b.iter(|| {
            codec.compress_f64(black_box(&momentum_scores), None).unwrap()
        });
    });

    let trend_compressed = codec.compress_f64(&trend_scores, None).unwrap();
    let volatility_compressed = codec.compress_f64(&volatility_scores, None).unwrap();
    let momentum_compressed = codec.compress_f64(&momentum_scores, None).unwrap();

    let total_original = size * 3 * 8;
    let total_compressed = trend_compressed.len() + volatility_compressed.len() + momentum_compressed.len();

    println!("\nRegime indicators (10K candles, 3 indicators):");
    println!("  Original size: {} KB", total_original / 1000);
    println!("  Compressed size: {} KB", total_compressed / 1000);
    println!("  Compression ratio: {:.2}x", total_original as f64 / total_compressed as f64);

    group.finish();
}

// Multiple timeframes (realistic MATHILDE use case)
fn bench_multi_timeframe(c: &mut Criterion) {
    let codec = FloatingCodec::default();

    // Simulate different timeframes for the same indicator
    let sizes = vec![
        ("1h", 8760),   // 1 year of hourly data
        ("4h", 2190),   // 1 year of 4h data
        ("12h", 730),   // 1 year of 12h data
        ("1d", 365),    // 1 year of daily data
    ];

    let mut group = c.benchmark_group("realistic_multi_timeframe");

    for (timeframe, size) in sizes {
        let data: Vec<f64> = (0..size).map(|i| {
            let t = i as f64;
            (t / 100.0).sin() * 0.5 + (t / 20.0).cos() * 0.3
        }).collect();

        group.throughput(Throughput::Bytes((size * 8) as u64));
        group.bench_with_input(BenchmarkId::new("compress", timeframe), &data, |b, data| {
            b.iter(|| {
                codec.compress_f64(black_box(data), None).unwrap()
            });
        });

        let compressed = codec.compress_f64(&data, None).unwrap();
        println!("\n{} timeframe ({} points):", timeframe, size);
        println!("  Original: {} KB, Compressed: {} KB, Ratio: {:.2}x",
                 (size * 8) / 1000,
                 compressed.len() / 1000,
                 (size * 8) as f64 / compressed.len() as f64);
    }

    group.finish();
}

// Log data: structured patterns with high repetition
fn bench_log_data(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 100_000;

    // Simulate log levels as integers (ERROR=3, WARN=2, INFO=1, DEBUG=0)
    // 80% INFO, 15% DEBUG, 4% WARN, 1% ERROR
    let mut log_levels = Vec::with_capacity(size);
    for i in 0..size {
        let rand = (i * 7919) % 100; // Pseudo-random
        let level = if rand < 80 {
            1 // INFO
        } else if rand < 95 {
            0 // DEBUG
        } else if rand < 99 {
            2 // WARN
        } else {
            3 // ERROR
        };
        log_levels.push(level);
    }

    let mut group = c.benchmark_group("realistic_log_levels");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_i64(black_box(&log_levels)).unwrap()
        });
    });

    let compressed = codec.compress_i64(&log_levels).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_i64(black_box(&compressed)).unwrap()
        });
    });

    println!("\nLog levels (100K entries):");
    println!("  Original size: {} KB", (size * 8) / 1000);
    println!("  Compressed size: {} KB", compressed.len() / 1000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

// Network metrics: packet counts, bytes transferred
fn bench_network_metrics(c: &mut Criterion) {
    let codec = IntegerCodec::default();
    let size = 100_000;

    // Simulate network traffic: gradually increasing with bursts
    let mut bytes_transferred = Vec::with_capacity(size);
    let mut cumulative = 0u64;
    for i in 0..size {
        let t = i as f64;
        // Base traffic + periodic bursts
        let base = 1500; // Average packet size
        let burst = if i % 1000 < 100 { 10000 } else { 0 }; // Periodic burst
        let noise = ((t * 0.1).sin() * 500.0) as u64;
        cumulative += base + burst + noise;
        bytes_transferred.push(cumulative as i64);
    }

    let mut group = c.benchmark_group("realistic_network_metrics");
    group.throughput(Throughput::Bytes((size * 8) as u64));

    group.bench_function("compress", |b| {
        b.iter(|| {
            codec.compress_i64(black_box(&bytes_transferred)).unwrap()
        });
    });

    let compressed = codec.compress_i64(&bytes_transferred).unwrap();

    group.bench_function("decompress", |b| {
        b.iter(|| {
            codec.decompress_i64(black_box(&compressed)).unwrap()
        });
    });

    println!("\nNetwork metrics (100K measurements):");
    println!("  Original size: {} KB", (size * 8) / 1000);
    println!("  Compressed size: {} KB", compressed.len() / 1000);
    println!("  Compression ratio: {:.2}x", (size * 8) as f64 / compressed.len() as f64);

    group.finish();
}

criterion_group!(
    benches,
    bench_stock_prices,
    bench_sensor_readings,
    bench_timestamps,
    bench_sparse_data,
    bench_database_ids,
    bench_regime_indicators,
    bench_multi_timeframe,
    bench_log_data,
    bench_network_metrics,
);

criterion_main!(benches);
