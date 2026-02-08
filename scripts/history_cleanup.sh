#!/bin/bash
# scripts/history_cleanup.sh
# One-time post-split cleanup to remove extracted crate source from Hub history.
# USE WITH CAUTION: This operation is irreversible.

set -e

# Verify git-filter-repo is installed
if ! command -v git-filter-repo &> /dev/null; then
    echo "❌ ERROR: git-filter-repo is not installed. Run: brew install git-filter-repo"
    exit 1
fi

echo "⚠️  WARNING: This script will rewrite the Git history of the current repository."
echo "   It removes the actual source code of the extracted crates while preserving"
echo "   the submodule references. This is a one-time migration step."
echo ""

if [ "$1" != "--force" ]; then
    echo "Dry-run mode. Run with --force to execute."
    echo "Paths to be removed (inverted):"
    echo "  - crates/toad-core/src"
    echo "  - crates/toad-git/src"
    echo "  - crates/toad-manifest/src"
    echo "  - crates/toad-ops/src"
    echo "  - crates/discovery/src"
    echo "  - crates/scaffold/src"
    exit 0
fi

read -p "Are you absolutely sure? This cannot be undone. [y/N]: " confirm
if [[ $confirm != [yY] ]]; then
    echo "Aborted."
    exit 1
fi

echo "🧹 Cleaning Hub history..."

# We invert paths to keep everything EXCEPT the actual source contents of the submodules.
# The submodule pointers themselves (.gitmodules and the folder entries) are preserved.
git filter-repo 
    --path crates/toad-core/src 
    --path crates/toad-git/src 
    --path crates/toad-manifest/src 
    --path crates/toad-ops/src 
    --path crates/discovery/src 
    --path crates/scaffold/src 
    --invert-paths

echo "✅ History cleanup complete."
echo "💡 You must force-push to update the remote: git push origin main --force"
