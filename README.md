# GitLeaks Guard 🔒

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub last commit](https://img.shields.io/github/last-commit/ruslanlap/pre-commit-auto-script)
![GitHub issues](https://img.shields.io/github/issues/ruslanlap/pre-commit-auto-script)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

**A powerful Rust CLI tool for automated gitleaks installation and pre-commit hook management**

GitLeaks Guard is a modern, fast CLI tool written in Rust that automates the installation and configuration of pre-commit hooks in your Git repository. It uses [gitleaks](https://github.com/gitleaks/gitleaks) to scan your codebase for sensitive information and prevent security leaks before they happen.

![Alt](data/Example0.png)

[![Typing SVG](https://readme-typing-svg.herokuapp.com?font=Fira+Code&duration=2000&pause=1000&color=07F758&center=true&vCenter=true&multiline=true&width=700&height=100&lines=pre-commit+hook+script+with+automatic+installation;just+copy+and+run+the+following+command+%F0%9F%9A%80)](https://git.io/typing-svg)

## Table of Contents 📋

- [Features](#features)
- [Installation](#installation)
  - [From Source](#from-source)
  - [Pre-built Binaries](#pre-built-binaries)
- [Requirements](#requirements)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Usage Examples](#usage-examples)
- [Legacy Bash Scripts](#legacy-bash-scripts)
- [Contributing](#contributing)
- [License](#license)

## Features ✨

- **Automated Installation**: One command to install and configure gitleaks
- **Cross-Platform**: Supports Linux (x64, ARM64), macOS (x64, ARM64), and Windows (x64, ARM64)
- **Fast & Reliable**: Written in Rust for performance and safety
- **Easy Management**: Simple commands to enable/disable security checks
- **Repository Scanning**: Scan any Git repository (local or remote) for secrets
- **Pre-commit Integration**: Automatically prevents commits containing sensitive data
- **Beautiful CLI**: Colorful output with progress indicators

## Requirements 💾

- **Git** - Version control system
- **Rust** (for building from source) - Install from [rustup.rs](https://rustup.rs/)
- **Linux/macOS**: sudo access (for installing gitleaks to `/usr/local/bin`)
- **Windows**: Administrator access (for installing gitleaks to `Program Files`)

## Installation

### From Source

**Linux/macOS:**
```bash
# Clone the repository
git clone https://github.com/ruslanlap/pre-commit-auto-script.git
cd pre-commit-auto-script

# Build and install
cargo build --release

# Optionally, install globally
sudo cp target/release/gitleaks-guard /usr/local/bin/
```

**Windows (PowerShell):**
```powershell
# Clone the repository
git clone https://github.com/ruslanlap/pre-commit-auto-script.git
cd pre-commit-auto-script

# Build and install
cargo build --release

# Optionally, add to PATH
Copy-Item target\release\gitleaks-guard.exe -Destination "C:\Program Files\gitleaks-guard\"
# Add C:\Program Files\gitleaks-guard to your PATH environment variable
```

### Pre-built Binaries

Pre-built binaries will be available in the [Releases](https://github.com/ruslanlap/pre-commit-auto-script/releases) page.

**Linux:**
```bash
# Download the latest release for your platform
curl -LO https://github.com/ruslanlap/pre-commit-auto-script/releases/latest/download/gitleaks-guard-linux-x64

# Make it executable
chmod +x gitleaks-guard-linux-x64

# Move to PATH
sudo mv gitleaks-guard-linux-x64 /usr/local/bin/gitleaks-guard
```

**macOS:**
```bash
# For Intel Macs (x64)
curl -LO https://github.com/ruslanlap/pre-commit-auto-script/releases/latest/download/gitleaks-guard-macos-x64

# For Apple Silicon (ARM64)
curl -LO https://github.com/ruslanlap/pre-commit-auto-script/releases/latest/download/gitleaks-guard-macos-arm64

# Make it executable and install
chmod +x gitleaks-guard-macos-*
sudo mv gitleaks-guard-macos-* /usr/local/bin/gitleaks-guard
```

**Windows (PowerShell as Administrator):**
```powershell
# Download the latest release
Invoke-WebRequest -Uri "https://github.com/ruslanlap/pre-commit-auto-script/releases/latest/download/gitleaks-guard-windows-x64.exe" -OutFile "gitleaks-guard.exe"

# Move to a directory in PATH
Move-Item gitleaks-guard.exe -Destination "C:\Program Files\gitleaks-guard\gitleaks-guard.exe"

# Add to PATH if needed
$env:Path += ";C:\Program Files\gitleaks-guard"
```

## Quick Start

Once installed, navigate to your Git repository and run:

```bash
# Install gitleaks and setup pre-commit hooks
gitleaks-guard install
```

That's it! Your repository is now protected against accidental secret commits.

## Commands

GitLeaks Guard provides the following commands:

### `install`

Install gitleaks and setup pre-commit hooks in your repository.

```bash
gitleaks-guard install

# Skip downloading gitleaks if already installed
gitleaks-guard install --skip-download
```

### `enable`

Enable the gitleaks pre-commit hook.

```bash
gitleaks-guard enable
```

### `disable`

Disable the gitleaks pre-commit hook.

```bash
gitleaks-guard disable
```

### `scan`

Scan a repository for secrets.

```bash
# Scan current directory
gitleaks-guard scan

# Scan a specific path
gitleaks-guard scan --path /path/to/repo

# Scan a remote repository
gitleaks-guard scan --url https://github.com/user/repo

# Scan and auto-cleanup
gitleaks-guard scan --url https://github.com/user/repo --cleanup
```

### `version`

Check installed versions.

```bash
gitleaks-guard version
```

## Usage Examples

### Initial Setup

```bash
# Navigate to your Git repository
cd my-project

# Install and configure gitleaks
gitleaks-guard install
```

### Scanning Repositories

```bash
# Scan a public repository before cloning
gitleaks-guard scan --url https://github.com/suspicious/repo --cleanup

# Scan your current project
gitleaks-guard scan

# Scan a specific directory
gitleaks-guard scan --path /path/to/another/project
```

### Managing Hooks

```bash
# Temporarily disable for a quick commit (not recommended!)
gitleaks-guard disable

# Re-enable protection
gitleaks-guard enable
```

### Customization

After installation, you can customize the gitleaks configuration by editing `.gitleaks.toml` in your repository root.

## Legacy Bash Scripts

The original bash scripts are still available in this repository for backward compatibility:

- `install.sh` - Original installation script
- `pre-commit.sh` - Pre-commit hook script
- `on-off-gitleaks.sh` - Enable/disable script
- `gitleaks_on_clone` - Repository scanning script

### Using Legacy Scripts

```bash
# Install using bash script
curl -sSfL https://raw.githubusercontent.com/ruslanlap/pre-commit-auto-script/main/install.sh | bash

# Enable/Disable
source on-off-gitleaks.sh; enable
source on-off-gitleaks.sh; disable

# Scan on clone
curl -sSfL https://raw.githubusercontent.com/ruslanlap/pre-commit-auto-script/main/gitleaks_on_clone && chmod +x gitleaks_on_clone && ./gitleaks_on_clone
```

**Note**: We recommend using the Rust CLI tool for better performance and maintainability.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Why Rust?

- **Performance**: Compiled binary is fast and lightweight
- **Safety**: Memory-safe without garbage collection
- **Cross-platform**: Easy to build for multiple platforms
- **Modern**: Better error handling and dependency management
- **Single Binary**: No runtime dependencies (unlike bash scripts)

## Roadmap

- [x] Windows support (✅ Completed in v0.1.0)
- [ ] Interactive configuration wizard
- [ ] Custom rule templates
- [ ] Integration with CI/CD pipelines
- [ ] Docker image
- [ ] Homebrew formula
- [ ] Chocolatey package (Windows)
- [ ] Snap package (Linux)

## License

MIT License - see [LICENSE.txt](LICENSE.txt) for details

---

🔒 Stay secure with GitLeaks Guard! Happy coding! 🚀
