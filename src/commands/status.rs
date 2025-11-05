use anyhow::Result;
use colored::*;
use comfy_table::{Table, Cell, Attribute, Color, ContentArrangement, presets::UTF8_FULL};
use std::path::Path;

use crate::{git, gitleaks, utils};

pub fn run() -> Result<()> {
    print_banner();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Component")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Status")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new("Details")
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
        ]);

    // Check if in git repository
    let in_git_repo = git::is_git_repo();
    table.add_row(vec![
        Cell::new("Git Repository"),
        if in_git_repo {
            Cell::new("✓").fg(Color::Green)
        } else {
            Cell::new("✗").fg(Color::Red)
        },
        if in_git_repo {
            Cell::new("Initialized")
        } else {
            Cell::new("Not a git repository")
        },
    ]);

    // Check gitleaks installation
    let gitleaks_installed = gitleaks::is_installed();
    let gitleaks_version = if gitleaks_installed {
        gitleaks::get_version().unwrap_or_else(|_| "Unknown".to_string())
    } else {
        "Not installed".to_string()
    };

    table.add_row(vec![
        Cell::new("Gitleaks"),
        if gitleaks_installed {
            Cell::new("✓").fg(Color::Green)
        } else {
            Cell::new("✗").fg(Color::Red)
        },
        Cell::new(&gitleaks_version),
    ]);

    // Check configuration file
    let config_exists = Path::new(".gitleaks.toml").exists();
    table.add_row(vec![
        Cell::new("Configuration"),
        if config_exists {
            Cell::new("✓").fg(Color::Green)
        } else {
            Cell::new("✗").fg(Color::Yellow)
        },
        if config_exists {
            Cell::new(".gitleaks.toml found")
        } else {
            Cell::new("No config file")
        },
    ]);

    // Check pre-commit hook
    let hook_path = Path::new(".git/hooks/pre-commit");
    let hook_exists = hook_path.exists();
    let hook_enabled = if hook_exists {
        hook_path.metadata()
            .map(|m| {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    m.permissions().mode() & 0o111 != 0
                }
                #[cfg(not(unix))]
                {
                    true
                }
            })
            .unwrap_or(false)
    } else {
        false
    };

    table.add_row(vec![
        Cell::new("Pre-commit Hook"),
        if hook_enabled {
            Cell::new("✓ Enabled").fg(Color::Green)
        } else if hook_exists {
            Cell::new("⚠ Disabled").fg(Color::Yellow)
        } else {
            Cell::new("✗ Not installed").fg(Color::Red)
        },
        if hook_enabled {
            Cell::new("Active and protecting")
        } else if hook_exists {
            Cell::new("Installed but disabled")
        } else {
            Cell::new("Not installed")
        },
    ]);

    println!("{table}");

    // Overall status summary
    println!();
    if gitleaks_installed && config_exists && hook_enabled && in_git_repo {
        println!("{}", "╔═══════════════════════════════════════════╗".green());
        println!("{}", "║  ✅ Your repository is fully protected!  ║".green().bold());
        println!("{}", "╚═══════════════════════════════════════════╝".green());
    } else {
        println!("{}", "╔═══════════════════════════════════════════╗".yellow());
        println!("{}", "║  ⚠️  Setup incomplete or disabled        ║".yellow().bold());
        println!("{}", "╚═══════════════════════════════════════════╝".yellow());
        println!();
        println!("{}", "Recommendations:".yellow().bold());

        if !in_git_repo {
            println!("  {} Initialize git repository: {}", "•".yellow(), "git init".cyan());
        }
        if !gitleaks_installed {
            println!("  {} Install gitleaks: {}", "•".yellow(), "gitleaks-guard init".cyan());
        }
        if !config_exists {
            println!("  {} Create configuration: {}", "•".yellow(), "gitleaks-guard init".cyan());
        }
        if !hook_enabled && hook_exists {
            println!("  {} Enable pre-commit hook: {}", "•".yellow(), "gitleaks-guard enable".cyan());
        } else if !hook_exists {
            println!("  {} Setup pre-commit hook: {}", "•".yellow(), "gitleaks-guard init".cyan());
        }
    }

    println!();
    Ok(())
}

fn print_banner() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════╗".blue());
    println!("{}", "║      📊 GitLeaks Guard Status 📊         ║".blue().bold());
    println!("{}", "╚═══════════════════════════════════════════╝".blue());
    println!();
}
