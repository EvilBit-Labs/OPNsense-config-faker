#[path = "_common/mod.rs"]
mod bench_common;

use bench_common::{ci_or_local, criterion_for_env};
use criterion::{criterion_group, criterion_main, Criterion};
use opnsense_config_faker::generator::vlan::{generate_vlan_configurations, VlanGenerator};
use std::hint::black_box;

fn bench_vlan_generation(c: &mut Criterion) {
    // Use CI-appropriate dataset sizes
    let sizes = ci_or_local(&[10u16, 100u16], &[10u16, 100u16, 1000u16]);

    for &size in &sizes {
        c.bench_function(&format!("generate_{}_vlans", size), |b| {
            b.iter(|| {
                let configs =
                    generate_vlan_configurations(black_box(size), Some(42), None).unwrap();
                black_box(configs)
            })
        });
    }

    // Enhanced benchmarks with new functionality
    for &size in &sizes {
        c.bench_function(&format!("generate_{}_vlans_enhanced", size), |b| {
            b.iter(|| {
                let mut generator = VlanGenerator::new(Some(42));
                let configs = generator
                    .generate_batch_enhanced(black_box(size as usize))
                    .unwrap();
                black_box(configs)
            })
        });
    }

    // RFC 1918 validation benchmarks with CI-appropriate size
    let validation_size = ci_or_local(&[50usize], &[100usize])[0];
    c.bench_function(&format!("rfc1918_validation_{}", validation_size), |b| {
        let mut generator = VlanGenerator::new(Some(42));
        let configs = generator.generate_batch(validation_size).unwrap();

        b.iter(|| {
            for config in &configs {
                let _result = black_box(config.validate_rfc1918());
            }
        })
    });

    // Memory efficiency test for large batches (within VLAN ID limits)
    let memory_test_size = ci_or_local(&[1000usize], &[3000usize])[0];
    c.bench_function(
        &format!("generate_{}_vlans_memory_test", memory_test_size),
        |b| {
            b.iter(|| {
                let mut generator = VlanGenerator::new(Some(42));
                let configs = generator
                    .generate_batch(black_box(memory_test_size))
                    .unwrap();
                black_box(configs)
            })
        },
    );
}

criterion_group! {
    name = benches;
    config = criterion_for_env();
    targets = bench_vlan_generation
}
criterion_main!(benches);
