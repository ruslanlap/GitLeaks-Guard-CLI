use anyhow::Result;
use colored::*;

use crate::{gitleaks, utils};

pub fn check() -> Result<()> {
    println!("{}", "═══════════════════════════════".blue());
    println!("  {}", "Version Information".blue().bold());
    println!("{}", "═══════════════════════════════".blue());
    println!();

    // GitLeaks Guard version
    let guard_version = env!("CARGO_PKG_VERSION");
    println!(
        "{} {}",
        "GitLeaks Guard:".bold(),
        guard_version.green()
    );

    // Gitleaks version
    match gitleaks::get_version() {
        Ok(version) => {
            println!("{} {}", "Gitleaks:".bold(), version.green());
        }
        Err(_) => {
            utils::print_warning("Gitleaks is not installed");
            println!("Run {} to install", "gitleaks-guard install".cyan());
        }
    }

    println!();
    Ok(())
}
