#!/usr/bin/env bash
# Sync dependency versions across all Cargo.toml files
# Ensures all toad-* dependencies match the current release version

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Source of truth: bin/toad/Cargo.toml
TOAD_VERSION=$(grep "^version =" bin/toad/Cargo.toml | head -n 1 | cut -d '"' -f 2)

if [ -z "$TOAD_VERSION" ]; then
    echo -e "${RED}❌ Error: Could not find version in bin/toad/Cargo.toml${NC}"
    exit 1
fi

echo -e "${BLUE}🔄 Syncing all dependencies to v${TOAD_VERSION}...${NC}\n"

# Function to update a single Cargo.toml file
update_cargo_toml() {
    local file="$1"
    local updated=false
    
    # Update package version
    if grep -q "^version = " "$file"; then
        sed -i '' "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$TOAD_VERSION\"/" "$file"
        updated=true
    fi
    
    # Update toad-* dependency versions
    # Pattern: toad-core = { version = "X.X.X", path = ...
    if grep -q "toad-" "$file"; then
        sed -i '' -E "s/(toad-[a-z]+[[:space:]]*=[[:space:]]*\{[[:space:]]*version[[:space:]]*=[[:space:]]*\")[0-9]+\.[0-9]+\.[0-9]+(\")/\1${TOAD_VERSION}\2/g" "$file"
        updated=true
    fi
    
    if [ "$updated" = true ]; then
        echo -e "${GREEN}✓${NC} Updated: $file"
    fi
}

# Update all Cargo.toml files in crates/
echo -e "${YELLOW}Updating crate dependencies...${NC}"
for toml in crates/*/Cargo.toml; do
    if [ -f "$toml" ]; then
        update_cargo_toml "$toml"
    fi
done

# Update all Cargo.toml files in bin/
echo -e "\n${YELLOW}Updating binary dependencies...${NC}"
for toml in bin/*/Cargo.toml; do
    if [ -f "$toml" ]; then
        update_cargo_toml "$toml"
    fi
done

# Verify all dependencies are synced
echo -e "\n${BLUE}Verifying dependency sync...${NC}"
MISMATCHES=0

for toml in crates/*/Cargo.toml bin/*/Cargo.toml; do
    if [ -f "$toml" ]; then
        # Check for any toad-* dependencies with wrong version
        if grep -E "toad-[a-z]+.*version.*=.*\"[0-9]+\.[0-9]+\.[0-9]+\"" "$toml" | grep -v "\"$TOAD_VERSION\"" > /dev/null 2>&1; then
            echo -e "${RED}✗${NC} Version mismatch in: $toml"
            grep -E "toad-[a-z]+.*version.*=.*\"[0-9]+\.[0-9]+\.[0-9]+\"" "$toml" | grep -v "\"$TOAD_VERSION\"" | sed 's/^/  /'
            MISMATCHES=$((MISMATCHES + 1))
        fi
    fi
done

if [ $MISMATCHES -eq 0 ]; then
    echo -e "${GREEN}✓${NC} All dependencies synced to v${TOAD_VERSION}"
    echo -e "\n${GREEN}✅ Dependency sync complete!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Found $MISMATCHES file(s) with version mismatches${NC}"
    echo -e "${YELLOW}Run this script again or manually fix the versions${NC}"
    exit 1
fi
