use clap::Subcommand;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;

#[derive(Subcommand, Debug)]
pub enum AgentCommands {
    /// List all configured agents
    List,

    /// Add a new agent
    Add {
        /// Agent name
        name: String,

        /// Agent role description
        #[arg(short, long)]
        role: Option<String>,

        /// Model override for this agent
        #[arg(short, long)]
        model: Option<String>,
    },

    /// Remove an agent
    Remove {
        /// Agent name to remove
        name: String,
    },

    /// Show details of an agent
    Show {
        /// Agent name
        name: String,
    },

    /// Set a property on an agent
    Set {
        /// Agent name
        name: String,

        /// Property to set (e.g., "role", "model", "approval_mode")
        key: String,

        /// Value to set
        value: String,
    },
}

pub fn execute(cmd: AgentCommands) -> Result<()> {
    match cmd {
        AgentCommands::List => list_agents(),
        AgentCommands::Add { name, role, model } => {
            add_agent(&name, role.as_deref(), model.as_deref())
        }
        AgentCommands::Remove { name } => remove_agent(&name),
        AgentCommands::Show { name } => show_agent(&name),
        AgentCommands::Set { name, key, value } => set_agent(&name, &key, &value),
    }
}

fn list_agents() -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;

    let agents = manifest.agents.as_ref().and_then(|a| a.list.as_ref());

    match agents {
        Some(list) if !list.is_empty() => {
            use comfy_table::{ContentArrangement, Table};

            let mut table = Table::new();
            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["Name", "Role", "Model", "Approval"]);

            let defaults = manifest.agents.as_ref().and_then(|a| a.defaults.as_ref());

            for agent in list {
                let model = agent
                    .model
                    .as_deref()
                    .or(defaults.and_then(|d| d.model.as_deref()))
                    .unwrap_or("-");
                let approval = agent
                    .approval_mode
                    .as_deref()
                    .or(defaults.and_then(|d| d.approval_mode.as_deref()))
                    .unwrap_or("-");

                table.add_row(vec![
                    &agent.name,
                    agent.role.as_deref().unwrap_or("-"),
                    model,
                    approval,
                ]);
            }

            println!("{table}");
        }
        _ => {
            println!("{} No agents configured.", style("ℹ").blue());
            println!(
                "  Run {} to add one.",
                style("clawup agent add <name>").cyan()
            );
        }
    }

    Ok(())
}

fn add_agent(name: &str, role: Option<&str>, model: Option<&str>) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.add_agent(name, role, model)?;
    manifest.save("clawup.toml")?;

    println!(
        "{} Added agent '{}'",
        style("✓").green().bold(),
        style(name).cyan()
    );
    Ok(())
}

fn remove_agent(name: &str) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.remove_agent(name)?;
    manifest.save("clawup.toml")?;

    println!(
        "{} Removed agent '{}'",
        style("✓").green().bold(),
        style(name).cyan()
    );
    Ok(())
}

fn show_agent(name: &str) -> Result<()> {
    let manifest = Manifest::load("clawup.toml")?;
    let agent = manifest.find_agent(name)?;

    println!("{}", style(format!("Agent: {}", agent.name)).bold());
    if let Some(ref role) = agent.role {
        println!("  Role:     {}", role);
    }
    if let Some(ref model) = agent.model {
        println!("  Model:    {}", model);
    }
    if let Some(ref mode) = agent.approval_mode {
        println!("  Approval: {}", mode);
    }
    if let Some(ref instructions) = agent.instructions {
        println!("  Instructions: {}", instructions);
    }

    Ok(())
}

fn set_agent(name: &str, key: &str, value: &str) -> Result<()> {
    let mut manifest = Manifest::load("clawup.toml")?;
    manifest.set_agent_property(name, key, value)?;
    manifest.save("clawup.toml")?;

    println!(
        "{} Set {}.{} = {}",
        style("✓").green().bold(),
        style(name).cyan(),
        style(key).yellow(),
        value
    );
    Ok(())
}
