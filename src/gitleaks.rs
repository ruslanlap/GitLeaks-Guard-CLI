use anyhow::{Context, Result};
use colored::*;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::Command;
use tar::Archive;

#[cfg(windows)]
use std::io::Read;
#[cfg(windows)]
use zip::ZipArchive;

use crate::utils;

#[derive(Debug, Deserialize, Serialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

/// Download and install gitleaks
pub fn install_gitleaks(os: &str, arch: &str) -> Result<()> {
    utils::print_info("Fetching latest gitleaks release...");

    let client = Client::new();
    let url = "https://api.github.com/repos/gitleaks/gitleaks/releases/latest";

    let response = client
        .get(url)
        .header("User-Agent", "gitleaks-guard")
        .send()
        .context("Failed to fetch gitleaks releases")?;

    let release: GithubRelease = response.json().context("Failed to parse release data")?;

    // Find the appropriate asset
    let platform_str = get_platform_string(os, arch);
    let version_str = release.tag_name.trim_start_matches('v');
    let asset_pattern = format!("gitleaks_{}_{}", version_str, platform_str);

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(&platform_str) && a.name.contains(&format!("gitleaks_{}", version_str)))
        .context(format!("No asset found for {} {} (looking for pattern: {})", os, arch, asset_pattern))?;

    utils::print_info(&format!("Downloading gitleaks {}...", release.tag_name));

    // Download the file
    let response = client
        .get(&asset.browser_download_url)
        .send()
        .context("Failed to download gitleaks")?;

    let total_size = response.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .expect("Failed to create progress bar")
            .progress_chars("#>-"),
    );

    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let download_path = temp_dir.path().join(&asset.name);
    let mut file = File::create(&download_path).context("Failed to create download file")?;

    let content = response.bytes().context("Failed to read download content")?;
    file.write_all(&content).context("Failed to write download")?;
    pb.finish_with_message("Download complete");

    // Extract the archive
    utils::print_info("Extracting gitleaks...");
    let extract_path = temp_dir.path().join("extracted");
    fs::create_dir_all(&extract_path).context("Failed to create extraction directory")?;

    #[cfg(not(windows))]
    {
        let tar_gz = File::open(&download_path).context("Failed to open downloaded file")?;
        let tar = GzDecoder::new(BufReader::new(tar_gz));
        let mut archive = Archive::new(tar);
        archive.unpack(&extract_path).context("Failed to extract archive")?;
    }

    #[cfg(windows)]
    {
        let file = File::open(&download_path).context("Failed to open downloaded file")?;
        let mut archive = ZipArchive::new(file).context("Failed to open zip archive")?;
        archive.extract(&extract_path).context("Failed to extract zip")?;
    }

    // Determine binary name and install path
    #[cfg(windows)]
    let binary_name = "gitleaks.exe";
    #[cfg(not(windows))]
    let binary_name = "gitleaks";

    let binary_path = extract_path.join(binary_name);
    
    // Verify binary exists after extraction
    if !binary_path.exists() {
        anyhow::bail!("Binary not found after extraction: {}", binary_path.display());
    }

    #[cfg(target_os = "linux")]
    let install_path = "/usr/local/bin/gitleaks";
    #[cfg(target_os = "macos")]
    let install_path = "/usr/local/bin/gitleaks";
    #[cfg(target_os = "windows")]
    let install_path = {
        let program_files = std::env::var("ProgramFiles")
            .unwrap_or_else(|_| "C:\\Program Files".to_string());
        format!("{}\\gitleaks\\gitleaks.exe", program_files)
    };

    // Install based on platform
    #[cfg(not(windows))]
    {
        utils::print_info("Installing gitleaks to /usr/local/bin...");

        // Try with sudo if needed
        let output = Command::new("sudo")
            .args(["mv", binary_path.to_str().unwrap(), install_path])
            .output()
            .context("Failed to install gitleaks")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to install gitleaks: {}", stderr);
        }

        // Make it executable
        let chmod_output = Command::new("sudo")
            .args(["chmod", "+x", install_path])
            .output()
            .context("Failed to make gitleaks executable")?;
        
        if !chmod_output.status.success() {
            let stderr = String::from_utf8_lossy(&chmod_output.stderr);
            anyhow::bail!("Failed to make gitleaks executable: {}", stderr);
        }
    }

    #[cfg(windows)]
    {
        utils::print_info("Installing gitleaks to Program Files...");

        let install_dir = Path::new(&install_path).parent().unwrap();
        fs::create_dir_all(install_dir).context("Failed to create installation directory")?;

        fs::copy(&binary_path, &install_path)
            .context("Failed to copy gitleaks.exe. You may need to run as Administrator.")?;

        utils::print_warning("Please ensure the installation directory is in your PATH:");
        utils::print_info(&format!("  {}", install_dir.display()));
    }

    utils::print_success("Gitleaks installed successfully!");
    Ok(())
}

/// Get platform string for gitleaks download
fn get_platform_string(os: &str, arch: &str) -> String {
    match (os, arch) {
        ("linux", "x64") => "linux_x64.tar.gz",
        ("linux", "arm64") => "linux_arm64.tar.gz",
        ("darwin", "x64") => "darwin_x64.tar.gz",
        ("darwin", "arm64") => "darwin_arm64.tar.gz",
        ("windows", "x64") => "windows_x64.zip",
        ("windows", "arm64") => "windows_arm64.zip",
        _ => "linux_x64.tar.gz", // default fallback
    }
    .to_string()
}

/// Get gitleaks version
pub fn get_version() -> Result<String> {
    let output = Command::new("gitleaks")
        .arg("version")
        .output()
        .context("Failed to get gitleaks version")?;

    if !output.status.success() {
        anyhow::bail!("Gitleaks is not installed or not accessible");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Create gitleaks configuration file
pub fn create_config() -> Result<()> {
    utils::print_info("Creating .gitleaks.toml configuration...");

    let config_header = r#"[[rules]]
regex = "API[_-]?KEY"
tags = ["api-key", "token"]

"#;

    // Download the default config from gitleaks repository
    let client = Client::new();
    let config_url = "https://raw.githubusercontent.com/gitleaks/gitleaks/master/config/gitleaks.toml";

    let response = client
        .get(config_url)
        .send()
        .context("Failed to download gitleaks config")?;

    let default_config = response.text().context("Failed to read config")?;

    // Combine configs
    let full_config = format!("{}{}", config_header, default_config);

    // Write to file
    fs::write(".gitleaks.toml", full_config).context("Failed to write config file")?;

    utils::print_success("Configuration file created!");
    Ok(())
}

/// Run gitleaks detect on current directory
pub fn detect(path: &str, config: Option<&str>) -> Result<()> {
    utils::print_info("Running gitleaks detect...");

    let mut args = vec!["detect", "--source", path, "--verbose"];

    if let Some(cfg) = config {
        args.push("--config");
        args.push(cfg);
    }

    let output = Command::new("gitleaks")
        .args(&args)
        .output()
        .context("Failed to run gitleaks")?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stdout);
        eprintln!("{}", stderr);
        anyhow::bail!("Gitleaks detected secrets in the repository!");
    }

    utils::print_success("No secrets detected!");
    Ok(())
}

/// Check if gitleaks is installed
pub fn is_installed() -> bool {
    utils::command_exists("gitleaks")
}

/// Create pre-commit hook script
pub fn create_pre_commit_hook() -> Result<()> {
    utils::print_info("Creating pre-commit hook...");

    let hook_content = r#"#!/bin/bash

# Color codes for terminal output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to check if Gitleaks is enabled in Git config
function is_gitleaks_enabled() {
    git config --bool hooks.gitleaks-enable
}

# Function to run Gitleaks and check for secrets
function run_gitleaks() {
    echo -e "${GREEN}Running Gitleaks...${NC}"
    gitleaksOutput=$(gitleaks detect --redact --verbose --report-format json --report-path gitleaks-report.json --config .gitleaks.toml)
    gitleaksExitCode=$?

    # Check if Gitleaks found any secrets in the repository
    if [[ $gitleaksExitCode -eq 1 ]]; then
        echo -e "${RED}Found the following secrets:${NC}"
        echo "$gitleaksOutput"
        echo -e "${RED}Committing with existing secrets is not allowed.${NC}"
        exit 1
    else
        echo -e "${GREEN}Secrets check passed successfully.${NC}"
    fi
}

# Main script execution
if [[ "$(is_gitleaks_enabled)" == "true" ]]; then
    run_gitleaks
else
    echo -e "${BLUE}Gitleaks is not enabled in the Git config.${NC}"
fi
"#;

    let hooks_dir = Path::new(".git/hooks");
    fs::create_dir_all(hooks_dir).context("Failed to create hooks directory")?;

    let hook_file = hooks_dir.join("pre-commit");
    fs::write(&hook_file, hook_content).context("Failed to write pre-commit hook")?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_file)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_file, perms)?;
    }

    utils::print_success("Pre-commit hook created!");
    Ok(())
}
