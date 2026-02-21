//! Command-line interface for OPNsense Config Faker

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

pub mod commands;
pub mod error;

/// Maximum number of unique VLAN IDs that can be generated
/// VLAN IDs range from 10-4094, giving us 4085 unique values
pub const MAX_UNIQUE_VLAN_IDS: u16 = 4085;

/// OPNsense Config Faker - Generate realistic network configuration test data
#[derive(Parser)]
#[command(name = "opnsense-config-faker")]
#[command(
    about = "A flexible tool for generating realistic network configuration test data for OPNsense"
)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "EvilBit Labs <contact@evilbitlabs.com>")]
#[command(after_help = r#"Examples:
  Generate CSV configuration data:
    opnsense-config-faker generate --count 25 --format csv --output my-config.csv

  Generate OPNsense XML configuration:
    opnsense-config-faker generate --count 25 --format xml --base-config config.xml

  Generate XML from existing CSV:
    opnsense-config-faker generate --format xml --base-config config.xml --csv-file data.csv

  Generate configurations with firewall rules:
    opnsense-config-faker generate --count 25 --format csv --output config.csv --include-firewall-rules

  Generate advanced firewall rules:
    opnsense-config-faker generate --count 10 --format xml --base-config config.xml --include-firewall-rules --firewall-rule-complexity advanced

  Generate from VLAN ranges:
    opnsense-config-faker generate --format csv --vlan-range "100-150,200-250" --output vlans.csv

  Generate with VPN configurations:
    opnsense-config-faker generate --count 10 --vpn-count 3 --format csv --output configs.csv

  Generate with NAT mappings:
    opnsense-config-faker generate --count 15 --nat-mappings 5 --format csv --output network.csv

  Generate with balanced WAN assignments:
    opnsense-config-faker generate --count 12 --wan-assignments balanced --format csv --output balanced.csv

  Generate comprehensive configuration:
    opnsense-config-faker generate --vlan-range "100-120" --vpn-count 2 --nat-mappings 3 --wan-assignments multi --format csv --output complete.csv

  Force overwrite existing files:
    opnsense-config-faker generate --count 10 --format csv --output test.csv --force

  Generate shell completions:
    opnsense-config-faker completions bash > opnsense-config-faker.bash

  Validate configuration data:
    opnsense-config-faker validate --input data.csv
    opnsense-config-faker validate --input config.xml --format xml

  Use global flags:
    opnsense-config-faker --quiet generate --count 10 --format csv
    opnsense-config-faker --no-color generate --count 10 --format xml --base-config config.xml"#)]
pub struct Cli {
    /// Global flags available for all subcommands
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate network configuration data in CSV or XML format
    Generate(GenerateArgs),
    /// Generate shell completions for the specified shell
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Validate configuration data for consistency and correctness
    Validate(ValidateArgs),
    /// DEPRECATED: Use 'generate --format csv' instead
    #[command(hide = true)]
    Csv(CsvArgs),
    /// DEPRECATED: Use 'generate --format xml' instead
    #[command(hide = true)]
    Xml(XmlArgs),
}

/// Global flags available for all subcommands
#[derive(Parser, Default)]
pub struct GlobalArgs {
    /// Suppress non-essential output (progress bars, summaries, etc.)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output (useful for scripts and CI)
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Global output file or directory (overrides command-specific output)
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,
}

/// Output format for generated configurations
#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Generate CSV file with VLAN configuration data
    Csv,
    /// Generate complete OPNsense XML configuration
    Xml,
}

/// WAN assignment strategy for VLAN distribution
#[derive(Clone, Debug, ValueEnum)]
pub enum WanAssignmentStrategy {
    /// Assign all VLANs to a single WAN connection
    Single,
    /// Distribute VLANs across multiple WAN connections
    Multi,
    /// Balance VLANs evenly across available WAN connections
    Balanced,
}

/// Shell types for completion generation
#[derive(Clone, Debug, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

/// Arguments for the generate command
#[derive(Parser)]
pub struct GenerateArgs {
    /// Output format (csv or xml)
    #[arg(short = 'f', long = "format")]
    #[arg(value_enum)]
    pub format: OutputFormat,

    /// Number of VLAN configurations to generate
    ///
    /// Note: For unique VLAN generation (XML format), maximum is 4085 due to
    /// VLAN ID range constraints (10-4094). CSV format may allow duplicates.
    #[arg(short, long, default_value_t = 10)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=10000))]
    pub count: u16,

    /// Output file path (for CSV format) or directory (for XML format)
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Output directory for generated XML files (XML format only)
    #[arg(long, default_value = "output")]
    pub output_dir: PathBuf,

    /// Base OPNsense configuration XML file (required for XML format)
    #[arg(short, long)]
    pub base_config: Option<PathBuf>,

    /// Use existing CSV file for configuration data (XML format only)
    #[arg(long, conflicts_with = "count")]
    pub csv_file: Option<PathBuf>,

    /// Firewall number for naming (used in filenames for XML format)
    #[arg(long, default_value_t = 1)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=999))]
    pub firewall_nr: u16,

    /// OPT interface counter starting value (XML format only)
    #[arg(long, default_value_t = 6)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=99))]
    pub opt_counter: u16,

    /// Force overwrite existing files
    #[arg(short = 'F', long)]
    pub force: bool,

    /// Random seed for reproducible generation
    #[arg(long)]
    pub seed: Option<u64>,

    /// Disable colored output (useful for scripts and CI)
    #[arg(long)]
    pub no_color: bool,

    /// Interactive mode - prompt for missing required arguments
    #[arg(short, long)]
    pub interactive: bool,

    /// Include firewall rules in generated configurations
    #[arg(long)]
    pub include_firewall_rules: bool,

    /// Number of firewall rules per VLAN (default: based on complexity level)
    #[arg(long)]
    pub firewall_rules_per_vlan: Option<u16>,

    /// Firewall rule complexity level (basic, intermediate, advanced)
    #[arg(long, default_value = "intermediate")]
    pub firewall_rule_complexity: String,

    /// VLAN range specification (e.g., "100-150" or "10,20,30-40")
    #[arg(long)]
    pub vlan_range: Option<String>,

    /// Number of VPN configurations to generate
    #[arg(long)]
    pub vpn_count: Option<u16>,

    /// Number of NAT mappings to generate
    #[arg(long)]
    pub nat_mappings: Option<u16>,

    /// WAN assignment strategy for VLANs
    #[arg(long, value_enum)]
    pub wan_assignments: Option<WanAssignmentStrategy>,

    /// Custom XML template file
    #[arg(long)]
    pub template: Option<PathBuf>,
}

impl GenerateArgs {
    /// Validate arguments after parsing, checking for VLAN ID constraints
    pub fn validate(&self) -> Result<(), String> {
        // For XML format, we require unique VLAN IDs, so check against maximum
        if matches!(self.format, OutputFormat::Xml) && self.count > MAX_UNIQUE_VLAN_IDS {
            return Err(format!(
                "Cannot generate {} unique VLAN configurations. Maximum is {} for XML format due to VLAN ID range constraints (10-4094). Consider using CSV format if duplicates are acceptable, or reduce the count.",
                self.count, MAX_UNIQUE_VLAN_IDS
            ));
        }

        // Validate VLAN range if provided
        if let Some(ref vlan_range) = self.vlan_range {
            self.validate_vlan_range(vlan_range)?;
        }

        // Validate conflicts between count and vlan_range
        if self.vlan_range.is_some() && self.count != 10 {
            return Err("Cannot specify both --count and --vlan-range. Use --vlan-range to specify exact VLANs or --count for auto-generated ranges.".to_string());
        }

        Ok(())
    }

    /// Validate VLAN range format and values
    fn validate_vlan_range(&self, vlan_range: &str) -> Result<(), String> {
        let ranges = parse_vlan_range(vlan_range)
            .map_err(|e| format!("Invalid VLAN range format '{}': {}", vlan_range, e))?;

        let total_vlans = ranges.iter().map(|r| r.1 - r.0 + 1).sum::<u16>();

        if matches!(self.format, OutputFormat::Xml) && total_vlans > MAX_UNIQUE_VLAN_IDS {
            return Err(format!(
                "VLAN range produces {} VLANs, but maximum is {} for XML format",
                total_vlans, MAX_UNIQUE_VLAN_IDS
            ));
        }

        Ok(())
    }
}

/// Legacy arguments for CSV generation (for backward compatibility)
#[derive(Parser)]
pub struct CsvArgs {
    /// Number of VLAN configurations to generate
    ///
    /// Note: CSV format may allow duplicate VLAN IDs if count exceeds 4085.
    #[arg(short, long, default_value_t = 10)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=10000))]
    pub count: u16,

    /// Output CSV file path
    #[arg(long, default_value = "vlan_configs.csv")]
    pub output: PathBuf,

    /// Force overwrite existing files
    #[arg(short, long)]
    pub force: bool,

    /// Random seed for reproducible generation
    #[arg(long)]
    pub seed: Option<u64>,
}

impl CsvArgs {
    /// Validate arguments after parsing
    pub fn validate(&self) -> Result<(), String> {
        // For CSV format, warn if count exceeds unique VLAN limit but don't error
        if self.count > MAX_UNIQUE_VLAN_IDS {
            eprintln!(
                "Warning: Requested {} VLAN configurations exceeds maximum unique VLANs ({}). Duplicate VLAN IDs may be generated in CSV output.",
                self.count, MAX_UNIQUE_VLAN_IDS
            );
        }
        Ok(())
    }
}

/// Legacy arguments for XML generation (for backward compatibility)
#[derive(Parser)]
pub struct XmlArgs {
    /// Base OPNsense configuration XML file
    #[arg(short, long)]
    pub base_config: PathBuf,

    /// Number of VLAN configurations to generate (if not using CSV)
    ///
    /// Note: For unique VLAN generation, maximum is 4085 due to VLAN ID range constraints (10-4094).
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=10000))]
    pub count: Option<u16>,

    /// Use existing CSV file for configuration data
    #[arg(long, conflicts_with = "count")]
    pub csv_file: Option<PathBuf>,

    /// Output directory for generated XML files
    #[arg(long, default_value = "output")]
    pub output_dir: PathBuf,

    /// Firewall number for naming (used in filenames)
    #[arg(long, default_value_t = 1)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=999))]
    pub firewall_nr: u16,

    /// OPT interface counter starting value
    #[arg(long, default_value_t = 6)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=99))]
    pub opt_counter: u16,

    /// Force overwrite existing files
    #[arg(short, long)]
    pub force: bool,

    /// Random seed for reproducible generation
    #[arg(long)]
    pub seed: Option<u64>,
}

impl XmlArgs {
    /// Validate arguments after parsing
    pub fn validate(&self) -> Result<(), String> {
        // For XML format with count specified, check against maximum unique VLANs
        if let Some(count) = self.count
            && count > MAX_UNIQUE_VLAN_IDS
        {
            return Err(format!(
                "Cannot generate {} unique VLAN configurations. Maximum is {} for XML format due to VLAN ID range constraints (10-4094). Consider using CSV format if duplicates are acceptable, or reduce the count.",
                count, MAX_UNIQUE_VLAN_IDS
            ));
        }
        Ok(())
    }
}

/// Arguments for the validate command
#[derive(Parser)]
pub struct ValidateArgs {
    /// Input file or directory to validate
    #[arg(short, long)]
    pub input: PathBuf,

    /// Format of the input data
    #[arg(short = 'f', long = "format", default_value = "auto")]
    #[arg(value_enum)]
    pub format: ValidationFormat,

    /// Detailed validation output
    #[arg(short, long)]
    pub verbose: bool,

    /// Maximum number of errors to report before stopping
    #[arg(long, default_value_t = 100)]
    pub max_errors: u32,

    /// Output validation report to file
    #[arg(long)]
    pub report: Option<PathBuf>,
}

/// Validation input format
#[derive(Clone, Debug, ValueEnum)]
pub enum ValidationFormat {
    /// Automatically detect format from file extension
    Auto,
    /// Validate CSV configuration data
    Csv,
    /// Validate OPNsense XML configuration
    Xml,
}

/// Parse VLAN range specification into individual ranges
/// Supports formats like "100-150", "10,20,30-40", "100"
pub fn parse_vlan_range(range_str: &str) -> Result<Vec<(u16, u16)>, String> {
    let mut ranges = Vec::new();

    for part in range_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if part.contains('-') {
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range format: '{}'", part));
            }

            let start: u16 = parts[0]
                .trim()
                .parse()
                .map_err(|_| format!("Invalid start VLAN ID: '{}'", parts[0]))?;
            let end: u16 = parts[1]
                .trim()
                .parse()
                .map_err(|_| format!("Invalid end VLAN ID: '{}'", parts[1]))?;

            if start > end {
                return Err(format!(
                    "Start VLAN ID {} must be less than or equal to end VLAN ID {}",
                    start, end
                ));
            }

            if !(10..=4094).contains(&start) || !(10..=4094).contains(&end) {
                return Err(format!(
                    "VLAN IDs must be between 10 and 4094, got range {}-{}",
                    start, end
                ));
            }

            ranges.push((start, end));
        } else {
            let vlan_id: u16 = part
                .parse()
                .map_err(|_| format!("Invalid VLAN ID: '{}'", part))?;

            if !(10..=4094).contains(&vlan_id) {
                return Err(format!("VLAN ID {} must be between 10 and 4094", vlan_id));
            }

            ranges.push((vlan_id, vlan_id));
        }
    }

    if ranges.is_empty() {
        return Err("No valid VLAN ranges found".to_string());
    }

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vlan_range() {
        // Test single VLAN
        let ranges = parse_vlan_range("100").unwrap();
        assert_eq!(ranges, vec![(100, 100)]);

        // Test simple range
        let ranges = parse_vlan_range("100-150").unwrap();
        assert_eq!(ranges, vec![(100, 150)]);

        // Test multiple ranges
        let ranges = parse_vlan_range("10,20-30,40").unwrap();
        assert_eq!(ranges, vec![(10, 10), (20, 30), (40, 40)]);

        // Test invalid range
        assert!(parse_vlan_range("150-100").is_err());
        assert!(parse_vlan_range("5-10").is_err()); // Below minimum
        assert!(parse_vlan_range("4095-5000").is_err()); // Above maximum
    }
}
