# CI Benchmark Monitoring Guide

This document provides guidance for monitoring CI benchmark reports and establishing performance baselines after the testing framework merge.

## Overview

The CI pipeline includes automated benchmarking using Criterion.rs to track performance of key operations. After merging the testing framework, we need to:

1. Monitor initial benchmark runs to establish baseline performance
2. Set up alerting for performance regressions
3. Track performance trends over time

## Benchmark Coverage

### Current Benchmarks

The project includes benchmarks for:

- **VLAN Generation** (`benches/vlan_generation.rs`): Tests VLAN ID generation and IP allocation
- **XML Generation** (`benches/xml_generation.rs`): Tests XML template rendering and configuration generation

### Benchmark Locations

```text
benches/
├── vlan_generation.rs    # VLAN and network generation performance
└── xml_generation.rs     # XML template rendering performance
```

## CI Benchmark Integration

### GitHub Actions Workflow

The CI pipeline (`.github/workflows/ci.yml`) includes a dedicated `benches` job that:

1. **Runs Benchmarks**: Executes all criterion benchmarks
2. **Performance Tracking**: Uses `github-action-benchmark` for trend analysis
3. **Regression Detection**: Alerts on performance degradation >150%
4. **Artifact Upload**: Saves detailed HTML reports

### Benchmark Job Configuration

```yaml
benches:
  name: Benchmarks
  runs-on: ubuntu-latest
  steps:
    - name: Run benchmarks
      run: cargo bench

    - name: Track benchmark performance (main/develop)
      if: github.event_name == 'push' && (github.ref == 'refs/heads/main' || 
        github.ref == 'refs/heads/develop')
      uses: rhysd/github-action-benchmark@v1
      with:
        tool: criterion
        output-file-path: target/criterion/*/base/estimates.json
        github-token: ${{ secrets.GITHUB_TOKEN }}
        auto-push: true
        comment-on-alert: true
        alert-threshold: 150%
        fail-on-alert: false
        max-items-in-chart: 100
```

## Monitoring Initial Runs

### Post-Merge Actions

After the testing framework is merged to main:

1. **First Run Monitoring**

   - Monitor the first CI run on main branch
   - Verify benchmarks execute successfully
   - Check that baseline data is established

2. **Baseline Establishment**

   - The first successful benchmark run establishes the baseline
   - Subsequent runs are compared against this baseline
   - Baseline data is stored in the `gh-pages` branch

3. **Validation Steps**

   ```bash
   # Check if benchmarks run locally
   cargo bench

   # Verify benchmark artifacts
   ls -la target/criterion/

   # Check HTML reports
   open target/criterion/report/index.html
   ```

### Expected Benchmark Results

Initial benchmarks should show:

- **VLAN Generation**: ~1-10ms for typical batch sizes
- **XML Generation**: ~10-100ms depending on configuration complexity
- **Memory Usage**: Reasonable allocation patterns without excessive heap growth

## Performance Baseline Targets

### Acceptable Performance Ranges

Based on typical usage patterns:

| Benchmark                     | Target Range | Alert Threshold |
| ----------------------------- | ------------ | --------------- |
| VLAN Generation (100 VLANs)   | 1-5ms        | >10ms           |
| XML Generation (small config) | 5-20ms       | >50ms           |
| XML Generation (large config) | 20-100ms     | >200ms          |
| Memory Usage                  | \<50MB peak  | >100MB          |

### Performance Characteristics

- **Linear Scaling**: Performance should scale linearly with input size
- **Memory Efficiency**: Memory usage should be bounded and not grow excessively
- **Consistent Performance**: Results should be consistent across runs (low variance)

## Regression Detection

### Automated Alerts

The CI is configured to:

- **Alert Threshold**: 150% performance degradation triggers alert
- **Comment on PRs**: Performance regressions are highlighted in PR comments
- **Fail Safe**: Regressions don't fail CI but are prominently reported

### Manual Monitoring

Developers should:

1. **Review PR Comments**: Check benchmark results in PR reviews
2. **Monitor Trends**: Regularly check the benchmark dashboard
3. **Investigate Alerts**: Follow up on performance regression alerts

## Benchmark Dashboard

### GitHub Pages Integration

Benchmark results are automatically published to GitHub Pages:

- **URL**: `https://evilbit-labs.github.io/OPNsense-config-faker/dev/bench/`
- **Update Schedule**: Updated on every main branch push
- **Historical Data**: Tracks up to 100 benchmark runs

### Dashboard Features

- **Performance Trends**: Line charts showing performance over time
- **Regression Highlighting**: Visual indicators for performance regressions
- **Detailed Metrics**: Mean, standard deviation, and percentile data
- **Comparison Views**: Side-by-side comparisons between runs

## Local Benchmark Development

### Running Benchmarks Locally

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench vlan_generation

# Run with additional output
cargo bench -- --verbose

# Generate detailed HTML reports
cargo bench && open target/criterion/report/index.html
```

### Benchmark Development

When adding new benchmarks:

1. **Create Benchmark File**: Add to `benches/` directory
2. **Update Cargo.toml**: Add benchmark section if needed
3. **Follow Patterns**: Use existing benchmarks as templates
4. **Test Locally**: Verify benchmarks run and produce meaningful results

### Benchmark Best Practices

- **Realistic Inputs**: Use data representative of actual usage
- **Multiple Sizes**: Test with small, medium, and large inputs
- **Warm-up Periods**: Allow for JIT optimization in measurements
- **Statistical Validity**: Use sufficient iterations for stable measurements

## Troubleshooting

### Common Issues

1. **Benchmarks Don't Run**

   - Check Cargo.toml configuration
   - Verify benchmark files compile correctly
   - Review CI logs for error messages

2. **Inconsistent Results**

   - Check for system load during benchmarking
   - Verify test data consistency
   - Consider longer measurement periods

3. **False Positive Alerts**

   - Review recent changes for expected performance impacts
   - Check if alert threshold needs adjustment
   - Verify benchmark test data hasn't changed

### Debug Commands

```bash
# Check benchmark compilation
cargo bench --no-run

# Run with debug output  
RUST_LOG=debug cargo bench

# Check criterion configuration
cargo bench -- --help
```

## Integration with Development Workflow

### Pre-merge Checklist

For significant changes:

- [ ] Run benchmarks locally to check for regressions
- [ ] Review benchmark results in PR comments
- [ ] Investigate any performance alerts
- [ ] Update benchmark tests if adding new performance-critical code

### Post-merge Monitoring

After merging to main:

- [ ] Monitor CI benchmark job completion
- [ ] Check benchmark dashboard for baseline establishment
- [ ] Verify no unexpected performance regressions
- [ ] Review and acknowledge any performance changes

### Regular Maintenance

Monthly tasks:

- [ ] Review benchmark trends and identify long-term patterns
- [ ] Update benchmark test data if needed
- [ ] Consider adding benchmarks for new features
- [ ] Review and adjust alert thresholds based on historical data

## Resources

- **Criterion.rs Documentation**: <https://bheisler.github.io/criterion.rs/>
- **GitHub Action Benchmark**: <https://github.com/rhysd/github-action-benchmark>
- **Benchmark Dashboard**: <https://evilbit-labs.github.io/OPNsense-config-faker/dev/bench/>
- **Performance Profiling Guide**: See `docs/performance-profiling.md` (if available)

This monitoring strategy ensures that performance regressions are caught early and that the project maintains optimal performance as it evolves.
