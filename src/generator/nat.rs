//! NAT mapping generator for OPNsense firewall configurations
//!
//! This module provides functionality to generate realistic NAT (Network Address Translation)
//! mappings including port forwarding, source NAT, and destination NAT rules.

use crate::model::ConfigError;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Result type for NAT generation operations
pub type NatResult<T> = Result<T, ConfigError>;

/// NAT rule types supported by OPNsense
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NatRuleType {
    /// Port forwarding from WAN to internal server
    PortForward,
    /// Source NAT (SNAT) for outbound traffic
    SourceNat,
    /// Destination NAT (DNAT) for inbound traffic
    DestinationNat,
    /// 1:1 NAT mapping
    OneToOneNat,
    /// Outbound NAT rule
    OutboundNat,
}

/// NAT mapping configuration with realistic settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatMapping {
    /// Unique identifier for the NAT rule
    pub id: String,
    /// Type of NAT rule
    pub rule_type: NatRuleType,
    /// Name/description of the NAT rule
    pub name: String,
    /// Source IP address or network
    pub source: String,
    /// Source port or port range
    pub source_port: String,
    /// Destination IP address or network
    pub destination: String,
    /// Destination port or port range
    pub destination_port: String,
    /// Protocol (TCP, UDP, or Both)
    pub protocol: String,
    /// Interface where the rule applies
    pub interface: String,
    /// Translation/target IP address
    pub target_ip: String,
    /// Translation/target port
    pub target_port: String,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Log packets matching this rule
    pub log: bool,
    /// Associated VLAN ID (if applicable)
    pub vlan_id: Option<u16>,
}

impl NatMapping {
    /// Create a new NAT mapping with validation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rule_type: NatRuleType,
        name: String,
        source: String,
        source_port: String,
        destination: String,
        destination_port: String,
        protocol: String,
        interface: String,
        target_ip: String,
        target_port: String,
        enabled: bool,
        log: bool,
        vlan_id: Option<u16>,
    ) -> NatResult<Self> {
        let mapping = Self {
            id: Uuid::new_v4().to_string(),
            rule_type,
            name,
            source,
            source_port,
            destination,
            destination_port,
            protocol,
            interface,
            target_ip,
            target_port,
            enabled,
            log,
            vlan_id,
        };

        mapping.validate()?;
        Ok(mapping)
    }

    /// Validate the NAT mapping
    pub fn validate(&self) -> NatResult<()> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ConfigError::validation(
                "NAT mapping name cannot be empty".to_string(),
            ));
        }

        // Validate protocol
        if !matches!(self.protocol.as_str(), "TCP" | "UDP" | "Both" | "ICMP") {
            return Err(ConfigError::validation(format!(
                "NAT protocol '{}' is invalid. Must be TCP, UDP, Both, or ICMP",
                self.protocol
            )));
        }

        // Validate port ranges (basic check)
        if !self.source_port.is_empty() && !self.is_valid_port_range(&self.source_port) {
            return Err(ConfigError::validation(format!(
                "Source port '{}' is invalid",
                self.source_port
            )));
        }

        if !self.destination_port.is_empty() && !self.is_valid_port_range(&self.destination_port) {
            return Err(ConfigError::validation(format!(
                "Destination port '{}' is invalid",
                self.destination_port
            )));
        }

        if !self.target_port.is_empty() && !self.is_valid_port_range(&self.target_port) {
            return Err(ConfigError::validation(format!(
                "Target port '{}' is invalid",
                self.target_port
            )));
        }

        // Validate VLAN ID if provided
        if let Some(vlan_id) = self.vlan_id
            && !(10..=4094).contains(&vlan_id)
        {
            return Err(ConfigError::validation(format!(
                "VLAN ID {} is invalid. Must be between 10 and 4094",
                vlan_id
            )));
        }

        Ok(())
    }

    /// Check if a port range is valid (basic validation)
    fn is_valid_port_range(&self, port_range: &str) -> bool {
        if port_range == "any" || port_range.is_empty() {
            return true;
        }

        // Handle single port
        if let Ok(port) = port_range.parse::<u16>() {
            return port > 0;
        }

        // Handle port range (e.g., "80-90")
        if port_range.contains('-') {
            let parts: Vec<&str> = port_range.split('-').collect();
            if parts.len() == 2
                && let (Ok(start), Ok(end)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>())
            {
                return start > 0 && end > 0 && start <= end;
            }
        }

        // Handle comma-separated ports (e.g., "80,443,8080")
        if port_range.contains(',') {
            return port_range
                .split(',')
                .all(|p| p.trim().parse::<u16>().is_ok_and(|port| port > 0));
        }

        false
    }
}

/// NAT mapping generator with realistic configurations
pub struct NatGenerator {
    rng: Box<dyn RngCore>,
    used_names: HashSet<String>,
    used_external_ports: HashSet<u16>,
}

impl NatGenerator {
    /// Create a new NAT generator with a random seed
    pub fn new() -> Self {
        Self::new_with_seed(None)
    }

    /// Create a new NAT generator with a specific seed for reproducibility
    pub fn new_with_seed(seed: Option<u64>) -> Self {
        let rng: Box<dyn RngCore> = if let Some(seed) = seed {
            Box::new(StdRng::seed_from_u64(seed))
        } else {
            Box::new(StdRng::from_rng(&mut rand::rng()))
        };

        Self {
            rng,
            used_names: HashSet::new(),
            used_external_ports: HashSet::new(),
        }
    }

    /// Generate a single NAT mapping
    pub fn generate_single(&mut self, rule_type: Option<NatRuleType>) -> NatResult<NatMapping> {
        let rule_type = rule_type.unwrap_or_else(|| self.random_nat_type());
        let name = self.generate_unique_name(&rule_type);
        let protocol = self.random_protocol();
        let (source, source_port) = self.generate_source(&rule_type);
        let (destination, destination_port) = self.generate_destination(&rule_type, &protocol)?;
        let interface = self.random_interface(&rule_type);
        let (target_ip, target_port) =
            self.generate_target(&rule_type, &destination_port, &protocol);
        let enabled = self.rng.random_bool(0.9); // 90% chance of being enabled
        let log = self.rng.random_bool(0.3); // 30% chance of logging
        let vlan_id = if self.rng.random_bool(0.6) {
            Some(self.rng.random_range(10..=4094))
        } else {
            None
        };

        NatMapping::new(
            rule_type,
            name,
            source,
            source_port,
            destination,
            destination_port,
            protocol,
            interface,
            target_ip,
            target_port,
            enabled,
            log,
            vlan_id,
        )
    }

    /// Generate multiple NAT mappings
    pub fn generate_batch(&mut self, count: u16) -> NatResult<Vec<NatMapping>> {
        let mut mappings = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let mapping = self.generate_single(None)?;
            mappings.push(mapping);
        }

        Ok(mappings)
    }

    /// Generate a random NAT rule type
    fn random_nat_type(&mut self) -> NatRuleType {
        match self.rng.random_range(0..5) {
            0 => NatRuleType::PortForward,
            1 => NatRuleType::SourceNat,
            2 => NatRuleType::DestinationNat,
            3 => NatRuleType::OneToOneNat,
            _ => NatRuleType::OutboundNat,
        }
    }

    /// Generate a unique NAT rule name
    fn generate_unique_name(&mut self, rule_type: &NatRuleType) -> String {
        const MAX_ATTEMPTS: usize = 100;

        for _ in 0..MAX_ATTEMPTS {
            let base_name = match rule_type {
                NatRuleType::PortForward => {
                    let services = [
                        "Web-Server",
                        "SSH",
                        "FTP",
                        "Mail-Server",
                        "Database",
                        "API",
                        "RDP",
                    ];
                    let service = services[self.rng.random_range(0..services.len())];
                    format!("Port-Forward-{}", service)
                }
                NatRuleType::SourceNat => {
                    let sources = ["LAN", "DMZ", "Guest", "VPN", "VLAN"];
                    let source = sources[self.rng.random_range(0..sources.len())];
                    format!("SNAT-{}", source)
                }
                NatRuleType::DestinationNat => {
                    let destinations = ["WebServer", "MailServer", "FTPServer", "VoIPServer"];
                    let dest = destinations[self.rng.random_range(0..destinations.len())];
                    format!("DNAT-{}", dest)
                }
                NatRuleType::OneToOneNat => {
                    format!("1to1-NAT-{}", self.rng.random_range(1..=99))
                }
                NatRuleType::OutboundNat => {
                    let vlans = ["VLAN", "LAN", "Guest", "DMZ"];
                    let vlan = vlans[self.rng.random_range(0..vlans.len())];
                    format!("Outbound-{}", vlan)
                }
            };

            let name = if self.rng.random_bool(0.3) {
                format!("{}-{:02}", base_name, self.rng.random_range(1..=99))
            } else {
                base_name
            };

            if self.used_names.insert(name.clone()) {
                return name;
            }
        }

        // Fallback with UUID suffix if we can't generate unique name
        format!(
            "{}-{}",
            match rule_type {
                NatRuleType::PortForward => "Port-Forward",
                NatRuleType::SourceNat => "SNAT",
                NatRuleType::DestinationNat => "DNAT",
                NatRuleType::OneToOneNat => "1to1-NAT",
                NatRuleType::OutboundNat => "Outbound",
            },
            Uuid::new_v4().to_string().split('-').next().unwrap()
        )
    }

    /// Generate random protocol
    fn random_protocol(&mut self) -> String {
        match self.rng.random_range(0..4) {
            0 => "TCP",
            1 => "UDP",
            2 => "Both",
            _ => "ICMP",
        }
        .to_string()
    }

    /// Generate source address and port based on rule type
    fn generate_source(&mut self, rule_type: &NatRuleType) -> (String, String) {
        match rule_type {
            NatRuleType::PortForward => ("any".to_string(), "any".to_string()),
            NatRuleType::SourceNat => {
                let networks = ["192.168.1.0/24", "10.0.0.0/16", "172.16.0.0/16"];
                let network = networks[self.rng.random_range(0..networks.len())];
                (network.to_string(), "any".to_string())
            }
            NatRuleType::DestinationNat => ("any".to_string(), "any".to_string()),
            NatRuleType::OneToOneNat => {
                let ip = format!(
                    "192.168.{}.{}",
                    self.rng.random_range(1..=254),
                    self.rng.random_range(1..=254)
                );
                (ip, "any".to_string())
            }
            NatRuleType::OutboundNat => {
                let networks = ["192.168.1.0/24", "10.0.0.0/8", "172.16.0.0/12"];
                let network = networks[self.rng.random_range(0..networks.len())];
                (network.to_string(), "any".to_string())
            }
        }
    }

    /// Generate destination address and port based on rule type
    fn generate_destination(
        &mut self,
        rule_type: &NatRuleType,
        protocol: &str,
    ) -> NatResult<(String, String)> {
        Ok(match rule_type {
            NatRuleType::PortForward => {
                let port = self.generate_unique_external_port()?;
                ("any".to_string(), port.to_string())
            }
            NatRuleType::SourceNat => ("any".to_string(), "any".to_string()),
            NatRuleType::DestinationNat => {
                let port = if protocol == "ICMP" {
                    "any".to_string()
                } else {
                    self.generate_service_port()
                };
                ("any".to_string(), port)
            }
            NatRuleType::OneToOneNat => ("any".to_string(), "any".to_string()),
            NatRuleType::OutboundNat => ("any".to_string(), "any".to_string()),
        })
    }

    /// Generate interface based on rule type
    fn random_interface(&mut self, rule_type: &NatRuleType) -> String {
        match rule_type {
            NatRuleType::PortForward => "WAN".to_string(),
            NatRuleType::SourceNat => {
                let interfaces = ["LAN", "OPT1", "OPT2", "DMZ"];
                interfaces[self.rng.random_range(0..interfaces.len())].to_string()
            }
            NatRuleType::DestinationNat => "WAN".to_string(),
            NatRuleType::OneToOneNat => "WAN".to_string(),
            NatRuleType::OutboundNat => {
                let interfaces = ["LAN", "OPT1", "OPT2", "DMZ"];
                interfaces[self.rng.random_range(0..interfaces.len())].to_string()
            }
        }
    }

    /// Generate target IP and port based on rule type
    fn generate_target(
        &mut self,
        rule_type: &NatRuleType,
        dest_port: &str,
        protocol: &str,
    ) -> (String, String) {
        match rule_type {
            NatRuleType::PortForward => {
                let internal_ip = format!("192.168.1.{}", self.rng.random_range(10..=254));
                let internal_port = if protocol == "ICMP" {
                    "any".to_string()
                } else if dest_port != "any" {
                    dest_port.to_string() // Usually same as destination port
                } else {
                    self.generate_service_port()
                };
                (internal_ip, internal_port)
            }
            NatRuleType::SourceNat => {
                // For SNAT, target is usually the WAN IP
                ("WAN address".to_string(), "any".to_string())
            }
            NatRuleType::DestinationNat => {
                let internal_ip = format!("192.168.1.{}", self.rng.random_range(10..=254));
                (internal_ip, dest_port.to_string())
            }
            NatRuleType::OneToOneNat => {
                let public_ip = format!(
                    "{}.{}.{}.{}",
                    self.rng.random_range(1..=223),
                    self.rng.random_range(0..=255),
                    self.rng.random_range(0..=255),
                    self.rng.random_range(1..=254)
                );
                (public_ip, "any".to_string())
            }
            NatRuleType::OutboundNat => ("WAN address".to_string(), "any".to_string()),
        }
    }

    /// Generate a unique external port for port forwarding
    fn generate_unique_external_port(&mut self) -> NatResult<u16> {
        const COMMON_PORTS: &[u16] = &[80, 443, 22, 21, 25, 53, 110, 143, 993, 995, 3389, 5900];
        const MAX_ATTEMPTS: usize = 100;

        // Try common ports first
        for &port in COMMON_PORTS {
            if self.used_external_ports.insert(port) {
                return Ok(port);
            }
        }

        // Try random ports
        for _ in 0..MAX_ATTEMPTS {
            let port = self.rng.random_range(1024..=65535);
            if self.used_external_ports.insert(port) {
                return Ok(port);
            }
        }

        // Linear scan as final fallback
        for port in 1024..=65535 {
            if self.used_external_ports.insert(port) {
                return Ok(port);
            }
        }

        Err(ConfigError::validation(
            "Unable to generate unique external port: all ports exhausted".to_string(),
        ))
    }

    /// Generate a service port
    fn generate_service_port(&mut self) -> String {
        let common_services = [
            ("80", "HTTP"),
            ("443", "HTTPS"),
            ("22", "SSH"),
            ("21", "FTP"),
            ("25", "SMTP"),
            ("53", "DNS"),
            ("3389", "RDP"),
            ("5432", "PostgreSQL"),
            ("3306", "MySQL"),
            ("1433", "SQL Server"),
            ("8080", "HTTP Alt"),
            ("8443", "HTTPS Alt"),
        ];

        if self.rng.random_bool(0.8) {
            // Use common service port
            let (port, _service) = common_services[self.rng.random_range(0..common_services.len())];
            port.to_string()
        } else {
            // Use random port
            self.rng.random_range(1024..=65535).to_string()
        }
    }
}

impl Default for NatGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate multiple NAT mappings with progress tracking
pub fn generate_nat_mappings(
    count: u16,
    seed: Option<u64>,
    progress_bar: Option<&indicatif::ProgressBar>,
) -> NatResult<Vec<NatMapping>> {
    let mut generator = NatGenerator::new_with_seed(seed);
    let mut mappings = Vec::with_capacity(count as usize);

    for i in 0..count {
        let mapping = generator.generate_single(None)?;
        mappings.push(mapping);

        if let Some(pb) = progress_bar {
            pb.set_position(i as u64 + 1);
        }
    }

    Ok(mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_mapping_creation() {
        let mapping = NatMapping::new(
            NatRuleType::PortForward,
            "Web-Server-Forward".to_string(),
            "any".to_string(),
            "any".to_string(),
            "any".to_string(),
            "80".to_string(),
            "TCP".to_string(),
            "WAN".to_string(),
            "192.168.1.100".to_string(),
            "80".to_string(),
            true,
            false,
            Some(100),
        );

        assert!(mapping.is_ok());
        let mapping = mapping.unwrap();
        assert_eq!(mapping.rule_type, NatRuleType::PortForward);
        assert_eq!(mapping.name, "Web-Server-Forward");
        assert_eq!(mapping.destination_port, "80");
    }

    #[test]
    fn test_nat_mapping_validation_invalid_protocol() {
        let mapping = NatMapping::new(
            NatRuleType::PortForward,
            "Test-Forward".to_string(),
            "any".to_string(),
            "any".to_string(),
            "any".to_string(),
            "80".to_string(),
            "INVALID".to_string(), // Invalid protocol
            "WAN".to_string(),
            "192.168.1.100".to_string(),
            "80".to_string(),
            true,
            false,
            None,
        );

        assert!(mapping.is_err());
        assert!(
            mapping
                .unwrap_err()
                .to_string()
                .contains("protocol 'INVALID' is invalid")
        );
    }

    #[test]
    fn test_nat_mapping_validation_invalid_vlan() {
        let mapping = NatMapping::new(
            NatRuleType::PortForward,
            "Test-Forward".to_string(),
            "any".to_string(),
            "any".to_string(),
            "any".to_string(),
            "80".to_string(),
            "TCP".to_string(),
            "WAN".to_string(),
            "192.168.1.100".to_string(),
            "80".to_string(),
            true,
            false,
            Some(5000), // Invalid VLAN ID
        );

        assert!(mapping.is_err());
        assert!(
            mapping
                .unwrap_err()
                .to_string()
                .contains("VLAN ID 5000 is invalid")
        );
    }

    #[test]
    fn test_nat_generator_single() {
        let mut generator = NatGenerator::new_with_seed(Some(42));
        let mapping = generator.generate_single(Some(NatRuleType::PortForward));

        assert!(mapping.is_ok());
        let mapping = mapping.unwrap();
        assert_eq!(mapping.rule_type, NatRuleType::PortForward);
        assert!(!mapping.name.is_empty());
        assert_eq!(mapping.interface, "WAN");
    }

    #[test]
    fn test_nat_generator_batch() {
        let mut generator = NatGenerator::new_with_seed(Some(42));
        let mappings = generator.generate_batch(5);

        assert!(mappings.is_ok());
        let mappings = mappings.unwrap();
        assert_eq!(mappings.len(), 5);

        // Check uniqueness of names
        let mut names = std::collections::HashSet::new();
        for mapping in &mappings {
            assert!(
                names.insert(&mapping.name),
                "Duplicate name: {}",
                mapping.name
            );
        }
    }

    #[test]
    fn test_port_validation() {
        let mapping = NatMapping {
            id: "test".to_string(),
            rule_type: NatRuleType::PortForward,
            name: "Test".to_string(),
            source: "any".to_string(),
            source_port: "80-90".to_string(), // Valid range
            destination: "any".to_string(),
            destination_port: "443".to_string(), // Valid single port
            protocol: "TCP".to_string(),
            interface: "WAN".to_string(),
            target_ip: "192.168.1.1".to_string(),
            target_port: "80,443,8080".to_string(), // Valid comma-separated
            enabled: true,
            log: false,
            vlan_id: None,
        };

        assert!(mapping.validate().is_ok());

        // Test invalid port
        let mut invalid_mapping = mapping.clone();
        invalid_mapping.source_port = "99999".to_string(); // Invalid port > 65535
        assert!(invalid_mapping.validate().is_err());
    }
}
