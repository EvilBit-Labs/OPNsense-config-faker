//! XML generation command implementation

use crate::Result;
use crate::cli::XmlArgs;
use crate::generator::vlan::generate_vlan_configurations;
use crate::io::csv::read_csv;
use crate::xml::template::XmlTemplate;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

/// Execute the XML generation command
pub fn execute(args: XmlArgs) -> Result<()> {
    println!(
        "{}",
        style("🔧 OPNsense Config Faker - XML Generator")
            .bold()
            .blue()
    );
    println!();

    // Validate base configuration file exists
    if !args.base_config.exists() {
        return Err(crate::model::ConfigError::ConfigNotFound {
            path: args.base_config.display().to_string(),
        });
    }

    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir)?;
    }

    // Generate or load VLAN configurations
    let configs = if let Some(csv_file) = &args.csv_file {
        println!("📄 Loading configurations from CSV: {}", csv_file.display());
        read_csv(csv_file)?
    } else if let Some(count) = args.count {
        println!("🔄 Generating {count} VLAN configurations...");

        let pb = ProgressBar::new(count as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Generating configurations...");

        let configs = generate_vlan_configurations(count, args.seed, Some(&pb))?;
        pb.finish_with_message("✅ Configurations generated");
        configs
    } else {
        return Err(crate::model::ConfigError::invalid_parameter(
            "count or csv_file",
            "Either --count or --csv-file must be specified",
        ));
    };

    println!("📝 Processing {} configurations...", configs.len());

    // Load base XML template
    let base_xml = fs::read_to_string(&args.base_config)?;
    let template = XmlTemplate::new(base_xml)?;

    // Set up progress for XML generation
    let pb = ProgressBar::new(configs.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Generating XML configurations...");

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
            )));
        }

        fs::write(&output_file, output_xml)?;
        pb.inc(1);
    }

    pb.finish_with_message("✅ XML configurations generated");

    println!();
    println!("{}", style("Summary:").bold());
    println!("  📊 Configurations: {}", configs.len());
    println!("  📁 Output directory: {}", args.output_dir.display());
    println!(
        "  🏷️  VLAN IDs: {} - {}",
        configs.iter().map(|c| c.vlan_id).min().unwrap_or(0),
        configs.iter().map(|c| c.vlan_id).max().unwrap_or(0)
    );
    println!("  🔧 Firewall number: {}", args.firewall_nr);

    Ok(())
}
