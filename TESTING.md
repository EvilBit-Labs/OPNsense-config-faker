# Testing Guide for OPNsense Config Faker

This document provides comprehensive guidance on running the various types of tests in this project and understanding the testing infrastructure.

## Quick Start

```bash
# Run all tests
cargo test --all-features

# Run all tests with environment normalization
TERM=dumb cargo test --all-features

# Run tests with coverage
just coverage

# Run QA pipeline (format check, lint, test)
just qa
```

## CI Environment Considerations

### TERM=dumb Support

When running in CI environments, the `TERM=dumb` environment variable is automatically respected by various tools to disable color output and interactive features:

- **Rich Library**: Automatically respects `TERM="dumb"` to disable color output
- **Cargo**: Respects terminal capabilities and adjusts output accordingly
- **Test Output**: All test runners adapt to non-interactive terminal environments

This ensures consistent, parseable output in CI pipelines without requiring special configuration.

### CI-Friendly Tasks

Use the following justfile tasks for CI environments:

```bash
# Standard QA pipeline (respects TERM=dumb)
just ci-qa

# Full CI validation with coverage
just ci-check

# Fast CI validation without coverage
just ci-check-fast
```

## Test Categories

### Unit Tests

Run core library functionality tests:

```bash
# Run all unit tests
cargo test --lib --all-features

# Run specific unit test module
cargo test --lib module_name

# Run unit tests with output
cargo test --lib --all-features -- --nocapture
```

### Integration Tests

Test CLI functionality with real command execution:

```bash
# Run all integration tests
cargo test --test '*' --all-features

# Run specific integration test file
cargo test --test integration_cli

# Run integration tests with environment normalization
TERM=dumb cargo test --test integration_cli --all-features
```

### Property-Based Tests (PropTest)

Run property-based testing for data generation:

```bash
# Run all property tests
cargo test proptest --all-features

# Run VLAN generation property tests
cargo test --test proptest_vlan

# Run with more test cases (slow tests)
cargo test proptest --all-features --features slow-tests
```

### Snapshot Tests

Validate CLI output consistency using insta snapshots:

```bash
# Run all snapshot tests
cargo test --test snapshot_tests

# Run CSV snapshot tests
cargo test --test snapshot_csv

# Run XML snapshot tests  
cargo test --test snapshot_xml

# Run with environment normalization (recommended)
TERM=dumb cargo test --test snapshot_tests
```

#### Updating Snapshots

When CLI output legitimately changes, update snapshots:

```bash
# Review and approve snapshot changes
cargo insta review

# Accept all snapshot changes (use with caution)
INSTA_UPDATE=auto cargo test --test snapshot_tests

# Force update specific snapshots
cargo insta test --accept --test snapshot_tests
```

**Best Practices for Snapshots:**

- Always review snapshot changes before accepting
- Use `TERM=dumb` to ensure deterministic output
- Run tests multiple times to ensure stability
- Keep snapshots focused and readable
- Update documentation when snapshot behavior changes

### Python Compatibility Tests

Test interoperability with Python components:

```bash
# Run Python compatibility tests
cargo test --features python-compat

# Run specific Python compatibility test
cargo test --test python_compat --features python-compat

# Run Python compatibility with verbose output
cargo test --features python-compat -- --nocapture
```

## Quality Assurance Workflow

### Local Development

```bash
# Format, lint, and test
just qa

# Include coverage report
just qa-cov

# Development workflow with coverage
just dev
```

### CI Pipeline

```bash
# Standard CI QA check
just ci-qa

# Full CI validation
just ci-check
```

## Coverage and Quality Assurance

### Running Coverage

Generate test coverage reports:

```bash
# Basic coverage report
just coverage

# Coverage with 90% threshold enforcement
just coverage

# HTML coverage report (opens in browser)
just coverage-html

# CI-friendly coverage (ignores test failures)
just coverage-ci

# Terminal coverage report
just coverage-report
```

The project enforces a **90% coverage threshold**. Coverage reports are generated using `cargo-llvm-cov`.

### Linting and Formatting

The project follows strict linting policies:

```bash
# Run clippy with warnings as errors (project policy)
cargo clippy -- -D warnings

# Or use the just command
just lint

# Format code
cargo fmt
just format

# Check formatting without modifying files
cargo fmt --check
just format-check
```

**Clippy Policy**: All warnings are treated as errors (`-D warnings`). This ensures high code quality and consistency across the project.

### Complete QA Pipeline

```bash
# Full quality assurance check
just qa

# QA with coverage
just qa-cov

# CI-friendly QA check
just ci-qa
```

## Benchmarks

Run performance benchmarks:

```bash
# Run all benchmarks
cargo bench --all-features

# Or use just command
just bench

# Run specific benchmark
cargo bench vlan_generation

# Generate HTML reports
cargo bench --all-features
# Results in target/criterion/reports/index.html
```

Benchmarks are excluded from coverage reports and use the Criterion framework.

## Environment Variables and Deterministic Testing

### TERM=dumb

The `TERM=dumb` environment variable is crucial for deterministic testing:

```bash
# Disable terminal formatting for consistent output
TERM=dumb cargo test

# Why this matters:
# - Removes ANSI color codes from output
# - Ensures consistent formatting across different terminals
# - Required for reliable snapshot testing
# - Prevents Rich library formatting in CLI output
```

The Rich library (used in the Python components) automatically respects `TERM=dumb` to disable color output.

### Deterministic Seeds

Tests use fixed seeds for reproducible results:

```bash
# Some tests use deterministic random seeds
# This is handled automatically in test utilities
# See tests/common/mod.rs for implementation details

# Property tests use configurable seeds:
PROPTEST_CASES=1000 cargo test proptest
```

### Additional Environment Variables

```bash
# Disable colored output completely
NO_COLOR=1 cargo test

# Disable Cargo colored output
CARGO_TERM_COLOR=never cargo test

# Comprehensive environment normalization (recommended)
TERM=dumb CARGO_TERM_COLOR=never NO_COLOR=1 cargo test
```

## Test Environment Setup

### Prerequisites

```bash
# Install coverage tooling
just install-cov

# Full development setup
just setup
```

### Running Specific Test Types

```bash
# Unit tests only
just test-unit

# Integration tests only
just test-integration

# Documentation tests
just test-doc

# All tests excluding benchmarks
just test-no-bench
```

## Continuous Integration

The CI pipeline automatically:

1. **Validates Formatting**: `just rust-fmt-check`
2. **Runs Linting**: `just rust-clippy` with strict warnings
3. **Executes Tests**: `just rust-test` with all features
4. **Generates Coverage**: `just coverage-ci` with 90% threshold
5. **Respects Environment**: Adapts output based on `TERM` variable

## Test Data and Fixtures

- **Property-Based Testing**: Uses `proptest` for generating test data
- **Snapshot Testing**: Uses `insta` for CLI output validation
- **Fixtures**: Test data located in `tests/fixtures/`
- **Snapshots**: Expected outputs stored in `tests/snapshots/`

## Troubleshooting

### Coverage Issues

If coverage falls below 90%:

```bash
# View detailed coverage report
just coverage-html

# Clean coverage artifacts and retry
just coverage-clean
just coverage
```

### Test Failures in CI

1. Check that `TERM=dumb` is set in CI environment
2. Verify all dependencies are properly installed
3. Use `just ci-check-fast` for quicker feedback
4. Review snapshot differences with `cargo insta review`

## Best Practices

1. **Write Tests First**: Follow TDD principles for new features
2. **Use Property-Based Testing**: Leverage `proptest` for edge cases
3. **Snapshot Critical Outputs**: Use `insta` for CLI behavior verification
4. **Maintain Coverage**: Keep above 90% line coverage
5. **CI-Friendly Output**: Ensure all tools respect `TERM=dumb`
