# Benchmarking Guide

This project uses [Criterion.rs](https://bheisler.github.io/criterion.rs/) for performance benchmarking with CI-aware optimizations for faster build times.

## Quick Start

### Run All Benchmarks Locally

```bash
cargo bench
```

### Run Specific Benchmark Suite

```bash
cargo bench --bench vlan_generation
cargo bench --bench xml_generation
cargo bench --bench csv_operations
cargo bench --bench performance_benchmarks
cargo bench --bench migration_benchmarks
```

## CI-Optimized Benchmarks

### Automatic CI Detection

Benchmarks automatically detect CI environments and apply optimizations:

- **GitHub Actions**: `CI=true` (automatic)
- **Manual CI simulation**: `CI=true cargo bench --quiet`
- **Local quick mode**: `BENCH_CI=1 cargo bench --quiet`

### CI Optimizations Applied

| Setting          | Local Default | CI Optimized |
| ---------------- | ------------- | ------------ |
| Sample size      | 100           | 30           |
| Warmup time      | 3s            | 500ms        |
| Measurement time | 5s            | 2s           |
| Noise threshold  | 2%            | 5%           |
| Plot generation  | Enabled       | Disabled     |
| Dataset sizes    | Full          | Reduced      |

### Dataset Size Reductions

**VLAN Generation:**

- CI: `[10, 100]` VLANs
- Local: `[10, 100, 1000]` VLANs

**CSV Operations:**

- CI: `[100, 500]` records
- Local: `[100, 500, 1000, 2000]` records

**Migration Benchmarks:**

- CI: `[10, 50, 100]` scale
- Local: `[10, 50, 100, 250, 500]` scale

## Advanced Usage

### Override Criterion Settings

```bash
# Custom sample size
cargo bench -- --sample-size 50

# Custom measurement time
cargo bench -- --measurement-time 10

# Generate plots locally
cargo bench -- --plotting-backend plotters
```

### Benchmark Architecture

```text
benches/
├── _common/mod.rs          # Shared CI-aware Criterion configuration
├── vlan_generation.rs      # VLAN configuration generation benchmarks
├── xml_generation.rs       # XML template processing benchmarks  
├── csv_operations.rs       # CSV read/write performance benchmarks
├── performance_benchmarks.rs # Comprehensive performance suite
└── migration_benchmarks.rs   # Python vs Rust migration validation
```

The `benches/_common/mod.rs` module provides:

- `criterion_for_env()`: CI-optimized Criterion configuration
- `ci_or_local()`: Environment-aware dataset selection
- `cap_ci()`: Dataset size limiting for CI
- `ci_counts()`: Predefined count sets for different scales

## CI Integration

Benchmarks run automatically in GitHub Actions with:

- Optimized configuration for faster execution
- Artifact upload of HTML reports
- Performance tracking on main/develop branches
- PR comments with benchmark results

### Accessing Benchmark Results

1. **GitHub Actions**: Check the "Benchmarks" job in CI
2. **HTML Reports**: Download `criterion-html-reports-{sha}` artifact
3. **Performance Tracking**: View charts in GitHub Pages (for main/develop)

## Performance Targets

The benchmarking suite validates:

- **3-5x performance improvement** over Python baseline
- **Memory efficiency** across different scales
- **Regression detection** for performance stability
- **Throughput validation** for large datasets

## Contributing

When adding new benchmarks:

1. **Import the shared module:**

   ```rust
   #[path = "_common/mod.rs"]
   mod bench_common;

   use bench_common::{ci_or_local, criterion_for_env};
   ```

2. **Use CI-aware configuration:**

   ```rust
   criterion_group! {
       name = benches;
       config = criterion_for_env();
       targets = your_benchmark_function
   }
   ```

3. **Apply dataset scaling:**

   ```rust
   let sizes = ci_or_local(&[10, 100], &[10, 100, 500, 1000]);
   ```

4. **Test both modes:**

   ```bash
   # CI mode
   CI=true cargo bench --bench your_benchmark

   # Local mode  
   cargo bench --bench your_benchmark
   ```

This ensures benchmarks provide meaningful feedback locally while maintaining fast CI execution times.
