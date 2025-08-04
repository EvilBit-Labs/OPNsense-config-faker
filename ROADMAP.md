# OPNsense Config Faker - Development Roadmap

This document outlines the planned development roadmap for the OPNsense Config Faker project. These features are planned for future releases and will enhance the tool's capabilities for generating realistic OPNsense configurations.

## Planned Features

### Additional Network Configuration Elements

- [ ] **Firewall Rules with Realistic Patterns**

  - Generate firewall rules that follow common security patterns
  - Include rules for different network segments (DMZ, internal, guest)
  - Realistic port configurations and protocol specifications
  - Stateful inspection rules with proper logging

- [ ] **DHCP Server Configurations with Realistic Scopes**

  - Dynamic IP range allocation based on network size
  - Realistic lease times and reservation configurations
  - DNS and gateway settings for each VLAN
  - DHCP options for different network types

- [ ] **Interface Configurations with Realistic Naming**

  - Industry-standard interface naming conventions
  - Proper interface descriptions and aliases
  - Realistic bandwidth and duplex settings
  - Interface grouping and aggregation configurations

- [ ] **NAT Rules with Port Mappings**

  - Port forwarding rules for common services
  - Source and destination NAT configurations
  - Realistic service port assignments
  - NAT reflection and hairpin configurations

- [ ] **CARP Virtual IP Configurations**

  - High availability virtual IP setups
  - Realistic failover configurations
  - VHID assignments and synchronization settings
  - Backup and primary node configurations

- [ ] **RADIUS User Accounts with Authentication Details**

  - User account generation with realistic credentials
  - Authentication method configurations
  - User group assignments and permissions
  - Password policies and security settings

### Enhanced Data Relationships

- [ ] **Cross-Reference VLANs with Appropriate Interfaces**

  - Ensure VLAN assignments match interface capabilities
  - Generate consistent interface-to-VLAN mappings
  - Validate interface bandwidth for VLAN requirements
  - Create realistic interface hierarchies

- [ ] **Link DHCP Scopes to Corresponding VLAN Networks**

  - Align DHCP ranges with VLAN network addresses
  - Generate consistent gateway and DNS settings
  - Ensure DHCP scope size matches VLAN requirements
  - Create realistic DHCP option configurations

- [ ] **Generate Consistent Firewall Rules Based on Network Topology**

  - Create rules that reflect actual network architecture
  - Generate appropriate access control between VLANs
  - Include rules for internet access and security policies
  - Ensure rule ordering follows logical security patterns

### Configuration Validation

- [ ] **Ensure Generated Configurations are Internally Consistent**

  - Validate all configuration elements work together
  - Check for logical conflicts in network design
  - Ensure proper dependency relationships
  - Verify configuration completeness

- [ ] **Validate IP Address Assignments Don't Conflict**

  - Check for overlapping IP ranges across VLANs
  - Ensure gateway addresses are within correct networks
  - Validate DHCP scope boundaries
  - Check for duplicate IP assignments

- [ ] **Check VLAN ID Uniqueness Across All Components**

  - Ensure VLAN IDs are unique across the entire configuration
  - Validate VLAN assignments in interface configurations
  - Check for conflicts in VLAN tagging
  - Verify VLAN consistency in all related components

## Implementation Priorities

### Phase 1: Core Enhancements

1. Enhanced firewall rule generation
2. Improved DHCP configuration realism
3. Better interface naming and configuration

### Phase 2: Data Relationships

1. Cross-referencing VLANs and interfaces
2. Linking DHCP scopes to VLAN networks
3. Topology-based firewall rule generation

### Phase 3: Validation and Quality

1. Configuration consistency validation
2. IP address conflict detection
3. VLAN ID uniqueness verification

## Success Criteria

- Generated configurations pass OPNsense validation
- Realistic network topologies that reflect real-world deployments
- Consistent data relationships across all configuration elements
- Comprehensive validation prevents configuration conflicts
- Enhanced user experience with better error handling and feedback

## Contributing to the Roadmap

If you have suggestions for additional features or improvements, please:

1. Open an issue on GitHub with the `enhancement` label
2. Provide detailed use cases and requirements
3. Consider the impact on existing functionality
4. Ensure proposals align with the project's OPNsense focus

## Notes

- All features will maintain backward compatibility where possible
- Focus remains on OPNsense-specific functionality
- Community feedback will influence priority and implementation details
- Features will be implemented incrementally with proper testing
