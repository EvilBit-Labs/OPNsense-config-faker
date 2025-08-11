//! CSV generation command implementation

use crate::cli::CsvArgs;
use crate::generator::vlan::generate_vlan_configurations;
use crate::io::csv::write_csv;
use crate::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

/// Execute the CSV generation command
pub fn execute(args: CsvArgs) -> Result<()> {
    println!("{}", style("🔧 OPNsense Config Faker - CSV Generator").bold().blue());
    println!();

    // Check if output file exists and handle force flag
    if args.output.exists() && !args.force {
        return Err(crate::model::ConfigError::config(format!(
            "Output file '{}' already exists. Use --force to overwrite.",
            args.output.display()
        )));
    }

    // Set up progress indicator
    let pb = ProgressBar::new(args.count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Generating VLAN configurations...");

    // Generate VLAN configurations
    let configs = generate_vlan_configurations(args.count, args.seed, Some(&pb))?;
    
    pb.set_message("Writing CSV file...");
    
    // Write to CSV file
    write_csv(&configs, &args.output)?;
    
    pb.finish_with_message(format!(
        "✅ Generated {} VLAN configurations in '{}'",
        configs.len(),
        args.output.display()
    ));

    println!();
    println!("{}", style("Summary:").bold());
    println!("  📊 Configurations: {}", configs.len());
    println!("  📁 Output file: {}", args.output.display());
    println!("  🏷️  VLAN IDs: {} - {}", 
        configs.iter().map(|c| c.vlan_id).min().unwrap_or(0),
        configs.iter().map(|c| c.vlan_id).max().unwrap_or(0)
    );

    Ok(())
}