//! CSV Parsing Tests with Proper CSV Crate Usage
//!
//! This module demonstrates proper CSV parsing using the csv crate instead of
//! brittle manual string splitting that breaks on quoted fields and commas.

use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a test CSV file with firewall rules
fn create_test_firewall_csv(temp_dir: &TempDir) -> std::path::PathBuf {
    let csv_content = r#"rule_id,action,interface,protocol,source,destination,ports,description,vlan_id,priority,log
rule_001,pass,vlan100,tcp,192.168.1.0/24,any,80,Allow web traffic,100,1,true
rule_002,block,vlan101,udp,10.0.1.0/24,any,53,Block DNS,101,2,false
rule_003,pass,vlan102,tcp,172.16.1.0/24,any,"80,443",Allow HTTPS,102,1,true
"rule_004",pass,vlan103,tcp,"192.168.2.0/24,10.0.2.0/24",any,22,Allow SSH,103,3,true
"rule_005",block,vlan104,icmp,any,any,,Block ping,104,1,false"#;

    let csv_path = temp_dir.path().join("firewall_rules.csv");
    fs::write(&csv_path, csv_content).unwrap();
    csv_path
}

/// Helper function to create an empty CSV file (just header)
fn create_empty_firewall_csv(temp_dir: &TempDir) -> std::path::PathBuf {
    let csv_content = r#"rule_id,action,interface,protocol,source,destination,ports,description,vlan_id,priority,log"#;

    let csv_path = temp_dir.path().join("empty_firewall_rules.csv");
    fs::write(&csv_path, csv_content).unwrap();
    csv_path
}

#[test]
fn test_csv_parsing_with_proper_csv_crate() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = create_test_firewall_csv(&temp_dir);

    // Use csv::ReaderBuilder for flexible parsing
    let mut reader = ReaderBuilder::new()
        .flexible(true) // Allow flexible quoting
        .from_path(&csv_path)
        .unwrap();

    // Read headers to find column indices
    let headers = reader.headers().unwrap().clone();
    let vlan_id_index = headers
        .iter()
        .position(|h| h == "vlan_id")
        .expect("vlan_id column not found");

    // Parse records using the csv crate
    let mut vlan_rule_counts: HashMap<u16, u32> = HashMap::new();
    let mut total_rules = 0;

    for result in reader.records() {
        let record = result.unwrap();
        total_rules += 1;

        // Extract vlan_id using the found index
        let vlan_id_str = record.get(vlan_id_index).unwrap();
        let vlan_id: u16 = vlan_id_str.parse().unwrap();

        *vlan_rule_counts.entry(vlan_id).or_insert(0) += 1;
    }

    // Verify parsing results
    assert_eq!(total_rules, 5);
    assert_eq!(vlan_rule_counts.get(&100), Some(&1));
    assert_eq!(vlan_rule_counts.get(&101), Some(&1));
    assert_eq!(vlan_rule_counts.get(&102), Some(&1));
    assert_eq!(vlan_rule_counts.get(&103), Some(&1));
    assert_eq!(vlan_rule_counts.get(&104), Some(&1));
}

#[test]
fn test_csv_parsing_with_quoted_fields() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = create_test_firewall_csv(&temp_dir);

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_path(&csv_path)
        .unwrap();

    let headers = reader.headers().unwrap().clone();
    let source_index = headers
        .iter()
        .position(|h| h == "source")
        .expect("source column not found");

    // Parse records and verify quoted fields are handled correctly
    for result in reader.records() {
        let record = result.unwrap();
        if let Some(source) = record.get(source_index) {
            // Verify that quoted fields with commas are parsed correctly
            if source.contains(",") {
                // This should be a quoted field like "192.168.2.0/24,10.0.2.0/24"
                assert!(source.contains("192.168.2.0/24"));
                assert!(source.contains("10.0.2.0/24"));
            }
        }
    }
}

#[test]
fn test_empty_csv_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = create_empty_firewall_csv(&temp_dir);

    // Read the file content
    let content = fs::read_to_string(&csv_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    // Accept either an empty file or a header-only file
    let is_valid_empty_file =
        lines.is_empty() || (lines.len() == 1 && lines[0].starts_with("rule_id,action,interface"));

    assert!(
        is_valid_empty_file,
        "File should be either empty or contain only a header row"
    );

    // Test with csv crate to ensure it handles empty files gracefully
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_path(&csv_path)
        .unwrap();

    let mut record_count = 0;
    for _result in reader.records() {
        record_count += 1;
    }

    assert_eq!(record_count, 0, "Empty CSV should have no data records");
}

#[test]
fn test_csv_parsing_by_header_name() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = create_test_firewall_csv(&temp_dir);

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_path(&csv_path)
        .unwrap();

    // Read headers to find column indices
    let headers = reader.headers().unwrap().clone();
    let vlan_id_index = headers
        .iter()
        .position(|h| h == "vlan_id")
        .expect("vlan_id column not found");

    // Parse records using header names instead of indices
    let mut vlan_rule_counts: HashMap<u16, u32> = HashMap::new();

    for result in reader.records() {
        let record = result.unwrap();

        if let Some(vlan_id_str) = record.get(vlan_id_index)
            && let Ok(vlan_id) = vlan_id_str.parse::<u16>()
        {
            *vlan_rule_counts.entry(vlan_id).or_insert(0) += 1;
        }
    }

    // Verify results
    assert_eq!(vlan_rule_counts.len(), 5);
    assert_eq!(vlan_rule_counts.get(&100), Some(&1));
    assert_eq!(vlan_rule_counts.get(&101), Some(&1));
    assert_eq!(vlan_rule_counts.get(&102), Some(&1));
    assert_eq!(vlan_rule_counts.get(&103), Some(&1));
    assert_eq!(vlan_rule_counts.get(&104), Some(&1));
}

#[test]
fn test_csv_parsing_error_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create a malformed CSV file
    let malformed_csv = r#"rule_id,action,interface,protocol,source,destination,ports,description,vlan_id,priority,log
rule_001,pass,vlan100,tcp,192.168.1.0/24,any,80,Allow web traffic,100,1,true
rule_002,block,vlan101,udp,10.0.1.0/24,any,53,Block DNS,101,2,false
rule_003,pass,vlan102,tcp,172.16.1.0/24,any,"80,443",Allow HTTPS,102,1,true
rule_004,pass,vlan103,tcp,"192.168.2.0/24,10.0.2.0/24",any,22,Allow SSH,103,3,true
rule_005,block,vlan104,icmp,any,any,,Block ping,104,1,false
rule_006,pass,vlan105,tcp,192.168.3.0/24,any,80,Invalid rule with missing fields"#; // Missing fields

    let csv_path = temp_dir.path().join("malformed_firewall_rules.csv");
    fs::write(&csv_path, malformed_csv).unwrap();

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_path(&csv_path)
        .unwrap();

    // Read headers to find column indices
    let headers = reader.headers().unwrap().clone();
    let vlan_id_index = headers
        .iter()
        .position(|h| h == "vlan_id")
        .expect("vlan_id column not found");

    let mut valid_records = 0;
    let mut invalid_records = 0;

    for result in reader.records() {
        match result {
            Ok(record) => {
                // Try to parse vlan_id using index
                if let Some(vlan_id_str) = record.get(vlan_id_index) {
                    if let Ok(_vlan_id) = vlan_id_str.parse::<u16>() {
                        valid_records += 1;
                    } else {
                        invalid_records += 1;
                    }
                } else {
                    invalid_records += 1;
                }
            }
            Err(_) => {
                invalid_records += 1;
            }
        }
    }

    // Should have 5 valid records and 1 invalid record
    assert_eq!(valid_records, 5);
    assert_eq!(invalid_records, 1);
}
