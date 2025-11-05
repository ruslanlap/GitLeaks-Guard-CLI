# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete rewrite in Rust for better performance and reliability
- New CLI interface with subcommands (install, enable, disable, scan, version)
- Cross-platform support (Linux x64/ARM64, macOS x64/ARM64, Windows x64/ARM64)
- Automatic gitleaks download and installation for all platforms
- Windows support with ZIP archive extraction
- Platform-specific installation paths (Windows: Program Files, Unix: /usr/local/bin)
- Repository scanning capability (local and remote)
- Beautiful colored CLI output with progress indicators
- Interactive prompts for cleanup operations
- GitHub Actions workflows for CI/CD with multi-platform builds
- Comprehensive documentation and examples with Windows-specific instructions

### Changed
- Migrated from bash scripts to Rust CLI tool
- Improved error handling and user feedback
- Better platform detection and architecture support
- Updated CI/CD workflows to use modern actions (dtolnay/rust-toolchain instead of actions-rs)
- Enhanced release workflow with fail-fast: false for better parallel builds

### Deprecated
- Legacy bash scripts (still available for backward compatibility)

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

[Unreleased]: https://github.com/ruslanlap/pre-commit-auto-script/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ruslanlap/pre-commit-auto-script/releases/tag/v0.1.0
