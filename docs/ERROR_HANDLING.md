# Error Handling Conventions

This document outlines the error handling patterns and conventions used throughout the OPNsense Config Faker project.

## Architecture Overview

The project uses a layered error handling approach:

```text
Library Layer (src/lib.rs and modules):
├── Use thiserror for all error types ✅
├── Return domain-specific Results
└── Provide rich error context

Binary Layer (src/main.rs and CLI):
├── Use anyhow::Result for main()
├── Add .context() for additional debugging info
└── Aggregate errors from multiple sources
```

## Library Code Error Handling

### Error Type Standards

All library error types use `thiserror` for automatic `Display` and `Error` trait implementations:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid VLAN ID: {id}. Must be between 1 and {max}")]
    InvalidVlanId { id: u16, max: u16 },

    #[error("Network range conflict: {range1} conflicts with {range2}")]
    NetworkRangeConflict { range1: String, range2: String },

    #[error("XML generation failed")]
    XmlGenerationFailed(#[from] quick_xml::Error),
}
```

### Result Type Usage

Use consistent `Result<T, E>` types throughout the library:

```rust
pub type Result<T> = std::result::Result<T, ConfigError>;

pub fn generate_vlan_config(count: u32, base_id: u16) -> Result<Vec<VlanConfig>> {
    if base_id == 0 || base_id > MAX_VLAN_ID {
        return Err(ConfigError::InvalidVlanId {
            id: base_id,
            max: MAX_VLAN_ID,
        });
    }

    // Implementation...
    Ok(vlans)
}
```

### Error Context Guidelines

- Include relevant parameters in error messages
- Provide actionable suggestions when possible
- Preserve error source chain using `#[from]` attributes
- Use descriptive error variants for different failure modes

## Binary/CLI Code Error Handling

### anyhow Integration

The binary layer uses `anyhow` for error aggregation and context preservation:

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up environment with context
    setup_environment(&cli).context("Failed to setup CLI environment")?;

    // Execute command with rich context
    match cli.command {
        Commands::Generate(args) => {
            generate::execute(args).context("Failed to generate configurations")?
        } // ... other commands
    }

    Ok(())
}
```

### Context Preservation

Add context to error operations using `.context()` and `.with_context()`:

```rust
pub fn execute(args: GenerateArgs) -> Result<()> {
    let configs = generate_vlan_configurations(args.count, args.seed, None)
        .with_context(|| format!("Failed to generate {} VLAN configurations", args.count))?;

    write_csv(&configs, &args.output)
        .with_context(|| format!("Failed to write CSV to {:?}", args.output))?;

    Ok(())
}
```

### CLI-Specific Error Types

Use `CliError` for CLI-specific error handling:

```rust
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid command-line argument: {0}")]
    InvalidArgument(String),

    #[error("Interactive mode failed: {0}")]
    InteractiveModeError(String),

    #[error(transparent)]
    Config(#[from] crate::model::ConfigError),
}
```

## Error Message Guidelines

### Structure

Error messages should follow this structure:

1. **Action**: What operation failed
2. **Context**: Relevant parameters or conditions
3. **Suggestion**: Actionable remediation steps

### Examples

```rust
// ✅ Good: Clear action, context, and suggestion
"Failed to generate 5000 VLAN configurations: VLAN ID pool exhausted (max: 4085). Reduce count or use CSV format for duplicates."

// ✅ Good: File operation with path context
"Failed to write CSV to '/nonexistent/path/test.csv': Permission denied. Check directory permissions and try again."

// ❌ Bad: Vague error message
"Error occurred during processing."
```

### Network Configuration Specific

For network-related errors, include technical details:

```rust
// VLAN ID errors
"Invalid VLAN ID: 5000. Must be between 1 and 4094. Use --base-id with a valid range."

// Network range conflicts
"Network range conflict: 192.168.1.0/24 overlaps with 192.168.1.0/25. Use --network-base to specify a different range."

// XML schema validation
"XML schema validation failed: Missing required element 'vlan' at path '/opnsense/vlans'. Check base configuration template."
```

## Error Testing

### Unit Tests

Test error conditions in unit tests:

```rust
#[test]
fn test_vlan_id_validation() {
    let result = VlanConfig::new(
        0,
        "Test".to_string(),
        "em0".to_string(),
        "192.168.1.0/24".parse().unwrap(),
    );
    assert!(matches!(
        result,
        Err(ConfigError::InvalidVlanId { id: 0, .. })
    ));
}
```

### Integration Tests

Test error handling in CLI commands:

```rust
#[test]
fn test_cli_error_context() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate").arg("--count").arg("99999");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "Failed to generate configurations",
        ))
        .stderr(predicate::str::contains("99999"));
}
```

### Property-Based Tests

Use property-based testing for error edge cases:

```rust
proptest! {
    #[test]
    fn test_invalid_vlan_id_range(id in 0u16..1u16) {
        let result = VlanConfig::new(id, "Test".to_string(), "em0".to_string(), "192.168.1.0/24".parse().unwrap());
        prop_assert!(result.is_err());
    }
}
```

## Error Logging

### Error Output

Use `eprintln!` for error output to stderr:

```rust
// Option 1: Using the log crate
use log::{error, info};

error!("Failed to generate VLAN configurations: count={}, base_id={}, error={}", 
    args.count, args.base_id, e);

// Option 2: Using tracing crate
use tracing::{error, info};

error!(count = args.count, base_id = args.base_id, error = ?e, 
    "Failed to generate VLAN configurations");
```

### Console Styling for User-Facing Messages

Use `console::style` for styled error messages in CLI output:

```rust
use console::style;

// Styled error message for user display
eprintln!(
    "{} {}",
    style("❌ Error:").red().bold(),
    style("Failed to generate VLAN configurations").red()
);

// Styled warning message
eprintln!(
    "{} {}",
    style("⚠️  Warning:").yellow().bold(),
    style("Some configurations may be invalid").yellow()
);
```

### Error Chain Preservation

Preserve the full error chain for debugging:

```rust
// The error chain is automatically preserved by anyhow
// Users can access the full chain with .chain()
for error in e.chain() {
    eprintln!("  Caused by: {}", error);
}
```

## Common Error Patterns

### File Operations

```rust
// File reading with context
let content = std::fs::read_to_string(&path)
    .with_context(|| format!("Failed to read file: {:?}", path))?;
```

### Network Configuration Validation

```rust
vlan.validate()
    .with_context(|| format!("VLAN {} validation failed", vlan.id))?
```

### Argument Validation

```rust
args.validate()
    .map_err(|e| CliError::invalid_argument(e))?
```

### Progress Indicator Creation

```rust
// ProgressBar::new is infallible - no error handling needed
let pb = ProgressBar::new(count);
```

## Error Recovery Strategies

### Graceful Degradation

When possible, provide fallback behavior:

```rust
// Detect dumb terminal and provide appropriate fallback
// Note: unwrap_or_default() is applied to env::var(), not ProgressBar::new()
let pb = if std::env::var("TERM").unwrap_or_default() == "dumb" {
    ProgressBar::hidden()  // Hidden for dumb terminals
} else {
    ProgressBar::new(count)  // Visible progress bar for interactive terminals
};

// Alternative: More comprehensive terminal detection
use std::env;
let pb = if env::var("NO_COLOR").is_ok() 
    || env::var("TERM").unwrap_or_default() == "dumb"
    || !atty::is(atty::Stream::Stderr) {
    ProgressBar::hidden()
} else {
    ProgressBar::new(count)
};

// Alternative: Use isatty/atty crate for robust terminal detection
use atty::Stream;
let pb = if atty::is(Stream::Stdout) && atty::is(Stream::Stderr) {
    ProgressBar::new(count)  // Interactive terminal
} else {
    ProgressBar::hidden()    // Non-interactive (pipes, redirects, etc.)
};
```

### User-Friendly Messages

Convert technical errors to user-friendly messages:

```rust
match error {
    ConfigError::InvalidVlanId { id, max } => {
        format!("VLAN ID {} is invalid. Use a value between 1 and {}.", id, max)
    }
    ConfigError::NetworkRangeConflict { range1, range2 } => {
        format!("Network ranges {} and {} conflict. Use different ranges.", range1, range2)
    }
    _ => error.to_string(),
}
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

## Migration Guide

### From Library Result to anyhow Result

```rust
// Before: Library Result
pub fn execute(args: GenerateArgs) -> crate::Result<()> {
    let configs = generate_vlan_configurations(args.count, args.seed, None)?;
    write_csv(&configs, &args.output)?;
    Ok(())
}

// After: anyhow Result with context
pub fn execute(args: GenerateArgs) -> anyhow::Result<()> {
    let configs = generate_vlan_configurations(args.count, args.seed, None)
        .with_context(|| format!("Failed to generate {} VLAN configurations", args.count))?;

    write_csv(&configs, &args.output)
        .with_context(|| format!("Failed to write CSV to {:?}", args.output))?;

    Ok(())
}
```

### Adding Error Context

```rust
// Before: Basic error propagation
let content = fs::read_to_string(&path)?;

// After: Rich error context
let content = fs::read_to_string(&path)
    .with_context(|| format!("Failed to read configuration file: {:?}", path))?;
```

This comprehensive error handling framework ensures that users receive clear, actionable error messages while maintaining full error context for debugging and development.
