//! Integration tests for OPNsense Config Faker

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_generate_csv_command_help() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_completions_command_help() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("completions").arg("--help");
    cmd.assert().success();
}

#[test]
fn test_csv_generation_new_format() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("test_output.csv");
    
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42");
    
    cmd.assert().success();
    
    // Verify file was created
    assert!(output_file.exists());
    
    // Verify file has content
    let content = fs::read_to_string(&output_file).unwrap();
    assert!(!content.is_empty());
    assert!(content.contains("VLAN"));
    assert!(content.contains("IP Range"));
    assert!(content.contains("Beschreibung"));
    assert!(content.contains("WAN"));
}

#[test]
fn test_csv_generation_with_force_new_format() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("test_output.csv");
    
    // Create file first
    fs::write(&output_file, "existing content").unwrap();
    
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--force")
        .arg("--seed")
        .arg("42");
    
    cmd.assert().success();
    
    // Verify file was overwritten
    let content = fs::read_to_string(&output_file).unwrap();
    assert_ne!(content, "existing content");
    assert!(content.contains("VLAN"));
}

#[test]
fn test_csv_generation_without_force_fails_new_format() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("test_output.csv");
    
    // Create file first
    fs::write(&output_file, "existing content").unwrap();
    
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42");
    
    cmd.assert().failure();
}

#[test]
fn test_completions_generation() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("completions").arg("bash");
    
    let output = cmd.assert().success().get_output().clone();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verify completion script contains expected content
    assert!(stdout.contains("_opnsense-config-faker"));
    assert!(stdout.contains("COMPREPLY"));
}

#[test]
fn test_missing_required_csv_output_fails() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5");
    
    cmd.assert().failure();
}

#[test]
fn test_missing_required_xml_base_config_fails() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("5");
    
    cmd.assert().failure();
}

// Legacy command structure tests (these should be removed once fully migrated)
// But keeping them here to ensure backward compatibility during transition

#[test]
fn test_csv_command_help() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("csv").arg("--help");
    cmd.assert().failure(); // Should fail now that csv subcommand is removed
}

#[test]
fn test_xml_command_help() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("xml").arg("--help");
    cmd.assert().failure(); // Should fail now that xml subcommand is removed
}

#[test]
fn test_csv_generation() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("test_output.csv");
    
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42");
    
    cmd.assert().failure(); // Should fail now that csv subcommand is removed
}

#[test]
fn test_csv_generation_with_force() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("test_output.csv");
    
    // Create file first
    fs::write(&output_file, "existing content").unwrap();
    
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--force")
        .arg("--seed")
        .arg("42");
    
    cmd.assert().failure(); // Should fail now that csv subcommand is removed
}

#[test]
fn test_csv_generation_without_force_fails() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("test_output.csv");
    
    // Create file first
    fs::write(&output_file, "existing content").unwrap();
    
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42");
    
    cmd.assert().failure(); // Should fail now that csv subcommand is removed
}