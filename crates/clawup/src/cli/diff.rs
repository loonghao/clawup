use clap::Args;
use color_eyre::Result;
use console::style;

#[derive(Args, Debug)]
pub struct DiffArgs {
    /// Compare with a specific profile
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Show full diff (not just summary)
    #[arg(long)]
    pub full: bool,
}

pub fn execute(_args: DiffArgs) -> Result<()> {
    println!("{} Diff is not yet implemented.", style("⚠").yellow());
    println!("  This will show differences between your clawup.toml and the applied OpenClaw config.");
    Ok(())
}
