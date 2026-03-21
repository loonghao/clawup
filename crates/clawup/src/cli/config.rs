use clap::Subcommand;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Get a configuration value
    Get {
        /// Configuration key (dot notation, e.g., "gateway.provider")
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Configuration key (dot notation)
        key: String,

        /// Value to set
        value: String,
    },

    /// Show the full configuration
    Show {
        /// Output format: toml, json
        #[arg(short, long, default_value = "toml")]
        format: String,
    },

    /// Merge another configuration file into the current one
    Merge {
        /// Path to the configuration file to merge
        path: String,
    },
}

pub fn execute(cmd: ConfigCommands) -> Result<()> {
    match cmd {
        ConfigCommands::Get { key } => get_config(&key),
        ConfigCommands::Set { key, value } => set_config(&key, &value),
        ConfigCommands::Show { format } => show_config(&format),
        ConfigCommands::Merge { path } => merge_config(&path),
    }
}

fn get_config(key: &str) -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;
    let value = manifest.get_value(key)?;
    println!("{}", value);
    Ok(())
}

fn set_config(key: &str, value: &str) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.set_value(key, value)?;
    manifest.save("clawup.toml")?;

    println!(
        "{} Set {} = {}",
        style("✓").green().bold(),
        style(key).cyan(),
        value
    );
    Ok(())
}

fn show_config(format: &str) -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&manifest)?;
            println!("{json}");
        }
        _ => {
            let toml_str = toml::to_string_pretty(&manifest)?;
            println!("{toml_str}");
        }
    }

    Ok(())
}

fn merge_config(path: &str) -> Result<()> {
    println!(
        "{} Merging configuration from {}...",
        style("→").cyan(),
        style(path).cyan()
    );
    println!(
        "{} Config merge is not yet implemented.",
        style("⚠").yellow()
    );
    Ok(())
}
