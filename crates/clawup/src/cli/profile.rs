use clap::Subcommand;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;

#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// List available profiles
    List,

    /// Switch to a different profile
    Switch {
        /// Profile name to switch to
        name: String,
    },

    /// Create a new profile
    Create {
        /// Profile name
        name: String,

        /// Base profile to extend from
        #[arg(short, long)]
        from: Option<String>,
    },
}

pub fn execute(cmd: ProfileCommands) -> Result<()> {
    match cmd {
        ProfileCommands::List => list_profiles(),
        ProfileCommands::Switch { name } => switch_profile(&name),
        ProfileCommands::Create { name, from } => create_profile(&name, from.as_deref()),
    }
}

fn list_profiles() -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;

    let profiles = manifest.profiles.as_ref();

    match profiles {
        Some(p) if !p.is_empty() => {
            println!("{}", style("Available profiles:").bold());
            for (name, profile) in p {
                let desc = profile
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                println!("  {} {} {}", style("•").dim(), style(name).cyan(), style(desc).dim());
            }
        }
        _ => {
            println!("{} No profiles configured.", style("ℹ").blue());
            println!(
                "  Add profiles to clawup.toml under [profiles.<name>]"
            );
        }
    }

    Ok(())
}

fn switch_profile(name: &str) -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;

    if let Some(profiles) = manifest.profiles.as_ref() {
        if profiles.contains_key(name) {
            println!(
                "{} Switched to profile '{}'",
                style("✓").green().bold(),
                style(name).cyan()
            );
            println!("  Run {} to apply the profile.", style("clawup apply").cyan());
        } else {
            println!(
                "{} Profile '{}' not found.",
                style("✗").red(),
                style(name).cyan()
            );
            println!("  Available profiles:");
            for key in profiles.keys() {
                println!("    {} {}", style("•").dim(), key);
            }
        }
    } else {
        println!("{} No profiles configured.", style("ℹ").blue());
    }

    Ok(())
}

fn create_profile(name: &str, from: Option<&str>) -> Result<()> {
    match from {
        Some(base) => {
            println!(
                "{} Creating profile '{}' based on '{}'...",
                style("→").cyan(),
                style(name).cyan(),
                style(base).yellow()
            );
        }
        None => {
            println!(
                "{} Creating profile '{}'...",
                style("→").cyan(),
                style(name).cyan()
            );
        }
    }
    println!("{} Profile creation is not yet fully implemented.", style("⚠").yellow());
    Ok(())
}
