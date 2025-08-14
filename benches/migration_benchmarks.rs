//! Performance Benchmarking Suite for Migration Validation
//!
//! This module provides comprehensive performance benchmarks that compare
//! the Rust implementation against the Python reference implementation
//! to validate the claimed 3-5x performance improvements.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use opnsense_config_faker::generator::performance::PerformantConfigGenerator;
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use std::hint::black_box;
use std::process::Command;
use std::time::Duration;
use tempfile::NamedTempFile;

/// Benchmark configuration for migration validation
#[derive(Debug, Clone)]
struct BenchmarkConfig {
    count: u16,
    seed: Option<u64>,
}

impl BenchmarkConfig {
    fn new(count: u16, seed: Option<u64>) -> Self {
        Self { count, seed }
    }
}

/// Benchmark the Rust implementation
fn bench_rust_implementation(config: &BenchmarkConfig) -> Duration {
    let start = std::time::Instant::now();

    let _configs = generate_vlan_configurations(config.count, config.seed, None)
        .expect("Rust implementation should succeed");

    start.elapsed()
}

/// Benchmark the optimized Rust implementation
fn bench_rust_optimized_implementation(config: &BenchmarkConfig) -> Duration {
    let start = std::time::Instant::now();

    let mut generator = PerformantConfigGenerator::new(config.seed);
    let _configs = generator
        .generate_batch(config.count as usize)
        .expect("Optimized Rust implementation should succeed");

    start.elapsed()
}

/// Benchmark the Python reference implementation
fn bench_python_implementation(config: &BenchmarkConfig) -> Duration {
    let temp_file = NamedTempFile::new().expect("Could not create temp file");
    let python_script = std::env::current_dir()
        .expect("Could not get current directory")
        .join("tests")
        .join("python_reference.py");

    let seed_str = config
        .seed
        .map(|s| s.to_string())
        .unwrap_or_else(|| "None".to_string());

    let start = std::time::Instant::now();

    let output = Command::new("python3")
        .arg(&python_script)
        .arg(config.count.to_string())
        .arg(temp_file.path())
        .arg(seed_str)
        .output()
        .expect("Python script should execute");

    let duration = start.elapsed();

    if !output.status.success() {
        panic!(
            "Python script failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    duration
}

/// Comprehensive performance comparison benchmarks
fn bench_migration_performance_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("migration_performance_comparison");

    // Test different scales to validate performance claims
    let test_scales = vec![10, 50, 100, 250, 500];

    for scale in test_scales {
        let config = BenchmarkConfig::new(scale, Some(42));

        // Benchmark Python reference implementation
        group.bench_with_input(
            BenchmarkId::new("python_reference", scale),
            &config,
            |b, config| {
                b.iter_custom(|_iters| {
                    // Note: We run only once per iteration to avoid Python startup overhead
                    bench_python_implementation(black_box(config))
                });
            },
        );

        // Benchmark standard Rust implementation
        group.bench_with_input(
            BenchmarkId::new("rust_standard", scale),
            &config,
            |b, config| {
                b.iter_custom(|_iters| bench_rust_implementation(black_box(config)));
            },
        );

        // Benchmark optimized Rust implementation
        group.bench_with_input(
            BenchmarkId::new("rust_optimized", scale),
            &config,
            |b, config| {
                b.iter_custom(|_iters| bench_rust_optimized_implementation(black_box(config)));
            },
        );
    }

    group.finish();
}

/// Memory efficiency comparison (approximate)
fn bench_memory_efficiency_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency_comparison");

    // Test memory usage patterns for different scales
    let test_scales = vec![100, 500, 1000];

    for scale in test_scales {
        let config = BenchmarkConfig::new(scale, Some(123));

        // Rust standard implementation memory usage
        group.bench_with_input(
            BenchmarkId::new("rust_memory_standard", scale),
            &config,
            |b, config| {
                b.iter(|| {
                    let configs = generate_vlan_configurations(config.count, config.seed, None)
                        .expect("Should generate configs");

                    // Estimate memory usage
                    let memory_estimate = configs.len() * std::mem::size_of_val(&configs[0]);
                    black_box((configs, memory_estimate))
                });
            },
        );

        // Rust optimized implementation memory usage
        group.bench_with_input(
            BenchmarkId::new("rust_memory_optimized", scale),
            &config,
            |b, config| {
                b.iter(|| {
                    let mut generator = PerformantConfigGenerator::new(config.seed);
                    let configs = generator
                        .generate_batch(config.count as usize)
                        .expect("Should generate configs");

                    // Estimate memory usage
                    let memory_estimate = configs.len() * std::mem::size_of_val(&configs[0]);
                    black_box((configs, memory_estimate))
                });
            },
        );
    }

    group.finish();
}

/// Throughput validation benchmarks
fn bench_throughput_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_validation");

    // Set longer measurement time for accurate throughput measurement
    group.measurement_time(Duration::from_secs(15));

    // Large scale throughput tests
    let config = BenchmarkConfig::new(1000, Some(456));

    group.bench_function("python_throughput_1000", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            for _ in 0..iters {
                total_duration += bench_python_implementation(black_box(&config));
            }
            total_duration
        });
    });

    group.bench_function("rust_throughput_1000", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            for _ in 0..iters {
                total_duration += bench_rust_implementation(black_box(&config));
            }
            total_duration
        });
    });

    group.bench_function("rust_optimized_throughput_1000", |b| {
        b.iter_custom(|iters| {
            let mut total_duration = Duration::new(0, 0);
            for _ in 0..iters {
                total_duration += bench_rust_optimized_implementation(black_box(&config));
            }
            total_duration
        });
    });

    group.finish();
}

/// Performance regression detection
fn bench_regression_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("regression_detection");

    // Baseline performance tests that should consistently pass
    let baseline_configs = [
        BenchmarkConfig::new(50, Some(100)),
        BenchmarkConfig::new(100, Some(200)),
        BenchmarkConfig::new(200, Some(300)),
    ];

    for (i, config) in baseline_configs.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("baseline_regression", format!("test_{}", i + 1)),
            config,
            |b, config| {
                b.iter(|| {
                    let configs =
                        generate_vlan_configurations(black_box(config.count), config.seed, None)
                            .expect("Should generate configs");
                    black_box(configs)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    migration_benchmarks,
    bench_migration_performance_comparison,
    bench_memory_efficiency_comparison,
    bench_throughput_validation,
    bench_regression_detection
);
criterion_main!(migration_benchmarks);
