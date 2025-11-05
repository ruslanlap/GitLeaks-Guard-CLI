use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::{git, gitleaks, utils};

pub async fn run(url: Option<String>, path: Option<String>, cleanup: bool) -> Result<()> {
    // Check if gitleaks is installed
    if !gitleaks::is_installed() {
        utils::print_error("Gitleaks is not installed. Please run 'gitleaks-guard install' first.");
        anyhow::bail!("Gitleaks not installed");
    }

    // Print ASCII art
    print_banner();

    let scan_path: PathBuf;
    let should_cleanup: bool;

    if let Some(repo_url) = url {
        // Clone and scan remote repository
        utils::print_info(&format!("Repository URL: {}", repo_url));

        let clone_dir = PathBuf::from("cloned_repos").join(
            repo_url
                .split('/')
                .last()
                .unwrap_or("repo")
                .trim_end_matches(".git"),
        );

        // Remove existing directory if it exists
        if clone_dir.exists() {
            utils::print_warning(&format!(
                "Removing existing clone: {}",
                clone_dir.display()
            ));
            fs::remove_dir_all(&clone_dir).context("Failed to remove existing clone")?;
        }

        // Clone repository
        utils::print_info("Cloning repository...");
        git::clone_repository(&repo_url, clone_dir.to_str().unwrap())?;
        utils::print_success("Repository cloned successfully!");

        scan_path = clone_dir;
        should_cleanup = if cleanup {
            true
        } else {
            // Ask user if they want to clean up
            Confirm::new()
                .with_prompt("Clean up cloned repository after scan?")
                .default(false)
                .interact()
                .unwrap_or(false)
        };
    } else if let Some(p) = path {
        // Scan local path
        scan_path = PathBuf::from(p);
        should_cleanup = false;

        if !scan_path.exists() {
            utils::print_error(&format!("Path does not exist: {}", scan_path.display()));
            anyhow::bail!("Path does not exist");
        }
    } else {
        // Scan current directory
        scan_path = env::current_dir().context("Failed to get current directory")?;
        should_cleanup = false;
        utils::print_info("Scanning current directory...");
    }

    // Run gitleaks scan
    println!();
    println!("{}", "═══════════════════════════════════════════".blue());
    println!("  {} {}", "🔍".bold(), "Running Gitleaks scan...".blue().bold());
    println!("{}", "═══════════════════════════════════════════".blue());
    println!();

    let result = gitleaks::detect(scan_path.to_str().unwrap(), None);

    match result {
        Ok(_) => {
            println!();
            println!("{}", "═══════════════════════════════════════════".green());
            println!(
                "  {} {}",
                "✔".green().bold(),
                "Scan completed successfully!".green().bold()
            );
            println!("{}", "═══════════════════════════════════════════".green());
            println!();
            utils::print_success("No secrets detected in the repository!");
        }
        Err(e) => {
            println!();
            println!("{}", "═══════════════════════════════════════════".red());
            println!(
                "  {} {}",
                "✘".red().bold(),
                "Secrets detected!".red().bold()
            );
            println!("{}", "═══════════════════════════════════════════".red());
            println!();
            utils::print_warning("Gitleaks found potential secrets in the repository.");
            println!("Please review the output above and remove any sensitive information.");

            if should_cleanup {
                cleanup_clone(&scan_path)?;
            }

            return Err(e);
        }
    }

    // Cleanup if requested
    if should_cleanup {
        cleanup_clone(&scan_path)?;
    }

    println!("{}", "✨ Task completed! ✨".green().bold());
    Ok(())
}

fn print_banner() {
    println!();
    println!("{}", "  ____   __   _  _  _ ".cyan());
    println!("{}", " / ___\\ / /  | || | || |".cyan());
    println!("{}", "|  _  |\\ \\  | || || || |".cyan());
    println!("{}", "| |_| | \\_\\ | || || || |".cyan());
    println!("{}", " \\____/  \\___|_||_||_|".cyan());
    println!();
    println!("{}", "** GitLeaks Scanner **".bold().blue());
    println!();
}

fn cleanup_clone(path: &PathBuf) -> Result<()> {
    utils::print_info("Cleaning up cloned repository...");
    fs::remove_dir_all(path).context("Failed to clean up cloned repository")?;

    // Also remove parent cloned_repos directory if empty
    if let Some(parent) = path.parent() {
        if parent.file_name() == Some(std::ffi::OsStr::new("cloned_repos")) {
            if let Ok(entries) = fs::read_dir(parent) {
                if entries.count() == 0 {
                    fs::remove_dir(parent).ok();
                }
            }
        }
    }

    utils::print_success("Cleanup completed!");
    Ok(())
}
