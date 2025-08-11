# Benchmarks

This directory contains performance benchmarks for the OPNsense Config Faker project.

## Available Benchmarks

- **`vlan_generation.rs`** - Benchmarks VLAN configuration generation at different scales (10, 100, 1000 VLANs)
- **`xml_generation.rs`** - Benchmarks XML template application for generated configurations

## Running Benchmarks Locally

To run all benchmarks:

```bash
cargo bench
```

To run a specific benchmark file:

```bash
cargo bench --bench vlan_generation
cargo bench --bench xml_generation
```

To run benchmarks with additional output:

```bash
cargo bench -- --verbose
```

## Benchmark Output

Criterion will generate HTML reports in `target/criterion/` showing:

- Performance measurements over time
- Statistical analysis of results
- Comparison with previous runs
- Detailed timing histograms

Open `target/criterion/report/index.html` in your browser to view detailed results.

## CI Integration

Benchmarks are automatically run on pull requests and compared against the base branch to detect performance regressions. Results are uploaded as build artifacts for review.
