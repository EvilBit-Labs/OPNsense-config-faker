//! Validate command for configuration data validation
//!
//! This module provides validation functionality for both CSV and XML configuration data,
//! ensuring consistency, correctness, and compliance with OPNsense standards.

use crate::cli::{GlobalArgs, ValidateArgs, ValidationFormat};
use crate::model::ConfigError;
use crate::validate::ValidationEngine;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs;
use std::path::Path;

/// Execute validation with global arguments
pub fn execute_with_global(args: ValidateArgs, global: &GlobalArgs) -> Result<()> {
    // Apply global settings
    configure_terminal(global);

    execute(args, global)
}

/// Execute validation command
pub fn execute(args: ValidateArgs, global: &GlobalArgs) -> Result<()> {
    if !global.quiet {
        println!("🔍 OPNsense Config Faker - Validation Mode");
        println!();
    }

    // Determine input format
    let format = determine_format(&args.input, &args.format)?;

    if !global.quiet {
        println!("📂 Input: {}", args.input.display());
        println!("📝 Format: {:?}", format);
        if args.verbose {
            println!("📊 Max errors: {}", args.max_errors);
        }
        println!();
    }

    // Validate based on format
    match format {
        ValidationFormat::Csv => validate_csv(&args, global),
        ValidationFormat::Xml => validate_xml(&args, global),
        ValidationFormat::Auto => Err(ConfigError::invalid_parameter(
            "format",
            "Could not automatically determine format. Please specify --format csv or --format xml",
        )
        .into()),
    }
}

/// Validate CSV configuration data
fn validate_csv(args: &ValidateArgs, global: &GlobalArgs) -> Result<()> {
    let mut engine = ValidationEngine::new();
    let mut error_count = 0;

    if !global.quiet {
        println!("📄 Reading CSV file: {}", args.input.display());
    }

    // Use the existing CSV reader function
    let configs = match crate::io::csv::read_csv(&args.input) {
        Ok(configs) => configs,
        Err(e) => {
            println!("❌ Failed to read CSV: {}", e);
            return Err(e);
        }
    };

    if !global.quiet {
        println!(
            "✅ Successfully loaded {} configurations from CSV",
            configs.len()
        );
    }

    // Create progress bar
    let pb = if !global.quiet {
        create_progress_bar("Validating configurations")
    } else {
        ProgressBar::hidden()
    };

    let mut valid_configs = Vec::new();
    for (index, config) in configs.iter().enumerate() {
        if error_count >= args.max_errors {
            if !global.quiet {
                println!(
                    "⚠️  Reached maximum error limit ({}). Stopping validation.",
                    args.max_errors
                );
            }
            break;
        }

        if let Err(e) = engine.validate_config(config) {
            error_count += 1;
            if args.verbose || !global.quiet {
                println!("❌ Configuration {}: {}", index + 1, e);
            }
        } else {
            valid_configs.push(config.clone());
        }

        pb.inc(1);
    }

    pb.finish_and_clear();

    // Report results
    if !global.quiet {
        println!();
        println!("📊 Validation Results:");
        println!("  ✅ Valid configurations: {}", valid_configs.len());
        println!("  ❌ Errors found: {}", error_count);

        if error_count == 0 {
            println!("🎉 All configurations are valid!");
        } else {
            println!("⚠️  Found {} validation errors", error_count);
        }
    }

    // Write report if requested
    if let Some(report_path) = &args.report {
        write_validation_report(report_path, &valid_configs, error_count)?;
        if !global.quiet {
            println!("📄 Validation report written to: {}", report_path.display());
        }
    }

    if error_count > 0 {
        return Err(ConfigError::config(format!(
            "Validation failed: {} error(s) found",
            error_count
        ))
        .into());
    }

    Ok(())
}

/// Validate XML configuration data
fn validate_xml(args: &ValidateArgs, global: &GlobalArgs) -> Result<()> {
    if !global.quiet {
        println!("🔍 XML validation not yet implemented");
        println!("📝 This feature will validate OPNsense XML configuration files");
    }

    // For now, just verify the file is valid XML
    let content = fs::read_to_string(&args.input)?;

    match quick_xml::Reader::from_str(&content).read_event() {
        Ok(_) => {
            if !global.quiet {
                println!("✅ File is valid XML");
            }
        }
        Err(e) => {
            println!("❌ Invalid XML: {}", e);
            return Err(
                ConfigError::invalid_parameter("input", format!("Invalid XML: {}", e)).into(),
            );
        }
    }

    Ok(())
}

/// Determine input format from file extension or explicit format
fn determine_format(input: &Path, format: &ValidationFormat) -> Result<ValidationFormat> {
    match format {
        ValidationFormat::Auto => match input.extension().and_then(|ext| ext.to_str()) {
            Some("csv") => Ok(ValidationFormat::Csv),
            Some("xml") => Ok(ValidationFormat::Xml),
            _ => Err(ConfigError::invalid_parameter(
                "input",
                "Could not determine format from file extension. Please specify --format",
            )
            .into()),
        },
        other => Ok(other.clone()),
    }
}

/// Configure terminal output based on global settings
fn configure_terminal(global: &GlobalArgs) {
    if global.no_color
        || env::var("NO_COLOR").is_ok()
        || env::var("TERM").unwrap_or_default() == "dumb"
    {
        env::set_var("NO_COLOR", "1");
    }
}
/// Create a progress bar with consistent styling
fn create_progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    if env::var("NO_COLOR").is_ok() || env::var("TERM").unwrap_or_default() == "dumb" {
        pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    } else {
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {elapsed_precise} {msg}")
                .unwrap(),
        );
    }

    pb.set_message(message.to_string());
    pb
}

/// Write validation report to file
fn write_validation_report(
    path: &Path,
    configs: &[crate::generator::VlanConfig],
    error_count: u32,
) -> Result<()> {
    use std::io::Write;

    let mut file = fs::File::create(path)?;

    writeln!(file, "# OPNsense Config Faker - Validation Report")?;
    writeln!(
        file,
        "Generated: {}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )?;
    writeln!(file)?;
    writeln!(file, "## Summary")?;
    writeln!(file, "- Valid configurations: {}", configs.len())?;
    writeln!(file, "- Errors found: {}", error_count)?;
    writeln!(file)?;

    if !configs.is_empty() {
        writeln!(file, "## Valid Configurations")?;
        writeln!(file, "| VLAN ID | IP Network | Description | WAN |")?;
        writeln!(file, "|---------|------------|-------------|-----|")?;

        for config in configs {
            writeln!(
                file,
                "| {} | {} | {} | {} |",
                config.vlan_id, config.ip_network, config.description, config.wan_assignment
            )?;
        }
    }

    Ok(())
}
