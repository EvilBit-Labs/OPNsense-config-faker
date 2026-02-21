//! Validate command for configuration data validation
//!
//! This module provides validation functionality for both CSV and XML configuration data,
//! ensuring consistency, correctness, and compliance with OPNsense standards.

use crate::cli::{GlobalArgs, ValidateArgs, ValidationFormat};
use crate::model::ConfigError;
use crate::validate::ValidationEngine;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs;
use std::path::Path;

/// Execute validation with global arguments
pub fn execute_with_global(args: ValidateArgs, global: &GlobalArgs) -> Result<()> {
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
    let mut error_count: u32 = 0;

    if !global.quiet {
        println!("📄 Reading CSV file: {}", args.input.display());
    }

    // Use the existing CSV reader function with proper error chaining
    let configs = crate::io::csv::read_csv(&args.input)
        .with_context(|| format!("Failed to read CSV: {}", args.input.display()))?;

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
                eprintln!("❌ Error in configuration {}: {}", index + 1, e);
            }
        } else {
            valid_configs.push(config.clone());
        }

        pb.inc(1);
    }

    pb.finish_with_message("✅ Validation complete");

    if !global.quiet {
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

    let mut reader = quick_xml::Reader::from_str(&content);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(_) => continue,
            Err(e) => {
                eprintln!("❌ Invalid XML: {}", e);
                return Err(
                    ConfigError::invalid_parameter("input", format!("Invalid XML: {}", e)).into(),
                );
            }
        }
    }

    if !global.quiet {
        println!("✅ File is valid XML");
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
    // Ensure parent directories exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("Failed to create parent directories for {}", path.display())
        })?;
    }

    let report_content = format!(
        "Validation Report\n\
         ================\n\
         \n\
         Valid configurations: {}\n\
         Error count: {}\n\
         \n\
         Valid VLAN configurations:\n",
        configs.len(),
        error_count
    );

    fs::write(path, report_content)
        .with_context(|| format!("writing validation report to {}", path.display()))?;

    Ok(())
}
