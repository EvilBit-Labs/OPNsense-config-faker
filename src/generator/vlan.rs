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
    pub fn new(vlan_id: u16, ip_network: String, description: String, wan_assignment: u8) -> Result<Self> {
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
        
        // Validate IP network format (basic check)
        if !ip_network.contains(".x") && !ip_network.contains(".0/24") {
            return Err(ConfigError::validation(format!(
                "IP network '{ip_network}' does not match expected format"
            )));
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
                "Cannot derive gateway from IP network: {}", self.ip_network
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
                "Cannot derive DHCP range from IP network: {}", self.ip_network
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
                "Cannot derive DHCP range from IP network: {}", self.ip_network
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
            "Sales", "IT", "HR", "Finance", "Marketing", "Operations",
            "Engineering", "Support", "Legal", "Procurement", "Security",
            "Development", "QA", "Research", "Training", "Management"
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

    #[test]
    fn test_vlan_config_creation() {
        let config = VlanConfig::new(
            100,
            "10.1.2.x".to_string(),
            "Test VLAN 100".to_string(),
            1,
        ).unwrap();
        
        assert_eq!(config.vlan_id, 100);
        assert_eq!(config.ip_network, "10.1.2.x");
        assert_eq!(config.description, "Test VLAN 100");
        assert_eq!(config.wan_assignment, 1);
    }

    #[test]
    fn test_vlan_config_validation() {
        // Invalid VLAN ID
        assert!(VlanConfig::new(9, "10.1.2.x".to_string(), "Test".to_string(), 1).is_err());
        assert!(VlanConfig::new(4095, "10.1.2.x".to_string(), "Test".to_string(), 1).is_err());
        
        // Invalid WAN assignment
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 0).is_err());
        assert!(VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 4).is_err());
    }

    #[test]
    fn test_gateway_ip_derivation() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.gateway_ip().unwrap(), "10.1.2.1");
    }

    #[test]
    fn test_dhcp_range_derivation() {
        let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();
        assert_eq!(config.dhcp_range_start().unwrap(), "10.1.2.100");
        assert_eq!(config.dhcp_range_end().unwrap(), "10.1.2.200");
    }

    #[test]
    fn test_vlan_generator() {
        let mut generator = VlanGenerator::new(Some(42)); // Fixed seed for reproducibility
        let config = generator.generate_single().unwrap();
        
        assert!((10..=4094).contains(&config.vlan_id));
        assert!((1..=3).contains(&config.wan_assignment));
        assert!(config.ip_network.starts_with("10."));
        assert!(config.ip_network.ends_with(".x"));
    }

    #[test]
    fn test_generate_multiple_configs() {
        let configs = generate_vlan_configurations(5, Some(42), None).unwrap();
        assert_eq!(configs.len(), 5);
        
        // Check uniqueness
        let mut vlan_ids = HashSet::new();
        let mut networks = HashSet::new();
        
        for config in &configs {
            assert!(vlan_ids.insert(config.vlan_id), "Duplicate VLAN ID: {}", config.vlan_id);
            assert!(networks.insert(&config.ip_network), "Duplicate network: {}", config.ip_network);
        }
    }
}