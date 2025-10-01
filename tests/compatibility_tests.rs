//! Compatibility tests for OPNsense Config Faker
//!
//! These tests ensure the CLI works correctly across different environments,
//! terminals, and system configurations.

mod common;

use common::{TestOutputExt, cli_command, create_temp_dir};

/// Test CLI behavior with TERM=dumb environment (no colors/formatting)
#[test]
fn test_dumb_terminal_compatibility() {
    let temp_dir = create_temp_dir("dumb_terminal_test");
    let output_file = temp_dir.path().join("test_output.csv");

    let output = cli_command()
        .env("TERM", "dumb")
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

    // Output should be clean with no ANSI sequences
    let stdout = &output.stdout;
    assert!(
        !stdout.contains("\u{001b}["),
        "ANSI escape sequences found in TERM=dumb output"
    );

    // But content should still be present
    output.assert_vlan_generation_success(3);
}

/// Test CLI behavior with NO_COLOR environment variable
#[test]
fn test_no_color_compatibility() {
    let temp_dir = create_temp_dir("no_color_test");
    let output_file = temp_dir.path().join("test_output.csv");

    let output = cli_command()
        .env("NO_COLOR", "1")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("2")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("123")
        .run_success();

    // Should have no color codes
    let stdout = &output.stdout;
    assert!(
        !stdout.contains("\u{001b}[3"),
        "Color codes found in NO_COLOR output"
    );

    output.assert_vlan_generation_success(2);
}

/// Test CLI behavior with --no-color flag
#[test]
fn test_explicit_no_color_flag() {
    let temp_dir = create_temp_dir("explicit_no_color_test");
    let output_file = temp_dir.path().join("test_output.csv");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--no-color")
        .arg("--count")
        .arg("2")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("456")
        .run_success();

    // Should explicitly disable colors
    let stdout = &output.stdout;
    assert!(
        !stdout.contains("\u{001b}["),
        "ANSI codes found with --no-color flag"
    );

    output.assert_vlan_generation_success(2);
}

/// Test behavior in different working directories
#[test]
fn test_working_directory_independence() {
    let temp_dir = create_temp_dir("working_dir_test");
    let output_file = temp_dir.path().join("output.csv");

    // Test from temp directory
    let output = cli_command()
        .current_dir(&temp_dir)
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1")
        .arg("--output")
        .arg("output.csv") // Relative path
        .arg("--seed")
        .arg("789")
        .run_success();

    output.assert_vlan_generation_success(1);
    assert!(
        output_file.exists(),
        "Output file not created in working directory"
    );
}

/// Test Unicode/UTF-8 compatibility in paths and output
#[test]
fn test_unicode_compatibility() {
    let temp_dir = create_temp_dir("unicode_test_");

    // Create a file with Unicode characters in the name
    let unicode_filename = "测试_файл_ファイル.csv";
    let output_file = temp_dir.path().join(unicode_filename);

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("101")
        .run_success();

    output.assert_vlan_generation_success(1);
    assert!(
        output_file.exists(),
        "Unicode filename not handled correctly"
    );
}

/// Test behavior with very long file paths
#[test]
fn test_long_path_compatibility() {
    let temp_dir = create_temp_dir("long_path_test_");

    // Create a deeply nested directory structure
    let long_subdir = "very/long/directory/path/with/many/nested/subdirectories";
    let full_dir = temp_dir.path().join(long_subdir);
    std::fs::create_dir_all(&full_dir).unwrap();

    let output_file = full_dir.join("deep_output.csv");

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("202")
        .run_success();

    output.assert_vlan_generation_success(1);
    assert!(output_file.exists(), "Long path not handled correctly");
}

/// Test behavior with various character encodings in environment
#[test]
fn test_encoding_compatibility() {
    // Test with different locale settings
    let temp_dir = create_temp_dir("encoding_test_");
    let output_file = temp_dir.path().join("encoding_test.csv");

    // Test with UTF-8 locale
    let output = cli_command()
        .env("LC_ALL", "en_US.UTF-8")
        .env("LANG", "en_US.UTF-8")
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1")
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("303")
        .run_success();

    output.assert_vlan_generation_success(1);

    // Verify the CSV content can be read properly
    let content = std::fs::read_to_string(&output_file).unwrap();
    assert!(content.contains("VLAN"));
    assert!(content.contains("IP Range"));
    assert!(content.contains("Beschreibung")); // German text should work
}

/// Test error handling across different environments
#[test]
fn test_error_handling_compatibility() {
    // Test with non-existent directory
    let nonexistent_path = "/this/path/does/not/exist/output.csv";

    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1")
        .arg("--output")
        .arg(nonexistent_path)
        .arg("--seed")
        .arg("404")
        .run_failure();

    // Error should be normalized and consistent across platforms
    let normalized_stderr = output.normalized_stderr();

    // Should contain some indication of file/directory error
    assert!(
        normalized_stderr.contains("No such file")
            || normalized_stderr.contains("cannot create")
            || normalized_stderr.contains("Permission denied")
            || normalized_stderr.contains("Invalid parameter"),
        "Expected file error message not found: {normalized_stderr}"
    );
}

/// Test memory and resource usage with larger datasets
#[test]
fn test_resource_usage_compatibility() {
    let temp_dir = create_temp_dir("resource_test_");
    let output_file = temp_dir.path().join("large_output.csv");

    // Generate a larger dataset to test resource handling
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("1000") // Larger count
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("505")
        .run_success();

    output.assert_vlan_generation_success(1000);

    // Verify file was created and has reasonable size
    let metadata = std::fs::metadata(&output_file).unwrap();
    assert!(metadata.len() > 1000, "Generated file seems too small");
    assert!(
        metadata.len() < 10_000_000,
        "Generated file seems unreasonably large"
    );
}

/// Test that large operations complete successfully (resource usage)
#[test]
fn test_large_operation_completion() {
    let temp_dir = create_temp_dir("large_op_test_");
    let output_file = temp_dir.path().join("large_output.csv");

    // Test a reasonably large operation that should complete
    let output = cli_command()
        .arg("generate")
        .arg("--format")
        .arg("csv")
        .arg("--count")
        .arg("100") // Reasonable count for testing
        .arg("--output")
        .arg(&output_file)
        .arg("--seed")
        .arg("606")
        .run_success();

    output.assert_vlan_generation_success(100);
    assert!(output_file.exists());

    // Verify reasonable file size
    let metadata = std::fs::metadata(&output_file).unwrap();
    assert!(metadata.len() > 100, "Generated file seems too small");
    assert!(
        metadata.len() < 1_000_000,
        "Generated file seems unreasonably large"
    );
}

/// Test concurrent execution compatibility
#[test]
fn test_concurrent_execution_compatibility() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    // Run multiple CLI instances concurrently
    for i in 0..4 {
        let success_count_clone = Arc::clone(&success_count);
        let handle = thread::spawn(move || {
            let temp_dir = create_temp_dir(&format!("concurrent_test_{i}_"));
            let output_file = temp_dir.path().join("concurrent_output.csv");

            let result = cli_command()
                .arg("generate")
                .arg("--format")
                .arg("csv")
                .arg("--count")
                .arg("5")
                .arg("--output")
                .arg(&output_file)
                .arg("--seed")
                .arg((700 + i).to_string())
                .run_and_capture();

            if result.success && output_file.exists() {
                success_count_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // All concurrent executions should succeed
    assert_eq!(
        success_count.load(Ordering::SeqCst),
        4,
        "Not all concurrent executions succeeded"
    );
}

/// Test CLI behavior with different shell environments
#[test]
fn test_shell_environment_compatibility() {
    let temp_dir = create_temp_dir("shell_test_");
    let output_file = temp_dir.path().join("shell_test.csv");

    // Test with different shell-like environment variables
    let test_cases = vec![
        ("SHELL", "/bin/bash"),
        ("SHELL", "/bin/zsh"),
        ("SHELL", "/bin/fish"),
        ("SHELL", "/bin/dash"),
    ];

    for (env_key, env_value) in test_cases {
        let output = cli_command()
            .env(env_key, env_value)
            .arg("generate")
            .arg("--format")
            .arg("csv")
            .arg("--count")
            .arg("1")
            .arg("--output")
            .arg(&output_file)
            .arg("--force") // Overwrite previous test output
            .arg("--seed")
            .arg("808")
            .run_success();

        output.assert_vlan_generation_success(1);
        assert!(output_file.exists());

        // Clean up for next iteration
        std::fs::remove_file(&output_file).unwrap();
    }
}

/// Test with various terminal width settings
#[test]
fn test_terminal_width_compatibility() {
    let temp_dir = create_temp_dir("width_test_");
    let output_file = temp_dir.path().join("width_test.csv");

    // Test with different terminal widths
    let widths = vec!["40", "80", "120", "200"];

    for width in widths {
        let output = cli_command()
            .env("COLUMNS", width)
            .env("TERM", "xterm") // Ensure we have a "smart" terminal for width tests
            .arg("generate")
            .arg("--format")
            .arg("csv")
            .arg("--count")
            .arg("2")
            .arg("--output")
            .arg(&output_file)
            .arg("--force")
            .arg("--seed")
            .arg("909")
            .run_success();

        output.assert_vlan_generation_success(2);

        // Clean up
        std::fs::remove_file(&output_file).unwrap();
    }
}
