# OPNsense Config Faker

[![CI](https://github.com/EvilBit-Labs/OPNsense-config-faker/actions/workflows/ci.yml/badge.svg)](https://github.com/EvilBit-Labs/OPNsense-config-faker/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/EvilBit-Labs/OPNsense-config-faker/branch/main/graph/badge.svg)](https://codecov.io/gh/EvilBit-Labs/OPNsense-config-faker)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

# Build the project
cargo build --release

# Generate 25 OPNsense configurations
cargo run --release -- xml --base-config legacy/opnsense/config-example.xml --count 25
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

- Rust 1.70+ (latest stable recommended)
- Cargo package manager

### Quick Setup

```bash
# Clone and build
git clone https://github.com/EvilBit-Labs/OPNsense-config-faker.git
cd OPNsense-config-faker
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Development Setup

### Prerequisites

The development workflow uses the `just` task runner for common development tasks:

```bash
# Install just (choose one method)

# macOS (using Homebrew)
brew install just

# Linux (using Cargo)
cargo install just

# Windows (using Chocolatey)
choco install just


# Or download directly

curl --proto '=https' --tlsv1.2 -sSf <https://just.systems/install.sh> | bash

```

### Development Workflow

```bash
# Complete development setup
just dev-setup

# Run all checks and tests
just check-all

# Format code
just format

# Run CI validation
just ci-check
```

## Usage Examples

### Generate Basic Configurations

```bash
# Create 10 OPNsense configurations
cargo run --release -- generate vlan --count 10 --output vlans.xml

# Generate CSV data only
cargo run --release -- generate vlan --count 50 --format csv --output network-data.csv
```

### Advanced Scenarios

```bash
# Generate configurations for a large enterprise (100 firewalls)
cargo run --release -- generate vlan --count 100 --output enterprise-configs.xml

# Create configurations with specific settings
cargo run --release -- generate vlan --count 50 --base-id 500 --output config.xml
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
cargo run --release -- xml --base-config config.xml --count 50 --output-dir test-configs

# Use with your automation tools
ansible-playbook -i test-configs deploy-opnsense.yml
```

### Training Environment Setup

```bash
# Create diverse training scenarios
cargo run --release -- xml --base-config config.xml --count 20 --output-dir training-configs

# Deploy to training lab
for config in training-configs/*.xml; do
  # Deploy to OPNsense instance
done
```

### Load Testing

```bash
# Generate large configuration set
cargo run --release -- xml --base-config config.xml --count 500 --output-dir load-test-configs

# Test OPNsense performance with large configs
```

### Firewall Rules Generation

```bash
# Generate configurations with firewall rules (default complexity)
cargo run --release -- generate --count 25 --format csv --output config.csv --include-firewall-rules

# Generate with specific firewall rule complexity
cargo run --release -- generate --count 10 --format xml --base-config config.xml --include-firewall-rules --firewall-rule-complexity advanced

# Limit firewall rules per VLAN (useful for testing with smaller rule sets)
cargo run --release -- generate --count 5 --format csv --output config.csv --include-firewall-rules --firewall-rules-per-vlan 2

# Generate XML with limited firewall rules per VLAN
cargo run --release -- generate --count 10 --format xml --base-config config.xml --include-firewall-rules --firewall-rules-per-vlan 3 --firewall-rule-complexity intermediate
```

**Firewall Rules Per VLAN**: The `--firewall-rules-per-vlan` flag allows you to control the number of firewall rules generated per VLAN. This is useful for:

- Testing with smaller, more manageable rule sets
- Creating focused security scenarios
- Reducing configuration complexity for specific use cases
- Performance testing with controlled rule counts

When specified, this flag overrides the default rule count based on complexity level and ensures priorities are reassigned sequentially (1, 2, 3, ...) for each VLAN.

## Troubleshooting

### Common Issues

- **Permission Errors**: Ensure write access to output directory
- **Invalid Base Config**: Use the provided example config or a valid OPNsense config

### Getting Help

```bash
# Command help
cargo run --release -- --help
cargo run --release -- generate --help
cargo run --release -- generate vlan --help
```

## Project Background

This tool is a complete Rust rewrite designed to generate realistic OPNsense firewall configurations for testing and development purposes. It provides comprehensive network configuration generation capabilities with modern Rust practices and performance.

## Acknowledgments

This project was inspired by the original concept of generating OPNsense configurations. We extend our gratitude to **Stefan Reichhard** and the **nett-media team** for their original work ([nett-media/opnsense-config-generator](https://github.com/nett-media/opnsense-config-generator)) that used CSV data to bootstrap OPNsense configurations for production use. This project evolved that concept to focus purely on generating realistic test data for testing and development purposes.

## Roadmap

For detailed development plans and upcoming features, see [ROADMAP.md](ROADMAP.md).

### Current Focus

- **Enhanced Configuration Elements**: More realistic firewall rules, DHCP scopes, and interface configurations
- **Configuration Validation**: Ensure generated configurations are internally consistent and conflict-free
- **Template System**: Support for different OPNsense versions and deployment scenarios
- **Advanced Data Relationships**: Cross-reference VLANs with interfaces and generate consistent network topologies
- **Comprehensive Testing**: Full test suite for validation and regression testing

**Note**: This tool is specifically designed for OPNsense configurations. Supporting other platforms is not planned.

## Quality Assurance

This project maintains high quality standards through comprehensive testing and strict linting policies:

- **Testing**: Comprehensive test suite with unit, integration, property-based, and snapshot tests
- **Coverage**: Enforces 80% test coverage threshold
- **Linting**: Uses `cargo clippy -- -D warnings` policy (all warnings treated as errors)
- **Documentation**: Complete testing guide available in [TESTING.md](TESTING.md)

## Support and Contributing

- **Issues**: Report problems or request features via GitHub issues
- **Contributions**: Submit pull requests for improvements
- **Documentation**: Help improve this README for other administrators
- **Testing**: Follow the guidelines in [TESTING.md](TESTING.md) when contributing

## License

MIT License - Free for commercial and non-commercial use.
