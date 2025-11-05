use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::process::Command;

/// Detect the operating system
pub fn detect_os() -> Result<String> {
    let os = env::consts::OS;
    match os {
        "linux" => Ok("linux".to_string()),
        "macos" => Ok("darwin".to_string()),
        "windows" => Ok("windows".to_string()),
        _ => anyhow::bail!("Unsupported operating system: {}", os),
    }
}

/// Detect the system architecture
pub fn detect_arch() -> Result<String> {
    let arch = env::consts::ARCH;
    match arch {
        "x86_64" => Ok("x64".to_string()),
        "aarch64" | "arm64" => Ok("arm64".to_string()),
        _ => anyhow::bail!("Unsupported architecture: {}", arch),
    }
}

/// Execute a shell command and return output
pub fn execute_command(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute command: {} {:?}", cmd, args))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Print a success message
pub fn print_success(msg: &str) {
    println!("{} {}", "✔".green().bold(), msg);
}

/// Print an error message
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✘".red().bold(), msg);
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

/// Print a warning message
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

/// Check if a command exists in PATH
pub fn command_exists(cmd: &str) -> bool {
    #[cfg(windows)]
    {
        Command::new("where")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[cfg(not(windows))]
    {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
