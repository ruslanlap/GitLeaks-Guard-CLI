use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod gitleaks;
mod git;
mod utils;

/// GitLeaks Guard - Automated security pre-commit hook installer
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "gitleaks-guard")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive setup wizard (recommended for first-time setup)
    Init,
    /// Install gitleaks and setup pre-commit hooks
    Install {
        /// Skip gitleaks download if already installed
        #[arg(short, long)]
        skip_download: bool,
    },
    /// Show current status and configuration
    Status,
    /// Enable gitleaks pre-commit hook
    Enable,
    /// Disable gitleaks pre-commit hook
    Disable,
    /// Scan a repository for secrets
    Scan {
        /// Repository URL to scan
        #[arg(short, long)]
        url: Option<String>,
        /// Local path to scan (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
        /// Clean up cloned repository after scan
        #[arg(short, long)]
        cleanup: bool,
    },
    /// Update gitleaks to the latest version
    Update {
        /// Force reinstall even if already on latest version
        #[arg(short, long)]
        force: bool,
    },
    /// Check gitleaks version
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            commands::init::run().await?;
        }
        Commands::Install { skip_download } => {
            println!("{}", "🔒 GitLeaks Guard - Installation".bold().blue());
            commands::install::run(skip_download).await?;
        }
        Commands::Status => {
            commands::status::run()?;
        }
        Commands::Enable => {
            println!("{}", "✅ Enabling GitLeaks".bold().green());
            commands::toggle::enable()?;
        }
        Commands::Disable => {
            println!("{}", "❌ Disabling GitLeaks".bold().red());
            commands::toggle::disable()?;
        }
        Commands::Scan { url, path, cleanup } => {
            println!("{}", "🕵️‍♂️ GitLeaks Scanner".bold().blue());
            commands::scan::run(url, path, cleanup).await?;
        }
        Commands::Update { force } => {
            commands::update::run(force).await?;
        }
        Commands::Version => {
            commands::version::check()?;
        }
    }

    Ok(())
}
