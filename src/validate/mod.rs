//! Validation framework for configuration consistency

use crate::Result;
use crate::generator::VlanConfig;
use crate::model::ConfigError;
use std::collections::HashSet;

/// Validation engine for cross-component consistency
pub struct ValidationEngine {
    unique_vlan_ids: HashSet<u16>,
    unique_networks: HashSet<String>,
}

impl ValidationEngine {
    /// Create a new validation engine
    pub fn new() -> Self {
        Self {
            unique_vlan_ids: HashSet::new(),
            unique_networks: HashSet::new(),
        }
    }

    /// Validate a single VLAN configuration
    pub fn validate_config(&mut self, config: &VlanConfig) -> Result<()> {
        // Check VLAN ID uniqueness
        if !self.unique_vlan_ids.insert(config.vlan_id) {
            return Err(ConfigError::validation(format!(
                "Duplicate VLAN ID: {}",
                config.vlan_id
            )));
        }

        // Check network uniqueness
        if !self.unique_networks.insert(config.ip_network.clone()) {
            return Err(ConfigError::validation(format!(
                "Duplicate IP network: {}",
                config.ip_network
            )));
        }

        // Validate VLAN ID range
        if !(10..=4094).contains(&config.vlan_id) {
            return Err(ConfigError::validation(format!(
                "VLAN ID {} is outside valid range 10-4094",
                config.vlan_id
            )));
        }

        // Validate WAN assignment
        if !(1..=3).contains(&config.wan_assignment) {
            return Err(ConfigError::validation(format!(
                "WAN assignment {} is outside valid range 1-3",
                config.wan_assignment
            )));
        }

        // Validate IP network format
        self.validate_ip_network(&config.ip_network)?;

        Ok(())
    }

    /// Validate multiple configurations
    pub fn validate_configs(&mut self, configs: &[VlanConfig]) -> Result<()> {
        for config in configs {
            self.validate_config(config)?;
        }
        Ok(())
    }

    /// Validate IP network format and RFC 1918 compliance
    fn validate_ip_network(&self, network: &str) -> Result<()> {
        // Check for expected format patterns
        if network.ends_with(".x") {
            let prefix = network.strip_suffix(".x").unwrap();
            self.validate_network_prefix(prefix)?;
        } else if network.ends_with(".0/24") {
            let prefix = network.strip_suffix(".0/24").unwrap();
            self.validate_network_prefix(prefix)?;
        } else {
            return Err(ConfigError::validation(format!(
                "IP network '{network}' does not match expected format (should end with .x or .0/24)"
            )));
        }

        Ok(())
    }

    /// Validate network prefix for RFC 1918 compliance
    fn validate_network_prefix(&self, prefix: &str) -> Result<()> {
        let parts: Vec<&str> = prefix.split('.').collect();
        if parts.len() != 3 {
            return Err(ConfigError::validation(format!(
                "Invalid network prefix format: {prefix}"
            )));
        }

        // Parse octets
        let first: u8 = parts[0]
            .parse()
            .map_err(|_| ConfigError::validation(format!("Invalid first octet: {}", parts[0])))?;
        let second: u8 = parts[1]
            .parse()
            .map_err(|_| ConfigError::validation(format!("Invalid second octet: {}", parts[1])))?;
        let third: u8 = parts[2]
            .parse()
            .map_err(|_| ConfigError::validation(format!("Invalid third octet: {}", parts[2])))?;

        // Check RFC 1918 compliance
        match first {
            10 => {
                // 10.0.0.0/8 - all values valid
                if second == 0 && third == 0 {
                    return Err(ConfigError::validation(
                        "Network 10.0.0.x is reserved".to_string(),
                    ));
                }
            }
            172 => {
                // 172.16.0.0/12 - 172.16.x.x to 172.31.x.x
                if !(16..=31).contains(&second) {
                    return Err(ConfigError::validation(format!(
                        "172.{second}.x.x is not in RFC 1918 range (should be 172.16-31.x.x)"
                    )));
                }
            }
            192 => {
                // 192.168.0.0/16 - 192.168.x.x only
                if second != 168 {
                    return Err(ConfigError::validation(format!(
                        "192.{second}.x.x is not in RFC 1918 range (should be 192.168.x.x)"
                    )));
                }
            }
            _ => {
                return Err(ConfigError::validation(format!(
                    "{first}.x.x.x is not an RFC 1918 private network"
                )));
            }
        }

        Ok(())
    }

    /// Reset validation state
    pub fn reset(&mut self) {
        self.unique_vlan_ids.clear();
        self.unique_networks.clear();
    }

    /// Get count of validated configurations
    pub fn config_count(&self) -> usize {
        self.unique_vlan_ids.len()
    }
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_engine() {
        let mut engine = ValidationEngine::new();

        let config1 =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test 1".to_string(), 1).unwrap();
        let config2 =
            VlanConfig::new(200, "10.3.4.x".to_string(), "Test 2".to_string(), 2).unwrap();

        assert!(engine.validate_config(&config1).is_ok());
        assert!(engine.validate_config(&config2).is_ok());
        assert_eq!(engine.config_count(), 2);
    }

    #[test]
    fn test_duplicate_vlan_id() {
        let mut engine = ValidationEngine::new();

        let config1 =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test 1".to_string(), 1).unwrap();
        let config2 =
            VlanConfig::new(100, "10.3.4.x".to_string(), "Test 2".to_string(), 2).unwrap();

        assert!(engine.validate_config(&config1).is_ok());
        assert!(engine.validate_config(&config2).is_err());
    }

    #[test]
    fn test_duplicate_network() {
        let mut engine = ValidationEngine::new();

        let config1 =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test 1".to_string(), 1).unwrap();
        let config2 =
            VlanConfig::new(200, "10.1.2.x".to_string(), "Test 2".to_string(), 2).unwrap();

        assert!(engine.validate_config(&config1).is_ok());
        assert!(engine.validate_config(&config2).is_err());
    }

    #[test]
    fn test_rfc1918_validation() {
        let engine = ValidationEngine::new();

        // Valid RFC 1918 networks
        assert!(engine.validate_network_prefix("10.1.2").is_ok());
        assert!(engine.validate_network_prefix("172.16.1").is_ok());
        assert!(engine.validate_network_prefix("172.31.255").is_ok());
        assert!(engine.validate_network_prefix("192.168.1").is_ok());

        // Invalid networks
        assert!(engine.validate_network_prefix("1.1.1").is_err());
        assert!(engine.validate_network_prefix("172.15.1").is_err());
        assert!(engine.validate_network_prefix("172.32.1").is_err());
        assert!(engine.validate_network_prefix("192.167.1").is_err());
        assert!(engine.validate_network_prefix("10.0.0").is_err()); // Reserved
    }
}
