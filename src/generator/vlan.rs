//! VLAN configuration generation

use crate::model::ConfigError;
use crate::Result;
use indicatif::ProgressBar;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

        // Validate IP network format - must be either "x.x.x.x" or "x.x.x.0/24" format
        let is_x_format = ip_network.ends_with(".x") && ip_network.matches('.').count() == 3;
        let is_cidr_format = ip_network.ends_with(".0/24") && ip_network.matches('.').count() == 3;

        if !is_x_format && !is_cidr_format {
            return Err(ConfigError::validation(format!(
                "IP network '{ip_network}' does not match expected format"
            )));
        }

        // Additional validation: check that we don't have empty octets (e.g., "10.1..x")
        if is_x_format {
            let prefix = ip_network.strip_suffix(".x").unwrap();
            let octets: Vec<&str> = prefix.split('.').collect();
            if octets.len() != 3 || octets.iter().any(|&octet| octet.is_empty()) {
                return Err(ConfigError::validation(format!(
                    "IP network '{ip_network}' does not match expected format"
                )));
            }
        } else if is_cidr_format {
            let prefix = ip_network.strip_suffix(".0/24").unwrap();
            let octets: Vec<&str> = prefix.split('.').collect();
            if octets.len() != 3 || octets.iter().any(|&octet| octet.is_empty()) {
                return Err(ConfigError::validation(format!(
                    "IP network '{ip_network}' does not match expected format"
                )));
            }
        }

        Ok(Self {
            vlan_id,
            ip_network,
            description,
            wan_assignment,
        })
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
}

/// VLAN configuration generator
pub struct VlanGenerator {
    rng: StdRng,
    used_vlan_ids: HashSet<u16>,
    used_networks: HashSet<String>,
}

impl VlanGenerator {
    /// Create a new generator with optional seed
    pub fn new(seed: Option<u64>) -> Self {
        let rng = if let Some(seed) = seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
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

        // Generate description
        let description = self.generate_description(vlan_id);

        // Generate WAN assignment
        let wan_assignment = self.rng.gen_range(1..=3);

        VlanConfig::new(vlan_id, ip_network, description, wan_assignment)
    }

    /// Generate unique VLAN ID
    fn generate_unique_vlan_id(&mut self, max_attempts: usize) -> Result<u16> {
        for _ in 0..max_attempts {
            let vlan_id = self.rng.gen_range(10..=4094);
            if self.used_vlan_ids.insert(vlan_id) {
                return Ok(vlan_id);
            }
        }

        Err(ConfigError::resource_exhausted("VLAN IDs"))
    }

    /// Generate unique RFC 1918 private IP network
    fn generate_unique_ip_network(&mut self, max_attempts: usize) -> Result<String> {
        for _ in 0..max_attempts {
            // Generate Class A private network (10.0.0.0/8)
            let second_octet = self.rng.gen_range(1..=254);
            let third_octet = self.rng.gen_range(1..=254);
            let network = format!("10.{second_octet}.{third_octet}.x");

            if self.used_networks.insert(network.clone()) {
                return Ok(network);
            }
        }

        Err(ConfigError::resource_exhausted("IP networks"))
    }

    /// Generate department-based description
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

        let department = DEPARTMENTS[self.rng.gen_range(0..DEPARTMENTS.len())];
        format!("{department} VLAN {vlan_id}")
    }
}

/// Generate multiple VLAN configurations
pub fn generate_vlan_configurations(
    count: u16,
    seed: Option<u64>,
    progress_bar: Option<&ProgressBar>,
) -> Result<Vec<VlanConfig>> {
    let mut generator = VlanGenerator::new(seed);
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
                "Network format '{}' should be invalid",
                invalid_network
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
                format!("Test VLAN {}", vlan_id),
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
}
