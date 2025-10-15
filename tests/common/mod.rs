//! Shared test utilities and fixtures for OPNsense Config Faker
//!
//! This module provides common test utilities used across integration tests,
//! snapshot tests, and compatibility tests.

use assert_cmd::Command;
use assert_fs::TempDir as AssertTempDir;
use regex::Regex;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test result with captured stdout and stderr
#[derive(Debug, Clone)]
pub struct TestOutput {
    pub stdout: String,
    pub stderr: String,
    #[allow(dead_code)]
    pub status: std::process::ExitStatus,
    pub success: bool,
}

/// Helper to create a CLI command with standardized environment for consistent testing
///
/// This helper automatically sets:
/// - `TERM=dumb` to disable Rich terminal formatting
/// - `CARGO_TERM_COLOR=never` to disable Cargo colored output
/// - `NO_COLOR=1` to disable all color output
///
/// # Example
/// ```
/// use tests::common::cli_command;
///
/// let output = cli_command()
///     .arg("generate")
///     .arg("--format")
///     .arg("csv")
///     .arg("--count")
///     .arg("5")
///     .run_and_capture()?;
///
/// assert!(output.success);
/// ```
pub fn cli_command() -> CliCommandBuilder {
    CliCommandBuilder::new()
}

/// Builder for CLI commands with environment control and output capture
pub struct CliCommandBuilder {
    command: Command,
    env_vars: HashMap<String, String>,
}

impl CliCommandBuilder {
    /// Create a new CLI command builder with standard test environment
    pub fn new() -> Self {
        let command = Command::cargo_bin("opnsense-config-faker").unwrap();
        let mut env_vars = HashMap::new();

        // Set environment variables for consistent test output
        env_vars.insert("TERM".to_string(), "dumb".to_string());
        env_vars.insert("CARGO_TERM_COLOR".to_string(), "never".to_string());
        env_vars.insert("NO_COLOR".to_string(), "1".to_string());

        Self { command, env_vars }
    }

    /// Add a command line argument
    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> Self {
        self.command.arg(arg);
        self
    }

    /// Add multiple command line arguments
    #[allow(dead_code)]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command.args(args);
        self
    }

    /// Add or override an environment variable
    pub fn env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.env_vars
            .insert(key.as_ref().to_string(), val.as_ref().to_string());
        self
    }

    /// Set the current working directory
    #[allow(dead_code)]
    pub fn current_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.command.current_dir(dir);
        self
    }

    /// Run the command and capture output, returning structured TestOutput
    pub fn run_and_capture(mut self) -> TestOutput {
        // Apply all environment variables
        for (key, value) in &self.env_vars {
            self.command.env(key, value);
        }

        let output = self.command.output().unwrap();

        TestOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            status: output.status,
            success: output.status.success(),
        }
    }

    /// Run the command expecting success and return clean output
    #[allow(dead_code)]
    pub fn run_success(self) -> TestOutput {
        let output = self.run_and_capture();
        assert!(
            output.success,
            "Command failed with exit code: {:?}\nstdout: {}\nstderr: {}",
            output.status.code(),
            output.stdout,
            output.stderr
        );
        output
    }

    /// Run the command expecting failure and return output
    #[allow(dead_code)]
    pub fn run_failure(self) -> TestOutput {
        let output = self.run_and_capture();
        assert!(
            !output.success,
            "Command unexpectedly succeeded\nstdout: {}\nstderr: {}",
            output.stdout, output.stderr
        );
        output
    }

    /// Get the underlying assert_cmd Command for advanced usage
    pub fn into_command(mut self) -> Command {
        // Apply all environment variables
        for (key, value) in &self.env_vars {
            self.command.env(key, value);
        }
        self.command
    }
}

/// Strip ANSI escape sequences and normalize whitespace for stable test assertions
///
/// This helper removes:
/// - ANSI color codes and escape sequences
/// - Progress indicators and terminal control sequences
/// - Temporary file paths (replaced with <TEMP_FILE> placeholder)
/// - Normalizes different types of whitespace
/// - Trims leading and trailing whitespace
/// - Converts multiple consecutive whitespace to single spaces
///
/// # Example
/// ```
/// use tests::common::normalize_output;
///
/// let raw_output = "\u001b[32m✅ Success\u001b[0m\n  Multiple   spaces\t\n";
/// let clean = normalize_output(raw_output);
/// assert_eq!(clean, "✅ Success Multiple spaces");
/// ```
pub fn normalize_output(text: &str) -> String {
    // Remove ANSI escape sequences
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*[mGKHF]").unwrap();
    let without_ansi = ansi_regex.replace_all(text, "");

    // Remove other terminal control sequences
    let control_regex =
        Regex::new(r"\x1b[\[\]()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><~]")
            .unwrap();
    let without_control = control_regex.replace_all(&without_ansi, "");

    // Remove progress indicators and spinner characters
    let progress_regex = Regex::new(r"[⠁⠂⠄⡀⢀⠠⠐⠈⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏▪▫]").unwrap();
    let without_progress = progress_regex.replace_all(&without_control, "");

    // Normalize temporary file paths to stable placeholders
    // Handle various temp directory patterns across different OS
    let temp_path_patterns = vec![
        // macOS temp paths
        r"/var/folders/[^/]+/[^/]+/T/[^\s]+\.(csv|xml|txt)",
        // Linux temp paths
        r"/tmp/[^\s]+\.(csv|xml|txt)",
        // Windows temp paths (if running on Windows) - more comprehensive
        r"[A-Z]:\\[^\\]*\\[Tt]emp[^\\]*\\[^\s]+\.(csv|xml|txt)",
        r"[A-Z]:\\[^\\]*\\AppData\\Local\\Temp\\[^\s]+\.(csv|xml|txt)",
        // Generic temp paths
        r"/(?:var/folders|tmp|temp)/[^\s]+\.(csv|xml|txt)",
    ];

    // Also handle Windows paths with forward slashes (for cross-platform compatibility)
    let windows_forward_slash_patterns = vec![
        r"[A-Z]:/[^/]*/[Tt]emp[^/]*/[^\s]+\.(csv|xml|txt)",
        r"[A-Z]:/[^/]*/AppData/Local/Temp/[^\s]+\.(csv|xml|txt)",
    ];

    let mut with_normalized_paths = without_progress.to_string();
    for pattern in temp_path_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            with_normalized_paths = regex
                .replace_all(&with_normalized_paths, "<TEMP_FILE>")
                .into_owned();
        }
    }

    // Process Windows forward slash patterns
    for pattern in windows_forward_slash_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            with_normalized_paths = regex
                .replace_all(&with_normalized_paths, "<TEMP_FILE>")
                .into_owned();
        }
    }

    // Also normalize just the filename parts for cases where only filename is shown
    // But only if they haven't already been normalized as full paths
    let temp_file_pattern = r"\b[a-zA-Z_][a-zA-Z0-9_]*_[A-Za-z0-9]{6,}\.(csv|xml|txt)\b";
    let temp_file_regex = Regex::new(temp_file_pattern).unwrap();
    let with_normalized_filenames =
        temp_file_regex.replace_all(&with_normalized_paths, "<TEMP_FILE>");

    // Normalize temporary directory paths to stable placeholders
    let temp_dir_patterns = vec![
        // macOS temp directories
        r"/var/folders/[^/]+/[^/]+/T/[^\s]*",
        // Linux temp directories
        r"/tmp/[^\s]*",
        // Windows temp directories (with multiple path levels)
        r"[A-Z]:[^:\s]*\\[Tt]emp[^:\s]*",
        r"[A-Z]:[^:\s]*\\AppData\\Local\\Temp[^:\s]*",
        // Generic temp directories
        r"/(?:var/folders|tmp|temp)/[^\s]*",
    ];

    let mut with_normalized_dirs = with_normalized_filenames.into_owned();
    for pattern in temp_dir_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            with_normalized_dirs = regex
                .replace_all(&with_normalized_dirs, "<TEMP_DIR>")
                .into_owned();
        }
    }

    // Normalize whitespace
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    let normalized = whitespace_regex.replace_all(&with_normalized_dirs, " ");

    // Trim and return
    normalized.trim().to_string()
}

/// Create a temporary directory using tempfile for basic temp needs
///
/// # Example
/// ```
/// use tests::common::create_temp_dir;
///
/// let temp_dir = create_temp_dir("test_prefix");
/// let file_path = temp_dir.path().join("test_file.csv");
/// // Directory is automatically cleaned up when temp_dir is dropped
/// ```
pub fn create_temp_dir(prefix: &str) -> TempDir {
    tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .expect("Failed to create temporary directory")
}

/// Create a temporary directory with assert_fs for advanced file system testing
///
/// This provides more sophisticated file system testing capabilities including:
/// - File content assertions
/// - Directory structure validation
/// - Predicate-based file testing
///
/// # Example
/// ```
/// use tests::common::create_assert_temp_dir;
/// use assert_fs::prelude::*;
///
/// let temp_dir = create_assert_temp_dir();
/// temp_dir.child("config.csv").assert(predicates::path::exists());
/// ```
pub fn create_assert_temp_dir() -> AssertTempDir {
    AssertTempDir::new()
        .unwrap()
        .into_persistent_if(std::env::var_os("TEST_PERSIST_TEMP").is_some())
}

/// Create a temporary file with specified content using tempfile
///
/// Returns the temporary file and its path. The file will be cleaned up
/// when the returned NamedTempFile is dropped.
///
/// # Example
/// ```
/// use tests::common::create_temp_file;
///
/// let (temp_file, file_path) = create_temp_file("test_", ".csv", "header1,header2\nvalue1,value2")?;
/// // Use file_path for testing...
/// // File is cleaned up when temp_file is dropped
/// ```
pub fn create_temp_file(
    prefix: &str,
    suffix: &str,
    content: &str,
) -> std::io::Result<(tempfile::NamedTempFile, PathBuf)> {
    use std::io::Write;

    let mut temp_file = tempfile::Builder::new()
        .prefix(prefix)
        .suffix(suffix)
        .tempfile()?;

    temp_file.write_all(content.as_bytes())?;
    temp_file.flush()?;

    let path = temp_file.path().to_path_buf();
    Ok((temp_file, path))
}

/// Create a temporary CSV file with test data
///
/// This is a convenience function for creating CSV files commonly used in tests.
///
/// # Example
/// ```
/// use tests::common::create_temp_csv;
///
/// let (temp_file, csv_path) = create_temp_csv("test_", &[
///     &["VLAN", "IP Range", "Description"],
///     &["100", "192.168.1.0/24", "Test Network"],
///     &["200", "192.168.2.0/24", "Another Network"],
/// ])?;
/// ```
pub fn create_temp_csv(
    prefix: &str,
    rows: &[&[&str]],
) -> std::io::Result<(tempfile::NamedTempFile, PathBuf)> {
    let csv_content = rows
        .iter()
        .map(|row| row.join(","))
        .collect::<Vec<_>>()
        .join("\n");

    create_temp_file(prefix, ".csv", &csv_content)
}

/// Create a temporary XML file with test configuration
///
/// This is a convenience function for creating XML configuration files for testing.
///
/// # Example
/// ```
/// use tests::common::create_temp_xml;
///
/// let xml_content = r#"<?xml version="1.0"?>
/// <opnsense>
///   <interfaces>
///     <lan>
///       <if>em0</if>
///     </lan>
///   </interfaces>
/// </opnsense>"#;
///
/// let (temp_file, xml_path) = create_temp_xml("base_config_", xml_content)?;
/// ```
#[allow(dead_code)]
pub fn create_temp_xml(
    prefix: &str,
    content: &str,
) -> std::io::Result<(tempfile::NamedTempFile, PathBuf)> {
    create_temp_file(prefix, ".xml", content)
}

/// Helper trait to extend TestOutput with additional assertion methods
#[allow(dead_code)]
pub trait TestOutputExt {
    /// Assert that stdout contains the expected text (after normalization)
    fn assert_stdout_contains(&self, expected: &str) -> &Self;

    /// Assert that stderr contains the expected text (after normalization)
    fn assert_stderr_contains(&self, expected: &str) -> &Self;

    /// Assert that stdout matches a regex pattern (after normalization)
    fn assert_stdout_matches(&self, pattern: &str) -> &Self;

    /// Assert that stderr matches a regex pattern (after normalization)
    fn assert_stderr_matches(&self, pattern: &str) -> &Self;

    /// Get normalized stdout
    fn normalized_stdout(&self) -> String;

    /// Get normalized stderr
    fn normalized_stderr(&self) -> String;

    /// Get combined normalized output (stdout + stderr)
    fn normalized_combined(&self) -> String;

    /// Assert that stdout contains a success message about generating VLANs (flexible matching)
    fn assert_vlan_generation_success(&self, count: u32) -> &Self;
}

impl TestOutputExt for TestOutput {
    fn assert_stdout_contains(&self, expected: &str) -> &Self {
        let normalized = self.normalized_stdout();
        assert!(
            normalized.contains(expected),
            "Expected stdout to contain '{expected}'\nActual normalized stdout: '{normalized}'"
        );
        self
    }

    fn assert_stderr_contains(&self, expected: &str) -> &Self {
        let normalized = self.normalized_stderr();
        assert!(
            normalized.contains(expected),
            "Expected stderr to contain '{expected}'\nActual normalized stderr: '{normalized}'"
        );
        self
    }

    fn assert_stdout_matches(&self, pattern: &str) -> &Self {
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        let normalized = self.normalized_stdout();
        assert!(
            regex.is_match(&normalized),
            "Expected stdout to match pattern '{pattern}'\nActual normalized stdout: '{normalized}'"
        );
        self
    }

    fn assert_stderr_matches(&self, pattern: &str) -> &Self {
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        let normalized = self.normalized_stderr();
        assert!(
            regex.is_match(&normalized),
            "Expected stderr to match pattern '{pattern}'\nActual normalized stderr: '{normalized}'"
        );
        self
    }

    fn normalized_stdout(&self) -> String {
        normalize_output(&self.stdout)
    }

    fn normalized_stderr(&self) -> String {
        normalize_output(&self.stderr)
    }

    fn normalized_combined(&self) -> String {
        normalize_output(&format!("{}{}", self.stdout, self.stderr))
    }

    fn assert_vlan_generation_success(&self, count: u32) -> &Self {
        let normalized = self.normalized_stdout();
        let singular = if count == 1 {
            "configuration"
        } else {
            "configurations"
        };

        let old_format = format!("Generated {count} VLAN {singular}");
        let count_check = format!("{count} VLAN {singular}");
        let summary_check = normalized.contains("Summary")
            && normalized.contains(&format!("Configurations: {count}"));

        assert!(
            normalized.contains(&old_format) || normalized.contains(&count_check) || summary_check,
            "Expected success message about generating {count} VLANs, got: {normalized}"
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_output() {
        // Test ANSI color removal
        let ansi_text = "\u{001b}[32mSuccess\u{001b}[0m";
        assert_eq!(normalize_output(ansi_text), "Success");

        // Test whitespace normalization
        let whitespace_text = "  Multiple   spaces\t\n  and\r\ntabs  ";
        assert_eq!(
            normalize_output(whitespace_text),
            "Multiple spaces and tabs"
        );

        // Test combined ANSI and whitespace
        let combined = "\u{001b}[1;34m  Status:  \u{001b}[0m\u{001b}[32m✅  OK  \u{001b}[0m\n";
        assert_eq!(normalize_output(combined), "Status: ✅ OK");

        // Test temp path normalization
        let macos_temp_file = "Output file: /var/folders/abc123/def456/T/temp_file_abc123.csv";
        assert_eq!(
            normalize_output(macos_temp_file),
            "Output file: <TEMP_FILE>"
        );

        let linux_temp_file = "Output file: /tmp/temp_file_abc123.xml";
        assert_eq!(
            normalize_output(linux_temp_file),
            "Output file: <TEMP_FILE>"
        );

        let temp_dir = "Working in /var/folders/abc123/def456/T/";
        assert_eq!(normalize_output(temp_dir), "Working in <TEMP_DIR>");

        let temp_filename_only = "File: temp_file_abc123.csv";
        assert_eq!(normalize_output(temp_filename_only), "File: <TEMP_FILE>");

        // Test mixed content with temp paths
        let mixed_content = "Processing /var/folders/abc123/def456/T/data.csv and /tmp/other.xml";
        assert_eq!(
            normalize_output(mixed_content),
            "Processing <TEMP_FILE> and <TEMP_FILE>"
        );
    }

    #[test]
    fn test_cli_command_builder() {
        let cmd = cli_command().arg("--help").env("TEST_VAR", "test_value");

        // Verify we can build the command without errors
        let _command = cmd.into_command();
    }

    #[test]
    fn test_temp_file_creation() {
        let (temp_file, path) = create_temp_file("test_", ".txt", "test content").unwrap();

        // Verify file exists and has correct content
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "test content");

        // File path should still be accessible while temp_file is in scope
        drop(temp_file);
        // After dropping temp_file, the file should be cleaned up
        // Note: On some systems there might be a delay
    }

    #[test]
    fn test_temp_csv_creation() {
        let rows: &[&[&str]] = &[
            &["Header1", "Header2"],
            &["Value1", "Value2"],
            &["Value3", "Value4"],
        ];

        let (temp_file, path) = create_temp_csv("test_", rows).unwrap();

        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        let expected = "Header1,Header2\nValue1,Value2\nValue3,Value4";
        assert_eq!(content, expected);

        drop(temp_file);
    }

    #[test]
    fn test_temp_directories() {
        // Test tempfile TempDir
        let temp_dir = create_temp_dir("test_");
        assert!(temp_dir.path().exists());
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "test").unwrap();
        assert!(test_file.exists());
        // Directory and contents are cleaned up when temp_dir is dropped
        drop(temp_dir);

        // Test assert_fs TempDir
        let assert_temp_dir = create_assert_temp_dir();
        assert!(assert_temp_dir.path().exists());

        // Test that the directory exists
        let test_file = assert_temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "test content").unwrap();
        assert!(test_file.exists());
    }
}
