# Code Quality

This document outlines the code quality standards and practices for the OPNsense Config Faker project.

## Quality Standards

### Zero Warnings Policy

All code must compile with zero warnings:

```bash
# This command must pass with zero warnings
cargo clippy --all-targets --all-features --benches -- -D warnings
```

### Code Formatting

Consistent code formatting using `cargo fmt`:

```bash
# Format all code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check
```

### Testing Requirements

Comprehensive testing coverage:

```bash
# Run all tests
cargo test --all-features

# Run with coverage
cargo llvm-cov --all-features --workspace --fail-under-lines 80
```

## Rust Code Quality

### Clippy Configuration

The project uses strict clippy settings:

```toml
# Cargo.toml - Workspace level clippy configuration
[workspace.lints.clippy]
# Mandatory lints - treat as errors
all = "deny"
correctness = "deny"
suspicious = "deny"
complexity = "deny"
perf = "deny"
style = "warn"
pedantic = "warn"
nursery = "warn"
cargo = "warn"
```

### Performance Lints

Specific performance-related linting:

```toml
# Performance lints
inefficient_to_string = "deny"
large_enum_variant = "deny"
large_types_passed_by_value = "warn"
linkedlist = "deny"
mutex_atomic = "deny"
naive_bytecount = "deny"
or_fun_call = "deny"
slow_vector_initialization = "deny"
stable_sort_primitive = "deny"
zero_sized_map_values = "deny"
```

### Correctness and Safety

Safety-focused linting:

```toml
# Correctness and safety
clone_on_ref_ptr = "deny"
cmp_null = "deny"
drop_copy = "deny"
drop_ref = "deny"
forget_copy = "deny"
forget_ref = "deny"
mem_forget = "deny"
mem_replace_with_default = "deny"
unneeded_field_pattern = "deny"
unused_self = "deny"
```

## Code Organization

### File Structure

Follow the established project structure:

```text
src/
├── cli/               # Command line interface
├── generator/         # Data generation logic
├── io/               # Input/output handling
├── model/            # Data models
├── validate/         # Validation logic
├── xml/              # XML processing
└── lib.rs            # Library entry point
```

### Module Organization

```rust
// File organization template
//! Module-level documentation
//!
//! Brief description of the module's purpose and responsibilities.

// Standard library imports first
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// External crate imports second, grouped by crate
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Internal imports last
use crate::generators::{NetworkRange, VlanGenerator};
use crate::models::{FirewallRule, VlanConfig};

// Constants and type aliases
const MAX_VLAN_ID: u16 = 4094;
type Result<T> = std::result::Result<T, ConfigGenerationError>;

// Public types first
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    pub vlans: Vec<VlanConfig>,
}

// Private types second
#[derive(Debug)]
struct ConfigurationBuilder {
    vlans: Vec<VlanConfig>,
}

// Implementations
impl NetworkConfiguration {
    /// Creates a new network configuration
    pub fn new() -> Self {
        Self { vlans: Vec::new() }
    }
}

// Tests at the end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_configuration_creation() {
        let config = NetworkConfiguration::new();
        assert!(config.vlans.is_empty());
    }
}
```

## Error Handling Quality

### Error Type Standards

Use `thiserror` for all error types:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigGenerationError {
    /// Network configuration errors
    #[error("Invalid VLAN ID: {id}. Must be between 1 and {max}")]
    InvalidVlanId { id: u16, max: u16 },

    #[error("Network range conflict: {range1} conflicts with {range2}")]
    NetworkRangeConflict { range1: String, range2: String },

    /// File and I/O errors
    #[error("Failed to write {format} output to {path}")]
    OutputWriteFailed {
        format: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
```

### Result Type Usage

Consistent `Result<T, E>` types:

```rust
pub type Result<T> = std::result::Result<T, ConfigGenerationError>;

pub fn generate_vlan_config(count: u32, base_id: u16) -> Result<Vec<VlanConfig>> {
    if base_id == 0 || base_id > MAX_VLAN_ID {
        return Err(ConfigGenerationError::InvalidVlanId {
            id: base_id,
            max: MAX_VLAN_ID,
        });
    }

    // Implementation...
    Ok(vlans)
}
```

## Documentation Quality

### Documentation Standards

Comprehensive documentation for all public APIs:

````rust
/// Generates realistic VLAN configurations for OPNsense testing.
///
/// This function creates VLAN configurations that comply with IEEE 802.1Q
/// standards and can be imported into OPNsense for comprehensive network
/// testing scenarios.
///
/// # Arguments
///
/// * `count` - Number of VLANs to generate (1-4094)
/// * `base_id` - Starting VLAN ID for sequential generation
/// * `base_network` - Base network range for VLAN subnets
///
/// # Returns
///
/// Returns `Ok(Vec<VlanConfig>)` containing valid VLAN configurations, or an error
/// if the parameters would result in invalid VLAN IDs or network conflicts.
///
/// # Errors
///
/// This function will return an error if:
/// - `base_id` is 0 or would cause VLAN ID overflow beyond 4094
/// - `count` is 0 or would result in too many VLANs
/// - Network range calculations result in invalid subnets
///
/// # Examples
///
/// ```rust
/// use opnsense_config_faker::generators::generate_vlan_config;
/// use ipnet::IpNet;
///
/// let base_network: IpNet = "192.168.100.0/24".parse()?;
/// let vlans = generate_vlan_config(5, 100, base_network)?;
/// assert_eq!(vlans.len(), 5);
/// ```
pub fn generate_vlan_config(
    count: u32,
    base_id: u16,
    base_network: IpNet,
) -> Result<Vec<VlanConfig>> {
    // Implementation...
}
````

## Testing Quality

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Test helper for creating valid test configurations
    fn create_test_vlan(id: u16) -> VlanConfig {
        VlanConfig::new(
            id,
            format!("TestVLAN{}", id),
            "em0".to_string(),
            "192.168.1.0/24".parse().unwrap(),
        )
        .unwrap()
    }

    /// Test basic VLAN configuration validation
    #[test]
    fn test_vlan_id_validation() {
        let result = VlanConfig::new(
            100,
            "Test VLAN".to_string(),
            "em0".to_string(),
            "192.168.1.0/24".parse().unwrap(),
        );
        assert!(result.is_ok());
    }

    /// Property-based testing for edge cases
    proptest! {
        #[test]
        fn test_vlan_generation_properties(
            count in 1..100u32,
            base_id in 1..4000u16
        ) {
            let base_network = "192.168.0.0/24".parse().unwrap();
            let vlans = generate_vlan_config(count, base_id, base_network);

            if let Ok(vlans) = vlans {
                prop_assert_eq!(vlans.len(), count as usize);

                // Verify all VLAN IDs are in valid range
                for vlan in &vlans {
                    prop_assert!(vlan.id >= 1 && vlan.id <= 4094);
                }
            }
        }
    }
}
```

## Quality Gates

### Pre-commit Checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo test` passes with no failures
- [ ] `cargo audit` shows no vulnerabilities
- [ ] Documentation updated if public API changed
- [ ] Benchmark performance within acceptable range

### CI Quality Checks

```bash
#!/bin/bash
# scripts/quality-check.sh

set -euo pipefail

echo "🔍 Running comprehensive quality checks..."

# 1. Formatting check
echo "📏 Checking code formatting..."
cargo fmt --all -- --check

# 2. Clippy with zero warnings
echo "🔧 Running Clippy with strict linting..."
cargo clippy --all-targets --all-features --benches -- -D warnings

# 3. Test execution
echo "🧪 Running all tests..."
cargo test --all-features --verbose

# 4. Documentation tests
echo "📚 Running documentation tests..."
cargo test --doc --all-features

# 5. Coverage check
echo "📊 Checking test coverage..."
cargo llvm-cov --all-features --workspace --fail-under-lines 80

echo "✅ All quality checks passed!"
```

## Performance Quality

### Benchmarking Standards

- All benchmarks must run in under 30 seconds
- Memory usage should be profiled for configurations > 1000 VLANs
- Performance regressions > 20% require investigation
- Benchmark results documented in commit messages

### Performance Testing

```rust
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_vlan_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("vlan_generation");

    for count in [10, 100, 1000].iter() {
        group.bench_with_input(format!("generate_{}_vlans", count), count, |b, &count| {
            b.iter(|| generate_vlan_config(black_box(count), 1, "192.168.0.0/24".parse().unwrap()))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_vlan_generation);
criterion_main!(benches);
```

## Code Review Guidelines

### What to Check

- **Network Validity**: Generated configurations are technically correct
- **CLI Usability**: Commands are intuitive with clear help
- **Rust Quality**: Zero clippy warnings, proper error handling
- **Testing**: Unit tests, integration tests, property tests
- **Performance**: Efficient data generation with reasonable memory usage

### Common Anti-Patterns to Avoid

```rust
// ❌ Avoid: Generating invalid VLAN IDs
let vlan_id = random_u16(); // Could be 0 or >4094

// ✅ Correct: Generate within valid range
let vlan_id = rng.gen_range(1..=4094);

// ❌ Avoid: Hardcoded network ranges
let network = "192.168.1.0/24";

// ✅ Correct: Configurable, non-conflicting ranges
let network = generate_test_network_range(base_network, subnet_size);

// ❌ Avoid: Unclear error messages
return Err("Invalid input".into());

// ✅ Correct: Actionable error messages
return Err(ConfigGenerationError::InvalidVlanCount {
    count,
    max: MAX_VLANS,
    suggestion: "Reduce the count or split into multiple files".to_string()
});
```

## Quality Metrics

### Code Quality Metrics

- **Cyclomatic Complexity**: Maximum 15 per function
- **Test Coverage**: Minimum 80% line coverage
- **Documentation Coverage**: 100% for public APIs
- **Clippy Warnings**: Zero warnings policy
- **Performance Regressions**: \<10% performance degradation per release

### Quality Dashboard

```toml
# Cargo.toml - Quality measurement tools
[dev-dependencies]
criterion = { version = "0.7", features = ["html_reports"] }
cargo-llvm-cov = "0.6"
rstest = "0.26"
proptest = "1.4"
assert_cmd = "2.0"
assert_fs = "1.1"

[package.metadata.coverage]
min-coverage = 80
exclude-files = ["tests/*", "benches/*", "examples/*"]
```

## Best Practices

### Do's

- ✅ Use `thiserror` for all error types
- ✅ Include relevant context in error messages
- ✅ Preserve error chains with `#[from]` attributes
- ✅ Add `.context()` to error operations in CLI code
- ✅ Test error conditions comprehensively
- ✅ Provide actionable error messages
- ✅ Use structured logging for debugging

### Don'ts

- ❌ Use `.unwrap()` in production code
- ❌ Ignore error conditions
- ❌ Provide vague error messages
- ❌ Lose error context in conversions
- ❌ Skip error testing
- ❌ Use generic error types when specific ones are available

This comprehensive quality framework ensures that OPNsense Config Faker maintains the highest standards of code quality, network configuration validity, and maintainability throughout its development lifecycle.
