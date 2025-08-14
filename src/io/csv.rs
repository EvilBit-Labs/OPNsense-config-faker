//! CSV input/output operations

use crate::generator::{FirewallRule, VlanConfig};
use crate::Result;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

/// CSV record structure matching Python implementation format
#[derive(Debug, Serialize, Deserialize)]
struct CsvRecord {
    #[serde(rename = "VLAN")]
    vlan_id: u16,

    #[serde(rename = "IP Range")]
    ip_range: String,

    #[serde(rename = "Beschreibung")]
    description: String,

    #[serde(rename = "WAN")]
    wan_assignment: u8,
}

impl From<&VlanConfig> for CsvRecord {
    fn from(config: &VlanConfig) -> Self {
        Self {
            vlan_id: config.vlan_id,
            ip_range: config.ip_network.clone(),
            description: config.description.clone(),
            wan_assignment: config.wan_assignment,
        }
    }
}

impl From<CsvRecord> for VlanConfig {
    fn from(record: CsvRecord) -> Self {
        // Note: This bypasses validation for CSV compatibility
        // The Python implementation may have generated data that doesn't
        // strictly conform to our validation rules
        Self {
            vlan_id: record.vlan_id,
            ip_network: record.ip_range,
            description: record.description,
            wan_assignment: record.wan_assignment,
        }
    }
}

/// Write VLAN configurations to a CSV file
pub fn write_csv<P: AsRef<Path>>(configs: &[VlanConfig], path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);

    // Write header and records
    for config in configs {
        let record = CsvRecord::from(config);
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

/// Read VLAN configurations from a CSV file
pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<VlanConfig>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    let mut configs = Vec::new();

    for result in reader.deserialize() {
        let record: CsvRecord = result?;
        configs.push(VlanConfig::from(record));
    }

    Ok(configs)
}

/// Read VLAN configurations from a CSV file with enhanced validation
pub fn read_csv_validated<P: AsRef<Path>>(path: P) -> Result<Vec<VlanConfig>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    let mut configs = Vec::new();
    let mut line_number = 1; // Start at 1 for header

    for result in reader.deserialize() {
        line_number += 1;
        let record: CsvRecord = result.map_err(|e| {
            crate::model::ConfigError::validation(format!(
                "CSV parsing error at line {line_number}: {e}"
            ))
        })?;

        // Validate the converted VlanConfig
        let config = VlanConfig::from(record);

        // Additional validation for CSV-loaded data
        if config.vlan_id < 10 || config.vlan_id > 4094 {
            return Err(crate::model::ConfigError::validation(format!(
                "Invalid VLAN ID {} at line {}: must be between 10 and 4094",
                config.vlan_id, line_number
            )));
        }

        if config.wan_assignment < 1 || config.wan_assignment > 3 {
            return Err(crate::model::ConfigError::validation(format!(
                "Invalid WAN assignment {} at line {}: must be between 1 and 3",
                config.wan_assignment, line_number
            )));
        }

        // Validate IP network format
        if !config.ip_network.ends_with(".x") && !config.ip_network.contains('/') {
            return Err(crate::model::ConfigError::validation(format!(
                "Invalid IP network format '{}' at line {}: must end with '.x' or contain '/'",
                config.ip_network, line_number
            )));
        }

        configs.push(config);
    }

    Ok(configs)
}

/// CSV record structure for firewall rules
#[derive(Debug, Serialize, Deserialize)]
struct FirewallRuleCsvRecord {
    #[serde(rename = "rule_id")]
    rule_id: String,

    #[serde(rename = "source")]
    source: String,

    #[serde(rename = "destination")]
    destination: String,

    #[serde(rename = "protocol")]
    protocol: String,

    #[serde(rename = "ports")]
    ports: String,

    #[serde(rename = "action")]
    action: String,

    #[serde(rename = "direction")]
    direction: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "log")]
    log: bool,

    #[serde(rename = "vlan_id")]
    vlan_id: Option<u16>,

    #[serde(rename = "priority")]
    priority: u16,

    #[serde(rename = "interface")]
    interface: String,
}

impl From<&FirewallRule> for FirewallRuleCsvRecord {
    fn from(rule: &FirewallRule) -> Self {
        Self {
            rule_id: rule.rule_id.clone(),
            source: rule.source.clone(),
            destination: rule.destination.clone(),
            protocol: rule.protocol.clone(),
            ports: rule.ports.clone(),
            action: rule.action.clone(),
            direction: rule.direction.clone(),
            description: rule.description.clone(),
            log: rule.log,
            vlan_id: rule.vlan_id,
            priority: rule.priority,
            interface: rule.interface.clone(),
        }
    }
}

impl From<FirewallRuleCsvRecord> for FirewallRule {
    fn from(record: FirewallRuleCsvRecord) -> Self {
        // Note: This bypasses validation for CSV compatibility
        Self {
            rule_id: record.rule_id,
            source: record.source,
            destination: record.destination,
            protocol: record.protocol,
            ports: record.ports,
            action: record.action,
            direction: record.direction,
            description: record.description,
            log: record.log,
            vlan_id: record.vlan_id,
            priority: record.priority,
            interface: record.interface,
        }
    }
}

/// Write firewall rules to a CSV file
pub fn write_firewall_rules_csv<P: AsRef<Path>>(rules: &[FirewallRule], path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);

    // Write header and records
    for rule in rules {
        let record = FirewallRuleCsvRecord::from(rule);
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

/// Read firewall rules from a CSV file
pub fn read_firewall_rules_csv<P: AsRef<Path>>(path: P) -> Result<Vec<FirewallRule>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    let mut rules = Vec::new();

    for result in reader.deserialize() {
        let record: FirewallRuleCsvRecord = result?;
        rules.push(FirewallRule::from(record));
    }

    Ok(rules)
}

/// Read firewall rules from a CSV file with enhanced validation
pub fn read_firewall_rules_csv_validated<P: AsRef<Path>>(path: P) -> Result<Vec<FirewallRule>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    let mut rules = Vec::new();
    let mut line_number = 1; // Start at 1 for header

    for result in reader.deserialize() {
        line_number += 1;
        let record: FirewallRuleCsvRecord = result.map_err(|e| {
            crate::model::ConfigError::validation(format!(
                "Firewall rule CSV parsing error at line {line_number}: {e}"
            ))
        })?;

        // Validate the converted FirewallRule
        let rule = FirewallRule::from(record);

        // Additional validation for CSV-loaded data
        if rule.rule_id.is_empty() {
            return Err(crate::model::ConfigError::validation(format!(
                "Empty rule ID at line {}",
                line_number
            )));
        }

        if let Some(vid) = rule.vlan_id {
            if !(10..=4094).contains(&vid) {
                return Err(crate::model::ConfigError::validation(format!(
                    "Invalid VLAN ID {} at line {}: must be between 10 and 4094",
                    vid, line_number
                )));
            }
        }

        // Validate action
        let valid_actions = ["pass", "block", "reject"];
        if !valid_actions.contains(&rule.action.to_lowercase().as_str()) {
            return Err(crate::model::ConfigError::validation(format!(
                "Invalid action '{}' at line {}: must be one of {:?}",
                rule.action, line_number, valid_actions
            )));
        }

        // Validate direction
        let valid_directions = ["in", "out"];
        if !valid_directions.contains(&rule.direction.to_lowercase().as_str()) {
            return Err(crate::model::ConfigError::validation(format!(
                "Invalid direction '{}' at line {}: must be one of {:?}",
                rule.direction, line_number, valid_directions
            )));
        }

        // Validate protocol
        let valid_protocols = ["tcp", "udp", "icmp", "any"];
        if !valid_protocols.contains(&rule.protocol.to_lowercase().as_str()) {
            return Err(crate::model::ConfigError::validation(format!(
                "Invalid protocol '{}' at line {}: must be one of {:?}",
                rule.protocol, line_number, valid_protocols
            )));
        }

        rules.push(rule);
    }

    Ok(rules)
}

/// Read VLAN configurations from CSV with streaming for large files
pub fn read_csv_streaming<P: AsRef<Path>, F>(path: P, mut callback: F) -> Result<usize>
where
    F: FnMut(VlanConfig) -> Result<()>,
{
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    let mut count = 0;

    for result in reader.deserialize() {
        let record: CsvRecord = result?;
        let config = VlanConfig::from(record);
        callback(config)?;
        count += 1;
    }

    Ok(count)
}

/// Write VLAN configurations to CSV with streaming for large datasets
pub fn write_csv_streaming<P, I>(configs: I, path: P) -> Result<usize>
where
    I: Iterator<Item = VlanConfig>,
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);
    let mut count = 0;

    for config in configs {
        let record = CsvRecord::from(&config);
        writer.serialize(record)?;
        count += 1;
    }

    writer.flush()?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_round_trip() {
        let original_configs = vec![
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap(),
            VlanConfig::new(200, "10.3.4.x".to_string(), "Test VLAN 200".to_string(), 2).unwrap(),
        ];

        // Write to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        write_csv(&original_configs, temp_file.path()).unwrap();

        // Read back from file
        let read_configs = read_csv(temp_file.path()).unwrap();

        assert_eq!(original_configs.len(), read_configs.len());
        for (original, read) in original_configs.iter().zip(read_configs.iter()) {
            assert_eq!(original.vlan_id, read.vlan_id);
            assert_eq!(original.ip_network, read.ip_network);
            assert_eq!(original.description, read.description);
            assert_eq!(original.wan_assignment, read.wan_assignment);
        }
    }

    #[test]
    fn test_csv_record_conversion() {
        let config =
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap();

        let record = CsvRecord::from(&config);
        assert_eq!(record.vlan_id, 100);
        assert_eq!(record.ip_range, "10.1.2.x");
        assert_eq!(record.description, "Test VLAN 100");
        assert_eq!(record.wan_assignment, 1);

        let converted_config = VlanConfig::from(record);
        assert_eq!(config.vlan_id, converted_config.vlan_id);
        assert_eq!(config.ip_network, converted_config.ip_network);
        assert_eq!(config.description, converted_config.description);
        assert_eq!(config.wan_assignment, converted_config.wan_assignment);
    }

    #[test]
    fn test_csv_header_exact() {
        let configs =
            vec![VlanConfig::new(10, "10.0.0.x".to_string(), "H".to_string(), 1).unwrap()];
        let tf = NamedTempFile::new().unwrap();
        write_csv(&configs, tf.path()).unwrap();
        let content = std::fs::read_to_string(tf.path()).unwrap();
        let first_line = content.lines().next().unwrap();
        assert_eq!(first_line, "VLAN,IP Range,Beschreibung,WAN");
    }

    #[test]
    fn test_csv_validated_reading() {
        let configs = vec![
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap(),
            VlanConfig::new(200, "10.3.4.x".to_string(), "Test VLAN 200".to_string(), 2).unwrap(),
        ];

        // Write to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        write_csv(&configs, temp_file.path()).unwrap();

        // Read back using validated reader
        let read_configs = read_csv_validated(temp_file.path()).unwrap();

        assert_eq!(configs.len(), read_configs.len());
        for (original, read) in configs.iter().zip(read_configs.iter()) {
            assert_eq!(original.vlan_id, read.vlan_id);
            assert_eq!(original.ip_network, read.ip_network);
            assert_eq!(original.description, read.description);
            assert_eq!(original.wan_assignment, read.wan_assignment);
        }
    }

    #[test]
    fn test_csv_validated_reading_invalid_vlan_id() {
        // Create a CSV with invalid VLAN ID
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(
            temp_file.path(),
            "VLAN,IP Range,Beschreibung,WAN\n5,10.1.2.x,Invalid VLAN,1\n",
        )
        .unwrap();

        let result = read_csv_validated(temp_file.path());
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid VLAN ID 5"));
        assert!(error_msg.contains("line 2"));
    }

    #[test]
    fn test_csv_validated_reading_invalid_wan() {
        // Create a CSV with invalid WAN assignment
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(
            temp_file.path(),
            "VLAN,IP Range,Beschreibung,WAN\n100,10.1.2.x,Test VLAN,5\n",
        )
        .unwrap();

        let result = read_csv_validated(temp_file.path());
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid WAN assignment 5"));
        assert!(error_msg.contains("line 2"));
    }

    #[test]
    fn test_csv_validated_reading_invalid_ip_format() {
        // Create a CSV with invalid IP network format
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(
            temp_file.path(),
            "VLAN,IP Range,Beschreibung,WAN\n100,10.1.2.1,Test VLAN,1\n",
        )
        .unwrap();

        let result = read_csv_validated(temp_file.path());
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid IP network format"));
        assert!(error_msg.contains("line 2"));
    }

    #[test]
    fn test_csv_streaming_read() {
        let configs = vec![
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap(),
            VlanConfig::new(200, "10.3.4.x".to_string(), "Test VLAN 200".to_string(), 2).unwrap(),
            VlanConfig::new(300, "10.5.6.x".to_string(), "Test VLAN 300".to_string(), 3).unwrap(),
        ];

        // Write to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        write_csv(&configs, temp_file.path()).unwrap();

        // Read back using streaming
        let mut read_configs = Vec::new();
        let count = read_csv_streaming(temp_file.path(), |config| {
            read_configs.push(config);
            Ok(())
        })
        .unwrap();

        assert_eq!(count, 3);
        assert_eq!(configs.len(), read_configs.len());
        for (original, read) in configs.iter().zip(read_configs.iter()) {
            assert_eq!(original.vlan_id, read.vlan_id);
            assert_eq!(original.ip_network, read.ip_network);
            assert_eq!(original.description, read.description);
            assert_eq!(original.wan_assignment, read.wan_assignment);
        }
    }

    #[test]
    fn test_csv_streaming_write() {
        let configs = vec![
            VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN 100".to_string(), 1).unwrap(),
            VlanConfig::new(200, "10.3.4.x".to_string(), "Test VLAN 200".to_string(), 2).unwrap(),
            VlanConfig::new(300, "10.5.6.x".to_string(), "Test VLAN 300".to_string(), 3).unwrap(),
        ];

        // Write using streaming
        let temp_file = NamedTempFile::new().unwrap();
        let count = write_csv_streaming(configs.clone().into_iter(), temp_file.path()).unwrap();

        assert_eq!(count, 3);

        // Read back and verify
        let read_configs = read_csv(temp_file.path()).unwrap();
        assert_eq!(configs.len(), read_configs.len());
        for (original, read) in configs.iter().zip(read_configs.iter()) {
            assert_eq!(original.vlan_id, read.vlan_id);
            assert_eq!(original.ip_network, read.ip_network);
            assert_eq!(original.description, read.description);
            assert_eq!(original.wan_assignment, read.wan_assignment);
        }
    }

    #[test]
    fn test_utf8_support() {
        let configs = vec![
            VlanConfig::new(
                100,
                "10.1.2.x".to_string(),
                "Büro VLAN 100 - Ümlaut Test".to_string(),
                1,
            )
            .unwrap(),
            VlanConfig::new(
                200,
                "10.3.4.x".to_string(),
                "Sales VLAN 200 - 日本語テスト".to_string(),
                2,
            )
            .unwrap(),
            VlanConfig::new(
                300,
                "10.5.6.x".to_string(),
                "Réseau VLAN 300 - Français".to_string(),
                3,
            )
            .unwrap(),
        ];

        // Write to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        write_csv(&configs, temp_file.path()).unwrap();

        // Read back and verify UTF-8 characters are preserved
        let read_configs = read_csv(temp_file.path()).unwrap();
        assert_eq!(configs.len(), read_configs.len());
        for (original, read) in configs.iter().zip(read_configs.iter()) {
            assert_eq!(original.vlan_id, read.vlan_id);
            assert_eq!(original.ip_network, read.ip_network);
            assert_eq!(original.description, read.description);
            assert_eq!(original.wan_assignment, read.wan_assignment);
        }

        // Verify the file content contains UTF-8 characters
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("Büro"));
        assert!(content.contains("Ümlaut"));
        assert!(content.contains("日本語テスト"));
        assert!(content.contains("Réseau"));
        assert!(content.contains("Français"));
    }

    #[test]
    fn test_python_csv_compatibility() {
        // Test reading the existing Python-generated CSV file
        let python_csv_path = "test_python.csv";

        if std::path::Path::new(python_csv_path).exists() {
            let result = read_csv(python_csv_path);
            assert!(
                result.is_ok(),
                "Should be able to read Python-generated CSV"
            );

            let configs = result.unwrap();
            assert!(!configs.is_empty(), "Should have configurations");

            // Verify all configurations are valid
            for config in &configs {
                assert!((10..=4094).contains(&config.vlan_id));
                assert!((1..=3).contains(&config.wan_assignment));
                assert!(
                    config.ip_network.ends_with(".x") || config.ip_network.ends_with(".0/24"),
                    "IP network should end with .x or .0/24"
                );
            }
        }
    }

    #[test]
    fn test_large_dataset_memory_efficiency() {
        // Test with a moderately large dataset to verify memory efficiency
        let configs: Vec<_> = (0..1000)
            .map(|i| {
                VlanConfig::new(
                    (i % 4085) as u16 + 10,
                    format!("10.{}.{}.x", (i % 254) + 1, ((i / 254) % 254) + 1),
                    format!("Test VLAN {}", i),
                    ((i % 3) + 1) as u8,
                )
                .unwrap()
            })
            .collect();

        // Write using streaming
        let temp_file = NamedTempFile::new().unwrap();
        let write_count =
            write_csv_streaming(configs.clone().into_iter(), temp_file.path()).unwrap();
        assert_eq!(write_count, 1000);

        // Read using streaming to avoid loading all into memory at once
        let mut read_count = 0;
        let stream_count = read_csv_streaming(temp_file.path(), |_config| {
            read_count += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(stream_count, 1000);
        assert_eq!(read_count, 1000);
    }
}
