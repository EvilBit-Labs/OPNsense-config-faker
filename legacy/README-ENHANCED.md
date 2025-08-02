# Enhanced OPNsense Config Generator

This is an enhanced version of the OPNsense Config Generator that includes test data generation using Faker.

## Features

- **Original functionality**: Generate OPNsense XML configurations from CSV files
- **Test data generation**: Use Faker to generate realistic test CSV configurations
- **Virtual environment support**: Isolated Python dependencies
- **Enhanced IP range generation**: Varied but valid private IP address ranges
- **Easy-to-use helper script**: Interactive script for common operations

## Setup

### Prerequisites

- Python 3.10+ (Python 3.13 recommended via Homebrew: `brew install python@3.13`)
- An OPNsense backup config.xml file

### Quick Setup

1. Clone/navigate to this repository
2. Run the helper script: `./run_generator.sh`
3. The script will automatically set up the virtual environment and dependencies

### Manual Setup

```bash
# Create virtual environment with Python 3.13
/opt/homebrew/bin/python3.13 -m venv venv

# Activate virtual environment
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt
```

## Usage

### Option 1: Interactive Helper Script

```bash
./run_generator.sh
```

This script provides a menu with options to:

1. Generate new test CSV file
2. Run XML config generator
3. Both (generate CSV then XML)

### Option 2: Manual Commands

```bash
# Activate virtual environment
source venv/bin/activate

# Generate test CSV data with default 10 VLANs
python generate_csv.py

# Generate test CSV data with custom count
python generate_csv.py --count 25

# Generate test CSV data with custom count and filename
python generate_csv.py -c 50 -o large-test.csv

# Generate XML configurations
python generateXMLConfig.py
```

### CSV Generator Command-Line Options

The `generate_csv.py` script supports the following options:

```bash
python generate_csv.py --help

Options:
  -h, --help            Show help message and exit
  -c, --count COUNT     Number of VLAN configurations to generate (default: 10)
  -o, --output OUTPUT   Output CSV filename (default: test-config.csv)

Examples:
  python generate_csv.py                    # Generate 10 VLANs (default)
  python generate_csv.py --count 25         # Generate 25 VLANs
  python generate_csv.py -c 50 -o my.csv   # Generate 50 VLANs to my.csv
```

## Generated Test Data

The `generate_csv.py` script creates realistic test data with:

- **Unique VLAN IDs**: Range 10-4094 (valid VLAN range)
- **Varied IP ranges**: Uses Faker's `ipv4_private()` provider for RFC 1918 compliant addresses:
  - 10.x.y.x (Class A private - 10.0.0.0/8)
  - 172.16-31.x.x (Class B private - 172.16.0.0/12)
  - 192.168.x.x (Class C private - 192.168.0.0/16)
- **Realistic descriptions**: Department-based naming (IT, Sales, HR, etc.)
- **Random WAN assignments**: 1, 2, or 3
- **Duplicate prevention**: Ensures unique VLAN IDs and network ranges

## Generated Files

After running the generator, you'll find these files in the `export/` directory:

- `config-example.xml` - Complete OPNsense configuration
- `part1_Interface.xml` - Interface configurations
- `part2_DHCP.xml` - DHCP server configurations
- `part3_NAT.xml` - NAT rules
- `part4_Rules.xml` - Firewall rules
- `part5_CARP.xml` - CARP virtual IP configurations
- `part6_VLAN.xml` - VLAN configurations
- `part7_RadiusUser.xml` - Radius user configurations

## Customization

### Modify CSV Generation

Edit `generate_csv.py` to customize:

- Number of records generated
- VLAN ID ranges
- IP address patterns
- Department names
- WAN assignment logic

### Modify XML Generation

Edit `generateXMLConfig.py` to customize:

- Firewall options
- Interface counter starting point
- WAN IP addresses

## Dependencies

- `faker` - Generate realistic test data
- `lxml` - XML processing
- `tzdata` - Timezone data for faker

## Original Documentation

See the original `README.md` for detailed information about the XML generation process and configuration options.
