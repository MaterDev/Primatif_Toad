#!/bin/bash
# Sync versions across all crates and README

# Source of truth: bin/toad/Cargo.toml
VERSION=$(grep "^version =" bin/toad/Cargo.toml | head -n 1 | cut -d '"' -f 2)

if [ -z "$VERSION" ]; then
    echo "❌ Error: Could not find version in bin/toad/Cargo.toml"
    exit 1
fi

echo "🏷️  Syncing ecosystem to v$VERSION..."

# 1. Update individual crate Cargo.toml files
for toml in crates/*/Cargo.toml; do
    if [ -f "$toml" ]; then
        # Update [package] version
        sed -i '' "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$VERSION\"/" "$toml"
        # Update any dependency starting with 'toad-' or matching 'discovery'/'scaffold'
        sed -i '' "s/\(toad-[a-z]*\|discovery\|scaffold\)[[:space:]]*=[[:space:]]*{[[:space:]]*version[[:space:]]*=[[:space:]]*\"[0-9]*\.[0-9]*\.[0-9]*\"/\1 = { version = \"$VERSION\"/g" "$toml"
    fi
done

# 2. Update Hub README version badges
sed -i '' "s/version-v[0-9]*\.[0-9]*\.[0-9]*/version-v$VERSION/g" README.md
sed -i '' "s/Version: v[0-9]*\.[0-9]*\.[0-9]*/Version: v$VERSION/g" README.md

echo "✅ Sync complete."
