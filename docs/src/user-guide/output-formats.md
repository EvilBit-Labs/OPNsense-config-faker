# Output Formats

OPNsense Config Faker supports multiple output formats to suit different use cases and integration needs.

## XML Format (Default)

XML is the primary format for OPNsense configurations and provides complete compatibility with OPNsense import/export functionality.

### Usage

```bash
# Generate XML configuration
cargo run --release -- generate vlan --count 25 --format xml --output config.xml

# XML is the default format
cargo run --release -- generate vlan --count 25 --output config.xml
```

### XML Structure

```xml
<opnsense>
  <vlans>
    <vlan>
      <vlanif>vlan100</vlanif>
      <tag>100</tag>
      <descr>IT Department VLAN</descr>
      <if>em0</if>
    </vlan>
    <vlan>
      <vlanif>vlan101</vlanif>
      <tag>101</tag>
      <descr>Engineering VLAN</descr>
      <if>em0</if>
    </vlan>
  </vlans>
  <interfaces>
    
  </interfaces>
  <firewall>
    
  </firewall>
</opnsense>
```

### XML Features

- **OPNsense Compatibility**: Direct import into OPNsense
- **Complete Configuration**: Full OPNsense config structure
- **Schema Validation**: Validates against OPNsense XSD schema
- **Hierarchical Structure**: Organized configuration sections

### Use Cases

- Direct OPNsense import
- Complete configuration backups
- Production-like testing environments
- Configuration templates

## CSV Format

CSV format provides structured data for analysis, processing, and integration with other tools.

### Usage

```bash
# Generate CSV data
cargo run --release -- generate vlan --count 25 --format csv --output data.csv

# Generate with specific output path
cargo run --release -- generate vlan --count 25 --format csv --output data.csv
```

### CSV Structure

```csv
vlan_id,name,description,interface,network,subnet_mask
100,IT_Department,IT Department VLAN,em0,192.168.100.0,255.255.255.0
101,Engineering,Engineering VLAN,em0,192.168.101.0,255.255.255.0
102,Sales,Sales Department VLAN,em0,192.168.102.0,255.255.255.0
```

### CSV Columns

**VLAN Configuration:**

- `vlan_id`: VLAN identifier (10-4094)
- `name`: VLAN name
- `description`: VLAN description
- `interface`: Parent interface
- `network`: Network address
- `subnet_mask`: Subnet mask

**Firewall Rules:**

- `rule_id`: Rule identifier
- `action`: Allow/Deny
- `protocol`: Network protocol
- `source`: Source address/network
- `destination`: Destination address/network
- `port`: Port number/range

### CSV Features

- **Data Processing**: Easy integration with data analysis tools
- **Custom Delimiters**: Support for different CSV formats
- **Structured Data**: Consistent column structure
- **Large Datasets**: Efficient for large data volumes

### Use Cases

- Data analysis and reporting
- Integration with other tools
- Custom processing pipelines
- Database import/export

## JSON Format

JSON format provides structured data for API integration and web applications.

### Usage

```bash
# Generate JSON data
cargo run --release -- generate vlan --count 25 --format json --output data.json

# Generate JSON output
cargo run --release -- generate vlan --count 25 --format json --output data.json
```

### JSON Structure

```json
{
  "vlans": [
    {
      "vlan_id": 100,
      "name": "IT_Department",
      "description": "IT Department VLAN",
      "interface": "em0",
      "network": "192.168.100.0/24"
    },
    {
      "vlan_id": 101,
      "name": "Engineering",
      "description": "Engineering VLAN",
      "interface": "em0",
      "network": "192.168.101.0/24"
    }
  ],
  "metadata": {
    "generated_at": "2024-01-15T10:30:00Z",
    "count": 2,
    "format_version": "1.0"
  }
}
```

### JSON Features

- **API Integration**: Easy consumption by web APIs
- **Structured Data**: Hierarchical JSON structure
- **Metadata**: Generation metadata included
- **Pretty Printing**: Human-readable formatting

### Use Cases

- Web application integration
- REST API consumption
- Configuration management systems
- Mobile application data

## Format Comparison

| Feature               | XML                 | CSV                    | JSON                   |
| --------------------- | ------------------- | ---------------------- | ---------------------- |
| **OPNsense Import**   | ✅ Direct           | ❌ Requires conversion | ❌ Requires conversion |
| **Data Processing**   | ⚠️ Complex          | ✅ Easy                | ✅ Easy                |
| **API Integration**   | ⚠️ Complex          | ⚠️ Limited             | ✅ Excellent           |
| **Human Readable**    | ⚠️ Verbose          | ✅ Clear               | ✅ Clear               |
| **Large Datasets**    | ⚠️ Memory intensive | ✅ Efficient           | ⚠️ Memory intensive    |
| **Schema Validation** | ✅ Built-in         | ❌ Manual              | ⚠️ Manual              |

## Choosing the Right Format

### Use XML When

- Importing directly into OPNsense
- Creating complete configuration files
- Need schema validation
- Working with OPNsense-specific tools

### Use CSV When

- Processing large datasets
- Integrating with data analysis tools
- Need efficient data processing
- Working with spreadsheets or databases

### Use JSON When

- Building web applications
- Creating REST APIs
- Need structured data for programming
- Integrating with modern web tools

## Advanced Format Options

### Custom Delimiters (CSV)

```bash
# Semicolon-delimited CSV
cargo run --release -- generate vlan --count 25 --format csv --delimiter ";" --output data.csv

# Tab-delimited CSV
cargo run --release -- generate vlan --count 25 --format csv --delimiter "\t" --output data.tsv
```

### JSON Output

```bash
# Generate JSON data
cargo run --release -- generate vlan --count 25 --format json --output data.json
```

## Format Conversion

### Converting Between Formats

```bash
# Generate XML and convert to CSV
cargo run --release -- generate vlan --count 25 --format xml --output config.xml
# Use external tools to convert XML to CSV

# Generate CSV and convert to JSON
cargo run --release -- generate vlan --count 25 --format csv --output data.csv
# Use jq or similar tools to convert CSV to JSON
```

### Batch Format Generation

```bash
# Generate multiple formats
cargo run --release -- generate vlan --count 25 --format xml --output config.xml
cargo run --release -- generate vlan --count 25 --format csv --output data.csv
cargo run --release -- generate vlan --count 25 --format json --output data.json
```

## Performance Considerations

### Format Performance

| Format   | Generation Speed | Memory Usage | File Size |
| -------- | ---------------- | ------------ | --------- |
| **XML**  | Medium           | High         | Large     |
| **CSV**  | Fast             | Low          | Small     |
| **JSON** | Medium           | Medium       | Medium    |

### Large Dataset Recommendations

```bash
# For large datasets, use CSV
cargo run --release -- generate vlan --count 1000 --format csv --output large-dataset.csv

# For very large datasets, generate in batches
cargo run --release -- generate vlan --count 5000 --format csv --output huge-dataset.csv
```

## Integration Examples

### Python Integration (CSV)

```python
import pandas as pd

# Read generated CSV
df = pd.read_csv('data.csv')

# Process VLAN data
for _, row in df.iterrows():
    print(f"VLAN {row['vlan_id']}: {row['name']} - {row['network']}")
```

### JavaScript Integration (JSON)

```javascript
// Read generated JSON
const fs = require('fs');
const data = JSON.parse(fs.readFileSync('data.json', 'utf8'));

// Process VLAN data
data.vlans.forEach(vlan => {
    console.log(`VLAN ${vlan.vlan_id}: ${vlan.name} - ${vlan.network}`);
});
```

### Shell Integration (CSV)

```bash
# Process CSV with awk
awk -F',' 'NR>1 {print "VLAN " $1 ": " $2 " - " $5}' data.csv

# Process CSV with cut
cut -d',' -f1,2,5 data.csv
```

## Troubleshooting

### Common Format Issues

**XML validation errors:**

- Check OPNsense schema compatibility
- Validate generated XML before use

**CSV parsing errors:**

- Verify delimiter consistency
- Check for special characters in data

**JSON parsing errors:**

- Validate JSON syntax
- Check for proper escaping

### Format Validation

```bash
# Validate XML
cargo run --release -- validate --input config.xml

# Validate JSON (using jq)
jq . data.json

# Validate CSV (using csvkit)
csvstat data.csv
```

## Next Steps

- Explore [Examples](examples.md) for format-specific use cases
- Check out [Configuration Generation](configuration-generation.md) for advanced generation options
- Review [Performance Optimization](advanced/performance.md) for large-scale usage
