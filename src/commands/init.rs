use anyhow::{Context, Result};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use console::Term;

use crate::{git, gitleaks, utils};

pub async fn run() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    // Welcome banner
    print_welcome_banner();

    // Check if we're in a git repository
    if !git::is_git_repo() {
        utils::print_error("Not a git repository.");

        let initialize = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to initialize a git repository here?")
            .default(false)
            .interact()?;

        if initialize {
            utils::execute_command("git", &["init"])?;
            utils::print_success("Git repository initialized!");
        } else {
            anyhow::bail!("Please run this command in a git repository");
        }
    }

    println!();
    utils::print_info("Welcome to GitLeaks Guard Interactive Setup!");
    println!();

    // Step 1: Check if gitleaks is installed
    let install_gitleaks = if gitleaks::is_installed() {
        let version = gitleaks::get_version().unwrap_or_else(|_| "unknown".to_string());
        utils::print_success(&format!("Gitleaks is already installed (version: {})", version));

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to reinstall/update gitleaks?")
            .default(false)
            .interact()?
    } else {
        utils::print_warning("Gitleaks is not installed");

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Install gitleaks now?")
            .default(true)
            .interact()?
    };

    if install_gitleaks {
        let os = utils::detect_os()?;
        let arch = utils::detect_arch()?;

        println!();
        println!("{} {}", "OS:".blue().bold(), os);
        println!("{} {}", "Architecture:".blue().bold(), arch);
        println!();

        gitleaks::install_gitleaks(&os, &arch)?;

        let version = gitleaks::get_version()?;
        utils::print_success(&format!("Gitleaks installed successfully! (version: {})", version));
    }

    // Step 2: Configuration options
    println!();
    let config_options = vec![
        "Standard (Recommended) - Detects common secrets",
        "Strict - More aggressive detection",
        "Minimal - Basic detection only",
        "Custom - I'll configure it myself later",
    ];

    let config_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose gitleaks configuration level")
        .default(0)
        .items(&config_options)
        .interact()?;

    match config_selection {
        0 => {
            utils::print_info("Creating standard configuration...");
            gitleaks::create_config()?;
        }
        1 => {
            utils::print_info("Creating strict configuration...");
            gitleaks::create_strict_config()?;
        }
        2 => {
            utils::print_info("Creating minimal configuration...");
            gitleaks::create_minimal_config()?;
        }
        3 => {
            utils::print_info("Skipping configuration - you can create .gitleaks.toml manually");
        }
        _ => unreachable!(),
    }

    // Step 3: Setup pre-commit hook
    println!();
    let setup_hook = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Setup pre-commit hook to prevent secret leaks?")
        .default(true)
        .interact()?;

    if setup_hook {
        gitleaks::create_pre_commit_hook()?;
        utils::print_success("Pre-commit hook installed!");

        git::enable_gitleaks()?;
        utils::print_success("Pre-commit hook enabled!");
    }

    // Step 4: Run initial scan
    println!();
    let run_scan = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Run an initial security scan now?")
        .default(true)
        .interact()?;

    if run_scan {
        println!();
        utils::print_info("Running initial security scan...");
        match gitleaks::detect(".", Some(".gitleaks.toml")) {
            Ok(_) => {
                utils::print_success("Initial scan completed - no secrets detected!");
            }
            Err(e) => {
                utils::print_error(&format!("Security scan failed: {}", e));
                utils::print_warning("Please fix the detected secrets before committing.");
            }
        }
    }

    // Final summary
    print_completion_summary(setup_hook);

    Ok(())
}

fn print_welcome_banner() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                                                           ║".cyan());
    println!("{}", "║          🔒 GitLeaks Guard Interactive Setup 🔒          ║".cyan().bold());
    println!("{}", "║                                                           ║".cyan());
    println!("{}", "║         Protect your code from secret leaks!             ║".cyan());
    println!("{}", "║                                                           ║".cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════╝".cyan());
    println!();
}

fn print_completion_summary(hook_enabled: bool) {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗".green());
    println!("{}", "║                                                           ║".green());
    println!("{}", "║              ✨ Setup Completed Successfully! ✨          ║".green().bold());
    println!("{}", "║                                                           ║".green());
    println!("{}", "╚═══════════════════════════════════════════════════════════╝".green());
    println!();

    if hook_enabled {
        println!("{}", "Your repository is now protected! 🛡️".green().bold());
        println!();
        println!("What happens next:");
        println!("  {} Gitleaks will scan your code before each commit", "•".green());
        println!("  {} Commits with secrets will be automatically blocked", "•".green());
        println!("  {} You'll be notified of any security issues", "•".green());
        println!();
    }

    println!("Useful commands:");
    println!("  {} - Show current status", "gitleaks-guard status".cyan());
    println!("  {} - Scan a repository", "gitleaks-guard scan".cyan());
    println!("  {} - Disable protection temporarily", "gitleaks-guard disable".cyan());
    println!("  {} - Re-enable protection", "gitleaks-guard enable".cyan());
    println!("  {} - Update gitleaks", "gitleaks-guard update".cyan());
    println!();
}
