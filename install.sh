#!/bin/bash

# Color codes and emojis
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color
CHECK_MARK="\xE2\x9C\x94"
CROSS_MARK="\xE2\x9D\x8C"

create_gitleaks_config() {
    echo '[[rules]]
regex = "API[_-]?KEY"
tags = ["api-key", "token" ]' > .gitleaks.toml

    # Download the additional configuration from the URL
    additional_config=$(curl -s https://raw.githubusercontent.com/gitleaks/gitleaks/master/config/gitleaks.toml)

    # Append the additional configuration to the .gitleaks.toml file
    echo "$additional_config" >> .gitleaks.toml
}

# Determine the operating system
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="darwin"
else
    echo -e "${RED}Unsupported operating system.${NC}"
    exit 1
fi

echo -e "${BLUE}Operating System: ${BOLD}$OS${NC}"

# Determine the architecture
ARCH=$(uname -m)
if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="x64"
elif [[ "$ARCH" == "aarch64" ]]; then
    ARCH="arm"
else
    echo -e "${RED}Unsupported architecture.${NC}"
    exit 1
fi

echo -e "${BLUE}Architecture: ${BOLD}$ARCH${NC}"

# Download and install gitleaks based on OS and architecture
echo -e "${GREEN}Downloading and installing gitleaks...${NC}"
if [[ "$OS" == "linux" && "$ARCH" == "arm" ]]; then
    curl -s https://api.github.com/repos/gitleaks/gitleaks/releases/latest | grep browser_download_url | cut -d '"' -f 4 | grep 'linux_arm' | wget -i -
    tar xf gitleaks_*_linux_arm.tar.gz
elif [[ "$OS" == "linux" && "$ARCH" == "x64" ]]; then
    curl -s https://api.github.com/repos/gitleaks/gitleaks/releases/latest | grep browser_download_url | cut -d '"' -f 4 | grep 'linux_x64' | wget -i -
    tar xf gitleaks_*_linux_x64.tar.gz
elif [[ "$OS" == "darwin" && "$ARCH" == "x64" ]]; then
    curl -s https://api.github.com/repos/gitleaks/gitleaks/releases/latest | grep browser_download_url | cut -d '"' -f 4 | grep 'darwin_x64' | wget -i -
    tar xf gitleaks_*_darwin_x64.tar.gz
else
    echo -e "${RED}Unsupported OS and architecture combination.${NC}"
    exit 1
fi

chmod +x gitleaks

# Move gitleaks to /usr/local/bin/ to ensure it's in the PATH
sudo mv gitleaks /usr/local/bin/

echo -e "${GREEN}Gitleaks installed.${NC}"

# Create .gitleaks.toml file
echo -e "${GREEN}Creating .gitleaks.toml file...${NC}"
create_gitleaks_config

# Clean up
rm gitleaks_*.tar.gz gitleaks
git restore README.md

GITLEAKS_VERSION=$(gitleaks version)
if [[ $? -ne 0 ]]; then
    echo -e "${RED}${CROSS_MARK} Failed to install gitleaks.${NC}"
    exit 1
else
    echo -e "Gitleaks version: ${GREEN}${BOLD}${GITLEAKS_VERSION}!${NC}"
fi

# Define the URL of the repository where the pre-commit script is located
REPO_URL="https://raw.githubusercontent.com/ruslanlap/pre-commit-auto-script/main/pre-commit.sh"

# Define the path to the hooks directory in your Git repository
HOOKS_DIR=".git/hooks"

# Ensure that the hooks directory exists
mkdir -p "$HOOKS_DIR"

# Install the pre-commit hook
echo -e "${BLUE}Downloading pre-commit hook...${NC}"
curl -sSfL "$REPO_URL" -o "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-commit"

if [ -f "$HOOKS_DIR/pre-commit" ]; then
    echo -e "${GREEN}${CHECK_MARK} Pre-commit hook script installed successfully!${NC}"
else
    echo -e "${RED}${CROSS_MARK} An error occurred while installing the pre-commit hook script.${NC}"
fi

download_onoffscript() {
    # Download the gitleaks.sh file from the GitHub repository
    curl -sSfL "https://raw.githubusercontent.com/ruslanlap/pre-commit-auto-script/main/on-off-gitleaks.sh" -o on-off-gitleaks.sh
    chmod +x on-off-gitleaks.sh
}

download_onoffscript

# Remove LICENSE file if it exists
if [ -f LICENSE ]; then
    rm LICENSE
fi

# Run gitleaks to detect any existing secrets
echo -e "${BLUE}Running gitleaks to detect any existing secrets...${NC}"
gitleaks detect --source . --config .gitleaks.toml
if [[ $? -ne 0 ]]; then
    echo -e "${RED}${CROSS_MARK} Secrets detected. Please remove them before committing.${NC}"
    exit 1
else
    echo -e "${GREEN}${CHECK_MARK} No secrets detected.${NC}"
fi

