//! CLI Integration Tests with ANSI/Color Output Hardening
//!
//! This module provides comprehensive integration testing for the OPNsense Config Faker CLI,
//! with special emphasis on ensuring no ANSI escape codes or color output in test environments.
//!
//! All tests enforce TERM=dumb and verify that no ANSI escape codes leak through to output.

mod common;

use assert_cmd::Command;
use common::{TestOutputExt, cli_command, create_temp_dir, create_temp_xml};
use predicates::prelude::*;
use regex::Regex;
use std::fs;
use tempfile::TempDir;

// ANSI escape sequence patterns for validation
const ANSI_COLOR_PATTERN: &str = r"\x1b\[[0-9;]*m";
const ANSI_CONTROL_PATTERN: &str = r"\x1b\[[0-9;]*[ABCDEFGHJKSTfhil]";
const PROGRESS_CHAR_PATTERN: &str = r"[⠁⠂⠄⡀⢀⠠⠐⠈⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏▪▫]";

/// Helper function to assert no ANSI escape codes in output
#[allow(clippy::needless_borrow)]
fn assert_no_ansi_escapes(output: &str) {
    let ansi_regex = Regex::new(ANSI_COLOR_PATTERN).unwrap();
    let control_regex = Regex::new(ANSI_CONTROL_PATTERN).unwrap();
    let progress_regex = Regex::new(PROGRESS_CHAR_PATTERN).unwrap();

    assert!(
        !ansi_regex.is_match(output),
        "Found ANSI color codes in output: {output}"
    );
    assert!(
        !control_regex.is_match(output),
        "Found ANSI control sequences in output: {output}"
    );
    assert!(
        !progress_regex.is_match(output),
        "Found progress spinner characters in output: {output}"
    );
}

/// Helper function to create a valid base XML configuration for testing
fn create_test_base_config() -> (TempDir, std::path::PathBuf, tempfile::NamedTempFile) {
    let xml_content = r#"<?xml version="1.0"?>
<opnsense>
  <version>24.1</version>
  <theme>opnsense</theme>
  <sysctl>
    <item>
      <descr>Increase UFS read-ahead speeds to match the state of hard drives and NCQ.</descr>
      <tunable>vfs.read_max</tunable>
      <value>default</value>
    </item>
  </sysctl>
  <system>
    <optimization>normal</optimization>
    <hostname>OPNsense</hostname>
    <domain>localdomain</domain>
  </system>
  <interfaces>
    <lan>
      <if>em0</if>
      <descr>LAN</descr>
      <enable>1</enable>
      <ipaddr>192.168.1.1</ipaddr>
      <subnet>24</subnet>
      <gateway></gateway>
    </lan>
    <wan>
      <if>em1</if>
      <descr>WAN</descr>
      <enable>1</enable>
      <ipaddr>dhcp</ipaddr>
    </wan>
  </interfaces>
  <vlans>
  </vlans>
</opnsense>"#;

    let (temp_file, path) = create_temp_xml("base_config_", xml_content).unwrap();
    let temp_dir = TempDir::new().unwrap();
    let base_config_path = temp_dir.path().join("base_config.xml");
    fs::copy(&path, &base_config_path).unwrap();

    // Return the temp_file to keep it alive until the test ends
    (temp_dir, base_config_path, temp_file)
}

// ===== TERM=dumb enforcement and ANSI escape prevention tests =====

#[test]
fn test_term_dumb_enforced_generate_csv() {
    let temp_dir = create_temp_dir("csv_term_test");
    let output_file = temp_dir.path().join("test.csv");

    let output = cli_command()
        .env("TERM", "dumb") // Explicitly set TERM=dumb
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Verify no ANSI escape codes in output
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);

    // Verify file was created
    assert!(output_file.exists());
}

#[test]
fn test_term_dumb_enforced_generate_xml() {
    let (temp_dir, base_config_path, _temp_file) = create_test_base_config();
    let output_dir = temp_dir.path().join("xml_output");

    let output = cli_command()
        .env("TERM", "dumb") // Explicitly set TERM=dumb
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("2")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Verify no ANSI escape codes in output
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);

    // Verify XML files were created
    assert!(output_dir.exists());
}

#[test]
fn test_term_dumb_enforced_completions() {
    let output = cli_command()
        .env("TERM", "dumb") // Explicitly set TERM=dumb
        .arg("completions")
        .arg("bash")
        .run_success();

    // Verify no ANSI escape codes in completion output
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);

    // Verify completion script content
    output.assert_stdout_contains("_opnsense-config-faker");
}

// ===== Generate command with CSV format tests =====

#[test]
fn test_generate_csv_with_force() {
    let temp_dir = create_temp_dir("csv_force_test");
    let output_file = temp_dir.path().join("test_force.csv");

    // Create existing file
    fs::write(&output_file, "existing,content\n").unwrap();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--force") // Force overwrite
        .arg("--seed")
        .arg("42")
        .run_success();

    // Verify success message (flexible matching to handle emojis and different output formats)
    let normalized = output.normalized_stdout();
    assert!(
        normalized.contains("Generated 3 VLAN configurations")
            || normalized.contains("3 VLAN configurations")
            || (normalized.contains("Configurations: 3") && normalized.contains("Summary")),
        "Expected success message about generating 3 VLANs, got: {normalized}"
    );

    // Verify file was overwritten
    let content = fs::read_to_string(&output_file).unwrap();
    assert!(!content.contains("existing,content"));
    assert!(content.contains("VLAN"));

    // Verify no ANSI escapes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_generate_csv_without_force_fails() {
    let temp_dir = create_temp_dir("csv_no_force_test");
    let output_file = temp_dir.path().join("test_no_force.csv");

    // Create existing file
    fs::write(&output_file, "existing,content\n").unwrap();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        // No --force flag
        .arg("--seed")
        .arg("42")
        .run_failure();

    // Verify error message contains helpful guidance
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("already exists")
            || combined_output.contains("Use --force to overwrite"),
        "Expected overwrite error message, got: {combined_output}"
    );

    // Verify no ANSI escapes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_generate_csv_missing_output_fails() {
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        // Missing --output argument
        .run_failure();

    // Verify helpful error message
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Output file path is required")
            || combined_output.contains("required") && combined_output.contains("output"),
        "Expected output file error message, got: {combined_output}"
    );

    // Verify no ANSI escapes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

// ===== Generate command with XML format tests =====

#[test]
fn test_generate_xml_with_base_config() {
    let (temp_dir, base_config_path, _temp_file) = create_test_base_config();
    let output_dir = temp_dir.path().join("xml_test");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("2")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Verify success message (flexible matching to handle emojis)
    let normalized = output.normalized_stdout();
    assert!(
        normalized.contains("XML configurations generated")
            || normalized.contains("configurations generated")
            || normalized.contains("Summary"),
        "Expected success message about XML generation, got: {normalized}"
    );

    // Verify XML files were created
    assert!(output_dir.exists());
    let files: Vec<_> = fs::read_dir(&output_dir).unwrap().collect();
    assert!(files.len() >= 2, "Expected at least 2 XML files");

    // Verify no ANSI escapes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_generate_xml_missing_base_config_fails() {
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("3")
        // Missing --base-config argument
        .run_failure();

    // Verify helpful error message
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("Base configuration file is required")
            || combined_output.contains("required") && combined_output.contains("base-config"),
        "Expected base config error message, got: {combined_output}"
    );

    // Verify no ANSI escapes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_generate_xml_nonexistent_base_config_fails() {
    let temp_dir = create_temp_dir("xml_nonexistent_test");
    let nonexistent_config = temp_dir.path().join("does_not_exist.xml");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--count")
        .arg("2")
        .arg("--base-config")
        .arg(&nonexistent_config)
        .run_failure();

    // Verify helpful error message about missing file
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("not found")
            || combined_output.contains("No such file")
            || combined_output.contains("does not exist")
            || combined_output.contains("ConfigNotFound"),
        "Expected file not found error, got: {combined_output}"
    );

    // Verify no ANSI escapes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

// ===== Shell completions tests =====

#[test]
fn test_completions_bash() {
    let output = cli_command().arg("completions").arg("bash").run_success();

    let stdout = &output.stdout;

    // Assert key tokens exist in bash completion script
    assert!(
        stdout.contains("_opnsense-config-faker"),
        "Missing main completion function"
    );
    assert!(stdout.contains("COMPREPLY"), "Missing bash COMPREPLY usage");
    assert!(stdout.contains("generate"), "Missing 'generate' subcommand");
    assert!(
        stdout.contains("completions"),
        "Missing 'completions' subcommand"
    );

    // Verify no ANSI escape codes in completions
    assert_no_ansi_escapes(stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_completions_zsh() {
    let output = cli_command().arg("completions").arg("zsh").run_success();

    let stdout = &output.stdout;

    // Assert key tokens exist in zsh completion script
    assert!(stdout.contains("#compdef"), "Missing zsh compdef directive");
    assert!(
        stdout.contains("opnsense-config-faker"),
        "Missing command name reference"
    );
    assert!(stdout.contains("generate"), "Missing 'generate' subcommand");

    // Verify no ANSI escape codes in completions
    assert_no_ansi_escapes(stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_completions_fish() {
    let output = cli_command().arg("completions").arg("fish").run_success();

    let stdout = &output.stdout;

    // Assert key tokens exist in fish completion script
    assert!(
        stdout.contains("complete"),
        "Missing fish complete commands"
    );
    assert!(
        stdout.contains("opnsense-config-faker"),
        "Missing command name reference"
    );
    assert!(stdout.contains("generate"), "Missing 'generate' subcommand");

    // Verify no ANSI escape codes in completions
    assert_no_ansi_escapes(stdout);
    assert_no_ansi_escapes(&output.stderr);
}

// ===== Deprecated command path tests =====

#[test]
fn test_deprecated_csv_command_shows_warning() {
    let temp_dir = create_temp_dir("deprecated_csv_test");
    let output_file = temp_dir.path().join("test.csv");

    let output = cli_command()
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42")
        .run_failure(); // Deprecated commands should fail

    let combined_output = output.normalized_combined();

    // Verify deprecation warning is shown
    assert!(
        combined_output.contains("DEPRECATED COMMAND"),
        "Missing deprecation warning"
    );
    assert!(
        combined_output.contains("generate --format csv"),
        "Missing migration guidance"
    );

    // Verify no ANSI escape codes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_deprecated_xml_command_shows_warning() {
    let (_temp_dir, base_config_path, _temp_file) = create_test_base_config();

    let output = cli_command()
        .arg("xml")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--count")
        .arg("2")
        .run_failure(); // Deprecated commands should fail

    let combined_output = output.normalized_combined();

    // Verify deprecation warning is shown
    assert!(
        combined_output.contains("DEPRECATED COMMAND"),
        "Missing deprecation warning"
    );
    assert!(
        combined_output.contains("generate --format xml"),
        "Missing migration guidance"
    );

    // Verify no ANSI escape codes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_deprecated_csv_help_works() {
    let output = cli_command().arg("csv").arg("--help").run_success(); // Help should work for migration guidance

    // Should show help content (even for deprecated commands)
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("count") || combined_output.contains("output"),
        "Expected help content"
    );

    // Verify no ANSI escape codes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_deprecated_xml_help_works() {
    let output = cli_command().arg("xml").arg("--help").run_success(); // Help should work for migration guidance

    // Should show help content (even for deprecated commands)
    let combined_output = output.normalized_combined();
    assert!(
        combined_output.contains("base-config") || combined_output.contains("count"),
        "Expected help content"
    );

    // Verify no ANSI escape codes
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

// ===== Negative path assertions using predicates =====

#[test]
fn test_invalid_shell_completion_fails() {
    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb")
        .env("NO_COLOR", "1")
        .arg("completions")
        .arg("invalid_shell");

    // Use predicates for negative path validation
    cmd.assert().failure().stderr(
        predicates::str::contains("invalid value").or(predicates::str::contains("possible values")),
    );
}

#[test]
fn test_invalid_count_parameter_fails() {
    let temp_dir = create_temp_dir("invalid_count_test");
    let output_file = temp_dir.path().join("test.csv");

    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb")
        .env("NO_COLOR", "1")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("15000") // Exceeds CLI limit of 10000
        .arg("--output")
        .arg(&output_file);

    // Use predicates for validation
    cmd.assert().failure().stderr(
        predicates::str::contains("invalid value").or(predicates::str::contains("out of range")),
    );
}

#[test]
fn test_conflicting_xml_options_fails() {
    let (temp_dir, base_config_path, _temp_file) = create_test_base_config();
    let csv_file = temp_dir.path().join("data.csv");

    // Create a dummy CSV file
    fs::write(
        &csv_file,
        "VLAN,IP Range,Description\n100,192.168.1.0/24,Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("opnsense-config-faker").unwrap();
    cmd.env("TERM", "dumb")
        .env("NO_COLOR", "1")
        .arg("generate")
        .arg("--format")
        .arg("xml")
        .arg("--base-config")
        .arg(&base_config_path)
        .arg("--count")
        .arg("5")
        .arg("--csv-file") // These two options conflict
        .arg(&csv_file);

    // Use predicates for validation
    cmd.assert().failure().stderr(
        predicates::str::contains("conflict").or(predicates::str::contains("cannot be used with")),
    );
}

#[test]
fn test_regex_pattern_validation_no_ansi() {
    let output = cli_command().arg("--help").run_success();

    let combined = format!("{}{}", output.stdout, output.stderr);

    // Use regex predicates to ensure no ANSI codes
    let ansi_pattern = Regex::new(r"\x1b\[[0-9;]*[mGKHF]").unwrap();
    assert!(
        !ansi_pattern.is_match(&combined),
        "Found ANSI escape sequences in help output"
    );
}

#[test]
fn test_stderr_contains_validation() {
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        // Missing required --output parameter
        .run_failure();

    // Use predicates to validate stderr contains error message
    let stderr_predicate = predicates::str::contains("Output file")
        .or(predicates::str::contains("required"))
        .or(predicates::str::contains("output"));

    assert!(
        stderr_predicate.eval(&output.stderr) || stderr_predicate.eval(&output.stdout),
        "Expected error message about missing output parameter"
    );
}

// ===== Terminal environment validation tests =====

#[test]
fn test_no_color_environment_respected() {
    let temp_dir = create_temp_dir("no_color_test");
    let output_file = temp_dir.path().join("test.csv");

    let output = cli_command()
        .env("NO_COLOR", "1") // Should disable all color output
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("2")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Verify no ANSI escape codes when NO_COLOR is set
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}

#[test]
fn test_multiple_environment_variables() {
    let temp_dir = create_temp_dir("multi_env_test");
    let output_file = temp_dir.path().join("test.csv");

    let output = cli_command()
        .env("TERM", "dumb")
        .env("NO_COLOR", "1")
        .env("CARGO_TERM_COLOR", "never")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("42")
        .run_success();

    // Should produce completely clean output
    assert_no_ansi_escapes(&output.stdout);
    assert_no_ansi_escapes(&output.stderr);
}
