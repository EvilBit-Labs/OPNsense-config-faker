//! Shell completions generation command

use crate::cli::{Cli, Shell};
use clap::CommandFactory;
use clap_complete::{generate, Generator};
use std::io;

/// Generate shell completions for the specified shell
pub fn execute(shell: Shell) -> crate::Result<()> {
    let mut app = Cli::command();

    match shell {
        Shell::Bash => generate_completions(clap_complete::shells::Bash, &mut app),
        Shell::Zsh => generate_completions(clap_complete::shells::Zsh, &mut app),
        Shell::Fish => generate_completions(clap_complete::shells::Fish, &mut app),
        Shell::PowerShell => generate_completions(clap_complete::shells::PowerShell, &mut app),
        Shell::Elvish => generate_completions(clap_complete::shells::Elvish, &mut app),
    }

    Ok(())
}

/// Generate completions for a specific shell
fn generate_completions<G: Generator>(gen: G, app: &mut clap::Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
