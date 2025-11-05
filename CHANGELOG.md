# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-11-05

### Added
- **Interactive Setup Wizard (`init` command)**: Step-by-step guided setup for beginners
  - Checks/initializes git repository
  - Installs or updates gitleaks with confirmation
  - Configuration level selection (Standard, Strict, Minimal, Custom)
  - Pre-commit hook setup with confirmation
  - Initial security scan

- **Status Dashboard (`status` command)**: Beautiful formatted table showing
  - Git repository status with visual indicators
  - Gitleaks installation and version information
  - Configuration file status
  - Pre-commit hook status (enabled/disabled)
  - Smart recommendations for incomplete setup

- **Auto-Update (`update` command)**: Easy gitleaks version management
  - Checks current installed version
  - Fetches latest version from GitHub API
  - Shows version comparison
  - Confirms before updating
  - Force reinstall option with `--force` flag
  - Verifies successful installation

- **Multiple Configuration Levels**:
  - Standard: Default gitleaks configuration with all rules
  - Strict: Minimal allowlist for maximum security
  - Minimal: Basic detection for common secrets only (API keys, AWS, GitHub, private keys)

- **Enhanced UI/UX**:
  - Beautiful ASCII banners for each command
  - Formatted tables using `comfy-table` library
  - Color-coded status indicators (green/red/yellow)
  - Progress spinners for long operations
  - Clear, structured output with boxes and borders
  - Better terminal interaction with console library

### Changed
- Improved command help documentation with detailed descriptions
- Better error messages and user guidance
- Enhanced README with detailed examples for each command
- Updated version to 0.2.0
- Improved package description in Cargo.toml

### Dependencies Added
- `comfy-table` 7.1 - Beautiful CLI tables
- `chrono` 0.4 - Date and time handling
- `semver` 1.0 - Semantic version parsing
- `console` 0.15 - Terminal manipulation and control

### Deprecated
- Direct use of `install` command (users should prefer `init` for first-time setup)

## [0.1.0] - 2024-01-XX

### Added
- Initial Rust implementation
- `install` command - automated gitleaks installation and pre-commit hook setup
- `enable` command - enable gitleaks pre-commit checks
- `disable` command - disable gitleaks pre-commit checks
- `scan` command - scan repositories for secrets
- `version` command - check installed versions
- Support for Linux (x64, ARM64) and macOS (x64, ARM64)
- Automated CI/CD pipeline for multi-platform builds
- Comprehensive README with usage examples

## Legacy Version (Bash Scripts)

The original bash script implementation provided:
- Pre-commit hook installation
- Gitleaks integration
- Repository scanning on clone
- Enable/disable functionality

[Unreleased]: https://github.com/ruslanlap/pre-commit-auto-script/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/ruslanlap/pre-commit-auto-script/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ruslanlap/pre-commit-auto-script/releases/tag/v0.1.0
