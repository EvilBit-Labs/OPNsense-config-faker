# Property-Based Testing for VLAN Configuration Generation

This document describes how to run and interpret the property-based tests for the VLAN configuration generation system using proptest.

## Running the Tests

### Regular Tests

Run the standard property-based tests with reasonable performance:

```bash
cargo test --test proptest_vlan
```

### Slow Tests

Run the comprehensive tests including very large cases (may take longer):

```bash
cargo test --test proptest_vlan --features slow-tests
```

### Individual Test Functions

Run a specific property test:

```bash
# Test core invariants
cargo test test_vlan_generation_invariants

# Test determinism
cargo test test_vlan_generation_determinism

# Test uniqueness properties
cargo test test_uniqueness_properties
```

## What the Tests Validate

### Core Invariants (`test_vlan_generation_invariants`)

For any count in [1, 1000] and any seed, validates that `generate_vlan_configurations(count, Some(seed), None)` produces:

- **VLAN ID Uniqueness**: All `vlan_id` values are unique and within range `10..=4094`
- **IP Network Uniqueness**: All `ip_network` values are unique and follow expected format (`10.x.y.x` or `10.x.y.0/24`)
- **WAN Assignment Range**: All `wan_assignment` values are within range `[1,3]`
- **IP Derivation Success**: Gateway IP and DHCP range derivations succeed and are within the /24 network

### Determinism (`test_vlan_generation_determinism`)

Validates that the same seed + same count produces identical sequences of configurations on multiple runs.

### Uniqueness Properties (`test_uniqueness_properties`)

Fast execution tests focusing specifically on uniqueness constraints with smaller counts.

## Slow Tests Feature

Tests gated behind the `slow-tests` feature (enabled with `--features slow-tests`):

- **Large Count Invariants**: Tests with counts 500-1000 (may be slower)
- **Large Count Determinism**: Determinism testing with large counts

## Test Coverage

The property-based tests complement the existing unit tests by:

1. **Randomized Input Testing**: Tests with thousands of different input combinations
2. **Invariant Validation**: Ensures core properties hold regardless of input
3. **Scalability Testing**: Validates behavior with large datasets
4. **Regression Prevention**: Catches edge cases that might be missed by unit tests

## Performance Considerations

- Regular tests use counts up to 1000 for good coverage without excessive runtime
- Slow tests extend to larger counts but are optional for regular development
- Proptest automatically manages test case generation and shrinking on failure

## Integration with CI/CD

```bash
# Standard CI pipeline
cargo test --test proptest_vlan

# Extended testing (optional, for nightly builds)
cargo test --test proptest_vlan --features slow-tests
```

The tests are designed to run efficiently in CI environments while providing comprehensive coverage of the VLAN generation system's core invariants.
