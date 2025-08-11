//! Command-line interface for OPNsense Config Faker

use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod commands;

/// OPNsense Config Faker - Generate realistic network configuration test data
#[derive(Parser)]
#[command(name = "opnsense-config-faker")]
#[command(about = "A flexible tool for generating realistic network configuration test data for OPNsense")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "EvilBit Labs <contact@evilbitlabs.com>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate CSV file with VLAN configuration data
    Csv(CsvArgs),
    /// Generate complete OPNsense XML configuration
    Xml(XmlArgs),
}

/// Arguments for CSV generation command
#[derive(Parser)]
pub struct CsvArgs {
    /// Number of VLAN configurations to generate
    #[arg(short, long, default_value_t = 10)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=4084))]
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

/// Arguments for XML generation command  
#[derive(Parser)]
pub struct XmlArgs {
    /// Base OPNsense configuration XML file
    #[arg(short, long)]
    pub base_config: PathBuf,

    /// Number of VLAN configurations to generate (if not using CSV)
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..=4084))]
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