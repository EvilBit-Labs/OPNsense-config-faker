# Data Generation Standards for OPNsense Config Faker

## 1. Faker Integration

- Use `faker.providers.internet.ipv4_private()` for private IP addresses
- Leverage faker's built-in providers for realistic data
- Avoid manual random generation when faker provides suitable methods
- Initialize faker once and reuse the instance

## 2. Network Configuration Standards

### VLAN IDs

- Valid range: 10-4094 (IEEE 802.1Q standard)
- Ensure uniqueness within each generated dataset
- Use realistic distribution, not purely random

### IP Addresses

- Use only RFC 1918 private address spaces:
  - 10.0.0.0/8 (Class A: 10.0.0.0 - 10.255.255.255)
  - 172.16.0.0/12 (Class B: 172.16.0.0 - 172.31.255.255)
  - 192.168.0.0/16 (Class C: 192.168.0.0 - 192.168.255.255)
- Format: `x.x.x.x` notation with `.x` as the host placeholder
- Ensure no duplicate networks in single generation

### Department/Organizational Naming

- Use realistic department names:
  - Technical: IT, Engineering, DevOps, Security
  - Business: Sales, Marketing, Finance, HR
  - Operations: Support, Admin, Operations
  - Special: Guest, Lab, Test
- Combine department with VLAN ID for descriptions

### WAN Assignments

- Use values 1-3 to represent different WAN connections
- Distribute assignments realistically across VLANs

## 3. Data Validation

- Validate all generated data before output
- Ensure internal consistency within datasets
- Check for conflicts (duplicate VLANs, overlapping networks)
- Verify data conforms to expected formats

## 4. Output Formats

### CSV Structure

- Header: `VLAN,IP Range,Beschreibung,WAN`
- Use German "Beschreibung" for description (legacy compatibility)
- No trailing whitespace or extra delimiters
- Consistent field ordering

### File Organization

- Default output to `output/` directory
- Create directories automatically if needed
- Use descriptive filenames with timestamps when appropriate
- Don't overwrite existing files without user confirmation

## 5. Extensibility

- Design data generators to be easily extensible
- Support additional network configuration elements:
  - Firewall rules
  - DHCP configurations
  - Interface configurations
  - NAT rules
- Maintain relationships between related configuration elements

## 6. Performance

- Generate data efficiently for large datasets
- Use appropriate data structures for uniqueness checking
- Avoid unnecessary computations in loops
- Profile and optimize for datasets >1000 records
