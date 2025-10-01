# Examples

Real-world examples and use cases for OPNsense Config Faker.

## Basic Examples

### Simple VLAN Generation

Generate a basic set of VLAN configurations:

```bash
# Generate 10 VLANs
cargo run --release -- generate vlan --count 10 --output vlans.xml

# Generate with custom base ID
cargo run --release -- generate vlan --count 15 --base-id 100 --output vlans.xml
```

### CSV Data Generation

Generate structured data for analysis:

```bash
# Generate CSV data
cargo run --release -- generate vlan --count 25 --format csv --output data.csv

# Generate with custom delimiter
cargo run --release -- generate vlan --count 25 --format csv --delimiter ";" --output data.csv
```

### JSON API Data

Generate data for API integration:

```bash
# Generate JSON data
cargo run --release -- generate vlan --count 20 --format json --output api-data.json

# Pretty-printed JSON
cargo run --release -- generate vlan --count 20 --format json --pretty --output api-data.json
```

## Advanced Examples

### Complete Lab Environment

Generate a complete lab environment configuration:

```bash
# Generate comprehensive lab configuration
cargo run --release -- generate --count 20 --format xml --include-firewall-rules --include-dhcp --include-nat --output lab-config.xml
```

### Security Testing Environment

Generate complex configurations for security testing:

```bash
# Advanced security testing configuration
cargo run --release -- generate --count 30 --firewall-rule-complexity advanced --include-nat --output security-test.xml

# With specific rule complexity
cargo run --release -- generate --count 15 --firewall-rule-complexity intermediate --firewall-rules-per-vlan 5 --output security-test.xml
```

### Department-Based Configuration

Generate configurations based on organizational structure:

```bash
# Department-specific VLANs
cargo run --release -- generate vlan --count 8 --departments IT,Engineering,Sales,HR --output dept-vlans.xml

# With department-specific firewall rules
cargo run --release -- generate --count 5 --departments IT,Engineering --include-firewall-rules --output dept-config.xml
```

## Real-World Scenarios

### Scenario 1: Network Administrator Testing

**Goal**: Test OPNsense configuration import/export functionality

```bash
# Generate test configuration
cargo run --release -- generate vlan --count 50 --output test-config.xml

# Validate the configuration
cargo run --release -- validate --input test-config.xml

# Generate with firewall rules
cargo run --release -- generate --count 25 --include-firewall-rules --output test-with-rules.xml
```

### Scenario 2: Security Tool Validation

**Goal**: Test security tools that parse OPNsense configurations

```bash
# Generate complex security configuration
cargo run --release -- generate --count 100 --firewall-rule-complexity advanced --include-nat --output security-test.xml

# Generate with specific rule types
cargo run --release -- generate firewall --rules 200 --complexity advanced --output firewall-rules.xml
```

### Scenario 3: Documentation Examples

**Goal**: Create sample configurations for documentation

```bash
# Generate simple examples
cargo run --release -- generate vlan --count 5 --output examples/vlan-examples.xml
cargo run --release -- generate firewall --rules 10 --output examples/firewall-examples.xml

# Generate comprehensive example
cargo run --release -- generate --count 10 --format xml --include-firewall-rules --output examples/complete-example.xml
```

### Scenario 4: Performance Testing

**Goal**: Test performance with large datasets

```bash
# Generate large dataset
cargo run --release -- generate vlan --count 1000 --format csv --output large-dataset.csv

# Generate very large dataset
cargo run --release -- generate vlan --count 5000 --format csv --output huge-dataset.csv
```

## Integration Examples

### Python Integration

Process generated CSV data with Python:

```python
import pandas as pd
import json

# Read generated CSV
df = pd.read_csv('data.csv')

# Process VLAN data
vlan_summary = df.groupby('interface').agg({
    'vlan_id': 'count',
    'network': 'nunique'
}).reset_index()

print("VLAN Summary by Interface:")
print(vlan_summary)

# Convert to JSON for API
vlan_data = df.to_dict('records')
with open('processed-data.json', 'w') as f:
    json.dump(vlan_data, f, indent=2)
```

### JavaScript Integration

Process generated JSON data with Node.js:

```javascript
const fs = require('fs');

// Read generated JSON
const data = JSON.parse(fs.readFileSync('data.json', 'utf8'));

// Process VLAN data
const vlanSummary = data.vlans.reduce((acc, vlan) => {
    const interface = vlan.interface;
    if (!acc[interface]) {
        acc[interface] = {
            count: 0,
            networks: new Set()
        };
    }
    acc[interface].count++;
    acc[interface].networks.add(vlan.network);
    return acc;
}, {});

console.log('VLAN Summary by Interface:');
Object.entries(vlanSummary).forEach(([interface, stats]) => {
    console.log(`${interface}: ${stats.count} VLANs, ${stats.networks.size} unique networks`);
});
```

### Shell Script Integration

Process generated data with shell scripts:

```bash
#!/bin/bash

# Generate data
cargo run --release -- generate vlan --count 25 --format csv --output data.csv

# Process with awk
echo "VLAN Summary:"
awk -F',' 'NR>1 {
    interface_count[$4]++
    total_vlans++
}
END {
    print "Total VLANs:", total_vlans
    print "VLANs by Interface:"
    for (iface in interface_count) {
        print "  " iface ": " interface_count[iface]
    }
}' data.csv

# Generate report
echo "Generating VLAN report..."
awk -F',' 'NR>1 {print "VLAN " $1 ": " $2 " (" $3 ") - " $5}' data.csv > vlan-report.txt
```

## Batch Processing Examples

### Multiple Format Generation

Generate the same data in multiple formats:

```bash
#!/bin/bash

# Generate in multiple formats
cargo run --release -- generate vlan --count 25 --format xml --output vlans.xml
cargo run --release -- generate vlan --count 25 --format csv --output vlans.csv
cargo run --release -- generate vlan --count 25 --format json --output vlans.json

echo "Generated VLAN data in XML, CSV, and JSON formats"
```

### Batch Configuration Generation

Generate multiple configuration types:

```bash
#!/bin/bash

# Create output directory
mkdir -p output/{vlans,firewalls,dhcp,nat}

# Generate different configuration types
cargo run --release -- generate vlan --count 20 --output output/vlans/vlans.xml
cargo run --release -- generate firewall --rules 30 --output output/firewalls/rules.xml
cargo run --release -- generate dhcp --count 10 --output output/dhcp/dhcp.xml
cargo run --release -- generate nat --rules 15 --output output/nat/nat.xml

echo "Generated configurations in organized directory structure"
```

### Automated Testing Pipeline

Create an automated testing pipeline:

```bash
#!/bin/bash

# Test configuration generation
echo "Testing VLAN generation..."
cargo run --release -- generate vlan --count 10 --output test-vlans.xml

# Validate generated configuration
echo "Validating configuration..."
cargo run --release -- validate --input test-vlans.xml

if [ $? -eq 0 ]; then
    echo "✅ Configuration validation passed"

    # Generate full dataset
    echo "Generating full dataset..."
    cargo run --release -- generate vlan --count 100 --output production-vlans.xml

    # Validate full dataset
    cargo run --release -- validate --input production-vlans.xml

    if [ $? -eq 0 ]; then
        echo "✅ Full dataset validation passed"
    else
        echo "❌ Full dataset validation failed"
        exit 1
    fi
else
    echo "❌ Configuration validation failed"
    exit 1
fi
```

## Performance Examples

### Large Dataset Generation

Generate and process large datasets:

```bash
# Generate large dataset
echo "Generating large dataset..."
cargo run --release -- generate vlan --count 1000 --format csv --output large-dataset.csv

# Process with streaming
echo "Processing large dataset..."
awk -F',' 'NR>1 {print "VLAN " $1 ": " $2}' large-dataset.csv | head -20

# Generate summary statistics
echo "Dataset statistics:"
wc -l large-dataset.csv
awk -F',' 'NR>1 {print $1}' large-dataset.csv | sort -n | tail -1
```

### Memory-Efficient Processing

Process large datasets efficiently:

```bash
# Generate with memory optimization
cargo run --release -- generate vlan --count 5000 --format csv --memory-efficient --output huge-dataset.csv

# Stream processing
cargo run --release -- generate vlan --count 10000 --format csv --stream --output streamed-data.csv
```

## Troubleshooting Examples

### Common Issue Resolution

```bash
# Issue: VLAN ID conflicts
echo "Resolving VLAN ID conflicts..."
cargo run --release -- generate vlan --count 25 --base-id 100 --output vlans.xml

# Issue: Network range conflicts
echo "Resolving network range conflicts..."
cargo run --release -- generate vlan --count 20 --base-network 10.0.0.0/8 --output vlans.xml

# Issue: Memory problems with large datasets
echo "Using CSV format for large datasets..."
cargo run --release -- generate vlan --count 1000 --format csv --output data.csv
```

### Validation and Testing

```bash
# Generate test configuration
echo "Generating test configuration..."
cargo run --release -- generate vlan --count 5 --output test.xml

# Validate configuration
echo "Validating configuration..."
cargo run --release -- validate --input test.xml

if [ $? -eq 0 ]; then
    echo "✅ Test configuration is valid"

    # Generate production configuration
    cargo run --release -- generate vlan --count 50 --output production.xml
else
    echo "❌ Test configuration validation failed"
    exit 1
fi
```

## Best Practices Examples

### Organized Output Structure

```bash
#!/bin/bash

# Create organized directory structure
mkdir -p output/{test,production,examples}/{xml,csv,json}

# Generate test data
cargo run --release -- generate vlan --count 5 --format xml --output output/test/xml/test-vlans.xml
cargo run --release -- generate vlan --count 5 --format csv --output output/test/csv/test-vlans.csv
cargo run --release -- generate vlan --count 5 --format json --output output/test/json/test-vlans.json

# Generate production data
cargo run --release -- generate vlan --count 50 --format xml --output output/production/xml/production-vlans.xml
cargo run --release -- generate vlan --count 50 --format csv --output output/production/csv/production-vlans.csv

# Generate examples
cargo run --release -- generate vlan --count 10 --format xml --output output/examples/xml/example-vlans.xml
cargo run --release -- generate vlan --count 10 --format csv --output output/examples/csv/example-vlans.csv

echo "Generated organized output structure"
```

### Automated Quality Checks

```bash
#!/bin/bash

# Quality check function
check_quality() {
    local file=$1
    local format=$2

    echo "Checking quality of $file..."

    if [ "$format" = "xml" ]; then
        cargo run --release -- validate --input "$file"
    elif [ "$format" = "json" ]; then
        jq . "$file" > /dev/null
    elif [ "$format" = "csv" ]; then
        csvstat "$file" > /dev/null
    fi

    if [ $? -eq 0 ]; then
        echo "✅ $file quality check passed"
    else
        echo "❌ $file quality check failed"
        return 1
    fi
}

# Generate and check quality
cargo run --release -- generate vlan --count 25 --format xml --output test.xml
check_quality test.xml xml

cargo run --release -- generate vlan --count 25 --format json --output test.json
check_quality test.json json

cargo run --release -- generate vlan --count 25 --format csv --output test.csv
check_quality test.csv csv
```

## Next Steps

- Explore [Configuration Generation](configuration-generation.md) for advanced generation options
- Check out [Output Formats](output-formats.md) for detailed format specifications
- Review [Performance Optimization](advanced/performance.md) for large-scale usage
