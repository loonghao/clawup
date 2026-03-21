use clap::Parser;
use color_eyre::Result;

mod cli;
mod error;
mod manifest;
mod openclaw;
mod utils;

// Feature modules
mod agent;
mod profile;
mod skill;
mod soul;

use cli::Cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    cli::dispatch(cli)
}
