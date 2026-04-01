# OPNsense Config Faker

A flexible Rust CLI tool for generating realistic network configuration test data specifically designed for OPNsense testing environments.

## What is OPNsense Config Faker?

OPNsense Config Faker is a specialized tool that creates valid OPNsense `config.xml` files with realistic faked data. It's designed for network administrators, security professionals, and testing teams who need comprehensive test configurations for OPNsense firewalls.

## Key Features

- **OPNsense-Specific**: Generates valid OPNsense XML configurations
- **Realistic Data**: Creates RFC-compliant network configurations that mirror real-world deployments
- **Multiple Formats**: Supports XML, CSV, and JSON output formats
- **High Performance**: Built in Rust for fast generation of large configuration sets
- **Offline-First**: No external dependencies or telemetry
- **Cross-Platform**: Works on macOS, Windows, and Linux

## What You Can Generate

- **VLAN Configurations**: IEEE 802.1Q compliant VLAN setups
- **Interface Configurations**: Physical and virtual network interfaces
- **Firewall Rules**: Realistic security policies and access controls
- **DHCP Settings**: Dynamic host configuration protocols
- **NAT Rules**: Network address translation configurations
- **Routing Tables**: Static and dynamic routing configurations
- **CARP VIPs**: Common Address Redundancy Protocol virtual IPs
- **RADIUS Users**: Authentication and authorization data

## Use Cases

- **Testing Environments**: Generate test data for OPNsense deployments
- **Security Testing**: Create realistic configurations for security tool validation
- **Development**: Provide sample data for OPNsense plugin development
- **Training**: Generate examples for network administration training
- **Documentation**: Create sample configurations for documentation

## Quick Example

```bash
# Generate 25 VLAN configurations
cargo run --release -- generate vlan --count 25 --output vlans.xml

# Generate firewall rules with VLANs
cargo run --release -- generate --count 10 --format xml --include-firewall-rules
```

## Why OPNsense Config Faker?

- **Network Validity**: Every generated configuration is technically correct and realistic
- **Performance**: Generate thousands of configurations in seconds
- **Reliability**: Built with Rust's memory safety and error handling
- **Flexibility**: Configurable generation parameters and output formats
- **Quality**: Comprehensive testing and validation of generated data

## Getting Started

Ready to start generating OPNsense configurations? Check out our [Quick Start Guide](getting-started/quick-start.md) to get up and running in minutes.
