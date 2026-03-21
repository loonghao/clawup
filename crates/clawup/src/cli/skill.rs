use clap::Subcommand;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;

#[derive(Subcommand, Debug)]
pub enum SkillCommands {
    /// List all configured skills
    List,

    /// Add a new skill
    Add {
        /// Skill name
        name: String,

        /// Skill source (path or URL)
        #[arg(short, long)]
        source: Option<String>,
    },

    /// Remove a skill
    Remove {
        /// Skill name to remove
        name: String,
    },

    /// Enable a skill
    Enable {
        /// Skill name to enable
        name: String,
    },

    /// Disable a skill
    Disable {
        /// Skill name to disable
        name: String,
    },

    /// Update skills from their sources
    Update {
        /// Specific skill to update (updates all if omitted)
        name: Option<String>,
    },
}

pub fn execute(cmd: SkillCommands) -> Result<()> {
    match cmd {
        SkillCommands::List => list_skills(),
        SkillCommands::Add { name, source } => add_skill(&name, source.as_deref()),
        SkillCommands::Remove { name } => remove_skill(&name),
        SkillCommands::Enable { name } => toggle_skill(&name, true),
        SkillCommands::Disable { name } => toggle_skill(&name, false),
        SkillCommands::Update { name } => update_skills(name.as_deref()),
    }
}

fn list_skills() -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;

    let has_skills = manifest.skills.is_some();

    if !has_skills {
        println!("{} No skills configured.", style("ℹ").blue());
        println!("  Run {} to add one.", style("clawup skill add <name>").cyan());
        return Ok(());
    }

    let skills = manifest.skills.as_ref().unwrap();

    // Show bundled skills
    if let Some(ref bundled) = skills.bundled {
        if let Some(ref enabled) = bundled.enabled {
            println!("{}", style("Bundled skills:").bold());
            for s in enabled {
                println!("  {} {}", style("✓").green(), s);
            }
        }
    }

    // Show skill entries
    if let Some(ref entries) = skills.entries {
        println!("{}", style("\nCustom skills:").bold());
        for entry in entries {
            let status = if entry.enabled.unwrap_or(true) {
                style("✓").green()
            } else {
                style("✗").red()
            };
            let source = entry.source.as_deref().unwrap_or("-");
            println!("  {} {} ({})", status, entry.name, style(source).dim());
        }
    }

    Ok(())
}

fn add_skill(name: &str, source: Option<&str>) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.add_skill(name, source)?;
    manifest.save("clawup.toml")?;

    println!(
        "{} Added skill '{}'",
        style("✓").green().bold(),
        style(name).cyan()
    );
    Ok(())
}

fn remove_skill(name: &str) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.remove_skill(name)?;
    manifest.save("clawup.toml")?;

    println!(
        "{} Removed skill '{}'",
        style("✓").green().bold(),
        style(name).cyan()
    );
    Ok(())
}

fn toggle_skill(name: &str, enabled: bool) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.toggle_skill(name, enabled)?;
    manifest.save("clawup.toml")?;

    let action = if enabled { "Enabled" } else { "Disabled" };
    println!(
        "{} {} skill '{}'",
        style("✓").green().bold(),
        action,
        style(name).cyan()
    );
    Ok(())
}

fn update_skills(name: Option<&str>) -> Result<()> {
    match name {
        Some(n) => println!("{} Updating skill '{}'...", style("→").cyan(), n),
        None => println!("{} Updating all skills...", style("→").cyan()),
    }
    println!("{} Skill update is not yet implemented.", style("⚠").yellow());
    Ok(())
}
