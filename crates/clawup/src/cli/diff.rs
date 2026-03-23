use clap::Args;
use color_eyre::Result;
use console::style;

use crate::manifest::Manifest;

#[derive(Args, Debug)]
pub struct DiffArgs {
    /// Compare with a specific profile
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Show full diff (not just summary)
    #[arg(long)]
    pub full: bool,

    /// Compare with another file
    #[arg(short, long)]
    pub compare: Option<String>,

    /// Path to clawup.toml (defaults to "clawup.toml" in current directory)
    #[arg(short = 'C', long, default_value = "clawup.toml")]
    pub config: String,
}

pub fn execute(args: DiffArgs) -> Result<()> {
    let manifest = Manifest::load(&args.config)?;
    let current_toml = toml::to_string_pretty(&manifest)?;

    let (other_toml, other_label) = if let Some(ref compare_path) = args.compare {
        let other = Manifest::load(compare_path)?;
        (toml::to_string_pretty(&other)?, compare_path.clone())
    } else {
        // Compare with default template
        let default = Manifest::from_template("default", "default")?;
        (
            toml::to_string_pretty(&default)?,
            "default template".to_string(),
        )
    };

    let diff = similar::TextDiff::from_lines(&other_toml, &current_toml);

    let mut has_changes = false;
    let mut additions = 0usize;
    let mut removals = 0usize;

    for change in diff.iter_all_changes() {
        match change.tag() {
            similar::ChangeTag::Insert => {
                additions += 1;
                has_changes = true;
            }
            similar::ChangeTag::Delete => {
                removals += 1;
                has_changes = true;
            }
            similar::ChangeTag::Equal => {}
        }
    }

    if !has_changes {
        println!(
            "{} No differences between {} and {}",
            style("✓").green().bold(),
            style(&args.config).cyan(),
            style(&other_label).cyan(),
        );
        return Ok(());
    }

    println!(
        "{} Comparing {} ↔ {}",
        style("→").cyan(),
        style(&args.config).cyan(),
        style(&other_label).cyan(),
    );
    println!(
        "  {} addition(s), {} removal(s)\n",
        style(additions).green().bold(),
        style(removals).red().bold(),
    );

    if args.full {
        for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
            if idx > 0 {
                println!("{}", style("---").dim());
            }
            for op in group {
                for change in diff.iter_changes(op) {
                    let (sign, color_fn): (&str, fn(&str) -> String) = match change.tag() {
                        similar::ChangeTag::Delete => {
                            ("-", |s: &str| format!("{}", style(s).red()))
                        }
                        similar::ChangeTag::Insert => {
                            ("+", |s: &str| format!("{}", style(s).green()))
                        }
                        similar::ChangeTag::Equal => (" ", |s: &str| format!("{}", style(s).dim())),
                    };
                    let text = change.as_str().unwrap_or("").trim_end_matches('\n');
                    println!("{}{}", color_fn(sign), color_fn(text));
                }
            }
        }
    } else {
        // Summary mode: show changed sections
        let current_parsed: toml::Value = toml::from_str(&current_toml)?;
        let other_parsed: toml::Value = toml::from_str(&other_toml)?;

        if let (toml::Value::Table(current_tbl), toml::Value::Table(other_tbl)) =
            (&current_parsed, &other_parsed)
        {
            for key in current_tbl.keys() {
                if !other_tbl.contains_key(key) {
                    println!("  {} [{}]", style("+").green().bold(), style(key).cyan());
                } else if current_tbl[key] != other_tbl[key] {
                    println!("  {} [{}]", style("~").yellow().bold(), style(key).cyan());
                }
            }
            for key in other_tbl.keys() {
                if !current_tbl.contains_key(key) {
                    println!("  {} [{}]", style("-").red().bold(), style(key).cyan());
                }
            }
        }

        println!("\n  Use {} to see the full diff.", style("--full").yellow());
    }

    Ok(())
}
