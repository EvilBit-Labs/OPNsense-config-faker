//! Snapshot tests for CSV output generation
//!
//! These tests capture deterministic CSV outputs using fixed seeds to ensure
//! consistent generation across runs and platforms.

mod common;

use common::{TestOutputExt, cli_command};
use insta::assert_snapshot;
use regex::Regex;
use std::fs;
use tempfile::NamedTempFile;

/// Normalize temporary file paths in output for consistent snapshots
fn normalize_temp_paths(output: &str) -> String {
    let temp_path_regex = Regex::new(r"/[^/]*/[^/]*/T/[^\s]+").unwrap();
    temp_path_regex
        .replace_all(output, "<TEMP_PATH>")
        .to_string()
}

/// Test CSV generation with fixed seed for deterministic output
#[test]
fn test_csv_generation_deterministic() {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_string_lossy();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("5")
        .arg("--output")
        .arg(&*temp_path)
        .arg("--seed")
        .arg("42")
        .arg("--force") // Force overwrite since tempfile already exists
        .run_success();

    // Snapshot the CLI output (normalize temp file paths)
    let stdout = normalize_temp_paths(&output.normalized_stdout());
    let stderr = normalize_temp_paths(&output.normalized_stderr());
    assert_snapshot!("csv_generation_stdout", stdout);
    assert_snapshot!("csv_generation_stderr", stderr);

    // Read and snapshot the generated CSV content
    let csv_content =
        fs::read_to_string(temp_path.as_ref()).expect("Failed to read generated CSV file");

    // Normalize line endings to \n for cross-platform stability
    let normalized_csv = csv_content.replace("\r\n", "\n").replace('\r', "\n");

    assert_snapshot!("csv_generation_file_content", normalized_csv);

    // Verify the CSV has the expected structure
    let lines: Vec<&str> = normalized_csv.trim().split('\n').collect();
    assert!(lines.len() > 1, "CSV should have header + data rows");

    // Verify header line exists
    let header = lines[0];
    assert!(header.contains("VLAN"), "Header should contain VLAN column");
    assert!(
        header.contains("IP Range"),
        "Header should contain IP Range column"
    );
    assert!(
        header.contains("Beschreibung"),
        "Header should contain Beschreibung column"
    );

    // Verify we have the correct number of data rows (header + 5 data rows)
    assert_eq!(lines.len(), 6, "Should have 1 header + 5 data rows");
}

/// Test CSV generation with different seed produces different but consistent content
#[test]
fn test_csv_generation_different_seed() {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_string_lossy();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("3")
        .arg("--output")
        .arg(&*temp_path)
        .arg("--seed")
        .arg("123")
        .arg("--force")
        .run_success();

    // Snapshot the CLI output for different seed (normalize temp file paths)
    let stdout = normalize_temp_paths(&output.normalized_stdout());
    assert_snapshot!("csv_generation_seed123_stdout", stdout);

    // Read and normalize the generated CSV content
    let csv_content =
        fs::read_to_string(temp_path.as_ref()).expect("Failed to read generated CSV file");
    let normalized_csv = csv_content.replace("\r\n", "\n").replace('\r', "\n");

    assert_snapshot!("csv_generation_seed123_content", normalized_csv);
}

/// Test CSV generation with larger count for performance validation
#[test]
fn test_csv_generation_larger_count() {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_string_lossy();

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("10")
        .arg("--output")
        .arg(&*temp_path)
        .arg("--seed")
        .arg("999")
        .arg("--force")
        .run_success();

    // Just verify the summary output for larger generation (normalize temp file paths)
    let stdout = normalize_temp_paths(&output.normalized_stdout());
    assert_snapshot!("csv_generation_count10_stdout", stdout);

    // Verify file was created with correct number of rows
    let csv_content =
        fs::read_to_string(temp_path.as_ref()).expect("Failed to read generated CSV file");
    let normalized_csv = csv_content.replace("\r\n", "\n").replace('\r', "\n");
    let lines: Vec<&str> = normalized_csv.trim().split('\n').collect();

    // Should have 11 lines (1 header + 10 data rows)
    assert_eq!(lines.len(), 11, "Should have 1 header + 10 data rows");
}

/// Test CSV force overwrite functionality
#[test]
fn test_csv_force_overwrite() {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_string_lossy();

    // Write some initial content to the file
    fs::write(temp_path.as_ref(), "existing,content\ntest,data")
        .expect("Failed to write initial content");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("2")
        .arg("--output")
        .arg(&*temp_path)
        .arg("--seed")
        .arg("42")
        .arg("--force")
        .run_success();

    let stdout = normalize_temp_paths(&output.normalized_stdout());
    assert_snapshot!("csv_force_overwrite_stdout", stdout);

    // Verify the file was overwritten with new content
    let csv_content =
        fs::read_to_string(temp_path.as_ref()).expect("Failed to read generated CSV file");
    let normalized_csv = csv_content.replace("\r\n", "\n").replace('\r', "\n");

    assert!(
        !normalized_csv.contains("existing,content"),
        "Old content should be overwritten"
    );
    assert!(
        normalized_csv.contains("VLAN"),
        "New content should contain VLAN header"
    );

    // Should have exactly 3 lines (1 header + 2 data rows)
    let lines: Vec<&str> = normalized_csv.trim().split('\n').collect();
    assert_eq!(lines.len(), 3, "Should have 1 header + 2 data rows");
}
