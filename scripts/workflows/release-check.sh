#!/bin/bash
set -e

echo "🐸 Pre-release verification..."

echo "Running tests..."
cargo test --workspace --quiet

echo "Checking formatting..."
cargo fmt --all -- --check

echo "Checking dprint formatting..."
dprint check

echo "Running clippy..."
cargo clippy --workspace -- -D warnings

echo "Verifying license boundaries..."
./scripts/check_license_boundary.sh

echo "Checking submodule alignment..."
toad ggit status | grep -q "✅ Clean" || (echo "❌ Submodules not aligned" && exit 1)

echo "✅ Release check passed"
