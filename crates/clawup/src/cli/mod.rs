mod agent;
mod apply;
mod config;
mod diff;
mod doctor;
mod export;
mod init;
mod profile;
mod skill;
mod soul;
mod sync;

use clap::{Parser, Subcommand};
use color_eyre::Result;

/// 🦀 clawup — Like rustup, but for OpenClaw.
///
/// A CLI tool for managing OpenClaw configurations, agents, skills, souls, and profiles.
#[derive(Parser, Debug)]
#[command(name = "clawup", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to clawup.toml configuration file
    #[arg(long, global = true, default_value = "clawup.toml")]
    pub config: String,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new clawup configuration
    Init(init::InitArgs),

    /// Apply configuration to OpenClaw
    Apply(apply::ApplyArgs),

    /// Sync configuration with Git remote
    Sync(sync::SyncArgs),

    /// Show differences between local and applied configuration
    Diff(diff::DiffArgs),

    /// Export configuration to various formats
    Export(export::ExportArgs),

    /// Health check for OpenClaw installation
    Doctor(doctor::DoctorArgs),

    /// Manage agents
    #[command(subcommand)]
    Agent(agent::AgentCommands),

    /// Manage skills
    #[command(subcommand)]
    Skill(skill::SkillCommands),

    /// Manage soul files
    #[command(subcommand)]
    Soul(soul::SoulCommands),

    /// Manage profiles
    #[command(subcommand)]
    Profile(profile::ProfileCommands),

    /// Manage configuration values
    #[command(subcommand)]
    Config(config::ConfigCommands),
}

/// Dispatch CLI commands to their handlers.
pub fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init(args) => init::execute(args),
        Commands::Apply(args) => apply::execute(args),
        Commands::Sync(args) => sync::execute(args),
        Commands::Diff(args) => diff::execute(args),
        Commands::Export(args) => export::execute(args),
        Commands::Doctor(args) => doctor::execute(args),
        Commands::Agent(cmd) => agent::execute(cmd),
        Commands::Skill(cmd) => skill::execute(cmd),
        Commands::Soul(cmd) => soul::execute(cmd),
        Commands::Profile(cmd) => profile::execute(cmd),
        Commands::Config(cmd) => config::execute(cmd),
    }
}
