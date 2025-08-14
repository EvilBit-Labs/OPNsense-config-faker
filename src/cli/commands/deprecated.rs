//! Deprecation handlers for legacy command structure

use crate::cli::{CsvArgs, XmlArgs};
use crate::Result;
use console::style;

/// Handle deprecated CSV command with migration guidance
pub fn handle_deprecated_csv(args: CsvArgs) -> Result<()> {
    // Validate arguments before showing deprecation message
    if let Err(e) = args.validate() {
        return Err(crate::model::ConfigError::invalid_parameter("count", &e));
    }

    println!("{}", style("⚠️  DEPRECATED COMMAND").bold().yellow());
    println!();
    println!("The 'csv' subcommand has been replaced with the unified 'generate' command.");
    println!();
    println!("{}", style("Migration Guide:").bold());
    println!("  Old command:");
    println!(
        "    {} csv --count {} --output {} {}{}",
        style("opnsense-config-faker").cyan(),
        args.count,
        args.output.display(),
        if args.force { "--force " } else { "" },
        if let Some(seed) = args.seed {
            format!("--seed {seed}")
        } else {
            String::new()
        }
    );
    println!();
    println!("  New command:");
    println!(
        "    {} generate --format csv --count {} --output {} {}{}",
        style("opnsense-config-faker").cyan(),
        args.count,
        args.output.display(),
        if args.force { "--force " } else { "" },
        if let Some(seed) = args.seed {
            format!("--seed {seed}")
        } else {
            String::new()
        }
    );
    println!();
    println!(
        "Use '{}' for more information.",
        style("opnsense-config-faker generate --help").cyan()
    );

    Err(crate::model::ConfigError::config(
        "Please use the new 'generate' command format. See migration guide above.",
    ))
}

/// Handle deprecated XML command with migration guidance
pub fn handle_deprecated_xml(args: XmlArgs) -> Result<()> {
    // Validate arguments before showing deprecation message
    if let Err(e) = args.validate() {
        return Err(crate::model::ConfigError::invalid_parameter("count", &e));
    }

    println!("{}", style("⚠️  DEPRECATED COMMAND").bold().yellow());
    println!();
    println!("The 'xml' subcommand has been replaced with the unified 'generate' command.");
    println!();
    println!("{}", style("Migration Guide:").bold());
    println!("  Old command:");

    let mut old_cmd = format!(
        "opnsense-config-faker xml --base-config {}",
        args.base_config.display()
    );
    if let Some(count) = args.count {
        old_cmd.push_str(&format!(" --count {count}"));
    }
    if let Some(csv_file) = &args.csv_file {
        old_cmd.push_str(&format!(" --csv-file {}", csv_file.display()));
    }
    old_cmd.push_str(&format!(" --output-dir {}", args.output_dir.display()));
    if args.firewall_nr != 1 {
        old_cmd.push_str(&format!(" --firewall-nr {}", args.firewall_nr));
    }
    if args.opt_counter != 6 {
        old_cmd.push_str(&format!(" --opt-counter {}", args.opt_counter));
    }
    if args.force {
        old_cmd.push_str(" --force");
    }
    if let Some(seed) = args.seed {
        old_cmd.push_str(&format!(" --seed {seed}"));
    }

    println!("    {}", style(old_cmd).cyan());
    println!();

    let mut new_cmd = format!(
        "opnsense-config-faker generate --format xml --base-config {}",
        args.base_config.display()
    );
    if let Some(count) = args.count {
        new_cmd.push_str(&format!(" --count {count}"));
    }
    if let Some(csv_file) = &args.csv_file {
        new_cmd.push_str(&format!(" --csv-file {}", csv_file.display()));
    }
    new_cmd.push_str(&format!(" --output-dir {}", args.output_dir.display()));
    if args.firewall_nr != 1 {
        new_cmd.push_str(&format!(" --firewall-nr {}", args.firewall_nr));
    }
    if args.opt_counter != 6 {
        new_cmd.push_str(&format!(" --opt-counter {}", args.opt_counter));
    }
    if args.force {
        new_cmd.push_str(" --force");
    }
    if let Some(seed) = args.seed {
        new_cmd.push_str(&format!(" --seed {seed}"));
    }

    println!("  New command:");
    println!("    {}", style(new_cmd).cyan());
    println!();
    println!(
        "Use '{}' for more information.",
        style("opnsense-config-faker generate --help").cyan()
    );

    Err(crate::model::ConfigError::config(
        "Please use the new 'generate' command format. See migration guide above.",
    ))
}
