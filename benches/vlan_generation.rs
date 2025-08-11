use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;

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
}

criterion_group!(benches, bench_vlan_generation);
criterion_main!(benches);
