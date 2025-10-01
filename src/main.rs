#![forbid(unsafe_code)]

//! OPNsense Config Faker CLI Application
//!
//! Command-line interface for generating realistic network configuration test data.

use anyhow::{Context, Result};
use clap::Parser;
use opnsense_config_faker::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up environment with context
    setup_environment(&cli).context("Failed to setup CLI environment")?;

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

/// Set up the CLI environment with proper configuration
fn setup_environment(cli: &Cli) -> Result<()> {
    // Colors are automatically disabled by checking env::var("NO_COLOR").is_ok()
    // and env::var("TERM") == "dumb" in the progress bar and console styling code
    let _ = cli; // Suppress unused parameter warnings
    Ok(())
}
