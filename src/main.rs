//! OPNsense Config Faker CLI Application
//!
//! Command-line interface for generating realistic network configuration test data.

use clap::Parser;
use opnsense_config_faker::cli::{Cli, Commands};
use opnsense_config_faker::Result;
use std::env;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle global flags first
    if cli.global.no_color
        || env::var("NO_COLOR").is_ok()
        || env::var("TERM").unwrap_or_default() == "dumb"
    {
        env::set_var("NO_COLOR", "1");
    }

    match cli.command {
        Commands::Generate(args) => {
            opnsense_config_faker::cli::commands::generate::execute_with_global(args, &cli.global)
        }
        Commands::Completions { shell } => {
            opnsense_config_faker::cli::commands::completions::execute(shell)
        }
        Commands::Validate(args) => {
            opnsense_config_faker::cli::commands::validate::execute_with_global(args, &cli.global)
        }
        Commands::Csv(args) => {
            opnsense_config_faker::cli::commands::deprecated::handle_deprecated_csv(args)
        }
        Commands::Xml(args) => {
            opnsense_config_faker::cli::commands::deprecated::handle_deprecated_xml(args)
        }
    }
}
