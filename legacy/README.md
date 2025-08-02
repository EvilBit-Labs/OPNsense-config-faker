# Legacy OPNsense Configuration Generator

This directory contains the original OPNsense-specific functionality that was preserved when this project was forked and transformed into a general-purpose network configuration data generator.

## Original Functionality

The `opnsense/` directory contains the complete original OPNsense config generator, including:

- **XML Generation Scripts**: All the original `gen*.py` scripts for creating OPNsense configuration parts
- **Configuration Processing**: Original XML processing and template injection code
- **Init Templates**: Default configuration templates in the `init/` directory
- **Export Directory**: Where generated OPNsense configurations are stored
- **Sample Configurations**: Example XML and CSV files

## Using the Original OPNsense Generator

If you need the original OPNsense functionality:

1. Navigate to the `legacy/opnsense/` directory
2. Use the original scripts as documented in `README-ENHANCED.md`
3. The original workflow remains intact and functional

## Migration Notes

- The main project now focuses on general network data generation
- Original OPNsense XML generation is preserved but moved to legacy
- All git history and attribution is maintained
- The core CSV generation has been enhanced with Faker integration

## Original Project Attribution

This functionality originated from:
- **Repository**: [nett-media/opnsense-config-generator](https://github.com/nett-media/opnsense-config-generator)
- **Purpose**: Batch creation of VLANs, Interfaces, DHCP Server, CARP IP, NAT, Firewall Rules and Radius User configurations for OPNsense
- **License**: Original project license applies to this legacy code
