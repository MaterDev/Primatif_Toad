#!/bin/bash
set -e

# Primatif_Toad Installer
# Installs the Toad Control CLI to the local Cargo bin directory.

echo "Installing Toad Control..."

# 0. Setup Git hooks and sync version
just setup-hooks
just sync-version

# 1. Install the binary
cargo install --path bin/toad --force

# 2. Verify installation
if ! command -v toad &> /dev/null; then
    echo "Error: 'toad' command not found after install."
    echo "Ensure that ~/.cargo/bin is in your PATH."
    exit 1
fi

# 3. Print Welcome Banner
echo -e "\nInstallation Complete!"
toad version
