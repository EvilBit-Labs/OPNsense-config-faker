# Configuration Generation

Learn how to generate different types of OPNsense configurations with detailed control over the output.

## Generation Types

### VLAN Configurations

Generate IEEE 802.1Q compliant VLAN configurations:

```bash
# Basic VLAN generation
cargo run --release -- generate vlan --count 25 --output vlans.xml

# With custom parameters
cargo run --release -- generate vlan --count 50 --base-id 100 --output vlans.xml
```

**VLAN Features:**

- Valid VLAN IDs (1-4094)
- Realistic network ranges
- Descriptive names and descriptions
- Interface assignments

### Interface Configurations

Generate network interface configurations:

```bash
# Physical interfaces
cargo run --release -- generate interface --count 10 --type physical --output interfaces.xml

# Virtual interfaces
cargo run --release -- generate interface --count 5 --type virtual --output virtual-interfaces.xml
```

### Firewall Rules

Generate comprehensive firewall rule sets:

```bash
# Basic firewall rules
cargo run --release -- generate firewall --rules 50 --output firewall.xml

# Advanced firewall rules
cargo run --release -- generate firewall --rules 100 --complexity advanced --output advanced-firewall.xml
```

**Firewall Rule Types:**

- Allow/Deny rules
- Port-based rules
- Protocol-specific rules
- Source/Destination filtering

### DHCP Configurations

Generate DHCP server configurations:

```bash
# DHCP pools
cargo run --release -- generate dhcp --count 10 --output dhcp.xml

# With custom ranges
cargo run --release -- generate dhcp --count 5 --base-network 192.168.100.0/24 --output dhcp.xml
```

### NAT Rules

Generate Network Address Translation rules:

```bash
# NAT rules
cargo run --release -- generate nat --rules 25 --output nat.xml

# Port forwarding rules
cargo run --release -- generate nat --rules 10 --type port-forward --output port-forward.xml
```

## Advanced Generation Options

### Combined Configurations

Generate complete OPNsense configurations with multiple components:

```bash
# Complete configuration
cargo run --release -- generate --count 20 --format xml --include-firewall-rules --include-dhcp --include-nat --output complete.xml
```

### Custom Network Ranges

Specify custom network ranges for generation:

```bash
# Custom base network
cargo run --release -- generate vlan --count 10 --base-network 10.0.0.0/8 --output vlans.xml

# Custom subnet size
cargo run --release -- generate vlan --count 15 --subnet-size 24 --output vlans.xml
```

### Department-Based Generation

Generate configurations based on organizational departments:

```bash
# Department-specific VLANs
cargo run --release -- generate vlan --count 8 --departments IT,Engineering,Sales,HR --output dept-vlans.xml

# With department-specific firewall rules
cargo run --release -- generate --count 5 --departments IT,Engineering --include-firewall-rules --output dept-config.xml
```

## Generation Parameters

### Count and Scale

Control the number of generated items:

```bash
# Small test dataset
cargo run --release -- generate vlan --count 5 --output test.xml

# Medium dataset
cargo run --release -- generate vlan --count 50 --output medium.xml

# Large dataset
cargo run --release -- generate vlan --count 500 --output large.xml
```

### ID Management

Control ID generation for sequential items:

```bash
# Custom starting ID
cargo run --release -- generate vlan --count 20 --base-id 100 --output vlans.xml

# Random ID distribution
cargo run --release -- generate vlan --count 25 --random-ids --output vlans.xml
```

### Network Configuration

Customize network parameters:

```bash
# Custom network base
cargo run --release -- generate vlan --count 10 --base-network 172.16.0.0/12 --output vlans.xml

# Custom subnet size
cargo run --release -- generate vlan --count 15 --subnet-size 28 --output vlans.xml
```

## Output Customization

### Format Options

Choose the appropriate output format:

```bash
# XML for OPNsense import
cargo run --release -- generate vlan --count 25 --format xml --output config.xml

# CSV for data processing
cargo run --release -- generate vlan --count 25 --format csv --output data.csv

# JSON for API integration
cargo run --release -- generate vlan --count 25 --format json --output data.json
```

### File Organization

Organize output files systematically:

```bash
# Create organized directory structure
mkdir -p output/{vlans,firewalls,dhcp,nat}

# Generate different types
cargo run --release -- generate vlan --count 20 --output output/vlans/vlans.xml
cargo run --release -- generate firewall --rules 30 --output output/firewalls/rules.xml
cargo run --release -- generate dhcp --count 10 --output output/dhcp/dhcp.xml
cargo run --release -- generate nat --rules 15 --output output/nat/nat.xml
```

## Quality Control

### Validation During Generation

Enable validation during generation:

```bash
# Generate with validation
cargo run --release -- generate vlan --count 25 --validate --output vlans.xml

# Generate with strict validation
cargo run --release -- generate vlan --count 25 --validate --strict --output vlans.xml
```

### Consistency Checks

Ensure generated configurations are consistent:

```bash
# Check for conflicts
cargo run --release -- generate vlan --count 50 --check-conflicts --output vlans.xml

# Validate network ranges
cargo run --release -- generate vlan --count 30 --validate-ranges --output vlans.xml
```

## Performance Optimization

### Large Dataset Generation

For generating large numbers of configurations:

```bash
# Use CSV format for large datasets
cargo run --release -- generate vlan --count 1000 --format csv --output large-dataset.csv

# Stream processing
cargo run --release -- generate vlan --count 5000 --stream --output huge-dataset.csv
```

### Memory Management

Optimize memory usage for large generations:

```bash
# Batch processing
cargo run --release -- generate vlan --count 2000 --batch-size 100 --output batched.xml

# Memory-efficient mode
cargo run --release -- generate vlan --count 1000 --memory-efficient --output efficient.xml
```

## Real-World Examples

### Lab Environment Setup

```bash
# Complete lab configuration
cargo run --release -- generate --count 15 --format xml --include-firewall-rules --include-dhcp --output lab-config.xml
```

### Security Testing

```bash
# Complex security testing configuration
cargo run --release -- generate --count 30 --firewall-rule-complexity advanced --include-nat --output security-test.xml
```

### Documentation Examples

```bash
# Sample configurations for documentation
cargo run --release -- generate vlan --count 5 --output examples/vlan-examples.xml
cargo run --release -- generate firewall --rules 10 --output examples/firewall-examples.xml
```

## Troubleshooting Generation

### Common Issues

**VLAN ID conflicts:**

```bash
# Use custom base ID
cargo run --release -- generate vlan --count 25 --base-id 200 --output vlans.xml
```

**Network range conflicts:**

```bash
# Use different base network
cargo run --release -- generate vlan --count 20 --base-network 10.0.0.0/8 --output vlans.xml
```

**Memory issues:**

```bash
# Use CSV format for large datasets
cargo run --release -- generate vlan --count 1000 --format csv --output data.csv
```

### Validation and Testing

Always validate generated configurations:

```bash
# Generate test configuration
cargo run --release -- generate vlan --count 5 --output test.xml

# Validate the output
cargo run --release -- validate --input test.xml

# If valid, generate full dataset
cargo run --release -- generate vlan --count 100 --output production.xml
```

## Next Steps

- Explore [Output Formats](output-formats.md) for detailed format specifications
- Check out [Examples](examples.md) for real-world scenarios
- Review [Performance Optimization](../advanced/performance.md) for large-scale usage
