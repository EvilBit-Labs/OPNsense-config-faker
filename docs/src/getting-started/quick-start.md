# Quick Start

Get up and running with OPNsense Config Faker in under 5 minutes.

## Your First Generation

Generate your first OPNsense configuration:

```bash
# Generate 10 VLAN configurations
cargo run --release -- generate vlan --count 10 --output my-vlans.xml
```

This creates a valid OPNsense XML file with 10 realistic VLAN configurations.

## Basic Commands

### Generate VLANs

```bash
# Simple VLAN generation
cargo run --release -- generate vlan --count 25 --output vlans.xml

# With custom base VLAN ID
cargo run --release -- generate vlan --count 50 --base-id 100 --output vlans.xml
```

### Generate Complete Configurations

```bash
# Generate VLANs with firewall rules
cargo run --release -- generate --count 10 --format xml --include-firewall-rules --output config.xml

# Generate with specific complexity
cargo run --release -- generate --count 5 --firewall-rule-complexity advanced --output advanced-config.xml
```

### Output Formats

```bash
# CSV format for data processing
cargo run --release -- generate vlan --count 20 --format csv --output data.csv

# JSON format for API integration
cargo run --release -- generate vlan --count 15 --format json --output data.json
```

## Understanding the Output

### XML Output Structure

The generated XML follows OPNsense configuration schema:

```xml
<opnsense>
  <vlans>
    <vlan>
      <vlanif>vlan100</vlanif>
      <tag>100</tag>
      <descr>IT Department VLAN</descr>
      <if>em0</if>
    </vlan>
  </vlans>
</opnsense>
```

### CSV Output Structure

CSV output includes columns for easy data processing:

```csv
vlan_id,name,description,interface,network
100,IT_Department,IT Department VLAN,em0,192.168.100.0/24
101,Engineering,Engineering VLAN,em0,192.168.101.0/24
```

## Common Use Cases

### Testing Environment Setup

```bash
# Generate test data for a lab environment
cargo run --release -- generate --count 20 --format xml --output lab-config.xml
```

### Security Tool Testing

```bash
# Generate complex configurations for security testing
cargo run --release -- generate --count 50 --firewall-rule-complexity advanced --output security-test.xml
```

### Documentation Examples

```bash
# Generate sample configurations for documentation
cargo run --release -- generate vlan --count 5 --output examples.xml
```

## Next Steps

- Explore [Basic Usage](basic-usage.md) for more detailed examples
- Check out [Configuration Generation](user-guide/configuration-generation.md) for advanced features
- Review [Examples](user-guide/examples.md) for real-world scenarios

## Getting Help

If you encounter any issues:

1. Check the command help: `cargo run --release -- --help`
2. Review the [Troubleshooting Guide](advanced/troubleshooting.md)
3. Look at the [Examples](user-guide/examples.md) for similar use cases
