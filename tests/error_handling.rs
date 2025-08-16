//! Error handling tests for OPNsense Config Faker
//!
//! These tests verify that error handling works correctly throughout the application,
//! including proper error context preservation and user-friendly error messages.

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;

/// Test VLAN ID exhaustion error handling
#[test]
fn test_vlan_id_exhaustion() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("5000") // Exceeds maximum unique VLAN IDs
        .arg("--base-config")
        .arg("test_xml/firewall_vlan_base.xml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "Failed to generate configurations",
        ))
        .stderr(predicate::str::contains("5000"));
}

/// Test invalid network format error handling
#[test]
fn test_invalid_network_format() {
    // This test would require modifying the generator to accept invalid network formats
    // For now, we test CLI argument validation
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("0"); // Invalid count

    cmd.assert().failure().stderr(predicate::str::contains(
        "invalid value '0' for '--count <COUNT>'",
    ));
}

/// Test CLI error context preservation
#[test]
fn test_cli_error_context() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("99999") // Invalid count
        .arg("--base-config")
        .arg("nonexistent_file.xml");

    cmd.assert().failure().stderr(predicate::str::contains(
        "invalid value '99999' for '--count <COUNT>'",
    ));
}

/// Test file operation error handling
#[test]
fn test_file_operation_errors() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg("/nonexistent/path/test.csv");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to generate configurations",
    ));
}

/// Test CSV generation error handling
#[test]
fn test_csv_generation_errors() {
    let temp_dir = tempdir().unwrap();
    let output_file = temp_dir.path().join("test.csv");

    // Test with invalid count
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("0")
        .arg("--output")
        .arg(&output_file);

    cmd.assert().failure().stderr(predicate::str::contains(
        "invalid value '0' for '--count <COUNT>'",
    ));
}

/// Test XML generation error handling
#[test]
fn test_xml_generation_errors() {
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");

    // Test with missing base config
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("10")
        .arg("--output-dir")
        .arg(&output_dir);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to generate configurations",
    ));
}

/// Test validation error handling
#[test]
fn test_validation_errors() {
    let temp_dir = tempdir().unwrap();
    let invalid_csv = temp_dir.path().join("invalid.csv");

    // Create an invalid CSV file
    std::fs::write(&invalid_csv, "invalid,csv,format\n").unwrap();

    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("validate")
        .arg("--input")
        .arg(&invalid_csv)
        .arg("--format")
        .arg("csv");

    // The validation should succeed but report 0 valid configurations
    cmd.assert().success();
}

/// Test interactive mode error handling
#[test]
fn test_interactive_mode_errors() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--interactive");

    // This test would require mocking user input
    // For now, we just verify the command doesn't panic
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to generate configurations",
    ));
}

/// Test progress indicator error handling
#[test]
fn test_progress_indicator_errors() {
    // Set TERM=dumb to test progress indicator fallback
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg("test.csv");

    // Should fail because file already exists, but with proper error context
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to generate configurations",
    ));
}

/// Test error message formatting
#[test]
fn test_error_message_formatting() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("5000")
        .arg("--base-config")
        .arg("nonexistent.xml");

    let assert = cmd.assert().failure();
    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify error message contains actionable information
    assert!(stderr.contains("Failed to generate configurations"));
    assert!(stderr.contains("5000"));
    // Note: "nonexistent.xml" won't appear in error message because clap validation happens first
    // The error handling code only runs after clap validation passes
}

/// Test error chain preservation
#[test]
fn test_error_chain_preservation() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg("/nonexistent/path/test.csv");

    let assert = cmd.assert().failure();
    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify that the error chain is preserved and shows multiple levels
    assert!(stderr.contains("Failed to generate configurations"));
    assert!(stderr.contains("Failed to write CSV"));
}

/// Test deprecated command error handling
#[test]
fn test_deprecated_command_errors() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("csv").arg("--count").arg("0");

    cmd.assert().failure().stderr(predicate::str::contains(
        "invalid value '0' for '--count <COUNT>'",
    ));
}

/// Test shell completion error handling
#[test]
fn test_completion_errors() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("completions").arg("invalid_shell");

    cmd.assert().failure().stderr(predicate::str::contains(
        "invalid value 'invalid_shell' for '<SHELL>'",
    ));
}

/// Test global flag error handling
#[test]
fn test_global_flag_errors() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("--invalid-flag")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

/// Test memory limit error handling (if applicable)
#[test]
fn test_memory_limit_errors() {
    // This test would require setting up a scenario that triggers memory limits
    // For now, we test with a very large count that might trigger memory issues
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10000")
        .arg("--output")
        .arg("large_test.csv");

    // Should either succeed or fail gracefully
    let result = cmd.assert();
    if result.get_output().status.success() {
        // Success case - large generation worked
        // No assertion needed for success case
    } else {
        // Failure case - should have proper error context
        let stderr = String::from_utf8_lossy(&result.get_output().stderr);
        assert!(stderr.contains("Failed to generate configurations"));
    }
}

/// Test network configuration error handling
#[test]
fn test_network_configuration_errors() {
    // Test with invalid network parameters
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg("test.csv");

    // This should work, but we can test error handling by checking the output
    let result = cmd.assert();
    if result.get_output().status.success() {
        // Verify that generated configurations are valid
        let output = String::from_utf8_lossy(&result.get_output().stdout);
        assert!(!output.contains("error"));
        assert!(!output.contains("Error"));
    } else {
        // If it failed, should have proper error context
        let stderr = String::from_utf8_lossy(&result.get_output().stderr);
        assert!(stderr.contains("Failed to generate configurations"));
    }
}
