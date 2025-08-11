//! OPNsense Config Faker CLI Application
//!
//! Command-line interface for generating realistic network configuration test data.

use clap::Parser;
use opnsense_config_faker::cli::{Cli, Commands};
use opnsense_config_faker::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Csv(args) => {
            opnsense_config_faker::cli::commands::csv::execute(args)
        }
        Commands::Xml(args) => {
            opnsense_config_faker::cli::commands::xml::execute(args)
        }
    }
}