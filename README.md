# OPNsense Config Faker

Generate realistic OPNsense firewall configurations for testing, training, and development. Create complete `config.xml` files with VLANs, interfaces, DHCP, NAT rules, firewall policies, and more using authentic network data.

## What This Tool Does

**Primary Use**: Generate complete OPNsense firewall configurations with realistic network data for:

- Testing OPNsense automation tools and scripts
- Validating configuration management systems
- Training network administrators on OPNsense
- Load testing OPNsense deployments
- Developing OPNsense management applications

**What You Get**: Fully functional OPNsense `config.xml` files containing:

- VLAN configurations with realistic IP ranges
- Network interface assignments
- DHCP server configurations
- NAT rules and port mappings
- Firewall policies and rules
- CARP virtual IP configurations
- RADIUS user accounts

## Quick Start

```bash
# Download and setup
git clone https://github.com/EvilBit-Labs/OPNsense-config-faker.git
cd OPNsense-config-faker

# Install dependencies
uv sync

# Generate 25 OPNsense configurations
python main.py xml --base-config legacy/opnsense/config-example.xml --count 25
```

## Real-World Use Cases

### Network Administrators

- **Testing Automation**: Validate your Ansible/Puppet/Chef OPNsense playbooks with realistic data
- **Training Scenarios**: Create diverse network topologies for team training
- **Migration Testing**: Test configuration migrations with complex network setups
- **Load Testing**: Generate large configurations to test OPNsense performance

### Security Administrators

- **Policy Testing**: Test firewall rule deployments across multiple network segments
- **Compliance Validation**: Generate configurations that match your security policies
- **Incident Response Training**: Create realistic network scenarios for security drills
- **Vulnerability Assessment**: Test security tools against varied network configurations

### DevOps Engineers

- **Infrastructure Testing**: Validate OPNsense deployments in CI/CD pipelines
- **Configuration Management**: Test Terraform/Pulumi OPNsense modules
- **Monitoring Setup**: Generate configurations to test monitoring and alerting
- **Backup Testing**: Validate backup and restore procedures

## Installation

### Requirements

- Python 3.10+ (3.13 recommended)
- UV package manager (recommended) or pip

### Quick Setup

```bash
# Using UV (recommended)
uv sync

# Or using pip
pip install faker typer rich lxml
```

## Usage Examples

### Generate Basic Configurations

```bash
# Create 10 OPNsense configurations
python main.py xml --base-config legacy/opnsense/config-example.xml --count 10

# Specify output directory
python main.py xml --base-config config.xml --count 25 --output-dir ./my-configs

# Generate CSV data only
python main.py csv --count 50 --output network-data.csv
```

### Advanced Scenarios

```bash
# Generate configurations for a large enterprise (100 firewalls)
python main.py xml --base-config config.xml --count 100 --output-dir enterprise-configs

# Create configurations with specific firewall settings
python main.py xml --base-config config.xml --count 50 \
  --firewall-nr 3 --opt-counter 15

# Use existing CSV data as input
python main.py xml --base-config config.xml --csv-file my-network-data.csv
```

### Interactive Mode

```bash
# Use the interactive helper
./run_generator.sh
```

## Generated Configuration Details

### Network Components

- **VLANs**: 10-4094 range with unique IDs
- **IP Ranges**: RFC 1918 compliant private networks
  - 10.0.0.0/8 (Class A)
  - 172.16.0.0/12 (Class B)
  - 192.168.0.0/16 (Class C)
- **Departments**: IT, Sales, HR, Finance, Marketing, Operations, Engineering, Support, Admin, Guest, Lab, Test, Security, DevOps, QA

### Sample Generated Data

```csv
VLAN,IP Range,Description,WAN
1234,10.123.45.x,IT1234,2
2567,172.16.78.x,Sales2567,1
3890,192.168.90.x,HR3890,3
```

## Configuration Features

### What's Included in Each Generated Config

- **Network Interfaces**: Realistic interface assignments and configurations
- **VLAN Segments**: Proper VLAN tagging and network segmentation
- **DHCP Servers**: Complete DHCP configurations with realistic IP pools
- **NAT Rules**: Port forwarding and address translation rules
- **Firewall Policies**: Security rules with appropriate source/destination
- **CARP Configurations**: High availability virtual IP setups
- **RADIUS Users**: Authentication user accounts and policies

### Data Quality

- **Realistic**: Uses industry-standard network practices
- **Unique**: No duplicate VLAN IDs or conflicting IP ranges
- **Compliant**: Follows RFC standards for private addressing
- **Varied**: Multiple department types and network topologies

## Common Workflows

### Testing OPNsense Automation

```bash
# Generate test configurations
python main.py xml --base-config config.xml --count 50 --output-dir test-configs

# Use with your automation tools
ansible-playbook -i test-configs deploy-opnsense.yml
```

### Training Environment Setup

```bash
# Create diverse training scenarios
python main.py xml --base-config config.xml --count 20 --output-dir training-configs

# Deploy to training lab
for config in training-configs/*.xml; do
  # Deploy to OPNsense instance
done
```

### Load Testing

```bash
# Generate large configuration set
python main.py xml --base-config config.xml --count 500 --output-dir load-test-configs

# Test OPNsense performance with large configs
```

## Troubleshooting

### Common Issues

- **Permission Errors**: Ensure write access to output directory
- **Invalid Base Config**: Use the provided example config or a valid OPNsense config

### Getting Help

```bash
# Command help
python main.py --help
python main.py xml --help
python main.py csv --help

# Interactive mode for guided usage
./run_generator.sh
```

## Project Background

This tool is based on the original work by Stefan Reichhard and the nett-media team, enhanced with modern Python practices and integrated functionality. It maintains compatibility with OPNsense while adding realistic data generation capabilities.

## Roadmap

For detailed development plans and upcoming features, see [ROADMAP.md](ROADMAP.md).

### Current Focus

- **Enhanced Configuration Elements**: More realistic firewall rules, DHCP scopes, and interface configurations
- **Configuration Validation**: Ensure generated configurations are internally consistent and conflict-free
- **Template System**: Support for different OPNsense versions and deployment scenarios
- **Advanced Data Relationships**: Cross-reference VLANs with interfaces and generate consistent network topologies
- **Comprehensive Testing**: Full test suite for validation and regression testing

**Note**: This tool is specifically designed for OPNsense configurations. Supporting other platforms is not planned.

## Support and Contributing

- **Issues**: Report problems or request features via GitHub issues
- **Contributions**: Submit pull requests for improvements
- **Documentation**: Help improve this README for other administrators

## License

MIT License - Free for commercial and non-commercial use.
