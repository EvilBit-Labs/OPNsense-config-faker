# Test Utilities for OPNsense Config Faker

This directory contains shared test utilities and fixtures that provide consistent testing patterns across integration tests, snapshot tests, and compatibility tests.

## Structure

- **`common/mod.rs`** - Shared test utilities and helper functions
- **`integration_tests.rs`** - Integration tests for CLI functionality
- **`snapshot_tests.rs`** - Snapshot tests using insta for consistent output validation
- **`compatibility_tests.rs`** - Cross-platform and environment compatibility tests

## Key Features

The shared test utilities provide the following capabilities:

### 1. Standardized CLI Testing (`cli_command()`)

The `cli_command()` helper automatically sets up a consistent test environment:

- `TERM=dumb` - Disables Rich terminal formatting
- `CARGO_TERM_COLOR=never` - Disables Cargo colored output
- `NO_COLOR=1` - Disables all color output

```rust
use common::{cli_command, TestOutputExt};

let output = cli_command()
    .arg("generate")
    .arg("--format")
    .arg("csv")
    .arg("--count")
    .arg("5")
    .run_success();

output.assert_stdout_contains("Generated 5 VLAN configurations");
```

### 2. ANSI Strip and Output Normalization

The `normalize_output()` function removes ANSI escape sequences and normalizes whitespace for stable test assertions:

```rust
use common::normalize_output;

let raw_output = "\u001b[32m✅ Success\u001b[0m\n  Multiple   spaces\t\n";
let clean = normalize_output(raw_output);
assert_eq!(clean, "✅ Success Multiple spaces");
```

### 3. Temporary File and Directory Creation

Multiple helpers for creating temporary test resources:

```rust
use common::{create_temp_dir, create_temp_csv, create_temp_xml};

// Basic temporary directory
let temp_dir = create_temp_dir("test_prefix");
let file_path = temp_dir.path().join("test_file.csv");

// CSV with test data
let (temp_file, csv_path) = create_temp_csv("test_", &[
    &["VLAN", "IP Range", "Description"],
    &["100", "192.168.1.0/24", "Test Network"],
]).unwrap();

// XML configuration file
let xml_content = r#"<?xml version="1.0"?>
<opnsense>
  <interfaces>
    <lan>
      <if>em0</if>
    </lan>
  </interfaces>
</opnsense>"#;
let (temp_file, xml_path) = create_temp_xml("config_", xml_content).unwrap();
```

### 4. Extended Test Output Assertions

The `TestOutputExt` trait provides additional assertion methods:

```rust
output
    .assert_stdout_contains("success message")
    .assert_stderr_contains("warning message")
    .assert_stdout_matches(r"Generated \d+ configurations");

// Access normalized output
let clean_stdout = output.normalized_stdout();
let clean_stderr = output.normalized_stderr();
let combined = output.normalized_combined();
```

## Test Categories

### Integration Tests

Test core CLI functionality with real command execution:

```bash
cargo test --test integration_tests
```

### Snapshot Tests

Capture and validate CLI output for consistency across changes:

```bash
cargo test --test snapshot_tests
```

Update snapshots when output legitimately changes:

```bash
cargo insta review
```

### Compatibility Tests

Test CLI behavior across different environments and edge cases:

```bash
cargo test --test compatibility_tests
```

## Running All Tests

```bash
# Run all test types
cargo test

# Run only tests using shared utilities
cargo test common::tests

# Run with specific environment settings
TERM=dumb CARGO_TERM_COLOR=never cargo test
```

## Writing New Tests

When writing new tests, prefer using the shared utilities for consistency:

1. Use `cli_command()` for CLI execution instead of raw `Command`
2. Use `normalize_output()` for stable output assertions
3. Use temp file helpers instead of manual temp file creation
4. Use `TestOutputExt` trait methods for rich assertions

### Example Test Pattern

```rust
#[test]
fn test_feature_xyz() {
    // Setup
    let temp_dir = create_temp_dir("feature_test");
    let output_file = temp_dir.path().join("output.csv");

    // Execute
    let output = cli_command()
        .arg("feature")
        .arg("--output")
        .arg(&output_file)
        .run_success();

    // Verify CLI output
    output.assert_stdout_contains("Feature completed successfully");

    // Verify file output
    assert!(output_file.exists());
    let content = std::fs::read_to_string(&output_file).unwrap();
    assert!(content.contains("expected_content"));
}
```

## Dependencies

The test utilities use the following key dependencies:

- **assert_cmd** - CLI command testing
- **assert_fs** - Advanced file system testing
- **tempfile** - Temporary file and directory creation
- **regex** - Pattern matching and ANSI sequence removal
- **insta** - Snapshot testing (snapshot tests only)

These are automatically available in the test environment through the dev-dependencies in `Cargo.toml`.
