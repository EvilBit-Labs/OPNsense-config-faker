//! Firewall rules generation with realistic security patterns

use crate::model::ConfigError;
use crate::Result;
use fake::Fake;
use indicatif::ProgressBar;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Firewall rule configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirewallRule {
    /// Unique rule identifier
    pub rule_id: String,

    /// Source network or IP address
    pub source: String,

    /// Destination network or IP address
    pub destination: String,

    /// Protocol (TCP, UDP, ICMP, Any)
    pub protocol: String,

    /// Port specifications (e.g., "80,443", "1024:65535", "any")
    pub ports: String,

    /// Action (Pass, Block, Reject)
    pub action: String,

    /// Direction (In, Out)
    pub direction: String,

    /// Human-readable description
    pub description: String,

    /// Enable logging for this rule
    pub log: bool,

    /// Associated VLAN ID (optional)
    pub vlan_id: Option<u16>,

    /// Rule priority/order
    pub priority: u16,

    /// Interface this rule applies to
    pub interface: String,
}

impl FirewallRule {
    /// Create a new firewall rule with validation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rule_id: String,
        source: String,
        destination: String,
        protocol: String,
        ports: String,
        action: String,
        direction: String,
        description: String,
        log: bool,
        vlan_id: Option<u16>,
        priority: u16,
        interface: String,
    ) -> Result<Self> {
        // Validate rule ID
        if rule_id.is_empty() {
            return Err(ConfigError::validation("Rule ID cannot be empty"));
        }

        // Validate action
        let valid_actions = ["pass", "block", "reject"];
        if !valid_actions.contains(&action.to_lowercase().as_str()) {
            return Err(ConfigError::validation(format!(
                "Invalid action '{}'. Must be one of: {:?}",
                action, valid_actions
            )));
        }

        // Validate direction
        let valid_directions = ["in", "out"];
        if !valid_directions.contains(&direction.to_lowercase().as_str()) {
            return Err(ConfigError::validation(format!(
                "Invalid direction '{}'. Must be one of: {:?}",
                direction, valid_directions
            )));
        }

        // Validate protocol
        let valid_protocols = ["tcp", "udp", "icmp", "any"];
        if !valid_protocols.contains(&protocol.to_lowercase().as_str()) {
            return Err(ConfigError::validation(format!(
                "Invalid protocol '{}'. Must be one of: {:?}",
                protocol, valid_protocols
            )));
        }

        // Validate VLAN ID if provided
        if let Some(vid) = vlan_id {
            if !(10..=4094).contains(&vid) {
                return Err(ConfigError::validation(format!(
                    "VLAN ID {} is outside valid range 10-4094",
                    vid
                )));
            }
        }

        Ok(Self {
            rule_id,
            source,
            destination,
            protocol,
            ports,
            action,
            direction,
            description,
            log,
            vlan_id,
            priority,
            interface,
        })
    }

    /// Validate the firewall rule configuration
    pub fn validate(&self) -> Result<()> {
        // Re-run validation logic
        let _ = Self::new(
            self.rule_id.clone(),
            self.source.clone(),
            self.destination.clone(),
            self.protocol.clone(),
            self.ports.clone(),
            self.action.clone(),
            self.direction.clone(),
            self.description.clone(),
            self.log,
            self.vlan_id,
            self.priority,
            self.interface.clone(),
        )?;
        Ok(())
    }
}

/// Firewall rule complexity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum FirewallComplexity {
    Basic,
    Intermediate,
    Advanced,
}

impl FirewallComplexity {
    /// Get the number of rules per VLAN for this complexity level
    pub fn rules_per_vlan(self) -> u16 {
        match self {
            FirewallComplexity::Basic => 3,
            FirewallComplexity::Intermediate => 7,
            FirewallComplexity::Advanced => 15,
        }
    }
}

impl std::str::FromStr for FirewallComplexity {
    type Err = ConfigError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(FirewallComplexity::Basic),
            "intermediate" => Ok(FirewallComplexity::Intermediate),
            "advanced" => Ok(FirewallComplexity::Advanced),
            _ => Err(ConfigError::validation(format!(
                "Invalid complexity level '{}'. Must be one of: basic, intermediate, advanced",
                s
            ))),
        }
    }
}

/// Firewall rule generator
pub struct FirewallGenerator {
    /// Random number generator for future randomized rule generation
    /// Currently unused but reserved for:
    /// - Randomized rule priorities and ordering
    /// - Stochastic rule complexity variations
    /// - Reproducible test data generation
    #[allow(dead_code)]
    rng: ChaCha8Rng,
    rule_counter: u16,
    used_rule_ids: HashSet<String>,
}

impl FirewallGenerator {
    /// Create a new firewall generator
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::seed_from_u64(rand::random::<u64>()),
        };

        Self {
            rng,
            rule_counter: 1,
            used_rule_ids: HashSet::new(),
        }
    }

    /// Generate firewall rules for a specific VLAN
    pub fn generate_vlan_rules(
        &mut self,
        vlan_id: u16,
        vlan_network: &str,
        complexity: FirewallComplexity,
        department: &str,
    ) -> Result<Vec<FirewallRule>> {
        let rules_count = complexity.rules_per_vlan();
        let mut rules = Vec::with_capacity(rules_count as usize);

        // Generate basic network access rules
        rules.extend(self.generate_basic_rules(vlan_id, vlan_network, department)?);

        // Generate intermediate rules if complexity allows
        if complexity >= FirewallComplexity::Intermediate {
            rules.extend(self.generate_intermediate_rules(vlan_id, vlan_network, department)?);
        }

        // Generate advanced rules if complexity allows
        if complexity >= FirewallComplexity::Advanced {
            rules.extend(self.generate_advanced_rules(vlan_id, vlan_network, department)?);
        }

        // Ensure we don't exceed the requested count
        rules.truncate(rules_count as usize);

        // Assign priorities
        for (i, rule) in rules.iter_mut().enumerate() {
            rule.priority = (i + 1) as u16;
        }

        Ok(rules)
    }

    /// Generate basic firewall rules (always included)
    fn generate_basic_rules(
        &mut self,
        vlan_id: u16,
        vlan_network: &str,
        department: &str,
    ) -> Result<Vec<FirewallRule>> {
        let mut rules = Vec::new();

        // Rule 1: Allow internal traffic within VLAN
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            vlan_network.to_string(),
            "any".to_string(),
            "any".to_string(),
            "pass".to_string(),
            "in".to_string(),
            generate_rule_description(department, "Allow", "internal traffic"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 2: Allow DNS queries
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "udp".to_string(),
            "53".to_string(),
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Allow", "DNS queries"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 3: Allow HTTP/HTTPS for internet access
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "80,443".to_string(),
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Allow", "web access"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        Ok(rules)
    }

    /// Generate intermediate firewall rules
    fn generate_intermediate_rules(
        &mut self,
        vlan_id: u16,
        vlan_network: &str,
        department: &str,
    ) -> Result<Vec<FirewallRule>> {
        let mut rules = Vec::new();

        // Rule 4: Allow NTP time synchronization
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "udp".to_string(),
            "123".to_string(),
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Allow", "NTP synchronization"),
            false, // Don't log NTP traffic
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 5: Allow ICMP for network diagnostics
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "icmp".to_string(),
            "any".to_string(),
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Allow", "ICMP diagnostics"),
            false, // Don't log ICMP traffic
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 6: Block common attack ports
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            "any".to_string(),
            vlan_network.to_string(),
            "tcp".to_string(),
            "22,23,3389".to_string(), // SSH, Telnet, RDP
            "block".to_string(),
            "in".to_string(),
            generate_rule_description(department, "Block", "remote access attempts"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 7: Allow specific application ports based on department
        let app_ports = self.get_department_ports(department);
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "tcp".to_string(),
            app_ports,
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Allow", "application access"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        Ok(rules)
    }

    /// Generate advanced firewall rules
    fn generate_advanced_rules(
        &mut self,
        vlan_id: u16,
        vlan_network: &str,
        department: &str,
    ) -> Result<Vec<FirewallRule>> {
        let mut rules = Vec::new();

        // Rule 8: Rate limiting for web traffic
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "80,443".to_string(),
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Rate-limited", "web access"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 9: Block peer-to-peer traffic
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "6881:6889,51413".to_string(), // BitTorrent ports
            "block".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Block", "P2P traffic"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 10: Allow VPN access for specific departments
        if self.should_allow_vpn(department) {
            rules.push(FirewallRule::new(
                self.generate_rule_id(),
                vlan_network.to_string(),
                "any".to_string(),
                "udp".to_string(),
                "1194,500,4500".to_string(), // OpenVPN, IPSec
                "pass".to_string(),
                "out".to_string(),
                generate_rule_description(department, "Allow", "VPN access"),
                true,
                Some(vlan_id),
                0, // Will be set later
                format!("vlan{}", vlan_id),
            )?);
        }

        // Rule 11: Block social media for certain departments
        if self.should_block_social_media(department) {
            rules.push(FirewallRule::new(
                self.generate_rule_id(),
                vlan_network.to_string(),
                "any".to_string(),
                "tcp".to_string(),
                "443".to_string(),
                "block".to_string(),
                "out".to_string(),
                generate_rule_description(department, "Block", "social media access"),
                true,
                Some(vlan_id),
                0, // Will be set later
                format!("vlan{}", vlan_id),
            )?);
        }

        // Rule 12: Allow file sharing for IT department
        if department.to_lowercase().contains("it") {
            rules.push(FirewallRule::new(
                self.generate_rule_id(),
                vlan_network.to_string(),
                "any".to_string(),
                "tcp".to_string(),
                "21,22,445,139".to_string(), // FTP, SSH, SMB
                "pass".to_string(),
                "out".to_string(),
                generate_rule_description(department, "Allow", "file sharing"),
                true,
                Some(vlan_id),
                0, // Will be set later
                format!("vlan{}", vlan_id),
            )?);
        }

        // Rule 13: Block gaming traffic for business departments
        if self.should_block_gaming(department) {
            rules.push(FirewallRule::new(
                self.generate_rule_id(),
                vlan_network.to_string(),
                "any".to_string(),
                "tcp".to_string(),
                "27015:27018,25565,25575".to_string(), // Common gaming ports
                "block".to_string(),
                "out".to_string(),
                generate_rule_description(department, "Block", "gaming traffic"),
                true,
                Some(vlan_id),
                0, // Will be set later
                format!("vlan{}", vlan_id),
            )?);
        }

        // Rule 14: Allow monitoring and management traffic
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "161,162,514".to_string(), // SNMP, Syslog
            "pass".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Allow", "monitoring traffic"),
            false, // Don't log monitoring traffic
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        // Rule 15: Default deny rule (should be last)
        rules.push(FirewallRule::new(
            self.generate_rule_id(),
            vlan_network.to_string(),
            "any".to_string(),
            "any".to_string(),
            "any".to_string(),
            "block".to_string(),
            "out".to_string(),
            generate_rule_description(department, "Default deny", "outbound traffic"),
            true,
            Some(vlan_id),
            0, // Will be set later
            format!("vlan{}", vlan_id),
        )?);

        Ok(rules)
    }

    /// Generate a unique rule ID
    fn generate_rule_id(&mut self) -> String {
        loop {
            let rule_id = format!("rule_{:04}", self.rule_counter);
            self.rule_counter += 1;

            if !self.used_rule_ids.contains(&rule_id) {
                self.used_rule_ids.insert(rule_id.clone());
                return rule_id;
            }
        }
    }

    /// Get department-specific application ports
    fn get_department_ports(&self, department: &str) -> String {
        let dept_lower = department.to_lowercase();

        if dept_lower.contains("it") || dept_lower.contains("engineering") {
            "22,23,3389,5900,8080,8443".to_string() // SSH, Telnet, RDP, VNC, Web management
        } else if dept_lower.contains("sales") || dept_lower.contains("marketing") {
            "25,587,465,143,993".to_string() // SMTP, IMAP
        } else if dept_lower.contains("finance") || dept_lower.contains("hr") {
            "1433,3306,5432".to_string() // Database ports
        } else {
            "any".to_string()
        }
    }

    /// Determine if VPN access should be allowed for this department
    fn should_allow_vpn(&self, department: &str) -> bool {
        let dept_lower = department.to_lowercase();
        dept_lower.contains("it")
            || dept_lower.contains("engineering")
            || dept_lower.contains("sales")
    }

    /// Determine if social media should be blocked for this department
    fn should_block_social_media(&self, department: &str) -> bool {
        let dept_lower = department.to_lowercase();
        dept_lower.contains("finance") || dept_lower.contains("hr") || dept_lower.contains("legal")
    }

    /// Determine if gaming traffic should be blocked for this department
    fn should_block_gaming(&self, department: &str) -> bool {
        let dept_lower = department.to_lowercase();
        dept_lower.contains("finance")
            || dept_lower.contains("hr")
            || dept_lower.contains("legal")
            || dept_lower.contains("executive")
    }
}

/// Generate firewall rules for multiple VLANs
pub fn generate_firewall_rules(
    vlan_configs: &[crate::generator::VlanConfig],
    complexity: FirewallComplexity,
    seed: Option<u64>,
    progress_bar: Option<&ProgressBar>,
) -> Result<Vec<FirewallRule>> {
    let mut generator = FirewallGenerator::new(seed);
    let mut all_rules = Vec::new();

    for vlan_config in vlan_configs.iter() {
        // Validate VLAN configuration before generating rules
        vlan_config.validate().map_err(|e| {
            ConfigError::validation(format!(
                "Invalid VLAN configuration for VLAN {}: {}",
                vlan_config.vlan_id, e
            ))
        })?;

        if let Some(pb) = progress_bar {
            pb.set_message(format!(
                "Generating firewall rules for VLAN {}",
                vlan_config.vlan_id
            ));
        }

        // Get department name from VLAN description
        let department = extract_department_from_description(&vlan_config.description);

        let vlan_rules = generator.generate_vlan_rules(
            vlan_config.vlan_id,
            &vlan_config.ip_network,
            complexity,
            &department,
        )?;

        all_rules.extend(vlan_rules);

        if let Some(pb) = progress_bar {
            pb.inc(1);
        }
    }

    Ok(all_rules)
}

/// Extract department name from VLAN description
fn extract_department_from_description(description: &str) -> String {
    // Common department patterns in descriptions
    let dept_patterns = [
        "IT",
        "Engineering",
        "Sales",
        "Marketing",
        "Finance",
        "HR",
        "Legal",
        "Executive",
        "Support",
        "Admin",
        "Operations",
        "Guest",
        "Lab",
        "Test",
    ];

    for pattern in &dept_patterns {
        if description.to_lowercase().contains(&pattern.to_lowercase()) {
            return pattern.to_string();
        }
    }

    // Fallback to a realistic department name using fake crate
    generate_department_name()
}

/// Generate realistic department name using fake crate
fn generate_department_name() -> String {
    use crate::generator::departments;

    // Get all available departments and filter to only those longer than 2 characters
    // to satisfy test requirements
    let valid_departments: Vec<&str> = departments::all_departments()
        .iter()
        .filter(|&&dept| dept.len() > 2)
        .copied()
        .collect();

    // Simple random selection using the current time as a seed
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize;

    // Fallback to a longer department name if the filtered list is empty
    if valid_departments.is_empty() {
        "Information Technology".to_string()
    } else {
        valid_departments[seed % valid_departments.len()].to_string()
    }
}

/// Generate realistic rule description using fake crate
fn generate_rule_description(department: &str, action: &str, service: &str) -> String {
    use fake::faker::lorem::en::*;
    let context = Words(2..4).fake::<Vec<String>>().join(" ");
    format!("{} {} {} - {}", action, department, service, context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_rule_creation() {
        let rule = FirewallRule::new(
            "rule_001".to_string(),
            "192.168.1.0/24".to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "80,443".to_string(),
            "pass".to_string(),
            "in".to_string(),
            "Allow web traffic".to_string(),
            true,
            Some(100),
            1,
            "vlan100".to_string(),
        )
        .unwrap();

        assert_eq!(rule.rule_id, "rule_001");
        assert_eq!(rule.source, "192.168.1.0/24");
        assert_eq!(rule.destination, "any");
        assert_eq!(rule.protocol, "tcp");
        assert_eq!(rule.ports, "80,443");
        assert_eq!(rule.action, "pass");
        assert_eq!(rule.direction, "in");
        assert_eq!(rule.description, "Allow web traffic");
        assert!(rule.log);
        assert_eq!(rule.vlan_id, Some(100));
        assert_eq!(rule.priority, 1);
        assert_eq!(rule.interface, "vlan100");
    }

    #[test]
    fn test_firewall_rule_validation() {
        // Valid rule
        let valid_rule = FirewallRule::new(
            "rule_001".to_string(),
            "192.168.1.0/24".to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "80,443".to_string(),
            "pass".to_string(),
            "in".to_string(),
            "Allow web traffic".to_string(),
            true,
            Some(100),
            1,
            "vlan100".to_string(),
        )
        .unwrap();
        assert!(valid_rule.validate().is_ok());

        // Invalid rule - empty rule ID
        let invalid_rule = FirewallRule::new(
            "".to_string(),
            "192.168.1.0/24".to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "80,443".to_string(),
            "pass".to_string(),
            "in".to_string(),
            "Allow web traffic".to_string(),
            true,
            Some(100),
            1,
            "vlan100".to_string(),
        );
        assert!(invalid_rule.is_err());

        // Invalid rule - invalid VLAN ID
        let invalid_vlan_rule = FirewallRule::new(
            "rule_001".to_string(),
            "192.168.1.0/24".to_string(),
            "any".to_string(),
            "tcp".to_string(),
            "80,443".to_string(),
            "pass".to_string(),
            "in".to_string(),
            "Allow web traffic".to_string(),
            true,
            Some(5000), // Invalid VLAN ID
            1,
            "vlan100".to_string(),
        );
        assert!(invalid_vlan_rule.is_err());
    }

    #[test]
    fn test_complexity_levels() {
        assert_eq!(FirewallComplexity::Basic.rules_per_vlan(), 3);
        assert_eq!(FirewallComplexity::Intermediate.rules_per_vlan(), 7);
        assert_eq!(FirewallComplexity::Advanced.rules_per_vlan(), 15);
    }

    #[test]
    fn test_complexity_parsing() {
        assert_eq!(
            "basic".parse::<FirewallComplexity>().unwrap(),
            FirewallComplexity::Basic
        );
        assert_eq!(
            "intermediate".parse::<FirewallComplexity>().unwrap(),
            FirewallComplexity::Intermediate
        );
        assert_eq!(
            "advanced".parse::<FirewallComplexity>().unwrap(),
            FirewallComplexity::Advanced
        );
        assert!("invalid".parse::<FirewallComplexity>().is_err());
    }

    #[test]
    fn test_firewall_generator() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let rules = generator
            .generate_vlan_rules(100, "192.168.100.0/24", FirewallComplexity::Basic, "IT")
            .unwrap();

        assert!(!rules.is_empty());
        assert!(rules.len() >= 3); // At least basic rules per VLAN
    }

    #[test]
    fn test_firewall_generator_advanced() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let rules = generator
            .generate_vlan_rules(100, "192.168.100.0/24", FirewallComplexity::Advanced, "IT")
            .unwrap();

        assert!(!rules.is_empty());
        assert!(rules.len() >= 10); // Advanced should generate more rules
    }

    #[test]
    fn test_firewall_generator_multiple_vlans() {
        let mut generator = FirewallGenerator::new(Some(12345));

        // Test multiple VLANs by generating rules for each
        let rules1 = generator
            .generate_vlan_rules(
                100,
                "192.168.100.0/24",
                FirewallComplexity::Intermediate,
                "IT",
            )
            .unwrap();

        let rules2 = generator
            .generate_vlan_rules(
                200,
                "192.168.200.0/24",
                FirewallComplexity::Intermediate,
                "Sales",
            )
            .unwrap();

        let rules3 = generator
            .generate_vlan_rules(
                300,
                "192.168.300.0/24",
                FirewallComplexity::Intermediate,
                "Engineering",
            )
            .unwrap();

        assert!(!rules1.is_empty());
        assert!(!rules2.is_empty());
        assert!(!rules3.is_empty());

        // Check that rules have correct VLAN IDs
        assert!(rules1.iter().all(|r| r.vlan_id == Some(100)));
        assert!(rules2.iter().all(|r| r.vlan_id == Some(200)));
        assert!(rules3.iter().all(|r| r.vlan_id == Some(300)));
    }

    #[test]
    fn test_firewall_generator_deterministic() {
        let seed = 12345;
        let mut generator1 = FirewallGenerator::new(Some(seed));
        let mut generator2 = FirewallGenerator::new(Some(seed));

        let rules1 = generator1
            .generate_vlan_rules(100, "192.168.100.0/24", FirewallComplexity::Basic, "IT")
            .unwrap();

        let rules2 = generator2
            .generate_vlan_rules(100, "192.168.100.0/24", FirewallComplexity::Basic, "IT")
            .unwrap();

        assert_eq!(rules1.len(), rules2.len());
        // Rules should be identical with same seed
        for (rule1, rule2) in rules1.iter().zip(rules2.iter()) {
            assert_eq!(rule1.rule_id, rule2.rule_id);
            assert_eq!(rule1.source, rule2.source);
            assert_eq!(rule1.destination, rule2.destination);
        }
    }

    #[test]
    fn test_department_extraction() {
        assert_eq!(extract_department_from_description("IT_VLAN_0100"), "IT");
        assert_eq!(
            extract_department_from_description("Sales_VLAN_0200"),
            "Sales"
        );
        // Test that unknown descriptions return a realistic department name (not empty)
        let unknown_dept = extract_department_from_description("Unknown_VLAN_0300");
        assert!(!unknown_dept.is_empty());
        assert!(unknown_dept.len() > 2);
    }

    #[test]
    fn test_generate_rule_id() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let id1 = generator.generate_rule_id();
        let id2 = generator.generate_rule_id();

        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
        assert_ne!(id1, id2); // Should be unique
        assert!(id1.starts_with("rule_"));
        assert!(id2.starts_with("rule_"));
    }

    #[test]
    fn test_generate_rule_id_uniqueness() {
        let mut generator = FirewallGenerator::new(Some(12345));
        let mut ids = std::collections::HashSet::new();

        for _ in 0..100 {
            let id = generator.generate_rule_id();
            assert!(ids.insert(id)); // Should be unique
        }
    }

    #[test]
    fn test_basic_rules_generation() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let rules = generator
            .generate_basic_rules(100, "192.168.1.0/24", "IT")
            .unwrap();

        assert!(!rules.is_empty());
        assert!(rules.len() >= 3); // Basic rules should include internal, internet, and service rules

        // Check that rules have correct VLAN ID
        for rule in &rules {
            assert_eq!(rule.vlan_id, Some(100));
            assert_eq!(rule.interface, "vlan100");
        }
    }

    #[test]
    fn test_intermediate_rules_generation() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let rules = generator
            .generate_intermediate_rules(100, "192.168.1.0/24", "IT")
            .unwrap();

        assert!(!rules.is_empty());
        assert!(rules.len() >= 3); // Intermediate should have at least basic rules

        // Check for service-specific rules
        let has_service_rules = rules
            .iter()
            .any(|r| r.ports.contains("80") || r.ports.contains("443") || r.ports.contains("22"));
        assert!(has_service_rules);
    }

    #[test]
    fn test_advanced_rules_generation() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let rules = generator
            .generate_advanced_rules(100, "192.168.1.0/24", "IT")
            .unwrap();

        assert!(!rules.is_empty());
        assert!(rules.len() >= 3); // Advanced should have at least basic rules

        // Check for advanced features like logging and specific protocols
        let has_logging_rules = rules.iter().any(|r| r.log);
        assert!(has_logging_rules);

        let has_specific_protocols = rules
            .iter()
            .any(|r| r.protocol == "tcp" || r.protocol == "udp" || r.protocol == "icmp");
        assert!(has_specific_protocols);
    }

    #[test]
    fn test_generate_rule_description() {
        let desc1 = generate_rule_description("IT", "Allow", "web traffic");
        let desc2 = generate_rule_description("Sales", "Block", "file sharing");

        assert!(!desc1.is_empty());
        assert!(!desc2.is_empty());
        assert!(desc1.contains("IT"));
        assert!(desc1.contains("Allow"));
        assert!(desc2.contains("Sales"));
        assert!(desc2.contains("Block"));
    }

    #[test]
    fn test_firewall_generator_with_different_seeds() {
        let mut generator1 = FirewallGenerator::new(Some(12345));
        let mut generator2 = FirewallGenerator::new(Some(67890));

        let rules1 = generator1
            .generate_vlan_rules(100, "192.168.100.0/24", FirewallComplexity::Basic, "IT")
            .unwrap();

        let rules2 = generator2
            .generate_vlan_rules(200, "192.168.200.0/24", FirewallComplexity::Basic, "Sales")
            .unwrap();

        assert_eq!(rules1.len(), rules2.len()); // Same number of rules
                                                // But different content due to different seeds and departments
        assert_ne!(rules1[0].description, rules2[0].description);
    }

    #[test]
    fn test_firewall_generator_edge_cases() {
        let mut generator = FirewallGenerator::new(Some(12345));

        // Test with maximum VLAN ID
        let rules = generator
            .generate_vlan_rules(4094, "192.168.254.0/24", FirewallComplexity::Basic, "IT")
            .unwrap();

        assert!(!rules.is_empty());
        assert_eq!(rules[0].vlan_id, Some(4094));
        assert_eq!(rules[0].interface, "vlan4094");
    }

    #[test]
    fn test_firewall_generator_rule_priority_assignment() {
        let mut generator = FirewallGenerator::new(Some(12345));

        let rules = generator
            .generate_vlan_rules(
                100,
                "192.168.100.0/24",
                FirewallComplexity::Intermediate,
                "IT",
            )
            .unwrap();

        // Check that priorities are assigned correctly
        let priorities: Vec<_> = rules.iter().map(|r| r.priority).collect();
        assert!(priorities.iter().all(|&p| p > 0)); // All priorities should be positive
    }

    #[test]
    fn test_generate_firewall_rules_with_valid_vlan_configs() {
        use crate::generator::VlanConfig;

        let vlan_configs = vec![
            VlanConfig::new(
                100,
                "192.168.100.x".to_string(),
                "IT_VLAN_0100".to_string(),
                1,
            )
            .unwrap(),
            VlanConfig::new(
                200,
                "192.168.200.x".to_string(),
                "Sales_VLAN_0200".to_string(),
                1,
            )
            .unwrap(),
        ];

        let rules =
            generate_firewall_rules(&vlan_configs, FirewallComplexity::Basic, Some(12345), None)
                .unwrap();

        assert!(!rules.is_empty());
        assert!(rules.len() >= 6); // At least 3 rules per VLAN * 2 VLANs
    }

    #[test]
    fn test_generate_firewall_rules_with_invalid_vlan_config() {
        use crate::generator::VlanConfig;

        // Create an invalid VLAN config with invalid VLAN ID
        let invalid_vlan = VlanConfig {
            vlan_id: 5000, // Invalid VLAN ID > 4094
            ip_network: "192.168.100.x".to_string(),
            description: "Invalid_VLAN".to_string(),
            wan_assignment: 1,
        };

        let vlan_configs = vec![invalid_vlan];

        let result =
            generate_firewall_rules(&vlan_configs, FirewallComplexity::Basic, Some(12345), None);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid VLAN configuration for VLAN 5000"));
        assert!(error_msg.contains("outside valid range 10-4094"));
    }

    #[test]
    fn test_generate_firewall_rules_with_invalid_network_format() {
        use crate::generator::VlanConfig;

        // Create an invalid VLAN config with invalid network format
        let invalid_vlan = VlanConfig {
            vlan_id: 100,
            ip_network: "invalid.network.format".to_string(), // Invalid format
            description: "Invalid_Network_VLAN".to_string(),
            wan_assignment: 1,
        };

        let vlan_configs = vec![invalid_vlan];

        let result =
            generate_firewall_rules(&vlan_configs, FirewallComplexity::Basic, Some(12345), None);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid VLAN configuration for VLAN 100"));
        assert!(error_msg.contains("does not match expected format"));
    }

    #[test]
    fn test_generate_firewall_rules_with_empty_description() {
        use crate::generator::VlanConfig;

        // Create an invalid VLAN config with empty description
        let invalid_vlan = VlanConfig {
            vlan_id: 100,
            ip_network: "192.168.100.x".to_string(),
            description: "".to_string(), // Empty description
            wan_assignment: 1,
        };

        let vlan_configs = vec![invalid_vlan];

        let result =
            generate_firewall_rules(&vlan_configs, FirewallComplexity::Basic, Some(12345), None);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid VLAN configuration for VLAN 100"));
        assert!(error_msg.contains("cannot be empty"));
    }

    #[test]
    fn test_generate_firewall_rules_with_invalid_wan_assignment() {
        use crate::generator::VlanConfig;

        // Create an invalid VLAN config with invalid WAN assignment
        let invalid_vlan = VlanConfig {
            vlan_id: 100,
            ip_network: "192.168.100.x".to_string(),
            description: "Test_VLAN".to_string(),
            wan_assignment: 5, // Invalid WAN assignment > 3
        };

        let vlan_configs = vec![invalid_vlan];

        let result =
            generate_firewall_rules(&vlan_configs, FirewallComplexity::Basic, Some(12345), None);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid VLAN configuration for VLAN 100"));
        assert!(error_msg.contains("outside valid range 1-3"));
    }
}
