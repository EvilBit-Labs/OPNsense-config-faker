//! CSV input/output operations

use crate::generator::VlanConfig;
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
}
