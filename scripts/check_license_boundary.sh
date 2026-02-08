#!/bin/bash
# scripts/check_license_boundary.sh
# Ensures MIT crates never depend on BUSL-1.1 crates.

set -e

# Define tiers
MIT_CRATES=("toad-core" "scaffold")
BUSL_CRATES=("discovery" "toad-git" "toad-manifest" "toad-ops")

VIOLATIONS=0

echo "🔍 Checking License Boundaries..."

for mit_crate in "${MIT_CRATES[@]}"; do
    toml_path="crates/$mit_crate/Cargo.toml"
    if [ ! -f "$toml_path" ]; then
        continue
    fi

    # Read the dependencies section
    # Use grep to find BUSL crates in the dependencies
    for busl_crate in "${BUSL_CRATES[@]}"; do
        if grep -qE "^$busl_crate[[:space:]]*=" "$toml_path"; then
            echo "❌ LICENSE BOUNDARY VIOLATION: $mit_crate (MIT) depends on $busl_crate (BUSL-1.1)"
            echo "   Location: $toml_path"
            VIOLATIONS=$((VIOLATIONS + 1))
        fi
    done
done

if [ "$VIOLATIONS" -gt 0 ]; then
    echo ""
    echo "⚠️  MIT crates must never depend on BUSL-1.1 crates."
    echo "   See: docs/releases/v1.0.2/evolution.md § Dependency Direction Rule"
    echo "   Fix: Remove the dependency or move the crate to BUSL-1.1."
    exit 1
fi

echo "✅ License boundaries are secure."
exit 0
