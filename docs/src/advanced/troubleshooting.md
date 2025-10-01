# Troubleshooting

Common issues and solutions for OPNsense Config Faker.

## Common Issues

### Build Issues

#### "No such file or directory" errors

**Problem**: Build fails with file not found errors.

**Solutions**:

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Reinstall Rust toolchain
rustup update
rustup component add clippy rustfmt
```

#### Permission denied errors

**Problem**: Permission errors during build or execution.

**Solutions**:

```bash
# Check file permissions
ls -la target/release/opnsense-config-faker

# Fix permissions if needed
chmod +x target/release/opnsense-config-faker

# Run with proper permissions
sudo cargo run --release -- generate vlan --count 10
```

#### Network connectivity issues

**Problem**: Build fails due to network issues.

**Solutions**:

```bash
# Use offline mode if dependencies are cached
cargo build --offline

# Check network connectivity
ping crates.io

# Use alternative registry if needed
cargo build --registry alternative-registry
```

### Runtime Issues

#### "Invalid VLAN ID" errors

**Problem**: VLAN ID is outside valid range (1-4094).

**Solutions**:

```bash
# Use valid base ID
cargo run --release -- generate vlan --count 25 --base-id 100 --output vlans.xml

# Reduce count to fit within range
cargo run --release -- generate vlan --count 100 --base-id 1 --output vlans.xml

# Check current VLAN ID range
cargo run --release -- generate vlan --count 1 --base-id 4094 --output test.xml
```

#### "Network range conflict" errors

**Problem**: Generated network ranges overlap or conflict.

**Solutions**:

```bash
# Use different base network
cargo run --release -- generate vlan --count 20 --base-network 10.0.0.0/8 --output vlans.xml

# Use larger subnet size
cargo run --release -- generate vlan --count 15 --subnet-size 28 --output vlans.xml

# Generate with conflict detection
cargo run --release -- generate vlan --count 25 --check-conflicts --output vlans.xml
```

#### Memory issues with large datasets

**Problem**: Out of memory errors with large generations.

**Solutions**:

```bash
# Use CSV format for large datasets
cargo run --release -- generate vlan --count 1000 --format csv --output data.csv

# Use memory-efficient mode
cargo run --release -- generate vlan --count 5000 --memory-efficient --output data.csv

# Process in batches
cargo run --release -- generate vlan --count 2000 --batch-size 100 --output batched.xml
```

#### File I/O errors

**Problem**: Cannot write to output file.

**Solutions**:

```bash
# Check directory permissions
ls -la output/

# Create output directory
mkdir -p output/

# Use absolute path
cargo run --release -- generate vlan --count 10 --output /tmp/vlans.xml

# Check disk space
df -h
```

### Validation Issues

#### XML schema validation errors

**Problem**: Generated XML doesn't validate against OPNsense schema.

**Solutions**:

```bash
# Validate generated XML
cargo run --release -- validate --input config.xml

# Check XML structure
xmllint --noout config.xml

# Regenerate with validation
cargo run --release -- generate vlan --count 10 --validate --output config.xml
```

#### CSV parsing errors

**Problem**: Generated CSV has parsing issues.

**Solutions**:

```bash
# Check CSV format
head -5 data.csv

# Validate CSV structure
csvstat data.csv

# Regenerate with proper delimiter
cargo run --release -- generate vlan --count 25 --format csv --delimiter "," --output data.csv
```

#### JSON parsing errors

**Problem**: Generated JSON is malformed.

**Solutions**:

```bash
# Validate JSON syntax
jq . data.json

# Check JSON structure
cat data.json | jq keys

# Regenerate with pretty printing
cargo run --release -- generate vlan --count 25 --format json --pretty --output data.json
```

## Performance Issues

### Slow generation

**Problem**: Generation takes too long.

**Solutions**:

```bash
# Use CSV format for large datasets
cargo run --release -- generate vlan --count 1000 --format csv --output data.csv

# Enable parallel processing
cargo run --release --features rayon -- generate vlan --count 1000 --parallel --output data.xml

# Use streaming for very large datasets
cargo run --release -- generate vlan --count 5000 --stream --output data.csv
```

### High memory usage

**Problem**: Application uses too much memory.

**Solutions**:

```bash
# Use memory-efficient mode
cargo run --release -- generate vlan --count 1000 --memory-efficient --output data.csv

# Process in smaller batches
cargo run --release -- generate vlan --count 2000 --batch-size 100 --output data.xml

# Use CSV format instead of XML
cargo run --release -- generate vlan --count 1000 --format csv --output data.csv
```

### Slow file I/O

**Problem**: File operations are slow.

**Solutions**:

```bash
# Use buffered I/O
cargo run --release -- generate vlan --count 1000 --buffered --output data.csv

# Enable compression
cargo run --release -- generate vlan --count 1000 --compress --output data.csv.gz

# Use faster storage
cargo run --release -- generate vlan --count 1000 --output /tmp/data.csv
```

## Environment Issues

### Terminal compatibility

**Problem**: Output formatting issues in different terminals.

**Solutions**:

```bash
# Disable color output
NO_COLOR=1 cargo run --release -- generate vlan --count 10

# Use dumb terminal mode
TERM=dumb cargo run --release -- generate vlan --count 10

# Check terminal capabilities
echo $TERM
```

### Path issues

**Problem**: Cannot find output files or input files.

**Solutions**:

```bash
# Use absolute paths
cargo run --release -- generate vlan --count 10 --output /home/user/vlans.xml

# Check current directory
pwd

# List files in directory
ls -la

# Use relative paths correctly
cargo run --release -- generate vlan --count 10 --output ./output/vlans.xml
```

### Environment variables

**Problem**: Environment variables not set correctly.

**Solutions**:

```bash
# Check environment variables
env | grep -E "(RUST|CARGO|PATH)"

# Set required variables
export RUST_BACKTRACE=1
export CARGO_TERM_COLOR=always

# Check Rust installation
rustc --version
cargo --version
```

## Debugging

### Enable debug output

```bash
# Enable debug logging
RUST_LOG=debug cargo run --release -- generate vlan --count 10

# Enable trace logging
RUST_LOG=trace cargo run --release -- generate vlan --count 10

# Enable backtrace
RUST_BACKTRACE=1 cargo run --release -- generate vlan --count 10
```

### Verbose output

```bash
# Verbose CLI output
cargo run --release -- generate vlan --count 10 --verbose

# Show progress
cargo run --release -- generate vlan --count 1000 --progress

# Show statistics
cargo run --release -- generate vlan --count 1000 --stats
```

### Test with small datasets

```bash
# Test with minimal data
cargo run --release -- generate vlan --count 1 --output test.xml

# Validate test output
cargo run --release -- validate --input test.xml

# If test passes, try larger dataset
cargo run --release -- generate vlan --count 10 --output test.xml
```

## Getting Help

### Command help

```bash
# Show general help
cargo run --release -- --help

# Show subcommand help
cargo run --release -- generate --help

# Show specific option help
cargo run --release -- generate vlan --help
```

### Logging and diagnostics

```bash
# Enable comprehensive logging
RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release -- generate vlan --count 10

# Save logs to file
RUST_LOG=debug cargo run --release -- generate vlan --count 10 2>&1 | tee debug.log

# Check system resources
top -p $(pgrep opnsense-config-faker)
```

### Community support

- **GitHub Issues**: [Report bugs and request features](https://github.com/EvilBit-Labs/OPNsense-config-faker/issues)
- **Documentation**: Check the [User Guide](user-guide/) for detailed usage
- **Examples**: Review [Examples](user-guide/examples.md) for common use cases

## Prevention

### Best practices

1. **Always validate** generated configurations before use
2. **Test with small datasets** before generating large ones
3. **Use appropriate formats** for your use case
4. **Monitor system resources** during large generations
5. **Keep dependencies updated** regularly

### Quality checks

```bash
# Run quality checks before generating
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo audit

# Validate generated configurations
cargo run --release -- validate --input config.xml
```

### Regular maintenance

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Run security audit
cargo audit

# Clean build artifacts
cargo clean
```

## Recovery

### From failed generations

```bash
# Clean up partial files
rm -f *.xml *.csv *.json

# Start with small test
cargo run --release -- generate vlan --count 1 --output test.xml

# Validate test
cargo run --release -- validate --input test.xml

# If valid, proceed with larger generation
cargo run --release -- generate vlan --count 10 --output vlans.xml
```

### From corrupted configurations

```bash
# Validate existing configuration
cargo run --release -- validate --input config.xml

# If invalid, regenerate
cargo run --release -- generate vlan --count 10 --output new-config.xml

# Compare with original
diff config.xml new-config.xml
```

### From performance issues

```bash
# Profile the application
cargo install flamegraph
cargo flamegraph --bin opnsense-config-faker -- generate vlan --count 1000

# Check memory usage
cargo install heaptrack
heaptrack cargo run --release -- generate vlan --count 1000

# Optimize based on results
cargo run --release -- generate vlan --count 1000 --memory-efficient --output data.csv
```

This comprehensive troubleshooting guide helps resolve common issues and provides strategies for preventing problems in the future.
