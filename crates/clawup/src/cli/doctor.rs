use clap::Args;
use color_eyre::Result;
use console::style;

use crate::openclaw::OpenClawPaths;

#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Fix issues automatically when possible
    #[arg(long)]
    pub fix: bool,
}

pub fn execute(_args: DoctorArgs) -> Result<()> {
    println!("{} Running health checks...\n", style("🩺").bold());

    let mut issues = 0;
    let mut passed = 0;

    // Check OpenClaw installation
    match OpenClawPaths::detect() {
        Ok(paths) => {
            println!(
                "  {} OpenClaw directory found at {}",
                style("✓").green(),
                style(paths.root().display()).cyan()
            );
            passed += 1;

            // Check for openclaw.json
            if paths.config_file().exists() {
                println!("  {} openclaw.json exists", style("✓").green());
                passed += 1;
            } else {
                println!("  {} openclaw.json not found", style("✗").red());
                issues += 1;
            }

            // Check workspace directory
            if paths.workspace_dir().exists() {
                println!("  {} workspace/ directory exists", style("✓").green());
                passed += 1;
            } else {
                println!("  {} workspace/ directory not found", style("✗").red());
                issues += 1;
            }
        }
        Err(_) => {
            println!("  {} OpenClaw directory not found", style("✗").red());
            issues += 1;
        }
    }

    // Check clawup.toml
    if std::path::Path::new("clawup.toml").exists() {
        println!("  {} clawup.toml found", style("✓").green());
        passed += 1;
    } else {
        println!(
            "  {} clawup.toml not found (run {} to create one)",
            style("✗").red(),
            style("clawup init").cyan()
        );
        issues += 1;
    }

    println!();
    if issues == 0 {
        println!(
            "{} All {} checks passed!",
            style("✓").green().bold(),
            passed
        );
    } else {
        println!(
            "{} {} passed, {} issue(s) found",
            style("⚠").yellow().bold(),
            passed,
            issues
        );
    }

    Ok(())
}
