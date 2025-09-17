# OPNsense Config Faker

[![CI](https://github.com/EvilBit-Labs/OPNsense-config-faker/actions/workflows/ci.yml/badge.svg)](https://github.com/EvilBit-Labs/OPNsense-config-faker/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/EvilBit-Labs/OPNsense-config-faker/branch/main/graph/badge.svg)](https://codecov.io/gh/EvilBit-Labs/OPNsense-config-faker)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Generate realistic OPNsense firewall configurations instantly.** Perfect for testing, training, and development without compromising real network data.

## Why Use This Tool?

• **Testing Made Simple**: Validate your automation scripts, configuration management, and deployment tools with realistic data\
• **Training Ready**: Create diverse network scenarios for team training and certification prep\
• **Fast & Reliable**: Generate hundreds of configurations in seconds with consistent, conflict-free data\
• **Security Focused**: Test firewall rules, policies, and network segmentation safely\
• **Production-Like**: Uses industry-standard network practices and RFC-compliant addressing

## What You Get

Each generated configuration includes:

- **Network Infrastructure**: VLANs, interfaces, and IP assignments
- **DHCP Services**: Complete DHCP server configurations with realistic IP pools
- **Security Policies**: Firewall rules, NAT configurations, and access controls
- **High Availability**: CARP virtual IP setups for redundancy
- **Authentication**: RADIUS user accounts and policies
- **Multiple Formats**: CSV data or complete OPNsense XML configurations

## Quick Start

**Get started in 3 steps:**

```bash
# 1. Download and build
git clone https://github.com/EvilBit-Labs/OPNsense-config-faker.git
cd OPNsense-config-faker
cargo build --release

# 2. Generate sample data (CSV format)
opnsense-config-faker generate --format csv --count 25 --output sample-networks.csv

# 3. Create complete OPNsense configurations (requires base config)
opnsense-config-faker generate --format xml --base-config your-config.xml --count 10
```

> [!NOTE]
> Start with CSV format to see the generated network data, then move to XML format for complete configurations.

## Popular Use Cases

### Network & Security Teams

- Test firewall automation (Ansible, Puppet, Chef, Terraform)
- Train staff on diverse network topologies
- Validate security policies across multiple segments
- Practice incident response with realistic scenarios
- Load test OPNsense performance with large configurations

### DevOps & Development

- Validate deployments in CI/CD pipelines
- Test configuration management tools
- Set up monitoring and alerting systems
- Verify backup and restore procedures
- Develop OPNsense management applications

### Training & Certification

- Create diverse scenarios for network certification prep
- Generate realistic lab environments for students
- Practice configuration management in safe environments
- Build demonstration networks for presentations

## Installation

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

### Setup

```bash
# Download and build
git clone https://github.com/EvilBit-Labs/OPNsense-config-faker.git
cd OPNsense-config-faker
cargo build --release

# Optional: Install globally
cargo install --path .
```

**After installation**, the tool is available as `opnsense-config-faker` command.

## Common Usage Examples

### Generate Test Data

```bash
# Create CSV with 25 network configurations
opnsense-config-faker generate --format csv --count 25 --output test-networks.csv

# Generate with firewall rules included
opnsense-config-faker generate --format csv --count 10 --output networks-with-fw.csv --include-firewall-rules

# Large dataset for performance testing
opnsense-config-faker generate --format csv --count 500 --output load-test-data.csv
```

### Create Complete OPNsense Configurations

```bash
# Generate XML configurations (requires base config file)
opnsense-config-faker generate --format xml --base-config my-base-config.xml --count 10

# Advanced firewall rules for security testing
opnsense-config-faker generate --format xml --base-config base.xml --count 5 \
  --include-firewall-rules --firewall-rule-complexity advanced

# Limit firewall rules per VLAN for focused testing
opnsense-config-faker generate --format xml --base-config base.xml --count 10 \
  --include-firewall-rules --firewall-rules-per-vlan 3
```

## What's Generated

### Network Components

✓ **VLANs**: Unique IDs in the 10-4094 range\
✓ **IP Addressing**: RFC 1918 compliant private networks (10.x.x.x, 172.16.x.x, 192.168.x.x)\
✓ **Departments**: Realistic organizational units (IT, Sales, HR, Finance, etc.)\
✓ **Interfaces**: Proper interface assignments and configurations\
✓ **DHCP**: Complete server configurations with IP pools\
✓ **Security**: Firewall rules, NAT policies, and access controls\
✓ **High Availability**: CARP virtual IP configurations\
✓ **Authentication**: RADIUS user accounts and policies

### Sample Output

```csv
VLAN,IP Range,Description,WAN
1234,10.123.45.x,IT1234,2
2567,172.16.78.x,Sales2567,1
3890,192.168.90.x,HR3890,3
```

### Quality Guarantees

• **No Conflicts**: Unique VLAN IDs and non-overlapping IP ranges\
• **Industry Standard**: Follows established network practices and RFC standards\
• **Realistic**: Authentic department names and network topologies\
• **Ready to Use**: Generated configurations work immediately

## Practical Workflows

### Testing Automation Tools

```bash
# Generate test data for your automation scripts
opnsense-config-faker generate --format csv --count 50 --output test-data.csv

# Test with Ansible, Terraform, or other tools
ansible-playbook -e "@test-data.csv" deploy-opnsense.yml
```

### Setting Up Training Labs

```bash
# Create diverse scenarios for training
opnsense-config-faker generate --format xml --base-config base.xml --count 20 --output-dir training-labs/

# Each generated config represents a different network topology
```

### Performance and Load Testing

```bash
# Generate large datasets for performance testing
opnsense-config-faker generate --format csv --count 1000 --output load-test.csv

# Test with complex firewall rules
opnsense-config-faker generate --format xml --base-config base.xml --count 100 \
  --include-firewall-rules --firewall-rule-complexity advanced
```

### Security Testing

```bash
# Generate focused firewall rule sets
opnsense-config-faker generate --format csv --count 10 --output security-test.csv \
  --include-firewall-rules --firewall-rules-per-vlan 5

# Test different complexity levels
opnsense-config-faker generate --format xml --base-config base.xml --count 5 \
  --include-firewall-rules --firewall-rule-complexity basic
```

## Getting Help

### Command Help

```bash
# General help
opnsense-config-faker --help

# Generate command help
opnsense-config-faker generate --help

# Validation help
opnsense-config-faker validate --help
```

### Common Issues

**Permission Errors**: Ensure write access to output directory\
**Invalid Base Config**: Use a valid OPNsense configuration file for XML generation\
**Large Datasets**: For 1000+ configurations, consider using `--quiet` flag to suppress progress output

### Need Support?

- **Found a bug?** [Open an issue](https://github.com/EvilBit-Labs/OPNsense-config-faker/issues)
- **Have questions?** Check existing [discussions](https://github.com/EvilBit-Labs/OPNsense-config-faker/discussions)
- **Feature request?** We welcome contributions and suggestions

## About This Project

OPNsense Config Faker is a modern Rust-based tool designed specifically for generating realistic OPNsense firewall configurations for testing and development. Built with performance and reliability in mind, it provides comprehensive network configuration generation capabilities.

### Key Features

- **High Performance**: Fast generation of large configuration sets
- **Quality Assured**: 80%+ test coverage with comprehensive validation
- **Production Ready**: Used by network teams for automation testing
- **Actively Maintained**: Regular updates and feature additions

### Roadmap

Upcoming features include enhanced configuration validation, template systems for different OPNsense versions, and advanced network topology generation. See [ROADMAP.md](ROADMAP.md) for details.

---

## Contributing & Support

**Contributing**: Submit pull requests, report issues, or suggest improvements\
**Documentation**: Help improve this README and user guides\
**Testing**: Follow guidelines in [TESTING.md](TESTING.md)

## License

MIT License - Free for commercial and non-commercial use.

---

### Acknowledgments

Inspired by the original [nett-media/opnsense-config-generator](https://github.com/nett-media/opnsense-config-generator) by Stefan Reichhard and the nett-media team. This project evolved that concept to focus on realistic test data generation for development and testing purposes.
