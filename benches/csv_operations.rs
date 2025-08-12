use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use opnsense_config_faker::io::csv::{
    read_csv, read_csv_streaming, write_csv, write_csv_streaming,
};
use tempfile::NamedTempFile;

fn bench_csv_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_serialization");

    for count in [100, 500, 1000, 2000].iter() {
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

    for count in [100, 500, 1000, 2000].iter() {
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

    for count in [100, 500, 1000].iter() {
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

criterion_group!(
    benches,
    bench_csv_serialization,
    bench_csv_deserialization,
    bench_csv_round_trip
);
criterion_main!(benches);
