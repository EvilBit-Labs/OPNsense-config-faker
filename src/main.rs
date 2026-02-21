#![forbid(unsafe_code)]

//! OPNsense Config Faker CLI Application
//!
//! Command-line interface for generating realistic network configuration test data.

use anyhow::{Context, Result};
use clap::Parser;
use opnsense_config_faker::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Execute command with rich context
    match cli.command {
        Commands::Generate(args) => {
            opnsense_config_faker::cli::commands::generate::execute_with_global(args, &cli.global)
                .context("Failed to generate configurations")?
        }
        Commands::Completions { shell } => {
            opnsense_config_faker::cli::commands::completions::execute(shell)
                .context("Failed to generate shell completions")?
        }
        Commands::Validate(args) => {
            opnsense_config_faker::cli::commands::validate::execute_with_global(args, &cli.global)
                .context("Failed to validate configurations")?
        }
        Commands::Csv(args) => {
            opnsense_config_faker::cli::commands::deprecated::handle_deprecated_csv(args)
                .context("Failed to process CSV command")?
        }
        Commands::Xml(args) => {
            opnsense_config_faker::cli::commands::deprecated::handle_deprecated_xml(args)
                .context("Failed to process XML command")?
        }
    }

    Ok(())
}
