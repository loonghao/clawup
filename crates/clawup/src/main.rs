use clap::Parser;
use color_eyre::Result;

mod cli;
mod error;

// Legacy modules — will be replaced by clawup-core and clawup-ops.
// See AGENTS.md "Known Technical Debt" section.
#[allow(dead_code)]
mod manifest;
#[allow(dead_code)]
mod openclaw;
#[allow(dead_code)]
mod utils;

#[allow(dead_code)]
mod agent;
#[allow(dead_code)]
mod profile;
#[allow(dead_code)]
mod skill;
#[allow(dead_code)]
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
