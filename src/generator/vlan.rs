//! VLAN configuration generation

use crate::generator::departments;
use crate::model::{ConfigError, VlanError, VlanResult};
use crate::utils::rfc1918;
use crate::Result;
use indicatif::ProgressBar;
use ipnetwork::Ipv4Network;
use rand::prelude::*;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Static DHCP reservation mapping MAC address to IP
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StaticReservation {
    /// MAC address in AA:BB:CC:DD:EE:FF format
    pub mac: String,
    /// Reserved IP address
    pub ip_addr: String,
    /// Hostname for the reservation
    pub hostname: String,
}

/// DHCP server configuration with realistic enterprise settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpServerConfig {
    /// Enable DHCP server
    pub enabled: bool,
    /// DHCP range start IP
    pub range_start: String,
    /// DHCP range end IP  
    pub range_end: String,
    /// Default lease time in seconds
    pub lease_time: u32,
    /// Maximum lease time in seconds
    pub max_lease_time: u32,
    /// DNS servers list
    pub dns_servers: Vec<String>,
    /// Domain name for clients
    pub domain_name: String,
    /// Gateway IP address
    pub gateway: String,
    /// NTP servers for time synchronization
    pub ntp_servers: Vec<String>,
    /// Static IP reservations
    pub static_reservations: Vec<StaticReservation>,
}

/// VLAN configuration structure matching Python implementation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlanConfig {
    /// VLAN ID in range 10-4094 (IEEE 802.1Q standard)
    pub vlan_id: u16,

    /// IP network in "10.x.x.x" format following RFC 1918
    pub ip_network: String,

    /// Human-readable description (department + VLAN ID)
    pub description: String,

    /// WAN assignment (1-3 for multi-WAN scenarios)
    pub wan_assignment: u8,
}

impl VlanConfig {
    /// Private helper to validate IP network format strictly
    fn validate_ip_format_strict(ip_network: &str) -> Result<()> {
        // Validate IP network format - must be either "x.x.x.x" or "x.x.x.0/24" format
        let is_x_format = ip_network.ends_with(".x") && ip_network.matches('.').count() == 3;
        let is_cidr_format = ip_network.ends_with(".0/24") && ip_network.matches('.').count() == 3;

        if !is_x_format && !is_cidr_format {
            return Err(ConfigError::validation(format!(
                "IP network '{ip_network}' does not match expected format (should end with .x or .0/24)"
            )));
        }

        // Additional validation: check that we don't have empty octets (e.g., "10.1..x")
        if is_x_format {
            let prefix = ip_network.strip_suffix(".x").unwrap();
            let octets: Vec<&str> = prefix.split('.').collect();
            if octets.len() != 3 || octets.iter().any(|&octet| octet.is_empty()) {
                return Err(ConfigError::validation(format!(
                    "IP network '{ip_network}' has invalid octet structure"
                )));
            }
        } else if is_cidr_format {
            let prefix = ip_network.strip_suffix(".0/24").unwrap();
            let octets: Vec<&str> = prefix.split('.').collect();
            if octets.len() != 3 || octets.iter().any(|&octet| octet.is_empty()) {
                return Err(ConfigError::validation(format!(
                    "IP network '{ip_network}' has invalid octet structure"
                )));
            }
        }

        Ok(())
    }

    /// Create a new VLAN configuration
    pub fn new(
        vlan_id: u16,
        ip_network: String,
        description: String,
        wan_assignment: u8,
    ) -> Result<Self> {
        // Validate VLAN ID range
        if !(10..=4094).contains(&vlan_id) {
            return Err(ConfigError::validation(format!(
                "VLAN ID {vlan_id} is outside valid range 10-4094"
            )));
        }

        // Validate WAN assignment
        if !(1..=3).contains(&wan_assignment) {
            return Err(ConfigError::validation(format!(
                "WAN assignment {wan_assignment} is outside valid range 1-3"
            )));
        }

        // Validate IP network format using helper function
        Self::validate_ip_format_strict(&ip_network)?;

        Ok(Self {
            vlan_id,
            ip_network,
            description,
            wan_assignment,
        })
    }

    /// Create a new VLAN configuration with enhanced validation
    pub fn new_with_network(
        vlan_id: u16,
        network: Ipv4Network,
        description: String,
        wan_assignment: Option<u8>,
    ) -> VlanResult<Self> {
        // Validate VLAN ID range
        if !(10..=4094).contains(&vlan_id) {
            return Err(VlanError::InvalidVlanId(vlan_id));
        }

        // Validate WAN assignment
        let wan = wan_assignment.unwrap_or(1);
        if !(1..=3).contains(&wan) {
            return Err(VlanError::InvalidWanAssignment(wan));
        }

        // Validate RFC 1918 compliance
        if !rfc1918::is_rfc1918_network(&network) {
            return Err(VlanError::NonRfc1918Network(network.to_string()));
        }

        // Convert network to string format for compatibility
        let ip_network = format!(
            "{}.x",
            network
                .network()
                .octets()
                .iter()
                .take(3)
                .map(|octet| octet.to_string())
                .collect::<Vec<_>>()
                .join(".")
        );

        Ok(Self {
            vlan_id,
            ip_network,
            description,
            wan_assignment: wan,
        })
    }

    /// Get the network as an Ipv4Network if possible
    pub fn as_ipv4_network(&self) -> VlanResult<Ipv4Network> {
        if let Some(base) = self.ip_network.strip_suffix(".x") {
            let network_str = format!("{base}.0/24");
            rfc1918::validate_rfc1918_network_string(&network_str)
        } else if let Some(base) = self.ip_network.strip_suffix(".0/24") {
            let network_str = format!("{base}.0/24");
            rfc1918::validate_rfc1918_network_string(&network_str)
        } else {
            Err(VlanError::network_parsing(format!(
                "Cannot parse network format: {}",
                self.ip_network
            )))
        }
    }

    /// Validate that this configuration is RFC 1918 compliant
    pub fn validate_rfc1918(&self) -> VlanResult<()> {
        self.as_ipv4_network().map(|_| ())
    }

    /// Comprehensive validation of VLAN configuration
    pub fn validate(&self) -> Result<()> {
        // Validate VLAN ID range
        if !(10..=4094).contains(&self.vlan_id) {
            return Err(ConfigError::validation(format!(
                "VLAN ID {} is outside valid range 10-4094",
                self.vlan_id
            )));
        }

        // Validate WAN assignment
        if !(1..=3).contains(&self.wan_assignment) {
            return Err(ConfigError::validation(format!(
                "WAN assignment {} is outside valid range 1-3",
                self.wan_assignment
            )));
        }

        // Validate IP network format using helper function
        Self::validate_ip_format_strict(&self.ip_network)?;

        // Validate description is not empty (including whitespace-only)
        if self.description.trim().is_empty() {
            return Err(ConfigError::validation("VLAN description cannot be empty"));
        }

        // Validate RFC 1918 compliance
        if let Err(e) = self.validate_rfc1918() {
            return Err(ConfigError::validation(format!(
                "VLAN network is not RFC 1918 compliant: {}",
                e
            )));
        }

        Ok(())
    }

    /// Get the subnet mask for this VLAN (always /24 for compatibility)
    pub fn subnet_mask(&self) -> &'static str {
        "255.255.255.0"
    }

    /// Get the gateway IP address (network + 1)
    pub fn gateway_ip(&self) -> Result<String> {
        if let Some(base) = self.ip_network.strip_suffix(".x") {
            Ok(format!("{base}.1"))
        } else if let Some(base) = self.ip_network.strip_suffix(".0/24") {
            Ok(format!("{base}.1"))
        } else {
            Err(ConfigError::validation(format!(
                "Cannot derive gateway from IP network: {}",
                self.ip_network
            )))
        }
    }

    /// Get the DHCP range start IP
    pub fn dhcp_range_start(&self) -> Result<String> {
        if let Some(base) = self.ip_network.strip_suffix(".x") {
            Ok(format!("{base}.100"))
        } else if let Some(base) = self.ip_network.strip_suffix(".0/24") {
            Ok(format!("{base}.100"))
        } else {
            Err(ConfigError::validation(format!(
                "Cannot derive DHCP range from IP network: {}",
                self.ip_network
            )))
        }
    }

    /// Get the DHCP range end IP
    pub fn dhcp_range_end(&self) -> Result<String> {
        if let Some(base) = self.ip_network.strip_suffix(".x") {
            Ok(format!("{base}.200"))
        } else if let Some(base) = self.ip_network.strip_suffix(".0/24") {
            Ok(format!("{base}.200"))
        } else {
            Err(ConfigError::validation(format!(
                "Cannot derive DHCP range from IP network: {}",
                self.ip_network
            )))
        }
    }

    /// Get the DHCP lease time based on department type (in seconds)
    pub fn dhcp_lease_time(&self) -> u32 {
        // Determine lease time based on department characteristics
        match self.description.split(' ').next().unwrap_or("Unknown") {
            // Corporate departments - longer lease times (24 hours)
            "IT" | "Finance" | "Accounting" | "Legal" | "Management" => 86400,
            // Production environments - medium lease times (12 hours)
            "Engineering" | "Development" | "QA" | "Research" | "Operations" => 43200,
            // Dynamic environments - shorter lease times (8 hours)
            "Sales" | "Marketing" | "Support" | "Customer Service" | "Training" => 28800,
            // High-mobility environments - very short lease times (4 hours)
            "HR" | "Procurement" | "Logistics" => 14400,
            // Security-sensitive - short lease times (6 hours) for easier tracking
            "Security" => 21600,
            // Default for unknown departments (8 hours)
            _ => 28800,
        }
    }

    /// Get the maximum DHCP lease time (typically 2x the default lease time)
    pub fn dhcp_max_lease_time(&self) -> u32 {
        self.dhcp_lease_time() * 2
    }

    /// Get the DHCP domain name based on department context
    pub fn dhcp_domain_name(&self) -> String {
        let department = self
            .description
            .split(' ')
            .next()
            .unwrap_or("unknown")
            .to_lowercase();
        format!("{}.company.local", department)
    }

    /// Get DNS servers list (gateway + reliable public DNS)
    pub fn dhcp_dns_servers(&self) -> Result<Vec<String>> {
        let mut dns_servers = Vec::new();

        // Add gateway as primary DNS
        if let Ok(gateway) = self.gateway_ip() {
            dns_servers.push(gateway);
        }

        // Add reliable public DNS servers as secondary
        dns_servers.push("8.8.8.8".to_string()); // Google DNS
        dns_servers.push("1.1.1.1".to_string()); // Cloudflare DNS

        Ok(dns_servers)
    }

    /// Get NTP servers appropriate for corporate environments
    pub fn dhcp_ntp_servers(&self) -> Vec<String> {
        vec![
            "pool.ntp.org".to_string(),
            "time.nist.gov".to_string(),
            "time.cloudflare.com".to_string(),
        ]
    }

    /// Generate static DHCP reservations with realistic MAC-IP mappings
    pub fn static_reservations(&self) -> Result<Vec<StaticReservation>> {
        let mut reservations = Vec::new();

        // Get base network for IP assignments
        let base = if let Some(base) = self.ip_network.strip_suffix(".x") {
            base
        } else if let Some(base) = self.ip_network.strip_suffix(".0/24") {
            base
        } else {
            return Err(ConfigError::validation(format!(
                "Cannot derive static reservations from IP network: {}",
                self.ip_network
            )));
        };

        // Generate department-specific static reservations
        let department = self
            .description
            .split(' ')
            .next()
            .unwrap_or("unknown")
            .to_lowercase();
        match department.as_str() {
            "it" | "engineering" | "development" => {
                // IT departments typically have servers and network equipment
                reservations.push(StaticReservation {
                    mac: format!("aa:bb:cc:dd:ee:{:02x}", self.vlan_id % 256),
                    ip_addr: format!("{}.10", base),
                    hostname: format!("server-{}-01", department),
                });
                reservations.push(StaticReservation {
                    mac: format!("aa:bb:cc:dd:ef:{:02x}", self.vlan_id % 256),
                    ip_addr: format!("{}.11", base),
                    hostname: format!("printer-{}-01", department),
                });
            }
            "finance" | "accounting" | "legal" => {
                // Finance departments typically have specialized workstations
                reservations.push(StaticReservation {
                    mac: format!("aa:bb:cc:dd:f0:{:02x}", self.vlan_id % 256),
                    ip_addr: format!("{}.15", base),
                    hostname: format!("workstation-{}-01", department),
                });
            }
            "sales" | "marketing" => {
                // Sales departments typically have presentation equipment
                reservations.push(StaticReservation {
                    mac: format!("aa:bb:cc:dd:f1:{:02x}", self.vlan_id % 256),
                    ip_addr: format!("{}.20", base),
                    hostname: format!("display-{}-01", department),
                });
            }
            _ => {
                // Default reservation for other departments
                reservations.push(StaticReservation {
                    mac: format!("aa:bb:cc:dd:f2:{:02x}", self.vlan_id % 256),
                    ip_addr: format!("{}.25", base),
                    hostname: format!("device-{}-01", department),
                });
            }
        }

        Ok(reservations)
    }

    /// Generate complete DHCP server configuration
    pub fn dhcp_server_config(&self) -> Result<DhcpServerConfig> {
        Ok(DhcpServerConfig {
            enabled: true,
            range_start: self.dhcp_range_start()?,
            range_end: self.dhcp_range_end()?,
            lease_time: self.dhcp_lease_time(),
            max_lease_time: self.dhcp_max_lease_time(),
            dns_servers: self.dhcp_dns_servers()?,
            domain_name: self.dhcp_domain_name(),
            gateway: self.gateway_ip()?,
            ntp_servers: self.dhcp_ntp_servers(),
            static_reservations: self.static_reservations()?,
        })
    }
}

/// VLAN configuration generator with enhanced RFC 1918 compliance
pub struct VlanGenerator {
    rng: Box<dyn RngCore>,
    used_vlan_ids: HashSet<u16>,
    used_networks: HashSet<String>,
}

impl VlanGenerator {
    /// Create a new generator with optional seed using ChaCha8Rng
    pub fn new(seed: Option<u64>) -> Self {
        let rng: Box<dyn RngCore> = if let Some(seed) = seed {
            Box::new(ChaCha8Rng::seed_from_u64(seed))
        } else {
            Box::new(ChaCha8Rng::from_rng(&mut rand::rng()))
        };

        Self {
            rng,
            used_vlan_ids: HashSet::new(),
            used_networks: HashSet::new(),
        }
    }

    /// Create a new generator with StdRng for compatibility
    pub fn new_with_std_rng(seed: Option<u64>) -> Self {
        let rng: Box<dyn RngCore> = if let Some(seed) = seed {
            Box::new(StdRng::seed_from_u64(seed))
        } else {
            Box::new(StdRng::from_rng(&mut rand::rng()))
        };

        Self {
            rng,
            used_vlan_ids: HashSet::new(),
            used_networks: HashSet::new(),
        }
    }

    /// Generate a single VLAN configuration
    pub fn generate_single(&mut self) -> Result<VlanConfig> {
        const MAX_ATTEMPTS: usize = 1000;

        // Generate unique VLAN ID
        let vlan_id = self.generate_unique_vlan_id(MAX_ATTEMPTS)?;

        // Generate unique IP network
        let ip_network = self.generate_unique_ip_network(MAX_ATTEMPTS)?;

        // Generate description using new department constants
        let description = self.generate_description(vlan_id);

        // Generate WAN assignment
        let wan_assignment = self.rng.random_range(1..=3);

        VlanConfig::new(vlan_id, ip_network, description, wan_assignment)
    }

    /// Generate a single VLAN configuration with enhanced validation
    pub fn generate_single_enhanced(&mut self) -> VlanResult<VlanConfig> {
        const MAX_ATTEMPTS: usize = 1000;

        // Generate unique VLAN ID
        let vlan_id = self.generate_unique_vlan_id_enhanced(MAX_ATTEMPTS)?;

        // Generate unique RFC 1918 network
        let network = self.generate_unique_rfc1918_network(MAX_ATTEMPTS)?;

        // Generate description using new department constants
        let description = self.generate_description_enhanced(vlan_id);

        // Generate WAN assignment
        let wan_assignment = Some(self.rng.random_range(1..=3));

        VlanConfig::new_with_network(vlan_id, network, description, wan_assignment)
    }

    /// Generate a batch of VLAN configurations
    pub fn generate_batch(&mut self, count: usize) -> Result<Vec<VlanConfig>> {
        let mut configs = Vec::with_capacity(count);

        for _ in 0..count {
            let config = self.generate_single()?;
            configs.push(config);
        }

        Ok(configs)
    }

    /// Generate a batch of VLAN configurations with enhanced validation
    pub fn generate_batch_enhanced(&mut self, count: usize) -> VlanResult<Vec<VlanConfig>> {
        let mut configs = Vec::with_capacity(count);

        for _ in 0..count {
            let config = self.generate_single_enhanced()?;
            configs.push(config);
        }

        Ok(configs)
    }

    /// Generate unique VLAN ID
    fn generate_unique_vlan_id(&mut self, max_attempts: usize) -> Result<u16> {
        for _ in 0..max_attempts {
            let vlan_id = self.rng.random_range(10..=4094);
            if self.used_vlan_ids.insert(vlan_id) {
                return Ok(vlan_id);
            }
        }

        Err(ConfigError::resource_exhausted("VLAN IDs"))
    }

    /// Generate unique VLAN ID with enhanced error handling
    fn generate_unique_vlan_id_enhanced(&mut self, max_attempts: usize) -> VlanResult<u16> {
        for _ in 0..max_attempts {
            let vlan_id = self.rng.random_range(10..=4094);
            if self.used_vlan_ids.insert(vlan_id) {
                return Ok(vlan_id);
            }
        }

        Err(VlanError::VlanIdExhausted)
    }

    /// Generate unique RFC 1918 private IP network
    fn generate_unique_ip_network(&mut self, max_attempts: usize) -> Result<String> {
        for _ in 0..max_attempts {
            // Generate Class A private network (10.0.0.0/8)
            let second_octet = self.rng.random_range(1..=254);
            let third_octet = self.rng.random_range(1..=254);
            let network = format!("10.{second_octet}.{third_octet}.x");

            if self.used_networks.insert(network.clone()) {
                return Ok(network);
            }
        }

        Err(ConfigError::resource_exhausted("IP networks"))
    }

    /// Generate unique RFC 1918 network using ipnetwork types
    fn generate_unique_rfc1918_network(&mut self, max_attempts: usize) -> VlanResult<Ipv4Network> {
        for _ in 0..max_attempts {
            // Prefer Class A networks for larger address space
            let network = if self.rng.random_bool(0.8) {
                rfc1918::generate_random_class_a_network(&mut self.rng)
            } else if self.rng.random_bool(0.6) {
                rfc1918::generate_random_class_b_network(&mut self.rng)
            } else {
                rfc1918::generate_random_class_c_network(&mut self.rng)
            };

            let network_key = network.to_string();
            if self.used_networks.insert(network_key) {
                return Ok(network);
            }
        }

        Err(VlanError::NetworkExhausted)
    }

    /// Generate department-based description using legacy constants
    fn generate_description(&mut self, vlan_id: u16) -> String {
        const DEPARTMENTS: &[&str] = &[
            "Sales",
            "IT",
            "HR",
            "Finance",
            "Marketing",
            "Operations",
            "Engineering",
            "Support",
            "Legal",
            "Procurement",
            "Security",
            "Development",
            "QA",
            "Research",
            "Training",
            "Management",
        ];

        let department = DEPARTMENTS[self.rng.random_range(0..DEPARTMENTS.len())];
        format!("{department} VLAN {vlan_id}")
    }

    /// Generate department-based description using new constants
    fn generate_description_enhanced(&mut self, vlan_id: u16) -> String {
        let department = departments::random_department(&mut self.rng);
        format!("{department} VLAN {vlan_id}")
    }
}

/// Generate multiple VLAN configurations using legacy StdRng for compatibility
pub fn generate_vlan_configurations(
    count: u16,
    seed: Option<u64>,
    progress_bar: Option<&ProgressBar>,
) -> Result<Vec<VlanConfig>> {
    let mut generator = VlanGenerator::new_with_std_rng(seed);
    let mut configs = Vec::with_capacity(count as usize);

    for i in 0..count {
        let config = generator.generate_single()?;
        configs.push(config);

        if let Some(pb) = progress_bar {
            pb.set_position(i as u64 + 1);
        }
    }

    Ok(configs)
}

/// Generate multiple VLAN configurations using enhanced ChaCha8Rng
pub fn generate_vlan_configurations_enhanced(
    count: u16,
    seed: Option<u64>,
    progress_bar: Option<&ProgressBar>,
) -> VlanResult<Vec<VlanConfig>> {
    let mut generator = VlanGenerator::new(seed);
    let mut configs = Vec::with_capacity(count as usize);

    for i in 0..count {
        let config = generator.generate_single_enhanced()?;
        configs.push(config);

        if let Some(pb) = progress_bar {
            pb.set_position(i as u64 + 1);
        }
    }

    Ok(configs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ConfigError;

    // ===== VlanConfig::new() Validation Tests =====

    #[test]
    fn test_vlan_config_new_validates_vlan_id_range() {
        // Test valid VLAN IDs (10-4094 range)
        assert!(VlanConfig::new(10, "10.1.2.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(2000, "10.1.2.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(4094, "10.1.2.x".to_string(), "Test".to_string(), 1).is_ok());

        // Test edge cases for valid range
        let min_valid = VlanConfig::new(10, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(min_valid.is_ok());
        assert_eq!(min_valid.unwrap().vlan_id, 10);

        let max_valid = VlanConfig::new(4094, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(max_valid.is_ok());
        assert_eq!(max_valid.unwrap().vlan_id, 4094);
    }

    #[test]
    fn test_vlan_config_new_fails_for_vlan_id_out_of_range() {
        // Test VLAN IDs below valid range
        let result = VlanConfig::new(9, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::Validation { .. }
        ));

        let result = VlanConfig::new(0, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(result.is_err());

        // Test VLAN IDs above valid range
        let result = VlanConfig::new(4095, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::Validation { .. }
        ));

        let result = VlanConfig::new(5000, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(result.is_err());

        // Test exact boundaries
        let below_min = VlanConfig::new(9, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(below_min.is_err());
        let error_msg = below_min.unwrap_err().to_string();
        assert!(error_msg.contains("VLAN ID 9 is outside valid range 10-4094"));

        let above_max = VlanConfig::new(4095, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(above_max.is_err());
        let error_msg = above_max.unwrap_err().to_string();
        assert!(error_msg.contains("VLAN ID 4095 is outside valid range 10-4094"));
    }

    #[test]
    fn test_vlan_config_new_validates_wan_assignment_range() {
        // Test valid WAN assignments (1-3 range)
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 2).is_ok());
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 3).is_ok());

        // Test edge cases for valid range
        let min_valid = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1);
        assert!(min_valid.is_ok());
        assert_eq!(min_valid.unwrap().wan_assignment, 1);

        let max_valid = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 3);
        assert!(max_valid.is_ok());
        assert_eq!(max_valid.unwrap().wan_assignment, 3);
    }

    #[test]
    fn test_vlan_config_new_fails_for_wan_out_of_range() {
        // Test WAN assignments below valid range
        let result = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::Validation { .. }
        ));

        // Test WAN assignments above valid range
        let result = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 4);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::Validation { .. }
        ));

        let result = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 10);
        assert!(result.is_err());

        // Test exact boundaries and verify error messages
        let below_min = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 0);
        assert!(below_min.is_err());
        let error_msg = below_min.unwrap_err().to_string();
        assert!(error_msg.contains("WAN assignment 0 is outside valid range 1-3"));

        let above_max = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 4);
        assert!(above_max.is_err());
        let error_msg = above_max.unwrap_err().to_string();
        assert!(error_msg.contains("WAN assignment 4 is outside valid range 1-3"));
    }

    #[test]
    fn test_vlan_config_new_validates_ip_network_format() {
        // Test valid IP network formats
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(100, "192.168.1.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(100, "172.16.0.x".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(100, "10.1.2.0/24".to_string(), "Test".to_string(), 1).is_ok());
        assert!(VlanConfig::new(100, "192.168.1.0/24".to_string(), "Test".to_string(), 1).is_ok());

        // Test creating config and verifying the network is preserved
        let config =
            VlanConfig::new(100, "10.123.45.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.ip_network, "10.123.45.x");
    }

    #[test]
    fn test_vlan_config_new_fails_for_malformed_network() {
        // Test invalid network formats that should fail
        let invalid_formats = vec![
            "10.1.2.1",    // Plain IP, no .x or .0/24
            "10.1.2",      // Incomplete
            "10.1.2.0",    // Missing /24
            "10.1.2.0/16", // Wrong CIDR
            "10.1.2.y",    // Wrong placeholder
            "192.168.1.z", // Wrong placeholder
            "invalid",     // Completely invalid
            "",            // Empty string
            "10.1.2.0/",   // Incomplete CIDR
            "10.1.2./24",  // Missing octet
            "10.1..x",     // Missing octet
        ];

        for invalid_network in invalid_formats {
            let result = VlanConfig::new(100, invalid_network.to_string(), "Test".to_string(), 1);
            assert!(
                result.is_err(),
                "Network format '{invalid_network}' should be invalid"
            );
            assert!(matches!(
                result.unwrap_err(),
                ConfigError::Validation { .. }
            ));
        }

        // Test specific error message
        let result = VlanConfig::new(100, "10.1.2.1".to_string(), "Test".to_string(), 1);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("IP network '10.1.2.1' does not match expected format"));
    }

    // ===== IP Address Derivation Tests =====

    #[test]
    fn test_gateway_ip_derives_from_x_format() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.gateway_ip().unwrap(), "10.1.2.1");

        let config2 =
            VlanConfig::new(200, "192.168.50.x".to_string(), "Test".to_string(), 2).unwrap();
        assert_eq!(config2.gateway_ip().unwrap(), "192.168.50.1");

        let config3 =
            VlanConfig::new(300, "172.16.100.x".to_string(), "Test".to_string(), 3).unwrap();
        assert_eq!(config3.gateway_ip().unwrap(), "172.16.100.1");
    }

    #[test]
    fn test_gateway_ip_derives_from_cidr_format() {
        let config =
            VlanConfig::new(100, "10.1.2.0/24".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.gateway_ip().unwrap(), "10.1.2.1");

        let config2 =
            VlanConfig::new(200, "192.168.50.0/24".to_string(), "Test".to_string(), 2).unwrap();
        assert_eq!(config2.gateway_ip().unwrap(), "192.168.50.1");
    }

    #[test]
    fn test_dhcp_range_start_derives_from_x_format() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.dhcp_range_start().unwrap(), "10.1.2.100");

        let config2 =
            VlanConfig::new(200, "192.168.50.x".to_string(), "Test".to_string(), 2).unwrap();
        assert_eq!(config2.dhcp_range_start().unwrap(), "192.168.50.100");
    }

    #[test]
    fn test_dhcp_range_start_derives_from_cidr_format() {
        let config =
            VlanConfig::new(100, "10.1.2.0/24".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.dhcp_range_start().unwrap(), "10.1.2.100");

        let config2 =
            VlanConfig::new(200, "192.168.50.0/24".to_string(), "Test".to_string(), 2).unwrap();
        assert_eq!(config2.dhcp_range_start().unwrap(), "192.168.50.100");
    }

    #[test]
    fn test_dhcp_range_end_derives_from_x_format() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.dhcp_range_end().unwrap(), "10.1.2.200");

        let config2 =
            VlanConfig::new(200, "192.168.50.x".to_string(), "Test".to_string(), 2).unwrap();
        assert_eq!(config2.dhcp_range_end().unwrap(), "192.168.50.200");
    }

    #[test]
    fn test_dhcp_range_end_derives_from_cidr_format() {
        let config =
            VlanConfig::new(100, "10.1.2.0/24".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.dhcp_range_end().unwrap(), "10.1.2.200");

        let config2 =
            VlanConfig::new(200, "192.168.50.0/24".to_string(), "Test".to_string(), 2).unwrap();
        assert_eq!(config2.dhcp_range_end().unwrap(), "192.168.50.200");
    }

    #[test]
    fn test_gateway_dhcp_methods_fail_when_network_invalid() {
        // Create a config with invalid network by bypassing validation
        // (This simulates a corrupted or manually modified config)
        let mut config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        config.ip_network = "invalid.network".to_string();

        // All IP derivation methods should fail
        assert!(config.gateway_ip().is_err());
        assert!(config.dhcp_range_start().is_err());
        assert!(config.dhcp_range_end().is_err());

        // Test specific error messages
        let gateway_error = config.gateway_ip().unwrap_err();
        assert!(gateway_error
            .to_string()
            .contains("Cannot derive gateway from IP network: invalid.network"));

        let dhcp_start_error = config.dhcp_range_start().unwrap_err();
        assert!(dhcp_start_error
            .to_string()
            .contains("Cannot derive DHCP range from IP network: invalid.network"));

        let dhcp_end_error = config.dhcp_range_end().unwrap_err();
        assert!(dhcp_end_error
            .to_string()
            .contains("Cannot derive DHCP range from IP network: invalid.network"));
    }

    // ===== VlanGenerator Tests =====

    #[test]
    fn test_vlan_generator_produces_unique_vlan_ids() {
        let mut generator = VlanGenerator::new(Some(42)); // Fixed seed for reproducibility
        let mut vlan_ids = HashSet::new();

        // Generate multiple configs and verify uniqueness
        for _ in 0..10 {
            let config = generator.generate_single().unwrap();
            assert!(
                vlan_ids.insert(config.vlan_id),
                "Duplicate VLAN ID generated: {}",
                config.vlan_id
            );
            assert!((10..=4094).contains(&config.vlan_id));
        }

        assert_eq!(vlan_ids.len(), 10);
    }

    #[test]
    fn test_vlan_generator_produces_unique_networks() {
        let mut generator = VlanGenerator::new(Some(42)); // Fixed seed for reproducibility
        let mut networks = HashSet::new();

        // Generate multiple configs and verify network uniqueness
        for _ in 0..10 {
            let config = generator.generate_single().unwrap();
            assert!(
                networks.insert(config.ip_network.clone()),
                "Duplicate network generated: {}",
                config.ip_network
            );
            // Verify network format
            assert!(config.ip_network.starts_with("10."));
            assert!(config.ip_network.ends_with(".x"));
        }

        assert_eq!(networks.len(), 10);
    }

    #[test]
    fn test_vlan_generator_single_produces_valid_config() {
        let mut generator = VlanGenerator::new(Some(12345));
        let config = generator.generate_single().unwrap();

        // Verify all invariants
        assert!((10..=4094).contains(&config.vlan_id));
        assert!((1..=3).contains(&config.wan_assignment));
        assert!(config.ip_network.starts_with("10."));
        assert!(config.ip_network.ends_with(".x"));
        assert!(!config.description.is_empty());
        assert!(config.description.contains(&config.vlan_id.to_string()));
    }

    #[test]
    fn test_generate_vlan_configurations_produces_unique_configs() {
        let configs = generate_vlan_configurations(20, Some(42), None).unwrap();
        assert_eq!(configs.len(), 20);

        // Check VLAN ID uniqueness
        let mut vlan_ids = HashSet::new();
        for config in &configs {
            assert!(
                vlan_ids.insert(config.vlan_id),
                "Duplicate VLAN ID: {}",
                config.vlan_id
            );
        }
        assert_eq!(vlan_ids.len(), 20);

        // Check network uniqueness
        let mut networks = HashSet::new();
        for config in &configs {
            assert!(
                networks.insert(&config.ip_network),
                "Duplicate network: {}",
                config.ip_network
            );
        }
        assert_eq!(networks.len(), 20);

        // Verify all configs meet invariants
        for config in &configs {
            assert!((10..=4094).contains(&config.vlan_id));
            assert!((1..=3).contains(&config.wan_assignment));
            assert!(config.ip_network.starts_with("10."));
            assert!(config.ip_network.ends_with(".x"));
        }
    }

    // ===== Edge Case and Stress Tests =====

    #[test]
    fn test_vlan_config_creation_with_boundary_values() {
        // Test all combinations of boundary values
        let boundary_configs = vec![
            (10, "10.1.1.x", 1),
            (10, "10.1.1.0/24", 3),
            (4094, "10.255.254.x", 1),
            (4094, "192.168.255.0/24", 3),
        ];

        for (vlan_id, network, wan) in boundary_configs {
            let config = VlanConfig::new(
                vlan_id,
                network.to_string(),
                format!("Test VLAN {vlan_id}"),
                wan,
            )
            .unwrap();

            assert_eq!(config.vlan_id, vlan_id);
            assert_eq!(config.ip_network, network);
            assert_eq!(config.wan_assignment, wan);

            // Verify derived addresses work
            assert!(config.gateway_ip().is_ok());
            assert!(config.dhcp_range_start().is_ok());
            assert!(config.dhcp_range_end().is_ok());
        }
    }

    #[test]
    fn test_vlan_generator_with_different_seeds() {
        // Test that different seeds produce different (but valid) results
        let mut gen1 = VlanGenerator::new(Some(12345));
        let mut gen2 = VlanGenerator::new(Some(67890));

        let config1 = gen1.generate_single().unwrap();
        let config2 = gen2.generate_single().unwrap();

        // Both should be valid but likely different
        assert!((10..=4094).contains(&config1.vlan_id));
        assert!((10..=4094).contains(&config2.vlan_id));
        assert!((1..=3).contains(&config1.wan_assignment));
        assert!((1..=3).contains(&config2.wan_assignment));

        // With different seeds, results should usually be different
        // (Not guaranteed, but extremely likely with good seeds)
    }

    #[test]
    fn test_subnet_mask_is_always_24() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.subnet_mask(), "255.255.255.0");

        // Should be consistent regardless of config parameters
        let config2 =
            VlanConfig::new(4094, "192.168.1.0/24".to_string(), "Other".to_string(), 3).unwrap();
        assert_eq!(config2.subnet_mask(), "255.255.255.0");
    }

    #[test]
    fn test_description_generation_includes_vlan_id() {
        let mut generator = VlanGenerator::new(Some(42));

        for _ in 0..5 {
            let config = generator.generate_single().unwrap();
            // Description should contain the VLAN ID
            assert!(config.description.contains(&config.vlan_id.to_string()));
            // Should contain "VLAN" keyword
            assert!(config.description.to_uppercase().contains("VLAN"));
            // Should not be empty
            assert!(!config.description.trim().is_empty());
        }
    }

    // ===== Legacy compatibility tests =====

    #[test]
    fn test_vlan_config_creation_legacy() {
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap();

        assert_eq!(config.vlan_id, 100);
        assert_eq!(config.ip_network, "10.1.2.x");
        assert_eq!(config.description, "Test VLAN 100");
        assert_eq!(config.wan_assignment, 1);
    }

    #[test]
    fn test_generate_multiple_configs_legacy() {
        let configs = generate_vlan_configurations(5, Some(42), None).unwrap();
        assert_eq!(configs.len(), 5);

        // Check uniqueness
        let mut vlan_ids = HashSet::new();
        let mut networks = HashSet::new();

        for config in &configs {
            assert!(
                vlan_ids.insert(config.vlan_id),
                "Duplicate VLAN ID: {}",
                config.vlan_id
            );
            assert!(
                networks.insert(&config.ip_network),
                "Duplicate network: {}",
                config.ip_network
            );
        }
    }

    // ===== Enhanced functionality tests =====

    #[test]
    fn test_vlan_config_new_with_network() {
        let network = "10.1.2.0/24".parse::<Ipv4Network>().unwrap();
        let config = VlanConfig::new_with_network(
            100,
            network,
            "Test Department VLAN 100".to_string(),
            Some(2),
        )
        .unwrap();

        assert_eq!(config.vlan_id, 100);
        assert_eq!(config.ip_network, "10.1.2.x");
        assert_eq!(config.description, "Test Department VLAN 100");
        assert_eq!(config.wan_assignment, 2);
    }

    #[test]
    fn test_vlan_config_new_with_network_validation() {
        let network = "10.1.2.0/24".parse::<Ipv4Network>().unwrap();

        // Invalid VLAN ID
        assert!(VlanConfig::new_with_network(9, network, "Test".to_string(), Some(1)).is_err());

        // Invalid WAN assignment
        assert!(VlanConfig::new_with_network(100, network, "Test".to_string(), Some(4)).is_err());

        // Non-RFC 1918 network
        let public_network = "8.8.8.0/24".parse::<Ipv4Network>().unwrap();
        assert!(
            VlanConfig::new_with_network(100, public_network, "Test".to_string(), Some(1)).is_err()
        );
    }

    #[test]
    fn test_vlan_config_as_ipv4_network() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        let network = config.as_ipv4_network().unwrap();
        assert_eq!(network.to_string(), "10.1.2.0/24");

        let config2 =
            VlanConfig::new(200, "192.168.1.0/24".to_string(), "Test".to_string(), 1).unwrap();
        let network2 = config2.as_ipv4_network().unwrap();
        assert_eq!(network2.to_string(), "192.168.1.0/24");
    }

    #[test]
    fn test_vlan_config_validate_rfc1918() {
        // Valid RFC 1918 networks
        let config1 = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert!(config1.validate_rfc1918().is_ok());

        let config2 =
            VlanConfig::new(200, "172.16.1.x".to_string(), "Test".to_string(), 1).unwrap();
        assert!(config2.validate_rfc1918().is_ok());

        let config3 =
            VlanConfig::new(300, "192.168.1.x".to_string(), "Test".to_string(), 1).unwrap();
        assert!(config3.validate_rfc1918().is_ok());
    }

    #[test]
    fn test_vlan_generator_enhanced() {
        let mut generator = VlanGenerator::new(Some(42));
        let config = generator.generate_single_enhanced().unwrap();

        // Verify all enhanced invariants
        assert!((10..=4094).contains(&config.vlan_id));
        assert!((1..=3).contains(&config.wan_assignment));
        assert!(config.validate_rfc1918().is_ok());
        assert!(!config.description.is_empty());
    }

    #[test]
    fn test_vlan_generator_batch_enhanced() {
        let mut generator = VlanGenerator::new(Some(42));
        let configs = generator.generate_batch_enhanced(10).unwrap();
        assert_eq!(configs.len(), 10);

        // Check uniqueness
        let mut vlan_ids = HashSet::new();
        let mut networks = HashSet::new();

        for config in &configs {
            assert!(vlan_ids.insert(config.vlan_id));
            assert!(networks.insert(&config.ip_network));
            assert!(config.validate_rfc1918().is_ok());
        }
    }

    #[test]
    fn test_vlan_generator_with_chacha_rng() {
        let mut gen1 = VlanGenerator::new(Some(12345));
        let mut gen2 = VlanGenerator::new(Some(12345));

        // Same seed should produce same sequence with ChaCha8Rng
        let config1 = gen1.generate_single().unwrap();
        let config2 = gen2.generate_single().unwrap();

        assert_eq!(config1.vlan_id, config2.vlan_id);
        assert_eq!(config1.ip_network, config2.ip_network);
        assert_eq!(config1.wan_assignment, config2.wan_assignment);
    }

    #[test]
    fn test_department_descriptions_enhanced() {
        let mut generator = VlanGenerator::new(Some(42));
        let config = generator.generate_single_enhanced().unwrap();

        // Should use the enhanced department list
        assert!(config.description.contains("VLAN"));
        assert!(config.description.contains(&config.vlan_id.to_string()));

        // Should be one of the valid departments
        let dept_names = crate::generator::departments::all_departments();
        let description_parts: Vec<&str> = config.description.split_whitespace().collect();
        assert!(description_parts.len() >= 3); // Dept + "VLAN" + ID

        let department = description_parts[0];
        assert!(dept_names.contains(&department));
    }

    #[test]
    fn test_rfc1918_network_generation() {
        let mut generator = VlanGenerator::new(Some(42));

        for _ in 0..20 {
            let config = generator.generate_single_enhanced().unwrap();
            let network = config.as_ipv4_network().unwrap();
            assert!(crate::utils::rfc1918::is_rfc1918_network(&network));
        }
    }

    #[test]
    fn test_generator_compatibility() {
        // Test that both old and new methods work
        let mut generator = VlanGenerator::new(Some(42));

        let old_config = generator.generate_single().unwrap();
        let new_config = generator.generate_single_enhanced().unwrap();

        // Both should meet basic requirements
        assert!((10..=4094).contains(&old_config.vlan_id));
        assert!((10..=4094).contains(&new_config.vlan_id));
        assert!(old_config.vlan_id != new_config.vlan_id); // Should be unique
    }

    #[test]
    fn test_memory_efficiency() {
        let mut generator = VlanGenerator::new(Some(42));

        // Generate large batch to test memory efficiency
        let configs = generator.generate_batch_enhanced(100).unwrap();
        assert_eq!(configs.len(), 100);

        // Verify all are unique and valid
        let mut vlan_ids = HashSet::new();
        let mut networks = HashSet::new();

        for config in &configs {
            assert!(vlan_ids.insert(config.vlan_id));
            assert!(networks.insert(&config.ip_network));
            assert!(config.validate_rfc1918().is_ok());
        }

        assert_eq!(vlan_ids.len(), 100);
        assert_eq!(networks.len(), 100);
    }

    #[test]
    fn test_enhanced_public_api() {
        use crate::generator::vlan::generate_vlan_configurations_enhanced;

        let configs = generate_vlan_configurations_enhanced(5, Some(42), None).unwrap();
        assert_eq!(configs.len(), 5);

        // Verify all configs are RFC 1918 compliant
        for config in &configs {
            assert!(config.validate_rfc1918().is_ok());
            assert!((10..=4094).contains(&config.vlan_id));
            assert!((1..=3).contains(&config.wan_assignment));
        }

        // Verify uniqueness
        let mut vlan_ids = HashSet::new();
        let mut networks = HashSet::new();

        for config in &configs {
            assert!(vlan_ids.insert(config.vlan_id));
            assert!(networks.insert(&config.ip_network));
        }

        assert_eq!(vlan_ids.len(), 5);
        assert_eq!(networks.len(), 5);
    }

    #[test]
    fn test_vlan_config_validate_success() {
        let valid_config = VlanConfig::new(
            100,
            "192.168.100.x".to_string(),
            "IT_VLAN_0100".to_string(),
            1,
        )
        .unwrap();

        assert!(valid_config.validate().is_ok());
    }

    #[test]
    fn test_vlan_config_validate_invalid_vlan_id() {
        let invalid_config = VlanConfig {
            vlan_id: 5000, // Invalid VLAN ID > 4094
            ip_network: "192.168.100.x".to_string(),
            description: "Test_VLAN".to_string(),
            wan_assignment: 1,
        };

        let result = invalid_config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("outside valid range 10-4094"));
    }

    #[test]
    fn test_vlan_config_validate_invalid_wan_assignment() {
        let invalid_config = VlanConfig {
            vlan_id: 100,
            ip_network: "192.168.100.x".to_string(),
            description: "Test_VLAN".to_string(),
            wan_assignment: 5, // Invalid WAN assignment > 3
        };

        let result = invalid_config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("outside valid range 1-3"));
    }

    #[test]
    fn test_vlan_config_validate_invalid_network_format() {
        let invalid_config = VlanConfig {
            vlan_id: 100,
            ip_network: "invalid.network.format".to_string(), // Invalid format
            description: "Test_VLAN".to_string(),
            wan_assignment: 1,
        };

        let result = invalid_config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("does not match expected format"));
    }

    #[test]
    fn test_vlan_config_validate_empty_description() {
        let invalid_config = VlanConfig {
            vlan_id: 100,
            ip_network: "192.168.100.x".to_string(),
            description: "".to_string(), // Empty description
            wan_assignment: 1,
        };

        let result = invalid_config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("cannot be empty"));
    }

    #[test]
    fn test_vlan_config_validate_cidr_format() {
        let valid_config = VlanConfig {
            vlan_id: 100,
            ip_network: "192.168.100.0/24".to_string(), // CIDR format
            description: "Test_VLAN".to_string(),
            wan_assignment: 1,
        };

        assert!(valid_config.validate().is_ok());
    }

    #[test]
    fn test_vlan_config_validate_invalid_octet_structure() {
        let invalid_config = VlanConfig {
            vlan_id: 100,
            ip_network: "192.168..x".to_string(), // Invalid octet structure
            description: "Test_VLAN".to_string(),
            wan_assignment: 1,
        };

        let result = invalid_config.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("invalid octet structure"));
    }

    // ===== Enhanced DHCP Configuration Tests =====

    #[test]
    fn test_dhcp_lease_time_based_on_department() {
        let it_config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        assert_eq!(it_config.dhcp_lease_time(), 86400); // 24 hours for IT

        let sales_config =
            VlanConfig::new(200, "10.1.3.x".to_string(), "Sales 200".to_string(), 1).unwrap();
        assert_eq!(sales_config.dhcp_lease_time(), 28800); // 8 hours for Sales

        let security_config =
            VlanConfig::new(300, "10.1.4.x".to_string(), "Security 300".to_string(), 1).unwrap();
        assert_eq!(security_config.dhcp_lease_time(), 21600); // 6 hours for Security

        let unknown_config =
            VlanConfig::new(400, "10.1.5.x".to_string(), "Unknown 400".to_string(), 1).unwrap();
        assert_eq!(unknown_config.dhcp_lease_time(), 28800); // 8 hours default
    }

    #[test]
    fn test_dhcp_max_lease_time() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        assert_eq!(config.dhcp_max_lease_time(), config.dhcp_lease_time() * 2);
    }

    #[test]
    fn test_dhcp_domain_name_department_specific() {
        let it_config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        assert_eq!(it_config.dhcp_domain_name(), "it.company.local");

        let sales_config =
            VlanConfig::new(200, "10.1.3.x".to_string(), "Sales 200".to_string(), 1).unwrap();
        assert_eq!(sales_config.dhcp_domain_name(), "sales.company.local");
    }

    #[test]
    fn test_dhcp_dns_servers() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        let dns_servers = config.dhcp_dns_servers().unwrap();

        assert!(dns_servers.len() >= 3);
        assert_eq!(dns_servers[0], "10.1.2.1"); // Gateway as primary
        assert!(dns_servers.contains(&"8.8.8.8".to_string())); // Google DNS
        assert!(dns_servers.contains(&"1.1.1.1".to_string())); // Cloudflare DNS
    }

    #[test]
    fn test_dhcp_ntp_servers() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        let ntp_servers = config.dhcp_ntp_servers();

        assert!(ntp_servers.len() >= 3);
        assert!(ntp_servers.contains(&"pool.ntp.org".to_string()));
        assert!(ntp_servers.contains(&"time.nist.gov".to_string()));
        assert!(ntp_servers.contains(&"time.cloudflare.com".to_string()));
    }

    #[test]
    fn test_static_reservations_department_specific() {
        let it_config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        let reservations = it_config.static_reservations().unwrap();

        assert!(reservations.len() >= 2); // IT should have server and printer
        assert!(reservations.iter().any(|r| r.hostname.contains("server")));
        assert!(reservations.iter().any(|r| r.hostname.contains("printer")));

        let finance_config =
            VlanConfig::new(200, "10.1.3.x".to_string(), "Finance 200".to_string(), 1).unwrap();
        let finance_reservations = finance_config.static_reservations().unwrap();

        assert!(!finance_reservations.is_empty());
        assert!(finance_reservations
            .iter()
            .any(|r| r.hostname.contains("workstation")));
    }

    #[test]
    fn test_dhcp_server_config_complete() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "IT 100".to_string(), 1).unwrap();
        let dhcp_config = config.dhcp_server_config().unwrap();

        assert!(dhcp_config.enabled);
        assert_eq!(dhcp_config.range_start, "10.1.2.100");
        assert_eq!(dhcp_config.range_end, "10.1.2.200");
        assert_eq!(dhcp_config.lease_time, 86400); // IT department
        assert_eq!(dhcp_config.max_lease_time, 172800);
        assert_eq!(dhcp_config.domain_name, "it.company.local");
        assert_eq!(dhcp_config.gateway, "10.1.2.1");
        assert!(dhcp_config.dns_servers.len() >= 3);
        assert!(dhcp_config.ntp_servers.len() >= 3);
        assert!(dhcp_config.static_reservations.len() >= 2);
    }
}
