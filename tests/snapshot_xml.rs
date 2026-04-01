//! Snapshot tests for XML output generation
//!
//! These tests capture deterministic XML outputs using fixed seeds and base configurations
//! to ensure consistent generation across runs and platforms.

mod common;

use common::{TestOutputExt, cli_command, create_temp_xml};
use insta::assert_snapshot;
use regex::Regex;
use std::fs;
use tempfile::TempDir;

/// Normalize temporary file paths in output for consistent snapshots
fn normalize_temp_paths(output: &str) -> String {
    let temp_path_regex = Regex::new(r"/[^/]*/[^/]*/T/[^\s]+").unwrap();
    temp_path_regex
        .replace_all(output, "<TEMP_PATH>")
        .to_string()
}

/// Create a minimal valid base config XML for testing
fn create_base_config_xml() -> &'static str {
    r#"<?xml version="1.0"?>
<opnsense version="24.1" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <system>
    <hostname>opnsense-{{FIREWALL_NR}}</hostname>
    <domain>test.local</domain>
  </system>
  <interfaces>
    <lan>
      <if>em0</if>
      <ipaddr>dhcp</ipaddr>
    </lan>
    <wan>
      <if>em1</if>
      <ipaddr>dhcp</ipaddr>
    </wan>
    <!-- VLAN interfaces will be added here -->
    <opt{{OPT_COUNTER}}>
      <if>em0.{{VLAN_ID}}</if>
      <ipaddr>{{GATEWAY_IP}}</ipaddr>
      <subnet>24</subnet>
      <descr>{{DESCRIPTION}}</descr>
    </opt{{OPT_COUNTER}}>
  </interfaces>
  <vlans>
    <vlan>
      <if>em0</if>
      <tag>{{VLAN_ID}}</tag>
      <descr>{{DESCRIPTION}}</descr>
    </vlan>
  </vlans>
  <dhcpd>
    <opt{{OPT_COUNTER}}>
      <enable>1</enable>
      <range>
        <from>{{DHCP_START}}</from>
        <to>{{DHCP_END}}</to>
      </range>
    </opt{{OPT_COUNTER}}>
  </dhcpd>
</opnsense>"#
}

/// Test XML generation with base config and deterministic seed
#[test]
fn test_xml_generation_deterministic() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path();

    // Create base config file
    let base_config_content = create_base_config_xml();
    let (base_config_file, base_config_path) = create_temp_xml("base_config_", base_config_content)
        .expect("Failed to create base config file");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("3")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--output-dir")
        .arg(temp_dir_path)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Snapshot the CLI output (normalize temp file paths)
    let stdout = normalize_temp_paths(&output.normalized_stdout());
    let stderr = normalize_temp_paths(&output.normalized_stderr());
    assert_snapshot!("xml_generation_stdout", stdout);
    assert_snapshot!("xml_generation_stderr", stderr);

    // Read generated XML files and snapshot a normalized subset
    let xml_files: Vec<_> = fs::read_dir(temp_dir_path)
        .expect("Failed to read output directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "xml")
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(xml_files.len(), 3, "Should have generated 3 XML files");

    // Sort files by name for consistent ordering
    let mut xml_files: Vec<_> = xml_files.into_iter().map(|entry| entry.path()).collect();
    xml_files.sort();

    // Read the first XML file and snapshot critical sections
    let first_xml_path = &xml_files[0];
    let xml_content =
        fs::read_to_string(first_xml_path).expect("Failed to read generated XML file");

    // Normalize line endings for cross-platform stability
    let normalized_xml = xml_content.replace("\r\n", "\n").replace('\r', "\n");

    // Extract and snapshot just the critical sections to avoid large file snapshots
    let vlan_section = extract_xml_section(&normalized_xml, "vlans");
    let interface_section = extract_xml_section(&normalized_xml, "interfaces");
    let dhcp_section = extract_xml_section(&normalized_xml, "dhcpd");

    assert_snapshot!("xml_generation_vlan_section", vlan_section);
    assert_snapshot!("xml_generation_interface_section", interface_section);
    assert_snapshot!("xml_generation_dhcp_section", dhcp_section);

    // Verify the XML contains expected placeholder replacements
    assert!(
        !normalized_xml.contains("{{VLAN_ID}}"),
        "VLAN_ID placeholder should be replaced"
    );
    assert!(
        !normalized_xml.contains("{{GATEWAY_IP}}"),
        "GATEWAY_IP placeholder should be replaced"
    );
    assert!(
        !normalized_xml.contains("{{DESCRIPTION}}"),
        "DESCRIPTION placeholder should be replaced"
    );

    drop(base_config_file);
}

/// Test XML generation with different firewall number and opt counter
#[test]
fn test_xml_generation_custom_parameters() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path();

    // Create base config file
    let base_config_content = create_base_config_xml();
    let (base_config_file, base_config_path) = create_temp_xml("base_config_", base_config_content)
        .expect("Failed to create base config file");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("2")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--output-dir")
        .arg(temp_dir_path)
        .arg("--firewall-nr")
        .arg("5")
        .arg("--opt-counter")
        .arg("10")
        .arg("--seed")
        .arg("123")
        .run_success();

    let stdout = normalize_temp_paths(&output.normalized_stdout());
    assert_snapshot!("xml_generation_custom_params_stdout", stdout);

    // Verify files were created with correct firewall number in filename
    let xml_files: Vec<_> = fs::read_dir(temp_dir_path)
        .expect("Failed to read output directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let filename = entry.file_name();
            let filename_str = filename.to_str().unwrap_or("");
            filename_str.starts_with("firewall_5_") && filename_str.ends_with(".xml")
        })
        .collect();

    assert_eq!(
        xml_files.len(),
        2,
        "Should have generated 2 XML files with firewall_5_ prefix"
    );

    drop(base_config_file);
}

/// Test XML generation with empty count (should fail validation)
#[test]
fn test_xml_generation_validation_error() {
    let base_config_content = create_base_config_xml();
    let (base_config_file, base_config_path) = create_temp_xml("base_config_", base_config_content)
        .expect("Failed to create base config file");

    // Test without count and without CSV file - should fail
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--count")
        .arg("0")
        .run_failure();

    assert_snapshot!(
        "xml_generation_validation_error_stderr",
        output.normalized_stderr()
    );

    drop(base_config_file);
}

/// Test XML generation force overwrite functionality
#[test]
fn test_xml_generation_force_overwrite() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path();

    // Create base config file
    let base_config_content = create_base_config_xml();
    let (base_config_file, base_config_path) = create_temp_xml("base_config_", base_config_content)
        .expect("Failed to create base config file");

    // First generation
    cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("1")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--output-dir")
        .arg(temp_dir_path)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Second generation with --force should succeed
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("1")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--output-dir")
        .arg(temp_dir_path)
        .arg("--seed")
        .arg("456")
        .arg("--force")
        .run_success();

    let stdout = normalize_temp_paths(&output.normalized_stdout());
    assert_snapshot!("xml_generation_force_overwrite_stdout", stdout);

    drop(base_config_file);
}

/// Helper function to extract a specific XML section for focused snapshots
fn extract_xml_section(xml_content: &str, section_name: &str) -> String {
    let pattern = format!(r"(?s)<{0}>(.*?)</{0}>", regex::escape(section_name));
    let regex = Regex::new(&pattern).expect("Invalid regex pattern");

    if let Some(captures) = regex.captures(xml_content)
        && let Some(section_match) = captures.get(1)
    {
        // Return the section with tags for context
        return format!(
            "<{}>{}</{}>",
            section_name,
            section_match.as_str(),
            section_name
        );
    }

    format!("Section '{section_name}' not found")
}
