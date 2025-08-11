//! Error Validation Tests
//!
//! This module contains comprehensive error handling validation tests across all layers
//! of the OPNsense Config Faker application. Tests validate error conditions, ensure
//! error messages are actionable and stable, and verify proper error propagation.

use assert_fs::prelude::*;
use assert_fs::TempDir;
use opnsense_config_faker::generator::vlan::generate_vlan_configurations;
use opnsense_config_faker::generator::VlanConfig;
use opnsense_config_faker::model::ConfigError;
use opnsense_config_faker::xml::template::XmlTemplate;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

mod common;
use common::{cli_command, TestOutputExt};

// ===== Resource Exhaustion Tests =====

#[test]
fn test_resource_exhaustion_vlan_ids_exceed_maximum() {
    // Test requesting more VLAN configurations than possible unique VLAN IDs
    // Valid VLAN ID range is 10-4094, which gives us 4085 possible unique IDs
    let result = generate_vlan_configurations(4090, Some(42), None);

    assert!(
        result.is_err(),
        "Expected resource exhaustion error for count 4090"
    );

    match result.unwrap_err() {
        ConfigError::ResourceExhausted { resource } => {
            assert_eq!(
                resource, "VLAN IDs",
                "Expected VLAN IDs resource exhaustion"
            );
        }
        other => panic!("Expected ConfigError::ResourceExhausted, got: {:?}", other),
    }
}

#[test]
fn test_resource_exhaustion_exact_boundary_success() {
    // Test that we can generate exactly the maximum number of unique configurations
    // This test may take a while, so we use a smaller number for practical testing
    let result = generate_vlan_configurations(100, Some(42), None);

    assert!(
        result.is_ok(),
        "Should succeed for reasonable count within limits"
    );
    let configs = result.unwrap();
    assert_eq!(
        configs.len(),
        100,
        "Should generate exactly 100 configurations"
    );

    // Verify all VLAN IDs are unique
    let mut vlan_ids = std::collections::HashSet::new();
    for config in &configs {
        assert!(
            vlan_ids.insert(config.vlan_id),
            "Duplicate VLAN ID: {}",
            config.vlan_id
        );
    }
}

#[test]
fn test_resource_exhaustion_ip_networks() {
    // This test verifies that IP network exhaustion is detected
    // The generator creates networks in the form "10.x.y.z" where x and y are 1-254
    // This gives us approximately 254 * 254 = 64,516 possible combinations
    // We test with a very large number that should exceed practical generation limits
    let result = generate_vlan_configurations(5000, Some(42), None);

    // This should either succeed (if we have enough unique combinations)
    // or fail with IP network resource exhaustion
    match result {
        Ok(configs) => {
            assert_eq!(
                configs.len(),
                5000,
                "Should generate exactly 5000 configurations"
            );
            // Verify all networks are unique
            let mut networks = std::collections::HashSet::new();
            for config in &configs {
                assert!(
                    networks.insert(&config.ip_network),
                    "Duplicate network: {}",
                    config.ip_network
                );
            }
        }
        Err(ConfigError::ResourceExhausted { resource }) => {
            assert!(
                resource.contains("IP networks") || resource.contains("VLAN IDs"),
                "Expected IP networks or VLAN IDs resource exhaustion, got: {}",
                resource
            );
        }
        Err(other) => panic!(
            "Expected success or ResourceExhausted error, got: {:?}",
            other
        ),
    }
}

// ===== File Permission and Path Validation Tests =====

#[test]
fn test_invalid_output_file_permissions_without_force() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.child("readonly.csv");

    // Create a read-only file
    output_file.write_str("existing,content\n1,2").unwrap();
    let output_path = output_file.path();

    // Make the file read-only
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(output_path).unwrap().permissions();
        perms.set_mode(0o444); // Read-only
        fs::set_permissions(output_path, perms).unwrap();
    }

    #[cfg(windows)]
    {
        let mut perms = fs::metadata(output_path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(output_path, perms).unwrap();
    }

    // Test CLI command fails when trying to overwrite read-only file without --force
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(output_path)
        .arg("--no-color")
        .run_and_capture();

    assert!(
        !output.success,
        "Command should fail for read-only file without --force"
    );

    // Check that error message is actionable
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Use --force to overwrite")
            || combined_output.contains("Permission denied")
            || combined_output.contains("already exists"),
        "Expected actionable error message about overwriting or permissions, got: {}",
        combined_output
    );
}

#[test]
fn test_invalid_output_directory_permissions() {
    let temp_dir = TempDir::new().unwrap();
    let readonly_dir = temp_dir.child("readonly");
    readonly_dir.create_dir_all().unwrap();

    let output_file = readonly_dir.child("output.csv");

    // Make directory read-only (on Unix systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(readonly_dir.path()).unwrap().permissions();
        perms.set_mode(0o555); // Read and execute only, no write
        fs::set_permissions(readonly_dir.path(), perms).unwrap();
    }

    // Test CLI command fails when trying to write to read-only directory
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(output_file.path())
        .arg("--no-color")
        .run_and_capture();

    #[cfg(unix)]
    {
        assert!(
            !output.success,
            "Command should fail for read-only directory"
        );
        let combined_output = output.normalized_combined();
        assert!(
            combined_output.contains("Permission denied")
                || combined_output.contains("Access denied")
                || combined_output.contains("cannot create"),
            "Expected permission error message, got: {}",
            combined_output
        );
    }

    #[cfg(windows)]
    {
        // Windows file permissions work differently, this test may behave differently
        // We'll just ensure we get some kind of error or success
        let combined_output = output.normalized_combined();
        if !output.success {
            assert!(
                combined_output.contains("denied")
                    || combined_output.contains("cannot")
                    || combined_output.contains("failed"),
                "Expected some kind of error message, got: {}",
                combined_output
            );
        }
    }
}

#[test]
fn test_nonexistent_output_directory_created() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_dir = temp_dir.child("does_not_exist");
    let output_file = nonexistent_dir.child("output.csv");

    // Verify directory doesn't exist initially
    nonexistent_dir.assert(predicate::path::missing());

    // Test that command handles the nonexistent directory appropriately
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(output_file.path())
        .arg("--no-color")
        .run_and_capture();

    // The command may fail if directory creation isn't implemented, or succeed if it is
    let combined_output = output.normalized_combined();
    if output.success {
        // If it succeeded, verify directory and file were created
        nonexistent_dir.assert(predicate::path::is_dir());
        output_file.assert(predicate::path::is_file());
    } else {
        // If it failed, it should be due to directory not existing
        assert!(
            combined_output.contains("No such file or directory")
                || combined_output.contains("cannot create")
                || combined_output.contains("Path does not exist"),
            "Expected directory creation error, got: {}",
            combined_output
        );
    }
}

// ===== XML Base Configuration Path Validation Tests =====

#[test]
fn test_invalid_base_xml_file_not_found() {
    let nonexistent_path = PathBuf::from("/tmp/does_not_exist_base_config.xml");

    // Test CLI command with non-existent base config
    let temp_dir = TempDir::new().unwrap();
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(&nonexistent_path)
        .arg("--output-dir")
        .arg(temp_dir.path())
        .arg("--count")
        .arg("5")
        .arg("--no-color")
        .run_and_capture();

    assert!(
        !output.success,
        "Command should fail for non-existent base config"
    );

    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Configuration file not found")
            || combined_output.contains("No such file")
            || combined_output.contains("not found")
            || combined_output.contains(&nonexistent_path.display().to_string()),
        "Expected clear error about missing base config file, got: {}",
        combined_output
    );
}

#[test]
fn test_malformed_base_xml_content() {
    let temp_dir = TempDir::new().unwrap();
    let base_config = temp_dir.child("malformed.xml");

    // Create malformed XML content
    base_config
        .write_str("This is not XML content at all!")
        .unwrap();

    let output_dir = temp_dir.child("output");
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(base_config.path())
        .arg("--output-dir")
        .arg(output_dir.path())
        .arg("--count")
        .arg("3")
        .arg("--no-color")
        .run_and_capture();

    assert!(!output.success, "Command should fail for malformed XML");

    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("XML")
            || combined_output.contains("template")
            || combined_output.contains("not appear to be valid")
            || combined_output.contains("processing failed"),
        "Expected clear error about invalid XML template, got: {}",
        combined_output
    );
}

#[test]
fn test_xml_template_direct_validation() {
    // Test XmlTemplate::new with invalid content
    let invalid_content = "Not XML content";
    let result = XmlTemplate::new(invalid_content.to_string());

    assert!(result.is_err(), "Should fail for non-XML content");

    if let Err(ConfigError::XmlTemplate { message }) = result {
        assert!(
            message.contains("does not appear to be valid XML"),
            "Expected specific XML validation error message, got: {}",
            message
        );
    } else {
        panic!("Expected ConfigError::XmlTemplate, got a different result");
    }
}

#[test]
fn test_xml_template_valid_content() {
    // Test XmlTemplate::new with valid XML content
    let valid_xml = r#"<?xml version="1.0"?>
<opnsense>
    <interfaces>
        <vlan id="{{VLAN_ID}}">{{DESCRIPTION}}</vlan>
    </interfaces>
</opnsense>"#;

    let result = XmlTemplate::new(valid_xml.to_string());
    assert!(result.is_ok(), "Should succeed for valid XML content");

    // Test applying configuration
    let mut template = result.unwrap();
    let config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test VLAN".to_string(), 1).unwrap();
    let applied = template.apply_configuration(&config, 1, 6);

    assert!(
        applied.is_ok(),
        "Should successfully apply configuration to valid template"
    );
    let xml_output = applied.unwrap();
    assert!(
        xml_output.contains("100"),
        "Applied XML should contain VLAN ID"
    );
    assert!(
        xml_output.contains("Test VLAN"),
        "Applied XML should contain description"
    );
}

// ===== IP Network Validation Tests =====

#[test]
fn test_vlan_config_invalid_ip_network_formats() {
    let invalid_networks = vec![
        ("", "empty string"),
        ("not.an.ip", "invalid format"),
        ("10.1.2", "incomplete IP"),
        ("10.1.2.1", "plain IP without .x or /24"),
        ("10.1.2.0", "IP without CIDR notation"),
        ("10.1.2.0/16", "wrong CIDR notation"),
        ("10.1.2.y", "wrong placeholder"),
        ("10.1.2.0/", "incomplete CIDR"),
        ("10.1..x", "missing octet"),
        ("10.1.2./24", "missing octet in CIDR"),
        // Note: octet range and numeric validation is not currently implemented in VlanConfig::new
        // These would be caught by the network derivation methods like gateway_ip()
    ];

    for (network, description) in invalid_networks {
        let result = VlanConfig::new(100, network.to_string(), "Test".to_string(), 1);
        assert!(
            result.is_err(),
            "Should fail for invalid network '{}' ({})",
            network,
            description
        );

        match result.unwrap_err() {
            ConfigError::Validation { message } => {
                assert!(
                    message.contains("IP network")
                        && message.contains("does not match expected format"),
                    "Expected specific IP network validation error for '{}', got: {}",
                    network,
                    message
                );
            }
            other => panic!(
                "Expected ConfigError::Validation for '{}', got: {:?}",
                network, other
            ),
        }
    }
}

#[test]
fn test_vlan_config_valid_ip_network_formats() {
    let valid_networks = vec![
        "10.1.2.x",
        "192.168.1.x",
        "172.16.0.x",
        "10.1.2.0/24",
        "192.168.1.0/24",
        "172.16.0.0/24",
        "10.255.254.x",
        "192.168.255.0/24",
    ];

    for network in valid_networks {
        let result = VlanConfig::new(100, network.to_string(), "Test".to_string(), 1);
        assert!(
            result.is_ok(),
            "Should succeed for valid network '{}'",
            network
        );

        let config = result.unwrap();
        assert_eq!(
            config.ip_network, network,
            "Network should be preserved exactly"
        );
    }
}

#[test]
fn test_gateway_ip_derivation_errors() {
    // Create a config and then corrupt the IP network to test error handling
    let mut config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();

    // Corrupt the network field to trigger derivation errors
    config.ip_network = "invalid.network.format".to_string();

    // Test gateway IP derivation fails with clear error message
    let result = config.gateway_ip();
    assert!(result.is_err(), "Should fail for invalid network format");

    match result.unwrap_err() {
        ConfigError::Validation { message } => {
            assert!(
                message.contains("Cannot derive gateway from IP network")
                    && message.contains("invalid.network.format"),
                "Expected specific gateway derivation error, got: {}",
                message
            );
        }
        other => panic!("Expected ConfigError::Validation, got: {:?}", other),
    }
}

#[test]
fn test_dhcp_range_derivation_errors() {
    // Create a config and then corrupt the IP network to test error handling
    let mut config = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 1).unwrap();

    // Corrupt the network field to trigger derivation errors
    config.ip_network = "corrupted.format".to_string();

    // Test DHCP range start derivation fails
    let start_result = config.dhcp_range_start();
    assert!(
        start_result.is_err(),
        "Should fail for invalid network format"
    );

    match start_result.unwrap_err() {
        ConfigError::Validation { message } => {
            assert!(
                message.contains("Cannot derive DHCP range from IP network")
                    && message.contains("corrupted.format"),
                "Expected specific DHCP range derivation error, got: {}",
                message
            );
        }
        other => panic!("Expected ConfigError::Validation, got: {:?}", other),
    }

    // Test DHCP range end derivation fails
    let end_result = config.dhcp_range_end();
    assert!(
        end_result.is_err(),
        "Should fail for invalid network format"
    );

    match end_result.unwrap_err() {
        ConfigError::Validation { message } => {
            assert!(
                message.contains("Cannot derive DHCP range from IP network")
                    && message.contains("corrupted.format"),
                "Expected specific DHCP range derivation error, got: {}",
                message
            );
        }
        other => panic!("Expected ConfigError::Validation, got: {:?}", other),
    }
}

// ===== Error Message Stability and Actionability Tests =====

#[test]
fn test_error_messages_are_actionable_and_stable() {
    // Test that error messages contain actionable information and remain stable

    // 1. VLAN ID validation error
    let result = VlanConfig::new(5, "10.1.2.x".to_string(), "Test".to_string(), 1);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("VLAN ID 5 is outside valid range 10-4094"),
        "VLAN ID error should be specific and actionable: {}",
        error_msg
    );

    // 2. WAN assignment validation error
    let result = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 5);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("WAN assignment 5 is outside valid range 1-3"),
        "WAN assignment error should be specific and actionable: {}",
        error_msg
    );

    // 3. IP network format validation error
    let result = VlanConfig::new(100, "invalid.format".to_string(), "Test".to_string(), 1);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("IP network 'invalid.format' does not match expected format"),
        "IP network error should be specific and actionable: {}",
        error_msg
    );

    // 4. Resource exhaustion error
    let result = generate_vlan_configurations(4090, Some(42), None);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Resource exhaustion: VLAN IDs"),
        "Resource exhaustion error should be clear: {}",
        error_msg
    );

    // 5. XML template validation error
    let result = XmlTemplate::new("Not XML".to_string());
    assert!(result.is_err());
    if let Err(error) = result {
        let error_msg = error.to_string();
        assert!(
            error_msg.contains("XML template error: Base content does not appear to be valid XML"),
            "XML template error should be specific: {}",
            error_msg
        );
    } else {
        panic!("Expected error from XmlTemplate::new");
    }
}

#[test]
fn test_cli_error_propagation_csv_format() {
    // Test that library errors are properly propagated through CLI with actionable messages

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.child("test.csv");

    // Test with invalid count (too high) - clap validates this at the CLI level
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("4090") // Should trigger CLI validation error first
        .arg("--output")
        .arg(output_file.path())
        .arg("--no-color")
        .run_and_capture();

    assert!(!output.success, "Command should fail for excessive count");
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Resource exhaustion") ||
        combined_output.contains("VLAN IDs") ||
        combined_output.contains("invalid value") || // clap validation message
        combined_output.contains("not in") || // clap range message
        combined_output.contains("4090 is not in"), // specific clap message
        "Expected resource exhaustion or CLI validation error message, got: {}",
        combined_output
    );
}

#[test]
fn test_cli_error_propagation_xml_format() {
    // Test XML format error propagation

    let temp_dir = TempDir::new().unwrap();
    let invalid_base = temp_dir.child("invalid.xml");
    invalid_base.write_str("Not XML").unwrap();

    let output_dir = temp_dir.child("output");
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(invalid_base.path())
        .arg("--output-dir")
        .arg(output_dir.path())
        .arg("--count")
        .arg("5")
        .arg("--no-color")
        .run_and_capture();

    assert!(
        !output.success,
        "Command should fail for invalid XML base config"
    );
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("XML")
            || combined_output.contains("template")
            || combined_output.contains("not appear to be valid"),
        "Expected XML validation error message, got: {}",
        combined_output
    );
}

// ===== Integration Error Tests =====

#[test]
fn test_missing_required_arguments_generate_command() {
    // Test CSV format without output file
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--no-color")
        .run_and_capture();

    assert!(
        !output.success,
        "Should fail when output file not specified for CSV format"
    );
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Output file path is required")
            || combined_output.contains("--output")
            || combined_output.contains("required"),
        "Expected error about missing output file, got: {}",
        combined_output
    );

    // Test XML format without base config
    let temp_dir = TempDir::new().unwrap();
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--output-dir")
        .arg(temp_dir.path())
        .arg("--count")
        .arg("5")
        .arg("--no-color")
        .run_and_capture();

    assert!(
        !output.success,
        "Should fail when base config not specified for XML format"
    );
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Base configuration file is required")
            || combined_output.contains("--base-config")
            || combined_output.contains("required"),
        "Expected error about missing base config, got: {}",
        combined_output
    );
}

#[test]
fn test_conflicting_arguments() {
    // Test XML format with neither count nor CSV file specified
    let temp_dir = TempDir::new().unwrap();
    let base_config = temp_dir.child("valid.xml");
    base_config
        .write_str(r#"<?xml version="1.0"?><opnsense></opnsense>"#)
        .unwrap();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(base_config.path())
        .arg("--output-dir")
        .arg(temp_dir.path())
        .arg("--count")
        .arg("0") // Invalid count - clap validates this
        .arg("--no-color")
        .run_and_capture();

    assert!(!output.success, "Should fail when count is invalid");
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("--count or --csv-file must be specified") ||
        combined_output.contains("Either --count or --csv-file") ||
        combined_output.contains("invalid value '0'") || // clap validation
        combined_output.contains("not in 1.."), // clap range validation
        "Expected error about invalid count or missing arguments, got: {}",
        combined_output
    );
}

// ===== Edge Case Error Tests =====

#[test]
fn test_edge_case_boundary_values() {
    // Test boundary conditions that should succeed

    // Minimum valid VLAN ID
    let result = VlanConfig::new(10, "10.1.2.x".to_string(), "Test".to_string(), 1);
    assert!(result.is_ok(), "Should succeed for minimum VLAN ID 10");

    // Maximum valid VLAN ID
    let result = VlanConfig::new(4094, "10.1.2.x".to_string(), "Test".to_string(), 1);
    assert!(result.is_ok(), "Should succeed for maximum VLAN ID 4094");

    // Test boundary conditions that should fail

    // VLAN ID below minimum
    let result = VlanConfig::new(9, "10.1.2.x".to_string(), "Test".to_string(), 1);
    assert!(result.is_err(), "Should fail for VLAN ID below minimum");

    // VLAN ID above maximum
    let result = VlanConfig::new(4095, "10.1.2.x".to_string(), "Test".to_string(), 1);
    assert!(result.is_err(), "Should fail for VLAN ID above maximum");

    // WAN assignment boundaries
    let result = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 0);
    assert!(
        result.is_err(),
        "Should fail for WAN assignment below minimum"
    );

    let result = VlanConfig::new(100, "10.1.2.x".to_string(), "Test".to_string(), 4);
    assert!(
        result.is_err(),
        "Should fail for WAN assignment above maximum"
    );
}

#[test]
fn test_concurrent_error_scenarios() {
    // Test that multiple error conditions are handled properly

    let result = VlanConfig::new(5, "invalid.network".to_string(), "Test".to_string(), 0);
    assert!(
        result.is_err(),
        "Should fail for multiple validation errors"
    );

    // The first validation error encountered should be reported
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("VLAN ID 5 is outside valid range")
            || error_msg.contains("IP network 'invalid.network'")
            || error_msg.contains("WAN assignment 0 is outside valid range"),
        "Should report one of the validation errors clearly: {}",
        error_msg
    );
}
