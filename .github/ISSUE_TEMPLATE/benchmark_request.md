---
name: Benchmark Request
about: Request performance benchmarking for specific functionality
title: '[BENCHMARK] '
labels: [type/benchmark, area/performance]
assignees: []
---

## Benchmark Objective

What performance characteristics need to be measured?

## Component/Function to Benchmark

Specify what needs to be benchmarked:

- **Module**: [e.g., generator, cli, xml, validation]
- **Function/Feature**: [e.g., VLAN generation, CSV export, XML parsing]
- **Code location**: \[e.g., `src/generator/mod.rs`, `src/xml/mod.rs`\]

## Performance Metrics

What metrics should be measured?

- [ ] **Execution time** (duration)
- [ ] **Memory usage** (peak/average)
- [ ] **CPU utilization**
- [ ] **Throughput** (items/operations per second)
- [ ] **Scalability** (performance vs. input size)
- [ ] **Other**: \_\_\_\_\_\_\_\_\_\_\_\_\_

## Test Scenarios

Define the benchmark scenarios:

### Scenario 1: [Name]

- **Input size**: [e.g., 10 VLANs, 1000 VLANs, 10000 VLANs]

- **Configuration**: [e.g., default settings, specific options]

- **Expected behaior**: [e.g., linear scaling, constant memory]

### Scenario 2: [Name]

- **Input size**:

- **Configuration**:

- **Expected behavior**:

### Scenario 3: [Name] (if applicable)

- **Input size**:
- **Configuration**:
- **Expected behavior**:

## Performance Targets

What are the performance goals or acceptance criteria?

- **Target execution time**: [e.g., < 100ms for 1000 VLANs]
- **Memory constraints**: [e.g., < 50MB peak memory]
- **Scalability requirements**: [e.g., O(n) time complexity]
- **Regression thresholds**: [e.g., no more than 10% slower than current]

## Environment Requirements

Specify the benchmarking environment:

- **Hardware**: [e.g., standardized CI runner, specific CPU/memory]
- **Dataset**: [e.g., synthetic data, real-world samples]
- **Baseline**: [e.g., current implementation, Python version]

## Implementation Notes

Technical requirements for the benchmark:

- **Framework**: [e.g., criterion.rs, custom timing]
- **Setup requirements**: [e.g., test data generation, initialization]
- **Measurement approach**: [e.g., multiple runs, statistical analysis]
- **Output format**: [e.g., criterion reports, custom metrics]

## Success Criteria

How will we know the benchmark is complete and successful?

- [ ] Benchmark suite is implemented and running
- [ ] Performance metrics are collected and documented
- [ ] Results meet performance targets (if applicable)
- [ ] Benchmark is integrated into CI/CD pipeline
- [ ] Performance regression detection is enabled

## Related Issues

Links to related performance work:

- Related to: #(issue number)
- Depends on: #(issue number)
- Performance target from: #(issue number)

## Additional Context

Any additional context about the performance requirements:

- User requirements driving this benchmark
- Known performance bottlenecks
- Comparison with similar tools/libraries
- Historical performance data
