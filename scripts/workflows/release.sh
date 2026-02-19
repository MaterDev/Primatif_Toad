#!/usr/bin/env bash
# Release automation script for Toad
# Ensures consistent release documentation and version management

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the new version from argument
NEW_VERSION="${1:-}"

if [[ -z "$NEW_VERSION" ]]; then
    echo -e "${RED}Error: Version argument required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 1.2.0"
    exit 1
fi

# Validate version format (semantic versioning)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must be in format: MAJOR.MINOR.PATCH (e.g., 1.2.0)"
    exit 1
fi

echo -e "${BLUE}=== Toad Release Process v${NEW_VERSION} ===${NC}\n"

# Step 1: Check for uncommitted changes
echo -e "${YELLOW}[1/7]${NC} Checking for uncommitted changes..."
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: You have uncommitted changes${NC}"
    echo "Please commit or stash your changes before releasing"
    exit 1
fi
echo -e "${GREEN}✓${NC} Working directory clean\n"

# Step 2: Create release directory
RELEASE_DIR="docs/releases/v${NEW_VERSION}"
echo -e "${YELLOW}[2/7]${NC} Creating release directory: ${RELEASE_DIR}"
mkdir -p "$RELEASE_DIR"
echo -e "${GREEN}✓${NC} Release directory created\n"

# Step 3: Check if changelog template exists
CHANGELOG_FILE="${RELEASE_DIR}/changelog.md"
echo -e "${YELLOW}[3/7]${NC} Checking changelog..."
if [[ ! -f "$CHANGELOG_FILE" ]]; then
    echo -e "${YELLOW}⚠${NC}  Changelog not found. Please create ${CHANGELOG_FILE}"
    echo "Template structure:"
    echo "  # v${NEW_VERSION} Changelog"
    echo "  **Release Date:** $(date +%Y-%m-%d)"
    echo "  **Codename:** —"
    echo ""
    echo "  ## Overview"
    echo "  [Brief description]"
    echo ""
    echo "  ## Added / Changed / Fixed / Documentation"
    echo ""
    exit 1
fi
echo -e "${GREEN}✓${NC} Changelog exists\n"

# Step 4: Archive previous releases (keep only latest)
echo -e "${YELLOW}[4/7]${NC} Archiving previous releases..."
ARCHIVE_DIR="docs/releases/archive"
mkdir -p "$ARCHIVE_DIR"

# Find all version directories except the new one
for dir in docs/releases/v*; do
    if [[ -d "$dir" && "$dir" != "$RELEASE_DIR" && "$dir" != "$ARCHIVE_DIR" ]]; then
        VERSION_NAME=$(basename "$dir")
        if [[ ! -d "${ARCHIVE_DIR}/${VERSION_NAME}" ]]; then
            echo "  Archiving ${VERSION_NAME}..."
            mv "$dir" "$ARCHIVE_DIR/"
        fi
    fi
done
echo -e "${GREEN}✓${NC} Previous releases archived\n"

# Step 5: Update CHANGELOG.md
echo -e "${YELLOW}[5/7]${NC} Updating CHANGELOG.md..."
# This is a placeholder - actual implementation would parse and update the table
echo -e "${YELLOW}⚠${NC}  Please manually verify CHANGELOG.md includes:"
echo "  | **v${NEW_VERSION}** | $(date +%Y-%m-%d) | — | [changelog](docs/releases/v${NEW_VERSION}/changelog.md) |"
echo ""

# Step 6: Update version in Cargo.toml files
echo -e "${YELLOW}[6/7]${NC} Updating version in Cargo.toml files..."
# Find current version
CURRENT_VERSION=$(grep -m 1 'version = ' bin/toad/Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
echo "  Current version: ${CURRENT_VERSION}"
echo "  New version: ${NEW_VERSION}"

# Update all Cargo.toml files
find . -name "Cargo.toml" -not -path "./target/*" -exec sed -i '' "s/version = \"${CURRENT_VERSION}\"/version = \"${NEW_VERSION}\"/g" {} \;
echo -e "${GREEN}✓${NC} Version updated in Cargo.toml files\n"

# Step 7: Summary
echo -e "${BLUE}=== Release Preparation Complete ===${NC}"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff"
echo "  2. Update Cargo.lock: cargo update"
echo "  3. Build and test: cargo build --release"
echo "  4. Commit changes: git add -A && git commit -m 'chore: Bump version to ${NEW_VERSION}'"
echo "  5. Push to submodules and hub"
echo "  6. Create git tag: git tag v${NEW_VERSION}"
echo "  7. Push tag: git push origin v${NEW_VERSION}"
echo ""
echo -e "${GREEN}Release ${NEW_VERSION} is ready!${NC}"
