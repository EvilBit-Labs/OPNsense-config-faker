//! Integration tests for OPNsense Config Faker
//!
//! These tests focus on core functionality and basic CLI operations.
//! For comprehensive CLI testing with ANSI/color hardening, see integration_cli.rs.

mod common;

use assert_cmd::Command;
use common::{cli_command, create_temp_dir, TestOutputExt};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_generate_csv_command_help() {
    let output = cli_command().arg("generate").arg("--help").run_success();

    output.assert_stdout_contains("Generate");
}

#[test]
fn test_completions_command_help() {
    let output = cli_command().arg("completions").arg("--help").run_success();

    output.assert_stdout_contains("Generate shell completions");
}

#[test]
fn test_csv_generation_new_format() {
    let temp_dir = create_temp_dir("csv_gen_test");
    let output_file = temp_dir.path().join("test_output.csv");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Verify success message appears in normalized output
    let normalized = output.normalized_stdout();
    assert!(
        normalized.contains("Generated 5 VLAN configurations")
            || normalized.contains("5 VLAN configurations")
            || (normalized.contains("Configurations: 5") && normalized.contains("Summary")),
        "Expected success message about generating 5 VLANs, got: {}",
        normalized
    );

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

// Legacy command structure tests - these should provide deprecation messages

#[test]
fn test_csv_command_help() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("csv").arg("--help");
    cmd.assert().success(); // Help should work for migration guidance
}

#[test]
fn test_xml_command_help() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("xml").arg("--help");
    cmd.assert().success(); // Help should work for migration guidance
}

#[test]
fn test_csv_generation() {
    let temp_dir = create_temp_dir("deprecated_csv_test");
    let output_file = temp_dir.path().join("test_output.csv");

    let output = cli_command()
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42")
        .run_failure();

    // Use normalized combined output for stable assertions regardless of ANSI
    let normalized = output.normalized_combined();
    assert!(normalized.contains("DEPRECATED COMMAND"));
    assert!(normalized.contains("generate --format csv"));
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

    let output = cmd.assert().failure().get_output().clone();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should show deprecation message (could be in stdout or stderr)
    let combined_output = format!("{stdout}{stderr}");
    assert!(combined_output.contains("DEPRECATED COMMAND"));
    assert!(combined_output.contains("generate --format csv"));
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

    let output = cmd.assert().failure().get_output().clone();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should show deprecation message (could be in stdout or stderr)
    let combined_output = format!("{stdout}{stderr}");
    assert!(combined_output.contains("DEPRECATED COMMAND"));
    assert!(combined_output.contains("generate --format csv"));
}
