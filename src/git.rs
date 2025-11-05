use anyhow::{Context, Result};
use std::process::Command;

/// Enable gitleaks by unsetting core.hooksPath
pub fn enable_gitleaks() -> Result<()> {
    Command::new("git")
        .args(["config", "--unset", "core.hooksPath"])
        .output()
        .context("Failed to enable gitleaks")?;
    Ok(())
}

/// Disable gitleaks by setting core.hooksPath to no-hooks
pub fn disable_gitleaks() -> Result<()> {
    Command::new("git")
        .args(["config", "core.hooksPath", "no-hooks"])
        .output()
        .context("Failed to disable gitleaks")?;
    Ok(())
}

/// Check if gitleaks is enabled
pub fn is_gitleaks_enabled() -> Result<bool> {
    let output = Command::new("git")
        .args(["config", "--bool", "hooks.gitleaks-enable"])
        .output()
        .context("Failed to check gitleaks status")?;

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(result.trim() == "true")
}

/// Clone a repository
pub fn clone_repository(url: &str, dest: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["clone", "--progress", url, dest])
        .output()
        .context("Failed to clone repository")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to clone repository: {}", stderr);
    }

    Ok(())
}

/// Check if current directory is a git repository
pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Restore a file to its committed state
pub fn restore_file(file: &str) -> Result<()> {
    Command::new("git")
        .args(["restore", file])
        .output()
        .context("Failed to restore file")?;
    Ok(())
}
