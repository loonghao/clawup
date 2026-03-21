use clap::Subcommand;
use color_eyre::Result;
use console::style;

#[derive(Subcommand, Debug)]
pub enum SoulCommands {
    /// Show soul files for an agent
    Show {
        /// Agent name (defaults to main agent)
        #[arg(short, long)]
        agent: Option<String>,

        /// Specific soul file (SOUL.md, IDENTITY.md, etc.)
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Edit a soul file
    Edit {
        /// Agent name
        #[arg(short, long)]
        agent: Option<String>,

        /// Soul file to edit
        file: String,
    },

    /// Show diff of soul files
    Diff {
        /// Agent name
        #[arg(short, long)]
        agent: Option<String>,
    },

    /// Generate soul files from a template
    FromTemplate {
        /// Template name
        template: String,

        /// Agent name
        #[arg(short, long)]
        agent: Option<String>,

        /// Force overwrite existing files
        #[arg(short, long)]
        force: bool,
    },
}

pub fn execute(cmd: SoulCommands) -> Result<()> {
    match cmd {
        SoulCommands::Show { agent, file } => show_soul(agent.as_deref(), file.as_deref()),
        SoulCommands::Edit { agent, file } => edit_soul(agent.as_deref(), &file),
        SoulCommands::Diff { agent } => diff_soul(agent.as_deref()),
        SoulCommands::FromTemplate {
            template,
            agent,
            force,
        } => from_template(&template, agent.as_deref(), force),
    }
}

fn show_soul(agent: Option<&str>, file: Option<&str>) -> Result<()> {
    let agent_name = agent.unwrap_or("default");
    let file_name = file.unwrap_or("SOUL.md");

    println!(
        "{} Soul file: {} (agent: {})",
        style("📜").bold(),
        style(file_name).cyan(),
        style(agent_name).yellow()
    );
    println!(
        "{} Soul show is not yet fully implemented.",
        style("⚠").yellow()
    );

    Ok(())
}

fn edit_soul(agent: Option<&str>, file: &str) -> Result<()> {
    let agent_name = agent.unwrap_or("default");
    println!(
        "{} Opening {} for agent '{}' in editor...",
        style("→").cyan(),
        style(file).cyan(),
        style(agent_name).yellow()
    );
    println!("{} Soul edit is not yet implemented.", style("⚠").yellow());
    Ok(())
}

fn diff_soul(agent: Option<&str>) -> Result<()> {
    let agent_name = agent.unwrap_or("default");
    println!(
        "{} Showing soul diff for agent '{}'",
        style("→").cyan(),
        style(agent_name).yellow()
    );
    println!("{} Soul diff is not yet implemented.", style("⚠").yellow());
    Ok(())
}

fn from_template(template: &str, agent: Option<&str>, _force: bool) -> Result<()> {
    let agent_name = agent.unwrap_or("default");
    println!(
        "{} Generating soul files from template '{}' for agent '{}'",
        style("→").cyan(),
        style(template).yellow(),
        style(agent_name).cyan()
    );
    println!(
        "{} Soul template generation is not yet implemented.",
        style("⚠").yellow()
    );
    Ok(())
}
