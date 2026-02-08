#!/bin/bash
# scripts/dev_setup.sh
# Fresh clone setup for Toad development.
# Registers the current directory as the 'toad-dev' context.

set -e

echo "🐸 Primatif Toad: Developer Environment Setup"

# 1. Initialize submodules
echo "📦 Initializing submodules..."
git submodule update --init --recursive

# 2. Build and verify
echo "🔨 Building the system..."
cargo build
echo "🧪 Running tests..."
cargo test --workspace

# 3. Register as a project context (if toad is installed)
if command -v toad &> /dev/null; then
    echo "🗺️  Registering 'toad-dev' context..."
    toad project register toad-dev "$(pwd)" 
        --description "Toad development workspace"
    toad project switch toad-dev
    echo "✅ Registered and switched to 'toad-dev' context."
else
    # Try using the local build if not in path
    echo "🗺️  Registering 'toad-dev' context via local build..."
    cargo run -p toad -- project register toad-dev "$(pwd)" 
        --description "Toad development workspace"
    cargo run -p toad -- project switch toad-dev
    echo "✅ Registered and switched to 'toad-dev' context via local build."
fi

echo ""
echo "🚀 Toad development environment is ready!"
