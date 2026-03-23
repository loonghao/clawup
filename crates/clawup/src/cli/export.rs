use clap::Args;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;

#[derive(Args, Debug)]
pub struct ExportArgs {
    /// Output format: toml, json
    #[arg(short, long, default_value = "json")]
    pub format: String,

    /// Output file path (defaults to stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Path to clawup.toml (defaults to "clawup.toml" in current directory)
    #[arg(short, long, default_value = "clawup.toml")]
    pub config: String,

    /// Compact output (no pretty-printing)
    #[arg(long)]
    pub compact: bool,
}

pub fn execute(args: ExportArgs) -> Result<()> {
    let manifest = Manifest::load(&args.config)?;

    let output = match args.format.as_str() {
        "json" => {
            if args.compact {
                serde_json::to_string(&manifest)?
            } else {
                serde_json::to_string_pretty(&manifest)?
            }
        }
        "toml" => toml::to_string_pretty(&manifest)?,
        other => {
            return Err(color_eyre::eyre::eyre!(
                "Unsupported format: '{}'. Supported formats: json, toml",
                other
            ));
        }
    };

    match args.output {
        Some(ref path) => {
            std::fs::write(path, &output)?;
            println!(
                "{} Exported configuration to {} ({})",
                style("✓").green().bold(),
                style(path).cyan(),
                style(&args.format).yellow()
            );
        }
        None => {
            println!("{output}");
        }
    }

    Ok(())
}
