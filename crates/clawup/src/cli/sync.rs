use clap::Args;
use color_eyre::Result;
use console::style;

#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Remote name
    #[arg(short, long, default_value = "origin")]
    pub remote: String,

    /// Branch name
    #[arg(short, long, default_value = "main")]
    pub branch: String,

    /// Pull before push
    #[arg(long)]
    pub pull_first: bool,
}

pub fn execute(_args: SyncArgs) -> Result<()> {
    println!("{} Sync is not yet implemented.", style("⚠").yellow());
    println!("  This will sync your clawup.toml with a Git remote.");
    Ok(())
}
