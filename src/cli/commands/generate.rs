//! Generate command implementation - unified CSV and XML generation

use crate::cli::{GenerateArgs, GlobalArgs, OutputFormat};
use crate::generator::vlan::generate_vlan_configurations;
use crate::generator::{FirewallComplexity, generate_firewall_rules};
use crate::io::csv::{read_csv, write_csv, write_firewall_rules_csv};
use crate::xml::template::XmlTemplate;
use anyhow::{Context, Result};
use console::{Term, style};
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
        // Also apply to XML output_dir when format is XML
        if matches!(args.format, OutputFormat::Xml) {
            args.output_dir = global_output.clone();
        }
    }

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

    // Generate VLAN configurations based on range or count
    let (configs, pb) = if let Some(ref vlan_range_str) = args.vlan_range {
        // Parse VLAN ranges
        let vlan_ranges = crate::cli::parse_vlan_range(vlan_range_str)
            .map_err(crate::model::ConfigError::validation)?;

        let total_vlans: u16 = vlan_ranges.iter().map(|(start, end)| end - start + 1).sum();

        if !global.quiet {
            println!(
                "📋 Using VLAN ranges: {} (total: {} VLANs)",
                vlan_range_str, total_vlans
            );
        }

        // Set up progress indicator
        let pb = create_progress_bar(
            total_vlans as u64,
            "Generating VLAN configurations from ranges...",
            global.quiet,
        );

        // Generate from ranges
        let configs = if args.wan_assignments.is_some() {
            crate::generator::vlan::generate_vlan_configurations_from_ranges_with_wan(
                &vlan_ranges,
                args.seed,
                args.wan_assignments.as_ref(),
                Some(&pb),
            )
        } else {
            crate::generator::vlan::generate_vlan_configurations_from_ranges(
                &vlan_ranges,
                args.seed,
                Some(&pb),
            )
        }
        .with_context(|| {
            format!(
                "Failed to generate VLAN configurations from ranges: {}",
                vlan_range_str
            )
        })?;

        (configs, pb)
    } else {
        // Set up progress indicator
        let pb = create_progress_bar(
            args.count as u64,
            "Generating VLAN configurations...",
            global.quiet,
        );

        // Generate VLAN configurations by count
        let configs = if args.wan_assignments.is_some() {
            crate::generator::vlan::generate_vlan_configurations_with_wan(
                args.count,
                args.seed,
                args.wan_assignments.as_ref(),
                Some(&pb),
            )
        } else {
            generate_vlan_configurations(args.count, args.seed, Some(&pb))
        }
        .with_context(|| format!("Failed to generate {} VLAN configurations", args.count))?;

        (configs, pb)
    };

    pb.set_message("Writing CSV file...");

    // Write to CSV file
    write_csv(&configs, output_file)
        .with_context(|| format!("Failed to write CSV to {:?}", output_file))?;

    pb.finish_with_message(format!(
        "✅ Generated {} VLAN configurations in '{}'",
        configs.len(),
        output_file.display()
    ));

    if !global.quiet {
        print_csv_summary(&configs, output_file);
    }

    // Generate VPN configurations if requested
    if let Some(vpn_count) = args.vpn_count {
        if !global.quiet {
            println!();
            println!("🔒 Generating VPN configurations...");
        }

        let vpn_pb = create_progress_bar(
            vpn_count as u64,
            "Generating VPN configurations...",
            global.quiet,
        );

        let vpn_configs =
            crate::generator::vpn::generate_vpn_configurations(vpn_count, args.seed, Some(&vpn_pb))
                .with_context(|| format!("Failed to generate {} VPN configurations", vpn_count))?;

        vpn_pb.finish_with_message(format!(
            "✅ Generated {} VPN configurations",
            vpn_configs.len()
        ));

        // TODO: Write VPN configurations to CSV (not yet implemented)
        if !global.quiet {
            println!(
                "ℹ️  VPN CSV export not yet implemented ({} configs generated in memory)",
                vpn_configs.len()
            );
        }
    }

    // Generate NAT mappings if requested
    if let Some(nat_count) = args.nat_mappings {
        if !global.quiet {
            println!();
            println!("🔗 Generating NAT mappings...");
        }

        let nat_pb =
            create_progress_bar(nat_count as u64, "Generating NAT mappings...", global.quiet);

        let nat_mappings =
            crate::generator::nat::generate_nat_mappings(nat_count, args.seed, Some(&nat_pb))
                .with_context(|| format!("Failed to generate {} NAT mappings", nat_count))?;

        nat_pb.finish_with_message(format!("✅ Generated {} NAT mappings", nat_mappings.len()));

        // TODO: Write NAT mappings to CSV (not yet implemented)
        if !global.quiet {
            println!(
                "ℹ️  NAT CSV export not yet implemented ({} mappings generated in memory)",
                nat_mappings.len()
            );
        }
    }

    // Generate firewall rules if requested
    if args.include_firewall_rules {
        if !global.quiet {
            println!();
            println!("🔥 Generating firewall rules...");
        }

        // Parse complexity level
        let complexity: FirewallComplexity =
            args.firewall_rule_complexity.parse().map_err(|e| {
                crate::model::ConfigError::validation(format!("Invalid firewall complexity: {}", e))
            })?;

        // Generate firewall rules
        let firewall_pb = create_progress_bar(
            configs.len() as u64,
            "Generating firewall rules...",
            global.quiet,
        );
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

        if !global.quiet {
            println!(
                "📄 Firewall rules written to: {}",
                firewall_output.display()
            );
            print_firewall_summary(&firewall_rules, &firewall_output);
        }
    }

    Ok(())
}

/// Execute XML generation
fn execute_xml_generation(args: &GenerateArgs, global: &GlobalArgs) -> Result<()> {
    let base_config = args.base_config.as_ref().unwrap(); // Validated in validate_arguments

    if !global.quiet {
        println!("🔧 Generating OPNsense XML configuration...");
    }

    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir)?;
    }

    // Generate or load VLAN configurations
    let configs = if let Some(csv_file) = &args.csv_file {
        if !global.quiet {
            println!("📄 Loading configurations from CSV: {}", csv_file.display());
        }
        read_csv(csv_file).with_context(|| format!("Failed to read CSV file: {:?}", csv_file))?
    } else if let Some(ref vlan_range_str) = args.vlan_range {
        // Parse VLAN ranges
        let vlan_ranges = crate::cli::parse_vlan_range(vlan_range_str)
            .map_err(crate::model::ConfigError::validation)?;

        let total_vlans: u16 = vlan_ranges.iter().map(|(start, end)| end - start + 1).sum();

        if !global.quiet {
            println!(
                "📋 Using VLAN ranges: {} (total: {} VLANs)",
                vlan_range_str, total_vlans
            );
        }

        let pb = create_progress_bar(
            total_vlans as u64,
            "Generating configurations from ranges...",
            global.quiet,
        );

        let configs = if args.wan_assignments.is_some() {
            crate::generator::vlan::generate_vlan_configurations_from_ranges_with_wan(
                &vlan_ranges,
                args.seed,
                args.wan_assignments.as_ref(),
                Some(&pb),
            )
        } else {
            crate::generator::vlan::generate_vlan_configurations_from_ranges(
                &vlan_ranges,
                args.seed,
                Some(&pb),
            )
        }
        .with_context(|| {
            format!(
                "Failed to generate VLAN configurations from ranges: {}",
                vlan_range_str
            )
        })?;

        pb.finish_with_message("✅ Configurations generated from ranges");
        configs
    } else {
        if !global.quiet {
            println!("🔄 Generating {} VLAN configurations...", args.count);
        }

        let pb = create_progress_bar(
            args.count as u64,
            "Generating configurations...",
            global.quiet,
        );

        let configs = if args.wan_assignments.is_some() {
            crate::generator::vlan::generate_vlan_configurations_with_wan(
                args.count,
                args.seed,
                args.wan_assignments.as_ref(),
                Some(&pb),
            )
        } else {
            generate_vlan_configurations(args.count, args.seed, Some(&pb))
        }
        .with_context(|| format!("Failed to generate {} VLAN configurations", args.count))?;

        pb.finish_with_message("✅ Configurations generated");
        configs
    };

    if !global.quiet {
        println!("📝 Processing {} configurations...", configs.len());
    }

    // Generate firewall rules if requested
    let firewall_rules = if args.include_firewall_rules {
        if !global.quiet {
            println!("🔥 Generating firewall rules...");
        }

        // Parse complexity level
        let complexity: FirewallComplexity =
            args.firewall_rule_complexity.parse().map_err(|e| {
                crate::model::ConfigError::validation(format!("Invalid firewall complexity: {}", e))
            })?;

        // Generate firewall rules
        let firewall_pb = create_progress_bar(
            configs.len() as u64,
            "Generating firewall rules...",
            global.quiet,
        );
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
        if !global.quiet {
            println!("📄 Firewall rules CSV: {}", firewall_csv.display());
        }

        Some(rules)
    } else {
        None
    };

    // Load base XML template
    let base_xml = fs::read_to_string(base_config)
        .with_context(|| format!("Failed to read base config file: {:?}", base_config))?;
    let template = XmlTemplate::new(base_xml)
        .with_context(|| "Failed to create XML template from base configuration")?;

    // Set up progress for XML generation
    let pb = create_progress_bar(
        configs.len() as u64,
        "Generating XML configurations...",
        global.quiet,
    );

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

    if !global.quiet {
        print_xml_summary(&configs, &args.output_dir, args.firewall_nr);
    }

    // Print firewall summary if rules were generated
    if let Some(ref rules) = firewall_rules {
        let firewall_csv = args
            .output_dir
            .join(format!("firewall_{}_rules.csv", args.firewall_nr));
        if !global.quiet {
            print_firewall_summary(rules, &firewall_csv);
        }
    }

    Ok(())
}

/// Create a progress bar with consistent styling
fn create_progress_bar(total: u64, message: &str, quiet: bool) -> ProgressBar {
    if quiet {
        return ProgressBar::hidden();
    }

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
