use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{gitleaks, utils};

pub async fn run(force: bool) -> Result<()> {
    print_banner();

    // Check if gitleaks is installed
    if !gitleaks::is_installed() {
        utils::print_error("Gitleaks is not installed.");
        utils::print_info("Run 'gitleaks-guard init' to install it.");
        anyhow::bail!("Gitleaks not installed");
    }

    // Get current version
    let current_version = gitleaks::get_version()?;
    println!("{} {}", "Current version:".blue().bold(), current_version);

    // Get latest version from GitHub
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    spinner.set_message("Checking for updates...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let latest_version = match get_latest_version().await {
        Ok(version) => {
            spinner.finish_with_message("✓ Update check complete");
            version
        }
        Err(e) => {
            spinner.finish_with_message("✗ Failed to check for updates");
            utils::print_error(&format!("Could not check for updates: {}", e));
            anyhow::bail!("Update check failed");
        }
    };

    println!("{} {}", "Latest version:".blue().bold(), latest_version);
    println!();

    // Compare versions
    let needs_update = compare_versions(&current_version, &latest_version);

    if !needs_update && !force {
        utils::print_success("You are already running the latest version!");
        return Ok(());
    }

    if !needs_update && force {
        utils::print_warning("You are already on the latest version, but forcing reinstall...");
    } else {
        utils::print_info(&format!("A new version is available: {} → {}", current_version, latest_version));
    }

    println!();
    let should_update = if force {
        true
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to update gitleaks?")
            .default(true)
            .interact()?
    };

    if !should_update {
        utils::print_info("Update cancelled.");
        return Ok(());
    }

    // Perform update
    println!();
    utils::print_info("Updating gitleaks...");

    let os = utils::detect_os()?;
    let arch = utils::detect_arch()?;

    gitleaks::install_gitleaks(&os, &arch)?;

    // Verify installation
    let new_version = gitleaks::get_version()?;

    println!();
    println!("{}", "╔═══════════════════════════════════════════╗".green());
    println!("{}", "║   ✨ Update completed successfully! ✨   ║".green().bold());
    println!("{}", "╚═══════════════════════════════════════════╝".green());
    println!();
    println!("{} {} → {}", "Version:".blue().bold(), current_version, new_version.green().bold());
    println!();

    Ok(())
}

async fn get_latest_version() -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("gitleaks-guard")
        .build()?;

    let response = client
        .get("https://api.github.com/repos/gitleaks/gitleaks/releases/latest")
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;

    let tag_name = json["tag_name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Could not parse version"))?
        .trim_start_matches('v');

    Ok(tag_name.to_string())
}

fn compare_versions(current: &str, latest: &str) -> bool {
    let current = current.trim_start_matches('v');
    let latest = latest.trim_start_matches('v');

    // Simple version comparison (assuming semantic versioning)
    current != latest
}

fn print_banner() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════╗".blue());
    println!("{}", "║       🔄 GitLeaks Update Manager 🔄      ║".blue().bold());
    println!("{}", "╚═══════════════════════════════════════════╝".blue());
    println!();
}
