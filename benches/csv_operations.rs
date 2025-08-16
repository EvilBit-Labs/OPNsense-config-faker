#[path = "_common/mod.rs"]
mod bench_common;

use bench_common::{ci_or_local, criterion_for_env};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use opnsense_config_faker::io::csv::{
    read_csv, read_csv_streaming, write_csv, write_csv_streaming,
};
use std::hint::black_box;
use tempfile::NamedTempFile;

fn bench_csv_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_serialization");

    // Use CI-appropriate dataset sizes for CSV operations
    let counts = ci_or_local(&[100u16, 500u16], &[100u16, 500u16, 1000u16, 2000u16]);
    for count in counts.iter() {
        let configs = generate_vlan_configurations(*count, Some(42), None).unwrap();

        group.bench_with_input(BenchmarkId::new("write_csv", count), count, |b, _| {
            b.iter(|| {
                let temp_file = NamedTempFile::new().unwrap();
                write_csv(black_box(&configs), temp_file.path()).unwrap();
            })
        });

        group.bench_with_input(
            BenchmarkId::new("write_csv_streaming", count),
            count,
            |b, _| {
                b.iter(|| {
                    let temp_file = NamedTempFile::new().unwrap();
                    write_csv_streaming(black_box(configs.clone().into_iter()), temp_file.path())
                        .unwrap();
                })
            },
        );
    }

    group.finish();
}

fn bench_csv_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_deserialization");

    // Use CI-appropriate dataset sizes for CSV operations
    let counts = ci_or_local(&[100u16, 500u16], &[100u16, 500u16, 1000u16, 2000u16]);
    for count in counts.iter() {
        let configs = generate_vlan_configurations(*count, Some(42), None).unwrap();
        let temp_file = NamedTempFile::new().unwrap();
        write_csv(&configs, temp_file.path()).unwrap();

        group.bench_with_input(BenchmarkId::new("read_csv", count), count, |b, _| {
            b.iter(|| {
                read_csv(black_box(temp_file.path())).unwrap();
            })
        });

        group.bench_with_input(
            BenchmarkId::new("read_csv_streaming", count),
            count,
            |b, _| {
                b.iter(|| {
                    let mut counter = 0;
                    read_csv_streaming(black_box(temp_file.path()), |_| {
                        counter += 1;
                        Ok(())
                    })
                    .unwrap();
                })
            },
        );
    }

    group.finish();
}

fn bench_csv_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_round_trip");

    // Use CI-appropriate dataset sizes for round-trip tests
    let counts = ci_or_local(&[100u16, 500u16], &[100u16, 500u16, 1000u16]);
    for count in counts.iter() {
        let configs = generate_vlan_configurations(*count, Some(42), None).unwrap();

        group.bench_with_input(BenchmarkId::new("round_trip", count), count, |b, _| {
            b.iter(|| {
                let temp_file = NamedTempFile::new().unwrap();
                write_csv(black_box(&configs), temp_file.path()).unwrap();
                let _read_configs = read_csv(temp_file.path()).unwrap();
            })
        });

        group.bench_with_input(
            BenchmarkId::new("round_trip_streaming", count),
            count,
            |b, _| {
                b.iter(|| {
                    let temp_file = NamedTempFile::new().unwrap();
                    write_csv_streaming(black_box(configs.clone().into_iter()), temp_file.path())
                        .unwrap();
                    let mut counter = 0;
                    read_csv_streaming(temp_file.path(), |_| {
                        counter += 1;
                        Ok(())
                    })
                    .unwrap();
                })
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion_for_env();
    targets = bench_csv_serialization,
        bench_csv_deserialization,
        bench_csv_round_trip
}
criterion_main!(benches);
