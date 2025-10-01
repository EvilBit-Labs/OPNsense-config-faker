# API Reference

Complete API reference for OPNsense Config Faker.

## Core Types

### VlanConfig

Represents a VLAN configuration.

```rust
pub struct VlanConfig {
    pub id: u16,
    pub name: String,
    pub description: Option<String>,
    pub interface: String,
    pub network: IpNetwork,
}
```

**Fields**:

- `id`: VLAN identifier (1-4094)
- `name`: VLAN name
- `description`: Optional VLAN description
- `interface`: Parent interface name
- `network`: Network address and subnet

**Example**:

```rust
use opnsense_config_faker::models::VlanConfig;
use ipnet::IpNetwork;

let vlan = VlanConfig {
    id: 100,
    name: "IT_Department".to_string(),
    description: Some("IT Department VLAN".to_string()),
    interface: "em0".to_string(),
    network: "192.168.100.0/24".parse().unwrap(),
};
```

### FirewallRule

Represents a firewall rule configuration.

```rust
pub struct FirewallRule {
    pub id: u32,
    pub action: RuleAction,
    pub protocol: Protocol,
    pub source: NetworkAddress,
    pub destination: NetworkAddress,
    pub port: Option<PortRange>,
    pub description: Option<String>,
}
```

**Fields**:

- `id`: Rule identifier
- `action`: Allow or deny action
- `protocol`: Network protocol (TCP, UDP, ICMP, etc.)
- `source`: Source network address
- `destination`: Destination network address
- `port`: Optional port range
- `description`: Optional rule description

### NetworkAddress

Represents a network address with optional port.

```rust
pub struct NetworkAddress {
    pub address: IpNetwork,
    pub port: Option<u16>,
}
```

### PortRange

Represents a port range.

```rust
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}
```

## Generator Functions

### generate_vlan_config

Generates VLAN configurations.

```rust
pub fn generate_vlan_config(
    count: u32,
    base_id: u16,
    base_network: IpNetwork,
) -> Result<Vec<VlanConfig>>
```

**Parameters**:

- `count`: Number of VLANs to generate
- `base_id`: Starting VLAN ID
- `base_network`: Base network for VLAN subnets

**Returns**: `Result<Vec<VlanConfig>>` containing generated VLAN configurations

**Example**:

```rust
use opnsense_config_faker::generators::generate_vlan_config;
use ipnet::IpNetwork;

let base_network: IpNetwork = "192.168.0.0/24".parse().unwrap();
let vlans = generate_vlan_config(10, 100, base_network)?;
```

### generate_firewall_rules

Generates firewall rules.

```rust
pub fn generate_firewall_rules(
    count: u32,
    complexity: RuleComplexity,
) -> Result<Vec<FirewallRule>>
```

**Parameters**:

- `count`: Number of rules to generate
- `complexity`: Rule complexity level

**Returns**: `Result<Vec<FirewallRule>>` containing generated firewall rules

### generate_complete_config

Generates a complete OPNsense configuration.

```rust
pub fn generate_complete_config(
    vlan_count: u32,
    firewall_rule_count: u32,
    include_dhcp: bool,
    include_nat: bool,
) -> Result<CompleteConfig>
```

**Parameters**:

- `vlan_count`: Number of VLANs to generate
- `firewall_rule_count`: Number of firewall rules to generate
- `include_dhcp`: Whether to include DHCP configurations
- `include_nat`: Whether to include NAT rules

**Returns**: `Result<CompleteConfig>` containing complete configuration

## Serialization Functions

### generate_xml

Generates XML output from configurations.

```rust
pub fn generate_xml(config: &CompleteConfig) -> Result<String>
```

**Parameters**:

- `config`: Configuration to serialize

**Returns**: `Result<String>` containing XML output

### generate_csv

Generates CSV output from configurations.

```rust
pub fn generate_csv(config: &CompleteConfig) -> Result<String>
```

**Parameters**:

- `config`: Configuration to serialize

**Returns**: `Result<String>` containing CSV output

### generate_json

Generates JSON output from configurations.

```rust
pub fn generate_json(config: &CompleteConfig) -> Result<String>
```

**Parameters**:

- `config`: Configuration to serialize

**Returns**: `Result<String>` containing JSON output

## Validation Functions

### validate_vlan_config

Validates VLAN configuration.

```rust
pub fn validate_vlan_config(vlan: &VlanConfig) -> Result<()>
```

**Parameters**:

- `vlan`: VLAN configuration to validate

**Returns**: `Result<()>` indicating validation success or failure

### validate_network_range

Validates network range.

```rust
pub fn validate_network_range(network: &IpNetwork) -> Result<()>
```

**Parameters**:

- `network`: Network range to validate

**Returns**: `Result<()>` indicating validation success or failure

### validate_complete_config

Validates complete configuration.

```rust
pub fn validate_complete_config(config: &CompleteConfig) -> Result<()>
```

**Parameters**:

- `config`: Complete configuration to validate

**Returns**: `Result<()>` indicating validation success or failure

## Error Types

### ConfigGenerationError

Main error type for configuration generation.

```rust
#[derive(Debug, Error)]
pub enum ConfigGenerationError {
    #[error("Invalid VLAN ID: {id}. Must be between 1 and {max}")]
    InvalidVlanId { id: u16, max: u16 },

    #[error("Network range conflict: {range1} conflicts with {range2}")]
    NetworkRangeConflict { range1: String, range2: String },

    #[error("Invalid interface name: '{name}'")]
    InvalidInterfaceName { name: String },

    #[error("Firewall rule validation failed: {rule_name} - {reason}")]
    InvalidFirewallRule { rule_name: String, reason: String },

    #[error("Failed to write {format} output to {path}")]
    OutputWriteFailed {
        format: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("XML generation failed for {config_type}")]
    XmlGenerationFailed {
        config_type: String,
        #[source]
        source: quick_xml::Error,
    },

    #[error("CSV generation failed for {config_type}")]
    CsvGenerationFailed {
        config_type: String,
        #[source]
        source: csv::Error,
    },

    #[error("Schema validation failed: {details}")]
    SchemaValidationFailed { details: String },
}
```

### CliError

CLI-specific error type.

```rust
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid command-line argument: {0}")]
    InvalidArgument(String),

    #[error("Interactive mode failed: {0}")]
    InteractiveModeError(String),

    #[error(transparent)]
    Config(#[from] crate::model::ConfigError),
}
```

## Configuration Types

### CompleteConfig

Complete OPNsense configuration.

```rust
pub struct CompleteConfig {
    pub vlans: Vec<VlanConfig>,
    pub firewall_rules: Vec<FirewallRule>,
    pub dhcp_pools: Vec<DhcpPool>,
    pub nat_rules: Vec<NatRule>,
    pub interfaces: Vec<InterfaceConfig>,
}
```

### DhcpPool

DHCP pool configuration.

```rust
pub struct DhcpPool {
    pub id: u32,
    pub network: IpNetwork,
    pub range_start: IpAddr,
    pub range_end: IpAddr,
    pub gateway: IpAddr,
    pub dns_servers: Vec<IpAddr>,
}
```

### NatRule

NAT rule configuration.

```rust
pub struct NatRule {
    pub id: u32,
    pub source: NetworkAddress,
    pub destination: NetworkAddress,
    pub target: NetworkAddress,
    pub description: Option<String>,
}
```

### InterfaceConfig

Interface configuration.

```rust
pub struct InterfaceConfig {
    pub name: String,
    pub interface_type: InterfaceType,
    pub enabled: bool,
    pub ip_address: Option<IpAddr>,
    pub subnet_mask: Option<IpAddr>,
    pub description: Option<String>,
}
```

## Enums

### RuleAction

Firewall rule action.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Allow,
    Deny,
}
```

### Protocol

Network protocol.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Any,
}
```

### RuleComplexity

Firewall rule complexity level.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleComplexity {
    Basic,
    Intermediate,
    Advanced,
}
```

### InterfaceType

Interface type.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Physical,
    Virtual,
    Vlan,
    Bridge,
}
```

## Utility Functions

### calculate_network_range

Calculates network range for VLAN.

```rust
pub fn calculate_network_range(
    base_network: IpNetwork,
    vlan_id: u16,
) -> Result<IpNetwork>
```

### generate_realistic_name

Generates realistic name for configuration.

```rust
pub fn generate_realistic_name(
    prefix: &str,
    id: u32,
) -> String
```

### validate_ip_range

Validates IP address range.

```rust
pub fn validate_ip_range(
    start: IpAddr,
    end: IpAddr,
) -> Result<()>
```

## Constants

### Network Constants

```rust
pub const MAX_VLAN_ID: u16 = 4094;
pub const MIN_VLAN_ID: u16 = 1;
pub const DEFAULT_SUBNET_SIZE: u8 = 24;
pub const MAX_FIREWALL_RULES: u32 = 10000;
```

### Performance Constants

```rust
pub const DEFAULT_BATCH_SIZE: usize = 1000;
pub const MAX_MEMORY_USAGE: usize = 100 * 1024 * 1024; // 100MB
pub const DEFAULT_THREAD_COUNT: usize = 4;
```

## Examples

### Basic Usage

```rust
use opnsense_config_faker::generators::generate_vlan_config;
use opnsense_config_faker::serializers::generate_xml;
use ipnet::IpNetwork;

// Generate VLAN configurations
let base_network: IpNetwork = "192.168.0.0/24".parse().unwrap();
let vlans = generate_vlan_config(10, 100, base_network)?;

// Generate XML output
let xml = generate_xml(&vlans)?;
println!("{}", xml);
```

### Advanced Usage

```rust
use opnsense_config_faker::generators::generate_complete_config;
use opnsense_config_faker::validators::validate_complete_config;

// Generate complete configuration
let config = generate_complete_config(20, 50, true, true)?;

// Validate configuration
validate_complete_config(&config)?;

// Generate multiple output formats
let xml = generate_xml(&config)?;
let csv = generate_csv(&config)?;
let json = generate_json(&config)?;
```

### Error Handling

```rust
use opnsense_config_faker::models::ConfigGenerationError;

match generate_vlan_config(100, 1, "192.168.0.0/24".parse().unwrap()) {
    Ok(vlans) => {
        println!("Generated {} VLANs", vlans.len());
    }
    Err(ConfigGenerationError::InvalidVlanId { id, max }) => {
        eprintln!("Invalid VLAN ID: {} (max: {})", id, max);
    }
    Err(ConfigGenerationError::NetworkRangeConflict { range1, range2 }) => {
        eprintln!("Network range conflict: {} vs {}", range1, range2);
    }
    Err(e) => {
        eprintln!("Generation failed: {}", e);
    }
}
```

This API reference provides comprehensive documentation for all public functions, types, and constants in the OPNsense Config Faker library.
