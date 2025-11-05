use anyhow::{Context, Result};
use colored::*;

use crate::{git, gitleaks, utils};

pub async fn run(skip_download: bool) -> Result<()> {
    // Check if we're in a git repository
    if !git::is_git_repo() {
        utils::print_error("Not a git repository. Please run this command in a git repository.");
        anyhow::bail!("Not a git repository");
    }

    // Detect OS and architecture
    let os = utils::detect_os()?;
    let arch = utils::detect_arch()?;

    println!("{} {}", "Operating System:".blue().bold(), os);
    println!("{} {}", "Architecture:".blue().bold(), arch);

    // Install gitleaks
    if !skip_download || !gitleaks::is_installed() {
        gitleaks::install_gitleaks(&os, &arch)?;
    } else {
        utils::print_info("Skipping gitleaks download (already installed)");
    }

    // Check version
    match gitleaks::get_version() {
        Ok(version) => {
            println!("{} {}", "Gitleaks version:".green().bold(), version);
        }
        Err(_) => {
            utils::print_error("Failed to install gitleaks");
            anyhow::bail!("Gitleaks installation failed");
        }
    }

    // Create configuration
    gitleaks::create_config()?;

    // Create pre-commit hook
    gitleaks::create_pre_commit_hook()?;

    // Enable gitleaks by default
    git::enable_gitleaks()?;
    utils::print_success("Gitleaks enabled by default");

    // Run initial scan
    utils::print_info("Running initial security scan...");
    match gitleaks::detect(".", Some(".gitleaks.toml")) {
        Ok(_) => {
            utils::print_success("Initial scan completed - no secrets detected!");
        }
        Err(e) => {
            utils::print_error(&format!("Security scan failed: {}", e));
            utils::print_warning("Please fix the detected secrets before committing.");
            return Err(e);
        }
    }

    println!();
    println!("{}", "═══════════════════════════════════════════".green());
    println!("{}", "  Installation completed successfully! ✨".green().bold());
    println!("{}", "═══════════════════════════════════════════".green());
    println!();
    println!("Pre-commit hook has been installed and enabled.");
    println!("Gitleaks will automatically scan for secrets before each commit.");
    println!();
    println!("Commands:");
    println!("  {} - Enable gitleaks", "gitleaks-guard enable".cyan());
    println!("  {} - Disable gitleaks", "gitleaks-guard disable".cyan());
    println!("  {} - Scan a repository", "gitleaks-guard scan".cyan());
    println!();

    Ok(())
}
