# Network Configuration Data Generator

A flexible tool for generating realistic network configuration test data, particularly useful for testing network automation tools, configuration management systems, and network infrastructure projects.

## Origin and Attribution

This project is forked from [nett-media/opnsense-config-generator](https://github.com/nett-media/opnsense-config-generator), originally designed for generating OPNsense firewall configurations. We've transformed it into a general-purpose network configuration data generator while preserving the original git history and respecting the source project.

**Original Project**: OPNsense Config Generation by nett-media  
**Fork Purpose**: General network configuration test data generation  
**License**: Preserves original project licensing

## Features

- **Realistic Network Data**: Generate VLAN configurations with RFC 1918 compliant private IP ranges
- **Flexible Output**: Configurable number of records and output formats
- **Command-Line Interface**: Easy-to-use CLI with comprehensive options
- **Faker Integration**: Uses Faker library for realistic, varied test data
- **Virtual Environment**: Isolated Python dependencies for clean development
- **Extensible**: Easy to modify for different network configuration formats

## Quick Start

```bash
# Clone the repository
git clone <your-repo-url> network-config-generator
cd network-config-generator

# Set up virtual environment and dependencies
./setup.sh

# Generate 10 VLAN configurations (default)
python generate_csv.py

# Generate 50 configurations with custom filename
python generate_csv.py --count 50 --output my-network-config.csv
```

## Installation

### Prerequisites
- Python 3.10+ (Python 3.13 recommended)
- Git

### Setup
```bash
# Create virtual environment
python3.13 -m venv venv

# Activate virtual environment
source venv/bin/activate  # Linux/macOS
# OR
venv\Scripts\activate     # Windows

# Install dependencies
pip install -r requirements.txt
```

## Usage

### Command Line Interface

```bash
# Basic usage - generates 10 VLANs to test-config.csv
python generate_csv.py

# Specify number of VLANs
python generate_csv.py --count 25

# Custom output file
python generate_csv.py --output production-test.csv

# Combination
python generate_csv.py -c 100 -o large-dataset.csv

# Help
python generate_csv.py --help
```

### Interactive Helper Script

```bash
./run_generator.sh
```

This provides an interactive menu for common operations.

## Generated Data Format

The tool generates CSV files with the following columns:

| Column | Description | Example |
|--------|-------------|---------|
| VLAN | VLAN ID (10-4094) | 1234 |
| IP Range | Private IP network | 10.123.45.x |
| Beschreibung | Department + VLAN ID | IT1234 |
| WAN | WAN assignment (1-3) | 2 |

### Sample Output
```csv
VLAN,IP Range,Beschreibung,WAN
1234,10.123.45.x,IT1234,2
2567,172.16.78.x,Sales2567,1
3890,192.168.90.x,HR3890,3
```

## Data Characteristics

- **Unique VLAN IDs**: No duplicates, valid range (10-4094)
- **RFC 1918 Compliant**: All IP ranges use private address spaces
  - 10.0.0.0/8 (Class A)
  - 172.16.0.0/12 (Class B) 
  - 192.168.0.0/16 (Class C)
- **Realistic Descriptions**: Department-based naming
- **Varied Data**: Uses Faker for natural distribution

## Customization

### Modify Data Generation

Edit `generate_csv.py` to customize:
- VLAN ID ranges
- IP address patterns
- Department names
- Output format
- Additional fields

### Example Customization

```python
# Add new department types
department = fake.random_element(elements=(
    'Sales', 'IT', 'HR', 'Finance', 'Marketing', 'Operations', 
    'Engineering', 'Support', 'Admin', 'Guest', 'Lab', 'Test',
    'Security', 'DevOps', 'QA'  # Add your departments here
))
```

## Use Cases

- **Network Testing**: Generate test configurations for network automation
- **Configuration Management**: Test infrastructure-as-code tools
- **Training Data**: Practice with realistic network datasets
- **Load Testing**: Generate large datasets for performance testing
- **Development**: Mock data for network management applications

## Dependencies

- `faker` - Generate realistic test data
- `lxml` - XML processing (inherited from original project)
- `tzdata` - Timezone data for faker

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project maintains the same license as the original [nett-media/opnsense-config-generator](https://github.com/nett-media/opnsense-config-generator) project.

## Acknowledgments

- **Original Authors**: nett-media team for the foundational OPNsense config generator
- **Faker Library**: For providing excellent test data generation capabilities
- **Python Community**: For the robust ecosystem that makes projects like this possible

## Migration from Original Project

If you're coming from the original OPNsense config generator:

1. The `generate_csv.py` script replaces manual CSV creation
2. Virtual environment setup is now standardized
3. The project focuses on data generation rather than OPNsense-specific XML
4. Original OPNsense functionality is preserved but not the primary focus

## Roadmap

- [ ] JSON output format support
- [ ] Additional network device configuration formats
- [ ] Web API for data generation
- [ ] Docker container for easy deployment
- [ ] Plugin system for custom data generators
