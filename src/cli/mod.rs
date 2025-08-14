//! Command-line interface for OPNsense Config Faker

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

pub mod commands;

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

  Force overwrite existing files:
    opnsense-config-faker generate --count 10 --format csv --output test.csv --force

  Generate shell completions:
    opnsense-config-faker completions bash > opnsense-config-faker.bash"#)]
pub struct Cli {
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
    /// DEPRECATED: Use 'generate --format csv' instead
    #[command(hide = true)]
    Csv(CsvArgs),
    /// DEPRECATED: Use 'generate --format xml' instead
    #[command(hide = true)]
    Xml(XmlArgs),
}

/// Output format for generated configurations
#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Generate CSV file with VLAN configuration data
    Csv,
    /// Generate complete OPNsense XML configuration
    Xml,
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
    #[arg(short, long)]
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
    #[arg(short, long, default_value = "vlan_configs.csv")]
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
    #[arg(short, long, default_value = "output")]
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
        if let Some(count) = self.count {
            if count > MAX_UNIQUE_VLAN_IDS {
                return Err(format!(
                    "Cannot generate {} unique VLAN configurations. Maximum is {} for XML format due to VLAN ID range constraints (10-4094). Consider using CSV format if duplicates are acceptable, or reduce the count.",
                    count, MAX_UNIQUE_VLAN_IDS
                ));
            }
        }
        Ok(())
    }
}
