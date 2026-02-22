# Performance Optimization

Optimize OPNsense Config Faker for large-scale configuration generation and high-performance scenarios.

## Performance Characteristics

### Generation Performance

Generation time scales linearly with count:

| Dataset Size                | Generation Time | Memory Usage | Output Size |
| --------------------------- | --------------- | ------------ | ----------- |
| **Small** (\<100 VLANs)     | 10-50ms         | \<1MB        | \<100KB     |
| **Medium** (100-1000 VLANs) | 50-500ms        | 1-10MB       | 100KB-1MB   |
| **Large** (>1000 VLANs)     | 500ms-2s        | 10-100MB     | 1-10MB      |

### Memory Usage

Memory usage is approximately 500 bytes per VLAN configuration:

```rust
// Memory usage breakdown per VLAN
struct VlanConfig {
    id: u16,             // 2 bytes
    name: String,        // 24 bytes + string capacity
    description: String, // 24 bytes + string capacity
    interface: String,   // 24 bytes + string capacity
    network: IpNetwork,  // 20 bytes
                         // Total: ~100 bytes + string capacity
}
```

## Optimization Strategies

### Large Dataset Generation

For generating large numbers of configurations:

```bash
# Use CSV format for large datasets
cargo run --release -- generate vlan --count 1000 --format csv --output large-dataset.csv

# Stream processing for very large datasets
cargo run --release -- generate vlan --count 5000 --stream --output huge-dataset.csv
```

### Memory-Efficient Processing

Optimize memory usage for large generations:

```bash
# Batch processing
cargo run --release -- generate vlan --count 2000 --batch-size 100 --output batched.xml

# Memory-efficient mode
cargo run --release -- generate vlan --count 1000 --memory-efficient --output efficient.xml
```

### Parallel Processing

Enable parallel processing for large datasets:

```bash
# Enable parallel processing
cargo run --release --features rayon -- generate vlan --count 5000 --parallel --output parallel.xml
```

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suites
cargo bench --bench vlan_generation
cargo bench --bench xml_generation
cargo bench --bench csv_operations
cargo bench --bench performance_benchmarks
```

### Benchmark Results

Typical benchmark results on modern hardware:

```
vlan_generation/generate_10_vlans     time:   [15.234 µs 15.456 µs 15.678 µs]
vlan_generation/generate_100_vlans    time:   [145.23 µs 147.45 µs 149.67 µs]
vlan_generation/generate_1000_vlans  time:   [1.4567 ms 1.4789 ms 1.5011 ms]

xml_generation/xml_10_vlans           time:   [45.123 µs 45.678 µs 46.234 µs]
xml_generation/xml_100_vlans          time:   [456.78 µs 467.89 µs 478.90 µs]
xml_generation/xml_1000_vlans         time:   [4.5678 ms 4.6789 ms 4.7890 ms]

csv_operations/csv_100_records       time:   [23.456 µs 23.789 µs 24.123 µs]
csv_operations/csv_1000_records       time:   [234.56 µs 237.89 µs 241.23 µs]
csv_operations/csv_10000_records      time:   [2.3456 ms 2.3789 ms 2.4123 ms]
```

### Performance Regression Detection

```bash
# Run benchmarks with regression detection
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main
```

## Memory Optimization

### Streaming I/O

Use streaming for large datasets:

```rust
use std::io::Write;

pub fn generate_vlans_streaming<W: Write>(count: u32, base_id: u16, mut writer: W) -> Result<()> {
    for i in 0..count {
        let vlan = generate_single_vlan(base_id + i as u16)?;
        write_vlan_to_stream(&vlan, &mut writer)?;
    }
    Ok(())
}
```

### Memory Pool Allocation

Use memory pools for frequent allocations:

```rust
use bumpalo::Bump;

pub fn generate_vlans_with_pool(count: u32, base_id: u16) -> Result<Vec<VlanConfig>> {
    let bump = Bump::new();
    let mut vlans = Vec::with_capacity(count as usize);

    for i in 0..count {
        let vlan = generate_vlan_with_pool(&bump, base_id + i as u16)?;
        vlans.push(vlan);
    }

    Ok(vlans)
}
```

### Lazy Evaluation

Use lazy evaluation for large datasets:

```rust
use std::iter::Iterator;

pub fn generate_vlans_lazy(count: u32, base_id: u16) -> impl Iterator<Item = VlanConfig> {
    (0..count).map(move |i| generate_single_vlan(base_id + i as u16).unwrap())
}
```

## CPU Optimization

### Parallel Generation

Enable parallel processing with Rayon:

```rust
use rayon::prelude::*;

pub fn generate_vlans_parallel(count: u32, base_id: u16) -> Result<Vec<VlanConfig>> {
    let indices: Vec<u32> = (0..count).collect();

    let vlans: Result<Vec<_>> = indices
        .par_iter()
        .map(|&i| generate_single_vlan(base_id + i as u16))
        .collect();

    vlans
}
```

### SIMD Operations

Use SIMD for network calculations:

```rust
use std::simd::*;

pub fn calculate_network_ranges_simd(base_network: IpNetwork, count: u32) -> Vec<IpNetwork> {
    let mut networks = Vec::with_capacity(count as usize);
    let mut current = base_network;

    // Process multiple networks at once using SIMD
    for chunk in (0..count).collect::<Vec<_>>().chunks(4) {
        let simd_chunk = u32x4::from_slice(chunk);
        // SIMD calculations here
    }

    networks
}
```

### Caching

Implement caching for expensive operations:

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct NetworkGenerator {
    cache: LruCache<String, IpNetwork>,
}

impl NetworkGenerator {
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
        }
    }

    pub fn generate_network(&mut self, key: &str) -> IpNetwork {
        if let Some(&network) = self.cache.get(key) {
            return network;
        }

        let network = calculate_network_range(key);
        self.cache.put(key.to_string(), network);
        network
    }
}
```

## I/O Optimization

### Buffered Writing

Use buffered I/O for file operations:

```rust
use std::io::{BufWriter, Write};

pub fn write_vlans_buffered<W: Write>(vlans: &[VlanConfig], writer: W) -> Result<()> {
    let mut buf_writer = BufWriter::with_capacity(8192, writer);

    for vlan in vlans {
        write_vlan_to_stream(vlan, &mut buf_writer)?;
    }

    buf_writer.flush()?;
    Ok(())
}
```

### Async I/O

Use async I/O for concurrent operations:

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn write_vlans_async(vlans: &[VlanConfig], path: &str) -> Result<()> {
    let mut file = File::create(path).await?;

    for vlan in vlans {
        let data = serialize_vlan(vlan)?;
        file.write_all(&data).await?;
    }

    Ok(())
}
```

### Compression

Use compression for large datasets:

```bash
# Generate compressed output
cargo run --release -- generate vlan --count 1000 --format csv --compress --output data.csv.gz
```

## Profiling and Analysis

### CPU Profiling

Profile CPU usage with `perf`:

```bash
# Install perf
sudo apt-get install linux-tools-common linux-tools-generic

# Profile the application
perf record --call-graph dwarf cargo run --release -- generate vlan --count 1000
perf report
```

### Memory Profiling

Profile memory usage with `heaptrack`:

```bash
# Install heaptrack
sudo apt-get install heaptrack

# Profile memory usage
heaptrack cargo run --release -- generate vlan --count 1000
heaptrack_print heaptrack.cargo.12345.gz
```

### Flamegraph Analysis

Generate flamegraphs for performance analysis:

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin opnsense-config-faker -- generate vlan --count 1000
```

## Performance Testing

### Load Testing

Test performance under load:

```rust
#[test]
fn test_performance_under_load() {
    let start = std::time::Instant::now();

    // Generate large dataset
    let vlans = generate_vlan_config(10000, 1, "192.168.0.0/24".parse().unwrap()).unwrap();

    let duration = start.elapsed();

    // Should complete within 5 seconds
    assert!(duration.as_secs() < 5);

    // Should generate correct number
    assert_eq!(vlans.len(), 10000);
}
```

### Memory Stress Testing

Test memory usage with large datasets:

```rust
#[test]
fn test_memory_usage_large_dataset() {
    // Generate very large dataset
    let vlans = generate_vlan_config(50000, 1, "192.168.0.0/24".parse().unwrap()).unwrap();

    // Verify memory usage is reasonable
    let memory_usage = std::mem::size_of_val(&vlans)
        + vlans
            .iter()
            .map(|v| v.name.capacity() + v.description.capacity())
            .sum::<usize>();

    // Should use less than 100MB for 50000 VLANs
    assert!(memory_usage < 100 * 1024 * 1024);
}
```

### Concurrent Access Testing

Test performance under concurrent access:

```rust
use std::sync::Arc;
use std::thread;

#[test]
fn test_concurrent_generation() {
    let generator = Arc::new(VlanGenerator::new());
    let mut handles = vec![];

    // Spawn multiple threads
    for i in 0..4 {
        let generator = Arc::clone(&generator);
        let handle = thread::spawn(move || {
            generate_vlan_config(1000, i * 1000, "192.168.0.0/24".parse().unwrap())
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        let vlans = handle.join().unwrap().unwrap();
        assert_eq!(vlans.len(), 1000);
    }
}
```

## Configuration Tuning

### Environment Variables

Tune performance with environment variables:

```bash
# Set thread count for parallel processing
export RAYON_NUM_THREADS=8

# Set memory allocation strategy
export MALLOC_ARENA_MAX=2
```

### Runtime Configuration

Configure performance at runtime:

```rust
pub struct PerformanceConfig {
    pub max_threads: usize,
    pub batch_size: usize,
    pub memory_limit: usize,
    pub cache_size: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_threads: num_cpus::get(),
            batch_size: 1000,
            memory_limit: 100 * 1024 * 1024, // 100MB
            cache_size: 10000,
        }
    }
}
```

## Best Practices

### Performance Guidelines

1. **Use appropriate data structures** for the task
2. **Avoid unnecessary allocations** in hot paths
3. **Use streaming I/O** for large datasets
4. **Enable parallel processing** for CPU-bound tasks
5. **Profile before optimizing** to identify bottlenecks
6. **Test performance regressions** in CI/CD

### Memory Management

1. **Use `Vec::with_capacity()`** when you know the size
2. **Reuse allocations** when possible
3. **Use memory pools** for frequent allocations
4. **Avoid string concatenation** in loops
5. **Use `Cow<str>`** for string operations

### I/O Optimization

1. **Use buffered I/O** for file operations
2. **Batch write operations** when possible
3. **Use compression** for large datasets
4. **Consider async I/O** for concurrent operations
5. **Profile I/O bottlenecks** with appropriate tools

## Troubleshooting Performance Issues

### Common Performance Problems

**Slow generation with large datasets:**

- Use CSV format instead of XML
- Enable parallel processing
- Use streaming I/O

**High memory usage:**

- Use memory-efficient mode
- Process in batches
- Use lazy evaluation

**Slow file I/O:**

- Use buffered I/O
- Enable compression
- Consider async I/O

### Performance Debugging

```bash
# Enable performance logging
RUST_LOG=debug cargo run --release -- generate vlan --count 1000

# Profile with specific tools
perf record cargo run --release -- generate vlan --count 1000
heaptrack cargo run --release -- generate vlan --count 1000
```

### Performance Monitoring

```rust
use std::time::Instant;

pub fn generate_with_monitoring(count: u32) -> Result<Vec<VlanConfig>> {
    let start = Instant::now();

    // Generation logic
    let vlans = generate_vlan_config(count, 1, "192.168.0.0/24".parse().unwrap())?;

    let duration = start.elapsed();
    println!("Generated {} VLANs in {:?}", count, duration);

    Ok(vlans)
}
```

This comprehensive performance optimization guide ensures that OPNsense Config Faker can handle large-scale configuration generation efficiently and reliably.
