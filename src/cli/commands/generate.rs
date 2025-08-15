//! Generate command implementation - unified CSV and XML generation

use crate::cli::{GenerateArgs, GlobalArgs, OutputFormat};
use crate::generator::vlan::generate_vlan_configurations;
use crate::generator::{generate_firewall_rules, FirewallComplexity};
use crate::io::csv::{read_csv, write_csv, write_firewall_rules_csv};
use crate::xml::template::XmlTemplate;
use anyhow::{Context, Result};
use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Execute the generate command with global arguments
pub fn execute_with_global(mut args: GenerateArgs, global: &GlobalArgs) -> Result<()> {
    // Apply global settings to args
    if global.no_color {
        args.no_color = true;
    }

    // Apply global output if specified and not overridden
    if let Some(ref global_output) = global.output {
        if args.output.is_none() {
            args.output = Some(global_output.clone());
        }
    }

    // Handle terminal compatibility
    configure_terminal_with_global(&args, global);

    execute_internal(args, global)
}

/// Execute the generate command (legacy function for backward compatibility)
pub fn execute(args: GenerateArgs) -> Result<()> {
    // Create empty global args for backward compatibility
    let global = GlobalArgs {
        quiet: false,
        no_color: args.no_color,
        output: None,
    };

    execute_with_global(args, &global)
}

/// Internal execution with global context
fn execute_internal(args: GenerateArgs, global: &GlobalArgs) -> Result<()> {
    // Handle terminal compatibility
    configure_terminal_with_global(&args, global);

    // Show header unless quiet
    if !global.quiet {
        println!(
            "{}",
            style("🔧 OPNsense Config Faker - Configuration Generator")
                .bold()
                .blue()
        );
        println!();
    }

    // Handle interactive mode if requested
    let args = if args.interactive {
        handle_interactive_mode(args)?
    } else {
        args
    };

    // Validate arguments based on format
    validate_arguments(&args)?;

    // Validate VLAN ID constraints
    if let Err(e) = args.validate() {
        return Err(crate::model::ConfigError::invalid_parameter("count", &e).into());
    }

    // Execute based on format
    match args.format {
        OutputFormat::Csv => execute_csv_generation(&args, global),
        OutputFormat::Xml => execute_xml_generation(&args, global),
    }
}

/// Configure terminal output based on environment and arguments with global context
fn configure_terminal_with_global(args: &GenerateArgs, global: &GlobalArgs) {
    // Handle TERM=dumb compatibility
    if env::var("TERM").unwrap_or_default() == "dumb" || args.no_color || global.no_color {
        env::set_var("NO_COLOR", "1");
    }
}

/// Handle interactive mode prompts for missing required arguments
fn handle_interactive_mode(mut args: GenerateArgs) -> Result<GenerateArgs> {
    let term = Term::stdout();

    match args.format {
        OutputFormat::Csv => {
            if args.output.is_none() {
                println!("📝 CSV output file not specified.");
                print!("Enter output filename (default: vlan_configs.csv): ");
                io::stdout().flush()?;
                let input = term.read_line()?;
                args.output = Some(if input.trim().is_empty() {
                    PathBuf::from("vlan_configs.csv")
                } else {
                    PathBuf::from(input.trim())
                });
            }
        }
        OutputFormat::Xml => {
            if args.base_config.is_none() {
                println!("📄 Base configuration file required for XML generation.");
                print!("Enter base config file path: ");
                io::stdout().flush()?;
                let input = term.read_line()?;
                if !input.trim().is_empty() {
                    args.base_config = Some(PathBuf::from(input.trim()));
                }
            }

            if args.csv_file.is_none() && args.count == 10 {
                print!("Enter number of configurations to generate (default: 10): ");
                io::stdout().flush()?;
                let input = term.read_line()?;
                if let Ok(count) = input.trim().parse::<u16>() {
                    args.count = count;
                }
            }
        }
    }

    Ok(args)
}

/// Validate arguments based on the selected format
fn validate_arguments(args: &GenerateArgs) -> Result<()> {
    match args.format {
        OutputFormat::Csv => {
            // CSV format requires output file
            if args.output.is_none() {
                return Err(crate::model::ConfigError::invalid_parameter(
                    "output",
                    "Output file path is required for CSV format. Use --output or -o to specify.",
                )
                .into());
            }
        }
        OutputFormat::Xml => {
            // XML format requires base config
            if args.base_config.is_none() {
                return Err(crate::model::ConfigError::invalid_parameter(
                    "base-config",
                    "Base configuration file is required for XML format. Use --base-config or -b to specify."
                ).into());
            }

            // Either count or csv_file must be specified
            if args.csv_file.is_none() && args.count == 0 {
                return Err(crate::model::ConfigError::invalid_parameter(
                    "count or csv-file",
                    "Either --count or --csv-file must be specified for XML generation.",
                )
                .into());
            }
        }
    }

    Ok(())
}

/// Execute CSV generation
fn execute_csv_generation(args: &GenerateArgs, global: &GlobalArgs) -> Result<()> {
    let output_file = args.output.as_ref().unwrap(); // Validated in validate_arguments

    if !global.quiet {
        println!("📊 Generating CSV configuration data...");
    }

    // Check if output file exists and handle force flag
    if output_file.exists() && !args.force {
        return Err(crate::model::ConfigError::config(format!(
            "Output file '{}' already exists. Use --force to overwrite.",
            output_file.display()
        ))
        .into());
    }

    // Set up progress indicator
    let pb = create_progress_bar(args.count as u64, "Generating VLAN configurations...");

    // Generate VLAN configurations
    let configs = generate_vlan_configurations(args.count, args.seed, Some(&pb))
        .with_context(|| format!("Failed to generate {} VLAN configurations", args.count))?;

    pb.set_message("Writing CSV file...");

    // Write to CSV file
    write_csv(&configs, output_file)
        .with_context(|| format!("Failed to write CSV to {:?}", output_file))?;

    pb.finish_with_message(format!(
        "✅ Generated {} VLAN configurations in '{}'",
        configs.len(),
        output_file.display()
    ));

    print_csv_summary(&configs, output_file);

    // Generate firewall rules if requested
    if args.include_firewall_rules {
        println!();
        println!("🔥 Generating firewall rules...");

        // Parse complexity level
        let complexity: FirewallComplexity =
            args.firewall_rule_complexity.parse().map_err(|e| {
                crate::model::ConfigError::validation(format!("Invalid firewall complexity: {}", e))
            })?;

        // Generate firewall rules
        let firewall_pb = create_progress_bar(configs.len() as u64, "Generating firewall rules...");
        let firewall_rules = generate_firewall_rules(
            &configs,
            complexity,
            args.seed,
            Some(&firewall_pb),
            args.firewall_rules_per_vlan,
        )?;

        firewall_pb.finish_with_message(format!(
            "✅ Generated {} firewall rules",
            firewall_rules.len()
        ));

        // Write firewall rules to separate CSV file
        let firewall_output = output_file.with_file_name(format!(
            "{}_firewall_rules.csv",
            output_file.file_stem().unwrap().to_str().unwrap()
        ));

        write_firewall_rules_csv(&firewall_rules, &firewall_output)
            .with_context(|| format!("Failed to write firewall rules to {:?}", firewall_output))?;

        println!(
            "📄 Firewall rules written to: {}",
            firewall_output.display()
        );
        print_firewall_summary(&firewall_rules, &firewall_output);
    }

    Ok(())
}

/// Execute XML generation
fn execute_xml_generation(args: &GenerateArgs, global: &GlobalArgs) -> Result<()> {
    let base_config = args.base_config.as_ref().unwrap(); // Validated in validate_arguments

    if !global.quiet {
        println!("🔧 Generating OPNsense XML configuration...");
    }

    // Validate base configuration file exists
    if !base_config.exists() {
        return Err(crate::model::ConfigError::ConfigNotFound {
            path: base_config.display().to_string(),
        }
        .into());
    }

    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir)?;
    }

    // Generate or load VLAN configurations
    let configs = if let Some(csv_file) = &args.csv_file {
        println!("📄 Loading configurations from CSV: {}", csv_file.display());
        read_csv(csv_file).with_context(|| format!("Failed to read CSV file: {:?}", csv_file))?
    } else {
        println!("🔄 Generating {} VLAN configurations...", args.count);

        let pb = create_progress_bar(args.count as u64, "Generating configurations...");
        let configs = generate_vlan_configurations(args.count, args.seed, Some(&pb))
            .with_context(|| format!("Failed to generate {} VLAN configurations", args.count))?;
        pb.finish_with_message("✅ Configurations generated");
        configs
    };

    println!("📝 Processing {} configurations...", configs.len());

    // Generate firewall rules if requested
    let firewall_rules = if args.include_firewall_rules {
        println!("🔥 Generating firewall rules...");

        // Parse complexity level
        let complexity: FirewallComplexity =
            args.firewall_rule_complexity.parse().map_err(|e| {
                crate::model::ConfigError::validation(format!("Invalid firewall complexity: {}", e))
            })?;

        // Generate firewall rules
        let firewall_pb = create_progress_bar(configs.len() as u64, "Generating firewall rules...");
        let rules = generate_firewall_rules(
            &configs,
            complexity,
            args.seed,
            Some(&firewall_pb),
            args.firewall_rules_per_vlan,
        )?;

        firewall_pb.finish_with_message(format!("✅ Generated {} firewall rules", rules.len()));

        // Write firewall rules to CSV for reference
        let firewall_csv = args
            .output_dir
            .join(format!("firewall_{}_rules.csv", args.firewall_nr));
        write_firewall_rules_csv(&rules, &firewall_csv)?;
        println!("📄 Firewall rules CSV: {}", firewall_csv.display());

        Some(rules)
    } else {
        None
    };

    // Load base XML template
    let base_xml = fs::read_to_string(base_config)
        .with_context(|| format!("Failed to read base config file: {:?}", base_config))?;
    let mut template = XmlTemplate::new(base_xml)
        .with_context(|| "Failed to create XML template from base configuration")?;

    // Set up progress for XML generation
    let pb = create_progress_bar(configs.len() as u64, "Generating XML configurations...");

    // Generate XML configurations
    for (index, config) in configs.iter().enumerate() {
        pb.set_message(format!("Processing VLAN {}", config.vlan_id));

        // Generate XML for this configuration
        let output_xml = template.apply_configuration(
            config,
            args.firewall_nr,
            args.opt_counter + index as u16,
        )?;

        // Write output file
        let output_file = args.output_dir.join(format!(
            "firewall_{}_vlan_{}.xml",
            args.firewall_nr, config.vlan_id
        ));

        if output_file.exists() && !args.force {
            return Err(crate::model::ConfigError::config(format!(
                "Output file '{}' already exists. Use --force to overwrite.",
                output_file.display()
            ))
            .into());
        }

        fs::write(&output_file, output_xml)?;
        pb.inc(1);
    }

    pb.finish_with_message("✅ XML configurations generated");

    print_xml_summary(&configs, &args.output_dir, args.firewall_nr);

    // Print firewall summary if rules were generated
    if let Some(ref rules) = firewall_rules {
        let firewall_csv = args
            .output_dir
            .join(format!("firewall_{}_rules.csv", args.firewall_nr));
        print_firewall_summary(rules, &firewall_csv);
    }

    Ok(())
}

/// Create a progress bar with consistent styling
fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);

    // Check if we should disable progress bar for non-interactive terminals
    if env::var("NO_COLOR").is_ok() || env::var("TERM").unwrap_or_default() == "dumb" {
        pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    } else {
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
    }

    pb.set_message(message.to_string());
    pb
}

/// Print summary for CSV generation
fn print_csv_summary(configs: &[crate::generator::vlan::VlanConfig], output_file: &Path) {
    println!();
    println!("{}", style("Summary:").bold());
    println!("  📊 Configurations: {}", configs.len());
    println!("  📁 Output file: {}", output_file.display());
    if !configs.is_empty() {
        println!(
            "  🏷️  VLAN IDs: {} - {}",
            configs.iter().map(|c| c.vlan_id).min().unwrap_or(0),
            configs.iter().map(|c| c.vlan_id).max().unwrap_or(0)
        );
    }
}

/// Print summary for XML generation
fn print_xml_summary(
    configs: &[crate::generator::vlan::VlanConfig],
    output_dir: &Path,
    firewall_nr: u16,
) {
    println!();
    println!("{}", style("Summary:").bold());
    println!("  📊 Configurations: {}", configs.len());
    println!("  📁 Output directory: {}", output_dir.display());
    if !configs.is_empty() {
        println!(
            "  🏷️  VLAN IDs: {} - {}",
            configs.iter().map(|c| c.vlan_id).min().unwrap_or(0),
            configs.iter().map(|c| c.vlan_id).max().unwrap_or(0)
        );
    }
    println!("  🔧 Firewall number: {firewall_nr}");
}

/// Print summary for firewall rule generation
fn print_firewall_summary(rules: &[crate::generator::FirewallRule], output_file: &Path) {
    println!();
    println!("{}", style("Firewall Rules Summary:").bold());
    println!("  🔥 Total rules: {}", rules.len());
    println!("  📁 Output file: {}", output_file.display());

    // Count rules by action
    let pass_count = rules
        .iter()
        .filter(|r| r.action.to_lowercase() == "pass")
        .count();
    let block_count = rules
        .iter()
        .filter(|r| r.action.to_lowercase() == "block")
        .count();
    let reject_count = rules
        .iter()
        .filter(|r| r.action.to_lowercase() == "reject")
        .count();

    println!("  ✅ Pass rules: {}", pass_count);
    println!("  🚫 Block rules: {}", block_count);
    println!("  ❌ Reject rules: {}", reject_count);

    // Count rules by VLAN
    let vlan_count = rules
        .iter()
        .filter_map(|r| r.vlan_id)
        .collect::<std::collections::HashSet<_>>()
        .len();
    println!("  🏷️  VLANs with rules: {}", vlan_count);
}
