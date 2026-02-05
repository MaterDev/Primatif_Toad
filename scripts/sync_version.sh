#!/bin/bash
# Sync README version badge with Cargo.toml version

# Source of truth: bin/toad/Cargo.toml
VERSION=$(grep "^version =" bin/toad/Cargo.toml | head -n 1 | cut -d '"' -f 2)

if [ -z "$VERSION" ]; then
    echo "❌ Error: Could not find version in bin/toad/Cargo.toml"
    exit 1
fi

echo "🏷️  Syncing README version to v$VERSION..."

# Update the badge URL in README.md
# Format: [![Version: vX.Y.Z](https://img.shields.io/badge/version-vX.Y.Z-green.svg)](Cargo.toml)
sed -i '' "s/version-v[0-9]*\.[0-9]*\.[0-9]*/version-v$VERSION/g" README.md
sed -i '' "s/Version: v[0-9]*\.[0-9]*\.[0-9]*/Version: v$VERSION/g" README.md

echo "✅ README.md updated."
