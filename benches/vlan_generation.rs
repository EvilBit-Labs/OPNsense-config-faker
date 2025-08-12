use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opnsense_config_faker::generator::vlan::{generate_vlan_configurations, VlanGenerator};

fn bench_vlan_generation(c: &mut Criterion) {
    c.bench_function("generate_10_vlans", |b| {
        b.iter(|| {
            let configs = generate_vlan_configurations(black_box(10), Some(42), None).unwrap();
            black_box(configs)
        })
    });

    c.bench_function("generate_100_vlans", |b| {
        b.iter(|| {
            let configs = generate_vlan_configurations(black_box(100), Some(42), None).unwrap();
            black_box(configs)
        })
    });

    c.bench_function("generate_1000_vlans", |b| {
        b.iter(|| {
            let configs = generate_vlan_configurations(black_box(1000), Some(42), None).unwrap();
            black_box(configs)
        })
    });

    // Enhanced benchmarks with new functionality
    c.bench_function("generate_10_vlans_enhanced", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch_enhanced(black_box(10)).unwrap();
            black_box(configs)
        })
    });

    c.bench_function("generate_100_vlans_enhanced", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch_enhanced(black_box(100)).unwrap();
            black_box(configs)
        })
    });

    c.bench_function("generate_1000_vlans_enhanced", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch_enhanced(black_box(1000)).unwrap();
            black_box(configs)
        })
    });

    // RFC 1918 validation benchmarks
    c.bench_function("rfc1918_validation_100", |b| {
        let mut generator = VlanGenerator::new(Some(42));
        let configs = generator.generate_batch(100).unwrap();

        b.iter(|| {
            for config in &configs {
                let _result = black_box(config.validate_rfc1918());
            }
        })
    });

    // Memory efficiency test for large batches (within VLAN ID limits)
    c.bench_function("generate_3000_vlans_memory_test", |b| {
        b.iter(|| {
            let mut generator = VlanGenerator::new(Some(42));
            let configs = generator.generate_batch(black_box(3000)).unwrap();
            black_box(configs)
        })
    });
}

criterion_group!(benches, bench_vlan_generation);
criterion_main!(benches);
