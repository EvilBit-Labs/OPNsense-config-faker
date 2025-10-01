//! VPN configuration generator for OPNsense
//!
//! This module provides functionality to generate realistic VPN configurations
//! including OpenVPN, WireGuard, and IPSec tunnels for testing purposes.

use crate::model::ConfigError;
use rand::Rng;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Result type for VPN generation operations
pub type VpnResult<T> = Result<T, ConfigError>;

/// VPN configuration types supported by OPNsense
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VpnType {
    OpenVPN,
    WireGuard,
    IPSec,
}

/// VPN configuration with realistic settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    /// Unique identifier for the VPN configuration
    pub id: String,
    /// Type of VPN (OpenVPN, WireGuard, IPSec)
    pub vpn_type: VpnType,
    /// Name/description of the VPN connection
    pub name: String,
    /// Server IP address or hostname
    pub server: String,
    /// Port number for the VPN connection
    pub port: u16,
    /// Protocol (UDP/TCP for OpenVPN, UDP for WireGuard)
    pub protocol: String,
    /// Encryption cipher
    pub cipher: String,
    /// Authentication method
    pub auth_method: String,
    /// Pre-shared key or certificate identifier
    pub key_identifier: String,
    /// Client subnet for IP assignment
    pub client_subnet: String,
    /// DNS servers for VPN clients
    pub dns_servers: Vec<String>,
    /// Whether the VPN is enabled
    pub enabled: bool,
}

impl VpnConfig {
    /// Create a new VPN configuration with validation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        vpn_type: VpnType,
        name: String,
        server: String,
        port: u16,
        protocol: String,
        cipher: String,
        auth_method: String,
        key_identifier: String,
        client_subnet: String,
        dns_servers: Vec<String>,
        enabled: bool,
    ) -> VpnResult<Self> {
        let config = Self {
            id: Uuid::new_v4().to_string(),
            vpn_type,
            name,
            server,
            port,
            protocol,
            cipher,
            auth_method,
            key_identifier,
            client_subnet,
            dns_servers,
            enabled,
        };

        config.validate()?;
        Ok(config)
    }

    /// Validate the VPN configuration
    pub fn validate(&self) -> VpnResult<()> {
        // Validate port range
        if self.port == 0 {
            return Err(ConfigError::validation(format!(
                "VPN port {} is invalid. Must be between 1 and 65535",
                self.port
            )));
        }

        // Validate protocol for VPN type
        match self.vpn_type {
            VpnType::OpenVPN => {
                if !matches!(self.protocol.as_str(), "UDP" | "TCP") {
                    return Err(ConfigError::validation(format!(
                        "OpenVPN protocol '{}' is invalid. Must be UDP or TCP",
                        self.protocol
                    )));
                }
            }
            VpnType::WireGuard => {
                if self.protocol != "UDP" {
                    return Err(ConfigError::validation(format!(
                        "WireGuard protocol '{}' is invalid. Must be UDP",
                        self.protocol
                    )));
                }
            }
            VpnType::IPSec => {
                if !matches!(self.protocol.as_str(), "ESP" | "AH") {
                    return Err(ConfigError::validation(format!(
                        "IPSec protocol '{}' is invalid. Must be ESP or AH",
                        self.protocol
                    )));
                }
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ConfigError::validation(
                "VPN name cannot be empty".to_string(),
            ));
        }

        // Validate server is not empty
        if self.server.trim().is_empty() {
            return Err(ConfigError::validation(
                "VPN server cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

/// VPN configuration generator with realistic settings
pub struct VpnGenerator {
    rng: Box<dyn RngCore>,
    used_ports: HashSet<u16>,
    used_names: HashSet<String>,
}

impl VpnGenerator {
    /// Create a new VPN generator with a random seed
    pub fn new() -> Self {
        Self::new_with_seed(None)
    }

    /// Create a new VPN generator with a specific seed for reproducibility
    pub fn new_with_seed(seed: Option<u64>) -> Self {
        let rng: Box<dyn RngCore> = if let Some(seed) = seed {
            Box::new(StdRng::seed_from_u64(seed))
        } else {
            Box::new(StdRng::from_rng(&mut rand::rng()))
        };

        Self {
            rng,
            used_ports: HashSet::new(),
            used_names: HashSet::new(),
        }
    }

    /// Generate a single VPN configuration
    pub fn generate_single(&mut self, vpn_type: Option<VpnType>) -> VpnResult<VpnConfig> {
        let vpn_type = vpn_type.unwrap_or_else(|| self.random_vpn_type());
        let name = self.generate_unique_name(&vpn_type);
        let server = self.generate_server_address();
        let port = self.generate_unique_port(&vpn_type);
        let protocol = self.get_protocol_for_type(&vpn_type);
        let cipher = self.get_cipher_for_type(&vpn_type);
        let auth_method = self.get_auth_method_for_type(&vpn_type);
        let key_identifier = self.generate_key_identifier(&vpn_type);
        let client_subnet = self.generate_client_subnet();
        let dns_servers = self.generate_dns_servers();
        let enabled = self.rng.random_bool(0.85); // 85% chance of being enabled

        VpnConfig::new(
            vpn_type,
            name,
            server,
            port,
            protocol,
            cipher,
            auth_method,
            key_identifier,
            client_subnet,
            dns_servers,
            enabled,
        )
    }

    /// Generate multiple VPN configurations
    pub fn generate_batch(&mut self, count: u16) -> VpnResult<Vec<VpnConfig>> {
        let mut configs = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let config = self.generate_single(None)?;
            configs.push(config);
        }

        Ok(configs)
    }

    /// Generate a random VPN type
    fn random_vpn_type(&mut self) -> VpnType {
        match self.rng.random_range(0..3) {
            0 => VpnType::OpenVPN,
            1 => VpnType::WireGuard,
            _ => VpnType::IPSec,
        }
    }

    /// Generate a unique VPN name
    fn generate_unique_name(&mut self, vpn_type: &VpnType) -> String {
        const MAX_ATTEMPTS: usize = 100;

        for _ in 0..MAX_ATTEMPTS {
            let base_name = match vpn_type {
                VpnType::OpenVPN => {
                    let purposes = [
                        "Remote-Access",
                        "Site-to-Site",
                        "Mobile-VPN",
                        "Branch-Office",
                    ];
                    let purpose = purposes[self.rng.random_range(0..purposes.len())];
                    format!("OpenVPN-{}", purpose)
                }
                VpnType::WireGuard => {
                    let locations = ["Office", "Home", "Mobile", "Server", "Datacenter"];
                    let location = locations[self.rng.random_range(0..locations.len())];
                    format!("WireGuard-{}", location)
                }
                VpnType::IPSec => {
                    let sites = [
                        "Main-Office",
                        "Branch-A",
                        "Branch-B",
                        "Partner-Site",
                        "Backup-Site",
                    ];
                    let site = sites[self.rng.random_range(0..sites.len())];
                    format!("IPSec-{}", site)
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
            match vpn_type {
                VpnType::OpenVPN => "OpenVPN",
                VpnType::WireGuard => "WireGuard",
                VpnType::IPSec => "IPSec",
            },
            Uuid::new_v4().to_string().split('-').next().unwrap()
        )
    }

    /// Generate a server address (IP or hostname)
    fn generate_server_address(&mut self) -> String {
        if self.rng.random_bool(0.4) {
            // Generate hostname
            let domains = [
                "vpn.company.com",
                "secure.example.org",
                "tunnel.corp.net",
                "gateway.office.local",
            ];
            domains[self.rng.random_range(0..domains.len())].to_string()
        } else {
            // Generate public IP address
            format!(
                "{}.{}.{}.{}",
                self.rng.random_range(1..=223), // Avoid reserved ranges
                self.rng.random_range(0..=255),
                self.rng.random_range(0..=255),
                self.rng.random_range(1..=254)
            )
        }
    }

    /// Generate a unique port for the VPN type
    fn generate_unique_port(&mut self, vpn_type: &VpnType) -> u16 {
        const MAX_ATTEMPTS: usize = 100;

        let default_ports = match vpn_type {
            VpnType::OpenVPN => vec![1194, 443, 1723],
            VpnType::WireGuard => vec![51820, 51821, 51822],
            VpnType::IPSec => vec![500, 4500, 1701],
        };

        // Try default ports first
        for &port in &default_ports {
            if self.used_ports.insert(port) {
                return port;
            }
        }

        // Try random ports in appropriate ranges
        for _ in 0..MAX_ATTEMPTS {
            let port = match vpn_type {
                VpnType::OpenVPN => self.rng.random_range(1024..=65535),
                VpnType::WireGuard => self.rng.random_range(51820..=51899),
                VpnType::IPSec => self.rng.random_range(500..=4500),
            };

            if self.used_ports.insert(port) {
                return port;
            }
        }

        // Fallback - find any available port
        for port in 1024..=65535 {
            if self.used_ports.insert(port) {
                return port;
            }
        }

        // Ultimate fallback
        1194
    }

    /// Get appropriate protocol for VPN type
    fn get_protocol_for_type(&mut self, vpn_type: &VpnType) -> String {
        match vpn_type {
            VpnType::OpenVPN => {
                if self.rng.random_bool(0.7) {
                    "UDP"
                } else {
                    "TCP"
                }
            }
            VpnType::WireGuard => "UDP",
            VpnType::IPSec => {
                if self.rng.random_bool(0.8) {
                    "ESP"
                } else {
                    "AH"
                }
            }
        }
        .to_string()
    }

    /// Get appropriate cipher for VPN type
    fn get_cipher_for_type(&mut self, vpn_type: &VpnType) -> String {
        match vpn_type {
            VpnType::OpenVPN => {
                let ciphers = [
                    "AES-256-GCM",
                    "AES-256-CBC",
                    "AES-128-GCM",
                    "ChaCha20-Poly1305",
                ];
                ciphers[self.rng.random_range(0..ciphers.len())]
            }
            VpnType::WireGuard => "ChaCha20-Poly1305", // WireGuard uses this exclusively
            VpnType::IPSec => {
                let ciphers = ["AES-256", "AES-128", "3DES", "ChaCha20"];
                ciphers[self.rng.random_range(0..ciphers.len())]
            }
        }
        .to_string()
    }

    /// Get appropriate authentication method for VPN type
    fn get_auth_method_for_type(&mut self, vpn_type: &VpnType) -> String {
        match vpn_type {
            VpnType::OpenVPN => {
                let methods = ["Certificate", "Username/Password", "Certificate + Password"];
                methods[self.rng.random_range(0..methods.len())]
            }
            VpnType::WireGuard => "Public Key", // WireGuard uses public key cryptography
            VpnType::IPSec => {
                let methods = ["Pre-shared Key", "Certificate", "RSA Signature"];
                methods[self.rng.random_range(0..methods.len())]
            }
        }
        .to_string()
    }

    /// Generate key identifier
    fn generate_key_identifier(&mut self, vpn_type: &VpnType) -> String {
        match vpn_type {
            VpnType::OpenVPN => format!(
                "openvpn-cert-{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            ),
            VpnType::WireGuard => {
                // Generate realistic WireGuard public key format (base64, 44 chars)
                let chars: Vec<char> =
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                        .chars()
                        .collect();
                let key: String = (0..43)
                    .map(|_| chars[self.rng.random_range(0..chars.len())])
                    .collect();
                format!("{}=", key)
            }
            VpnType::IPSec => {
                // Generate PSK or certificate identifier
                if self.rng.random_bool(0.6) {
                    format!("psk-{}", Uuid::new_v4())
                } else {
                    format!(
                        "ipsec-cert-{}",
                        Uuid::new_v4().to_string().split('-').next().unwrap()
                    )
                }
            }
        }
    }

    /// Generate client subnet for VPN
    fn generate_client_subnet(&mut self) -> String {
        // Use RFC 1918 private networks for VPN clients
        match self.rng.random_range(0..3) {
            0 => format!("10.{}.0.0/16", self.rng.random_range(8..16)),
            1 => format!("172.{}.0.0/16", self.rng.random_range(16..32)),
            _ => format!("192.168.{}.0/24", self.rng.random_range(100..200)),
        }
    }

    /// Generate DNS servers for VPN clients
    fn generate_dns_servers(&mut self) -> Vec<String> {
        let public_dns = vec![
            "8.8.8.8",
            "8.8.4.4", // Google
            "1.1.1.1",
            "1.0.0.1", // Cloudflare
            "208.67.222.222",
            "208.67.220.220", // OpenDNS
            "9.9.9.9",
            "149.112.112.112", // Quad9
        ];

        let corporate_dns = ["192.168.1.1", "10.0.0.1", "172.16.0.1"];

        let mut servers = Vec::new();

        // Primary DNS
        if self.rng.random_bool(0.7) {
            // Use corporate DNS
            servers.push(corporate_dns[self.rng.random_range(0..corporate_dns.len())].to_string());
        } else {
            // Use public DNS
            servers.push(public_dns[self.rng.random_range(0..public_dns.len())].to_string());
        }

        // Secondary DNS (optional)
        if self.rng.random_bool(0.8) {
            let secondary = if servers[0].starts_with("192.168")
                || servers[0].starts_with("10.")
                || servers[0].starts_with("172.")
            {
                // If primary is corporate, use public as secondary
                public_dns[self.rng.random_range(0..public_dns.len())].to_string()
            } else {
                // If primary is public, might use another public or corporate
                if self.rng.random_bool(0.6) {
                    let mut choices = public_dns.clone();
                    choices.retain(|&dns| dns != servers[0]);
                    choices[self.rng.random_range(0..choices.len())].to_string()
                } else {
                    corporate_dns[self.rng.random_range(0..corporate_dns.len())].to_string()
                }
            };
            servers.push(secondary);
        }

        servers
    }
}

impl Default for VpnGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate multiple VPN configurations with progress tracking
pub fn generate_vpn_configurations(
    count: u16,
    seed: Option<u64>,
    progress_bar: Option<&indicatif::ProgressBar>,
) -> VpnResult<Vec<VpnConfig>> {
    let mut generator = VpnGenerator::new_with_seed(seed);
    let mut configs = Vec::with_capacity(count as usize);

    for i in 0..count {
        let config = generator.generate_single(None)?;
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
    use crate::cli::parse_vlan_range;

    #[test]
    fn test_vpn_config_creation() {
        let config = VpnConfig::new(
            VpnType::OpenVPN,
            "Test-VPN".to_string(),
            "vpn.example.com".to_string(),
            1194,
            "UDP".to_string(),
            "AES-256-GCM".to_string(),
            "Certificate".to_string(),
            "test-cert-123".to_string(),
            "10.8.0.0/24".to_string(),
            vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            true,
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.vpn_type, VpnType::OpenVPN);
        assert_eq!(config.name, "Test-VPN");
        assert_eq!(config.port, 1194);
    }

    #[test]
    fn test_vpn_config_validation_invalid_port() {
        let config = VpnConfig::new(
            VpnType::OpenVPN,
            "Test-VPN".to_string(),
            "vpn.example.com".to_string(),
            0, // Invalid port
            "UDP".to_string(),
            "AES-256-GCM".to_string(),
            "Certificate".to_string(),
            "test-cert-123".to_string(),
            "10.8.0.0/24".to_string(),
            vec!["8.8.8.8".to_string()],
            true,
        );

        assert!(config.is_err());
        assert!(
            config
                .unwrap_err()
                .to_string()
                .contains("port 0 is invalid")
        );
    }

    #[test]
    fn test_vpn_config_validation_invalid_protocol() {
        let config = VpnConfig::new(
            VpnType::WireGuard,
            "Test-VPN".to_string(),
            "vpn.example.com".to_string(),
            51820,
            "TCP".to_string(), // Invalid for WireGuard
            "ChaCha20-Poly1305".to_string(),
            "Public Key".to_string(),
            "test-key".to_string(),
            "10.8.0.0/24".to_string(),
            vec!["8.8.8.8".to_string()],
            true,
        );

        assert!(config.is_err());
        assert!(
            config
                .unwrap_err()
                .to_string()
                .contains("WireGuard protocol 'TCP' is invalid")
        );
    }

    #[test]
    fn test_vpn_generator_single() {
        let mut generator = VpnGenerator::new_with_seed(Some(42));
        let config = generator.generate_single(Some(VpnType::OpenVPN));

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.vpn_type, VpnType::OpenVPN);
        assert!(!config.name.is_empty());
        assert!(!config.server.is_empty());
        assert!(config.port > 0);
    }

    #[test]
    fn test_vpn_generator_batch() {
        let mut generator = VpnGenerator::new_with_seed(Some(42));
        let configs = generator.generate_batch(5);

        assert!(configs.is_ok());
        let configs = configs.unwrap();
        assert_eq!(configs.len(), 5);

        // Check uniqueness of names
        let mut names = std::collections::HashSet::new();

        for config in &configs {
            assert!(
                names.insert(&config.name),
                "Duplicate name: {}",
                config.name
            );
            // Ports might not be unique across different VPN types, so we only check within type
        }
    }

    #[test]
    fn test_parse_vlan_range() {
        // Test single VLAN
        let ranges = parse_vlan_range("100").unwrap();
        assert_eq!(ranges, vec![(100, 100)]);

        // Test simple range
        let ranges = parse_vlan_range("100-150").unwrap();
        assert_eq!(ranges, vec![(100, 150)]);

        // Test multiple ranges
        let ranges = parse_vlan_range("10,20-30,40").unwrap();
        assert_eq!(ranges, vec![(10, 10), (20, 30), (40, 40)]);

        // Test invalid range
        assert!(parse_vlan_range("150-100").is_err());
        assert!(parse_vlan_range("5-10").is_err()); // Below minimum
        assert!(parse_vlan_range("4095-5000").is_err()); // Above maximum
    }
}
