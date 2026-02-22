# Basic Usage

Learn the fundamental concepts and commands for using OPNsense Config Faker.

## Command Structure

The tool follows a consistent command structure:

```bash
cargo run --release -- <command> <subcommand> [options]
```

### Main Commands

- `generate` - Generate configuration data
- `validate` - Validate existing configurations
- `help` - Show help information

## Generate Command

The `generate` command is the primary way to create OPNsense configurations.

### Basic Syntax

```bash
cargo run --release -- generate [options]
```

### Common Options

| Option      | Description                           | Default |
| ----------- | ------------------------------------- | ------- |
| `--count`   | Number of items to generate           | 10      |
| `--output`  | Output file path                      | stdout  |
| `--format`  | Output format (xml, csv, json)        | xml     |
| `--base-id` | Starting ID for sequential generation | 1       |

### Examples

```bash
# Generate 25 VLANs
cargo run --release -- generate vlan --count 25 --output vlans.xml

# Generate with custom base ID
cargo run --release -- generate vlan --count 10 --base-id 100 --output vlans.xml

# Generate CSV format
cargo run --release -- generate vlan --count 50 --format csv --output data.csv
```

## Output Formats

### XML Format (Default)

XML output creates valid OPNsense configuration files:

```bash
cargo run --release -- generate vlan --count 5 --output config.xml
```

**Use cases:**

- Direct import into OPNsense
- Complete configuration files
- Production-like testing

### CSV Format

CSV output provides structured data for processing:

```bash
cargo run --release -- generate vlan --count 20 --format csv --output data.csv
```

**Use cases:**

- Data analysis and processing
- Integration with other tools
- Custom processing pipelines

### JSON Format

JSON output for API integration:

```bash
cargo run --release -- generate vlan --count 15 --format json --output data.json
```

**Use cases:**

- API integration
- Web applications
- Configuration management systems

## Advanced Generation

### Firewall Rules

Generate configurations with firewall rules:

```bash
# Include firewall rules
cargo run --release -- generate --count 10 --include-firewall-rules --output config.xml

# Specify rule complexity
cargo run --release -- generate --count 5 --firewall-rule-complexity advanced --output config.xml

# Rules per VLAN
cargo run --release -- generate --count 8 --firewall-rules-per-vlan 3 --output config.xml
```

### Rule Complexity Levels

| Level          | Description                            |
| -------------- | -------------------------------------- |
| `basic`        | Simple allow/deny rules                |
| `intermediate` | Rules with port specifications         |
| `advanced`     | Complex rules with multiple conditions |

### Complete Configurations

Generate comprehensive OPNsense configurations:

```bash
# Full configuration with all components
cargo run --release -- generate --count 15 --format xml --include-firewall-rules --output complete-config.xml
```

## Validation

### Validate Generated Configurations

```bash
# Validate XML configuration
cargo run --release -- validate --input config.xml

# Validate with specific checks
cargo run --release -- validate --input config.xml --check-network-ranges
```

### Validation Checks

- **XML Schema**: Ensures valid OPNsense XML structure
- **Network Ranges**: Validates IP address ranges and subnets
- **VLAN IDs**: Checks IEEE 802.1Q compliance
- **Configuration Logic**: Validates configuration consistency

## Performance Considerations

### Large Datasets

For generating large numbers of configurations:

```bash
# Use CSV format for large datasets
cargo run --release -- generate vlan --count 1000 --format csv --output large-dataset.csv

# Stream processing for very large datasets
cargo run --release -- generate vlan --count 5000 --format csv --output huge-dataset.csv
```

### Memory Usage

- **Small datasets** (\<100 items): Minimal memory usage
- **Medium datasets** (100-1000 items): Moderate memory usage
- **Large datasets** (>1000 items): Consider CSV format for efficiency

## Best Practices

### File Organization

```bash
# Organize output by type
mkdir -p output/{vlans,firewalls,complete}

# Generate different types
cargo run --release -- generate vlan --count 20 --output output/vlans/vlans.xml
cargo run --release -- generate --count 10 --include-firewall-rules --output output/firewalls/rules.xml
```

### Naming Conventions

```bash
# Use descriptive filenames
cargo run --release -- generate vlan --count 25 --output "test-vlans-$(date +%Y%m%d).xml"

# Include parameters in filename
cargo run --release -- generate vlan --count 50 --base-id 100 --output "vlans-50-from-100.xml"
```

### Testing Workflows

```bash
# Quick test generation
cargo run --release -- generate vlan --count 5 --output test.xml

# Validate before using
cargo run --release -- validate --input test.xml

# If valid, generate full dataset
cargo run --release -- generate vlan --count 100 --output production.xml
```

## Troubleshooting

### Common Issues

**"No such file or directory" errors:**

- Ensure the output directory exists
- Check file permissions

**"Invalid VLAN ID" errors:**

- Use `--base-id` to specify starting ID
- Reduce `--count` if it exceeds valid range (10-4094)

**Memory issues with large datasets:**

- Use CSV format for large datasets
- Consider generating in smaller batches

### Getting Help

```bash
# Command help
cargo run --release -- --help

# Subcommand help
cargo run --release -- generate --help

# Validate help
cargo run --release -- validate --help
```

## Next Steps

- Explore [Configuration Generation](user-guide/configuration-generation.md) for advanced features
- Check out [Examples](user-guide/examples.md) for real-world scenarios
- Review [Performance Optimization](advanced/performance.md) for large-scale usage
