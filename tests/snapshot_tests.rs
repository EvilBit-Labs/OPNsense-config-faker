//! Snapshot tests for OPNsense Config Faker
//!
//! These tests capture output snapshots to ensure CLI output remains consistent
//! across changes and updates.

mod common;

use common::{cli_command, create_temp_csv, create_temp_xml, normalize_output, TestOutputExt};
use insta::assert_snapshot;

/// Test CSV generation output with stable snapshots
#[test]
fn test_csv_output_snapshot() {
    let (temp_file, csv_path) = create_temp_csv(
        "snapshot_test_",
        &[
            &["VLAN", "IP Range", "Description"],
            &["100", "192.168.1.0/24", "Test Network"],
            &["200", "192.168.2.0/24", "Production"],
        ],
    )
    .unwrap();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&csv_path)
        .arg("--force") // Force overwrite existing temp file
        .arg("--seed")
        .arg("12345") // Fixed seed for reproducible output
        .run_success();

    // Normalize output for stable snapshots
    let normalized_stdout = normalize_output(&output.stdout);
    let normalized_stderr = normalize_output(&output.stderr);

    // Snapshot the normalized CLI output
    assert_snapshot!("csv_generation_success_stdout", normalized_stdout);
    assert_snapshot!("csv_generation_success_stderr", normalized_stderr);

    // Also snapshot the generated CSV content
    let csv_content = std::fs::read_to_string(&csv_path).unwrap();
    assert_snapshot!("generated_csv_content", csv_content);

    drop(temp_file);
}

/// Test help output snapshots
#[test]
fn test_help_output_snapshots() {
    // Test main help
    let output = cli_command().arg("--help").run_success();

    assert_snapshot!("main_help_output", output.normalized_stdout());

    // Test generate help
    let output = cli_command().arg("generate").arg("--help").run_success();

    assert_snapshot!("generate_help_output", output.normalized_stdout());

    // Test completions help
    let output = cli_command().arg("completions").arg("--help").run_success();

    assert_snapshot!("completions_help_output", output.normalized_stdout());
}

/// Test error output snapshots for consistent error messages
#[test]
fn test_error_output_snapshots() {
    // Test missing output file error
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .run_failure();

    assert_snapshot!("missing_csv_output_error", output.normalized_stderr());

    // Test missing XML base config error
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("5")
        .run_failure();

    assert_snapshot!("missing_xml_base_config_error", output.normalized_stderr());

    // Test file exists without force error
    let (temp_file, csv_path) = create_temp_csv("error_test_", &[&["existing", "data"]]).unwrap();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("2")
        .arg("--output")
        .arg(&csv_path)
        .arg("--seed")
        .arg("42")
        .run_failure();

    assert_snapshot!(
        "file_exists_without_force_error",
        output.normalized_stderr()
    );

    drop(temp_file);
}

/// Test deprecated command warnings
#[test]
fn test_deprecated_command_snapshots() {
    let (temp_file, csv_path) =
        create_temp_csv("deprecated_test_", &[&["header1", "header2"]]).unwrap();

    // Test deprecated CSV command
    let output = cli_command()
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&csv_path)
        .arg("--seed")
        .arg("42")
        .run_failure();

    // Capture both stdout and stderr for deprecation messages
    assert_snapshot!("deprecated_csv_stdout", output.normalized_stdout());
    assert_snapshot!("deprecated_csv_stderr", output.normalized_stderr());

    drop(temp_file);
}

/// Test XML generation output snapshots
#[test]
fn test_xml_generation_snapshots() {
    let base_xml_content = r#"<?xml version="1.0"?>
<opnsense version="24.1" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <system>
    <hostname>opnsense</hostname>
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
  </interfaces>
</opnsense>"#;

    let (temp_xml_file, xml_path) = create_temp_xml("base_config_", base_xml_content).unwrap();
    let (temp_csv_file, csv_path) = create_temp_csv(
        "vlan_data_",
        &[
            &["VLAN", "IP Range", "Beschreibung", "WAN"],
            &["100", "192.168.100.0/24", "Development", "1"],
            &["200", "192.168.200.0/24", "Testing", "2"],
        ],
    )
    .unwrap();

    // Test XML generation using CSV input
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(&xml_path)
        .arg("--csv-file")
        .arg(&csv_path)
        .arg("--output-dir")
        .arg("/tmp") // Ensure consistent output directory for snapshots
        .arg("--force") // Force overwrite existing files
        .run_success();

    assert_snapshot!("xml_generation_with_csv_stdout", output.normalized_stdout());
    assert_snapshot!("xml_generation_with_csv_stderr", output.normalized_stderr());

    drop(temp_xml_file);
    drop(temp_csv_file);
}

/// Test progress output normalization
#[test]
fn test_progress_normalization() {
    let (temp_file, csv_path) = create_temp_csv("progress_test_", &[]).unwrap();

    // Remove temp file first to avoid force requirement
    drop(temp_file);
    std::fs::remove_file(&csv_path).ok(); // Ignore error if file doesn't exist

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10") // Larger count to trigger progress indicators
        .arg("--output")
        .arg(&csv_path)
        .arg("--seed")
        .arg("999")
        .run_success();

    // The normalize_output function should remove progress indicators
    let normalized = normalize_output(&output.stdout);

    // Verify progress characters are removed
    assert!(!normalized.contains("⠁"));
    assert!(!normalized.contains("⠂"));
    assert!(!normalized.contains("⠄"));
    assert!(!normalized.contains("▪"));
    assert!(!normalized.contains("▫"));

    assert_snapshot!("progress_normalized_output", normalized);
}

/// Test bash completion script snapshot
#[test]
fn test_bash_completion_snapshot() {
    let output = cli_command().arg("completions").arg("bash").run_success();

    // Bash completions should be consistent
    let normalized = normalize_output(&output.stdout);
    assert_snapshot!("bash_completion_script", normalized);
}

/// Test zsh completion script snapshot
#[test]
fn test_zsh_completion_snapshot() {
    let output = cli_command().arg("completions").arg("zsh").run_success();

    let normalized = normalize_output(&output.stdout);
    assert_snapshot!("zsh_completion_script", normalized);
}

/// Test fish completion script snapshot
#[test]
fn test_fish_completion_snapshot() {
    let output = cli_command().arg("completions").arg("fish").run_success();

    let normalized = normalize_output(&output.stdout);
    assert_snapshot!("fish_completion_script", normalized);
}
