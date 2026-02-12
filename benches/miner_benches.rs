use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use solo_miner::core::{Block, miner::{Sha256Miner, RandomXMiner}, MinerAlgorithm, DeviceType};
use std::time::Duration;

fn bench_sha256_cpu(c: &mut Criterion) {
    let miner = Sha256Miner::new(DeviceType::CPU);
    let mut block = create_test_block("0000");
    
    let mut group = c.benchmark_group("sha256_cpu");
    group.throughput(Throughput::Elements(10_000));
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("mine_batch_10k", |b| {
        b.iter(|| {
            let result = miner.mine(black_box(&mut block));
            black_box(result)
        })
    });
    group.finish();
}

fn bench_sha256_gpu(c: &mut Criterion) {
    let miner = Sha256Miner::new(DeviceType::GPU);
    let mut block = create_test_block("0000");
    
    let mut group = c.benchmark_group("sha256_gpu");
    group.throughput(Throughput::Elements(100_000));
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("mine_batch_100k", |b| {
        b.iter(|| {
            let result = miner.mine(black_box(&mut block));
            black_box(result)
        })
    });
    group.finish();
}

fn bench_randomx_cpu(c: &mut Criterion) {
    let miner = RandomXMiner::new(DeviceType::CPU);
    let mut block = create_test_block("0000");
    
    let mut group = c.benchmark_group("randomx_cpu");
    group.throughput(Throughput::Elements(5_000));
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("mine_batch_5k", |b| {
        b.iter(|| {
            let result = miner.mine(black_box(&mut block));
            black_box(result)
        })
    });
    group.finish();
}

fn bench_randomx_gpu(c: &mut Criterion) {
    let miner = RandomXMiner::new(DeviceType::GPU);
    let mut block = create_test_block("0000");
    
    let mut group = c.benchmark_group("randomx_gpu");
    group.throughput(Throughput::Elements(50_000));
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("mine_batch_50k", |b| {
        b.iter(|| {
            let result = miner.mine(black_box(&mut block));
            black_box(result)
        })
    });
    group.finish();
}

fn bench_hashrate_calculation(c: &mut Criterion) {
    let metrics = std::sync::Arc::new(solo_miner::telemetry::MinerMetrics::new());
    
    c.bench_function("hashrate_recording", |b| {
        b.iter(|| {
            futures::executor::block_on(async {
                metrics.record_hashrate(black_box(1000000)).await;
                black_box(metrics.snapshot().await)
            })
        })
    });
}

fn bench_config_loading(c: &mut Criterion) {
    c.bench_function("config_loading", |b| {
        b.iter(|| {
            let config = solo_miner::config::Config::load().unwrap_or_default();
            black_box(config)
        })
    });
}

fn create_test_block(difficulty: &str) -> Block {
    Block {
        id: 1,
        timestamp: 1641168000,
        data: "Benchmarking Lonely Solo Miner Performance".to_string(),
        previous_hash: "0".repeat(64),
        nonce: 0,
        difficulty: difficulty.to_string(),
    }
}

fn bench_system_detection(c: &mut Criterion) {
    c.bench_function("system_cpu_detection", |b| {
        b.iter(|| {
            let num_cores = num_cpus::get();
            black_box(num_cores)
        })
    });
}

criterion_group!(
    benches,
    bench_sha256_cpu,
    bench_sha256_gpu,
    bench_randomx_cpu,
    bench_randomx_gpu,
    bench_hashrate_calculation,
    bench_config_loading,
    bench_system_detection
);
criterion_main!(benches);
