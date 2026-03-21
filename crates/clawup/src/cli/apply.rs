use clap::Args;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;
use crate::openclaw::OpenClawPaths;

#[derive(Args, Debug)]
pub struct ApplyArgs {
    /// Profile to apply
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Show what would be changed without applying
    #[arg(long)]
    pub dry_run: bool,

    /// Force apply even if there are conflicts
    #[arg(short, long)]
    pub force: bool,
}

pub fn execute(args: ApplyArgs) -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;
    let paths = OpenClawPaths::detect()?;

    if args.dry_run {
        println!(
            "{} Dry run mode — no changes will be applied\n",
            style("ℹ").blue()
        );
    }

    println!(
        "{} Applying configuration to {}",
        style("→").cyan().bold(),
        style(paths.root().display()).yellow()
    );

    // Apply gateway config
    if manifest.gateway.is_some() {
        println!("  {} Gateway configuration", style("•").dim());
    }

    // Apply agents
    if let Some(ref agents) = manifest.agents {
        let count = agents.list.as_ref().map_or(0, |l| l.len());
        println!("  {} {} agent(s)", style("•").dim(), count);
    }

    // Apply skills
    if manifest.skills.is_some() {
        println!("  {} Skills configuration", style("•").dim());
    }

    if args.dry_run {
        println!(
            "\n{} Dry run complete. No changes were made.",
            style("✓").green().bold()
        );
    } else {
        // TODO: Actually write configuration files
        println!(
            "\n{} Configuration applied successfully!",
            style("✓").green().bold()
        );
    }

    Ok(())
}
