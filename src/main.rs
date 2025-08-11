//! OPNsense Config Faker CLI Application
//!
//! Command-line interface for generating realistic network configuration test data.

use clap::Parser;
use opnsense_config_faker::cli::{Cli, Commands};
use opnsense_config_faker::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate(args) => {
            opnsense_config_faker::cli::commands::generate::execute(args)
        }
        Commands::Completions { shell } => {
            opnsense_config_faker::cli::commands::completions::execute(shell)
        }
    }
}