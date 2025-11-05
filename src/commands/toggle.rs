use anyhow::Result;
use colored::*;

use crate::{git, utils};

pub fn enable() -> Result<()> {
    if !git::is_git_repo() {
        utils::print_error("Not a git repository. Please run this command in a git repository.");
        anyhow::bail!("Not a git repository");
    }

    git::enable_gitleaks()?;

    println!("{}", "═══════════════════════════════".green());
    println!("{} {}", "✔".green().bold(), "Gitleaks enabled successfully!".green().bold());
    println!("{}", "═══════════════════════════════".green());
    println!();
    println!("Pre-commit hooks will now check for secrets before each commit.");

    Ok(())
}

pub fn disable() -> Result<()> {
    if !git::is_git_repo() {
        utils::print_error("Not a git repository. Please run this command in a git repository.");
        anyhow::bail!("Not a git repository");
    }

    git::disable_gitleaks()?;

    println!("{}", "═══════════════════════════════".red());
    println!("{} {}", "✘".red().bold(), "Gitleaks disabled".red().bold());
    println!("{}", "═══════════════════════════════".red());
    println!();
    println!("{}", "⚠ Warning: Pre-commit hooks are now disabled.".yellow());
    println!("Your commits will not be checked for secrets.");
    println!();
    println!("To re-enable, run: {}", "gitleaks-guard enable".cyan());

    Ok(())
}
