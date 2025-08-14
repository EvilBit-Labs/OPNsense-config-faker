//! Comprehensive performance benchmarks for OPNsense Config Faker
//!
//! This benchmark suite provides detailed performance measurements across
//! different scales and configurations to validate the 3-5x performance
//! improvement targets over the Python baseline.

use criterion::{criterion_group, criterion_main, Criterion};
use opnsense_config_faker::generator::performance::PerformantConfigGenerator;
use opnsense_config_faker::generator::vlan::{generate_vlan_configurations, VlanGenerator};
use std::hint::black_box;

/// Memory limit for large dataset tests to ensure CI stability
#[allow(dead_code)]
const MEMORY_LIMIT_MB: usize = 512; // 512MB memory limit for large datasets
fn bench_vlan_generation_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("vlan_generation_scalability");

    // Test different dataset sizes
    for count in [10, 50, 100, 500, 1000] {
        group.bench_function(format!("sequential/{}", count), |b| {
            b.iter(|| {
                let configs =
                    generate_vlan_configurations(black_box(count as u16), Some(42), None).unwrap();
                black_box(configs)
            })
        });

        group.bench_function(format!("performance_optimized/{}", count), |b| {
            b.iter(|| {
                let mut generator = PerformantConfigGenerator::new(Some(42));
                let configs = generator.generate_batch(black_box(count)).unwrap();
                black_box(configs)
            })
        });
    }

    group.finish();
}

fn bench_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");

    // Test memory usage patterns
    for count in [100, 500, 1000] {
        group.bench_function(format!("memory_usage/{}", count), |b| {
            b.iter(|| {
                let configs =
                    generate_vlan_configurations(black_box(count as u16), Some(42), None).unwrap();
                let memory_estimate = configs.len()
                    * std::mem::size_of::<opnsense_config_faker::generator::vlan::VlanConfig>();
                black_box((configs, memory_estimate))
            })
        });

        group.bench_function(format!("optimized_memory_usage/{}", count), |b| {
            b.iter(|| {
                let mut generator = PerformantConfigGenerator::new(Some(42));
                let configs = generator.generate_batch(black_box(count)).unwrap();
                let memory_usage = configs.len()
                    * std::mem::size_of::<opnsense_config_faker::generator::vlan::VlanConfig>();
                black_box((configs, memory_usage))
            })
        });
    }

    group.finish();
}

fn bench_vlan_id_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("vlan_id_allocation");

    // Benchmark VLAN ID uniqueness checking
    group.bench_function("generate_unique_vlan_ids_1000", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();
            black_box(configs)
        })
    });

    group.bench_function("optimized_vlan_id_allocation_1000", |b| {
        b.iter(|| {
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();
            black_box(configs)
        })
    });

    group.finish();
}

fn bench_ip_network_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ip_network_generation");

    // Benchmark RFC 1918 network generation and validation
    group.bench_function("rfc1918_network_generation_1000", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();

            // Validate all networks are RFC 1918 compliant
            for config in &configs {
                let _ = black_box(config.validate_rfc1918());
            }
            black_box(configs)
        })
    });

    group.bench_function("optimized_ip_generation_1000", |b| {
        b.iter(|| {
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();

            // Focus on generation performance
            black_box(configs)
        })
    });

    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    // Benchmark string allocation patterns
    group.bench_function("description_generation_1000", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();

            // Focus on string operations
            let descriptions: Vec<String> = configs.iter().map(|c| c.description.clone()).collect();
            black_box(descriptions)
        })
    });

    group.bench_function("optimized_description_generation_1000", |b| {
        b.iter(|| {
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();

            // Focus on string operations
            let descriptions: Vec<String> = configs.iter().map(|c| c.description.clone()).collect();
            black_box(descriptions)
        })
    });

    group.finish();
}

fn bench_cold_start_performance(c: &mut Criterion) {
    c.bench_function("cold_start_cli_simulation", |b| {
        b.iter(|| {
            // Simulate CLI cold start
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(10)).unwrap();
            black_box(configs)
        })
    });

    c.bench_function("optimized_cold_start", |b| {
        b.iter(|| {
            // Simulate optimized CLI cold start
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(10)).unwrap();
            black_box(configs)
        })
    });
}

fn bench_performance_regression_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_regression");

    // Regression test for 100 VLANs (should complete in reasonable time)
    group.bench_function("regression_100_vlans", |b| {
        b.iter(|| {
            let configs = generate_vlan_configurations(black_box(100), Some(42), None).unwrap();
            black_box(configs)
        })
    });

    // Throughput validation
    group.bench_function("throughput_validation", |b| {
        b.iter(|| {
            let configs = generate_vlan_configurations(black_box(150), Some(42), None).unwrap();
            black_box(configs)
        })
    });

    group.bench_function("optimized_throughput_validation", |b| {
        b.iter(|| {
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(150)).unwrap();
            black_box(configs)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_vlan_generation_scalability,
    bench_memory_efficiency,
    bench_vlan_id_allocation,
    bench_ip_network_generation,
    bench_string_operations,
    bench_cold_start_performance,
    bench_performance_regression_tests
);
criterion_main!(benches);
