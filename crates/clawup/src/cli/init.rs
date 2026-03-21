use clap::Args;
use color_eyre::Result;
use console::style;
use dialoguer::{Confirm, Input, Select};
use std::path::Path;

use crate::manifest::Manifest;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Directory to initialize (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// Template to use: default, multi-agent, team
    #[arg(short, long)]
    pub template: Option<String>,

    /// Skip interactive prompts
    #[arg(long)]
    pub non_interactive: bool,
}

pub fn execute(args: InitArgs) -> Result<()> {
    let target = Path::new(&args.path);
    let config_path = target.join("clawup.toml");

    if config_path.exists() {
        let overwrite = if args.non_interactive {
            false
        } else {
            Confirm::new()
                .with_prompt("clawup.toml already exists. Overwrite?")
                .default(false)
                .interact()?
        };

        if !overwrite {
            println!("{} Configuration already exists, skipping.", style("⚠").yellow());
            return Ok(());
        }
    }

    let template = if let Some(t) = args.template {
        t
    } else if args.non_interactive {
        "default".to_string()
    } else {
        let templates = vec!["default (single agent)", "multi-agent", "team"];
        let selection = Select::new()
            .with_prompt("Select a template")
            .items(&templates)
            .default(0)
            .interact()?;
        match selection {
            0 => "default".to_string(),
            1 => "multi-agent".to_string(),
            2 => "team".to_string(),
            _ => "default".to_string(),
        }
    };

    let description = if args.non_interactive {
        "My OpenClaw setup".to_string()
    } else {
        Input::new()
            .with_prompt("Description")
            .default("My OpenClaw setup".to_string())
            .interact_text()?
    };

    let manifest = Manifest::from_template(&template, &description)?;
    let toml_content = toml::to_string_pretty(&manifest)?;

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&config_path, toml_content)?;

    println!(
        "{} Created {} using '{}' template",
        style("✓").green().bold(),
        style(config_path.display()).cyan(),
        style(&template).yellow()
    );
    println!(
        "\n  Next steps:\n    {} Edit clawup.toml to customize your setup\n    {} Run {} to apply the configuration",
        style("1.").bold(),
        style("2.").bold(),
        style("clawup apply").cyan()
    );

    Ok(())
}
