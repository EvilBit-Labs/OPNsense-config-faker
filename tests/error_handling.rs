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

/// Test interactive mode behavior (should succeed with defaults when no input)
#[test]
fn test_interactive_mode_behavior() {
    let temp_dir = tempdir().unwrap();
    let expected_output = temp_dir.path().join("vlan_configs.csv");

    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--interactive")
        .stdin(std::process::Stdio::null()); // No user input - should use defaults

    // Interactive mode should succeed with defaults when no input is provided
    cmd.assert().success().stdout(predicate::str::contains(
        "OPNsense Config Faker - Configuration Generator",
    ));

    // Verify output file was created with default name
    assert!(
        expected_output.exists(),
        "Default output file should be created"
    );
}

/// Test progress indicator error handling
#[test]
fn test_progress_indicator_errors() {
    let temp_dir = tempdir().unwrap();
    let output_file = temp_dir.path().join("test.csv");

    // Create the output file so it exists and triggers "file already exists" error
    std::fs::File::create(&output_file).unwrap();

    // Set TERM=dumb to test progress indicator fallback
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg(&output_file);

    // Should fail because file already exists, but with proper error context
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
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
    let temp_dir = tempdir().unwrap();
    let output_file = temp_dir.path().join("large_test.csv");

    // This test would require setting up a scenario that triggers memory limits
    // For now, we test with a very large count that might trigger memory issues
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb") // Ensure deterministic output
        .env("NO_COLOR", "1")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10000")
        .arg("--output")
        .arg(&output_file);

    // Should either succeed or fail gracefully
    let result = cmd.assert();
    if result.get_output().status.success() {
        // Success case - large generation worked
        // Verify the file was created in temp directory, not repository
        assert!(
            output_file.exists(),
            "Output file should be created in temp directory"
        );
        assert!(
            output_file.starts_with(temp_dir.path()),
            "File should be in temp directory"
        );
    } else {
        // Failure case - should have proper error context
        let stderr = String::from_utf8_lossy(&result.get_output().stderr);
        assert!(stderr.contains("Failed to generate configurations"));
    }

    // Explicit cleanup verification - temp_dir will be dropped automatically
    // but we ensure the file path is correct
    drop(temp_dir);
}

/// Test network configuration error handling using isolated temporary files
#[test]
fn test_network_configuration_errors() {
    // Create isolated temporary directory that won't pollute repository
    let temp_dir = tempdir().unwrap();
    let output_file = temp_dir.path().join("network_config_test.csv");

    // Ensure we're not writing to the repository directory
    assert!(
        !output_file.starts_with("."),
        "Output file should not be in current directory"
    );
    assert!(
        output_file.starts_with(temp_dir.path()),
        "Output file must be in temp directory"
    );

    // Test with network parameters in isolated environment
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb") // Ensure no color output in tests
        .env("NO_COLOR", "1")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("12345"); // Deterministic output for testing

    // This should work, but we can test error handling by checking the output
    let result = cmd.assert();
    if result.get_output().status.success() {
        // Verify the file was created in the correct location
        assert!(
            output_file.exists(),
            "Output file should exist in temp directory"
        );

        // Verify that generated configurations are valid
        let output_content = std::fs::read_to_string(&output_file)
            .expect("Should be able to read generated CSV file");
        assert!(
            output_content.contains("VLAN"),
            "CSV should contain VLAN data"
        );
        assert!(
            output_content.contains("IP Range"),
            "CSV should contain IP range data"
        );

        // Verify no error messages in stdout
        let stdout = String::from_utf8_lossy(&result.get_output().stdout);
        assert!(
            !stdout.to_lowercase().contains("error"),
            "Stdout should not contain errors"
        );
    } else {
        // If it failed, should have proper error context
        let stderr = String::from_utf8_lossy(&result.get_output().stderr);
        assert!(
            stderr.contains("Failed to generate configurations"),
            "Error message should contain context"
        );
    }

    // Verify cleanup - temp_dir automatically cleans up when dropped
    // But we explicitly verify the file path structure
    drop(output_file);
    drop(temp_dir);
}
