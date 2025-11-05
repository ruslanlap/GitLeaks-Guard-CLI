#!/bin/bash

# Color codes for terminal output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
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

# Function to disable Gitleaks
function disable_gitleaks() {
    echo -e "${RED}Disabling Gitleaks...${NC}"
    git config core.hooksPath no-hooks
}

# Function to enable Gitleaks
function enable_gitleaks() {
    echo -e "${GREEN}Enabling Gitleaks...${NC}"
    git config --unset core.hooksPath
}

# Main script execution
if [[ "$(is_gitleaks_enabled)" == "true" ]]; then
    run_gitleaks
else
    echo -e "${BLUE}Gitleaks is not enabled in the Git config.${NC}"
fi

# Make the 'on-off-gitleaks.sh' script executable
chmod +x on-off-gitleaks.sh

