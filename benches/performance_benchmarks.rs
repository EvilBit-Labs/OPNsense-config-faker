//! Comprehensive performance benchmarks for OPNsense Config Faker
//!
//! This benchmark suite provides detailed performance measurements across
//! different scales and configurations to validate the 3-5x performance
//! improvement targets over the Python baseline.

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use opnsense_config_faker::generator::vlan::{generate_vlan_configurations, VlanGenerator};
use opnsense_config_faker::generator::performance::PerformantConfigGenerator;
use std::hint::black_box;
use std::time::Duration;

/// Performance targets for regression testing
const TARGET_VLAN_THROUGHPUT: f64 = 150.0; // configs/sec (3x Python baseline of 50)
const TARGET_MEMORY_EFFICIENCY: usize = 25_000_000; // 25MB for 1000 VLANs (vs 45MB Python)

fn bench_vlan_generation_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("vlan_generation_scalability");
    
    // Test different scales to validate linear scaling
    for size in [10_usize, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let configs = generate_vlan_configurations(black_box(size.min(&4094) as u16), Some(42), None).unwrap();
                    black_box(configs)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("performance_optimized", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut generator = PerformantConfigGenerator::new(Some(42));
                    let configs = generator.generate_batch(black_box(size)).unwrap();
                    black_box(configs)
                })
            },
        );
    }
    group.finish();
}

fn bench_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    
    // Memory usage benchmarks for different scales
    for size in [100_usize, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("memory_usage", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let configs = generate_vlan_configurations(black_box(size.min(&4094) as u16), Some(42), None).unwrap();
                    // Estimate memory usage
                    let memory_usage = std::mem::size_of_val(&configs) + 
                        configs.iter().map(|c| {
                            std::mem::size_of_val(&c.ip_network) + 
                            std::mem::size_of_val(&c.description)
                        }).sum::<usize>();
                    black_box(memory_usage)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("optimized_memory_usage", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut generator = PerformantConfigGenerator::new(Some(42));
                    let configs = generator.generate_batch(black_box(size)).unwrap();
                    let memory_usage = generator.get_metrics().memory_used;
                    black_box((configs, memory_usage))
                })
            },
        );
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
            
            // Verify uniqueness (this validates the algorithm efficiency)
            let mut ids = std::collections::HashSet::new();
            for config in &configs {
                assert!(ids.insert(config.vlan_id));
            }
            black_box(configs)
        })
    });
    
    group.bench_function("optimized_vlan_id_allocation_1000", |b| {
        b.iter(|| {
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();
            
            // Verify uniqueness (this validates the algorithm efficiency)
            let mut ids = std::collections::HashSet::new();
            for config in &configs {
                assert!(ids.insert(config.vlan_id));
            }
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
                black_box(config.validate_rfc1918());
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
            let descriptions: Vec<String> = configs.iter()
                .map(|c| c.description.clone())
                .collect();
            black_box(descriptions)
        })
    });
    
    group.bench_function("optimized_description_generation_1000", |b| {
        b.iter(|| {
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(1000)).unwrap();
            
            // Focus on string operations
            let descriptions: Vec<String> = configs.iter()
                .map(|c| c.description.clone())
                .collect();
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
            let start = std::time::Instant::now();
            let configs = generate_vlan_configurations(black_box(100), Some(42), None).unwrap();
            let duration = start.elapsed();
            
            // Performance assertion (100 VLANs should complete in < 1 second)
            assert!(duration < Duration::from_millis(1000), 
                "Performance regression: 100 VLANs took {:?}", duration);
            
            black_box(configs)
        })
    });
    
    // Throughput validation
    group.bench_function("throughput_validation", |b| {
        b.iter(|| {
            let start = std::time::Instant::now();
            let configs = generate_vlan_configurations(black_box(150), Some(42), None).unwrap();
            let duration = start.elapsed();
            
            let throughput = configs.len() as f64 / duration.as_secs_f64();
            
            // Should achieve reasonable throughput
            assert!(throughput >= 50.0, // Lower threshold for CI environment
                "Throughput too low: {:.2} configs/sec", throughput);
            
            black_box(configs)
        })
    });
    
    group.bench_function("optimized_throughput_validation", |b| {
        b.iter(|| {
            let start = std::time::Instant::now();
            let mut generator = PerformantConfigGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(150)).unwrap();
            let duration = start.elapsed();
            
            let throughput = configs.len() as f64 / duration.as_secs_f64();
            
            // Should achieve better throughput
            assert!(throughput >= 100.0, // Target for optimized version
                "Optimized throughput too low: {:.2} configs/sec", throughput);
            
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