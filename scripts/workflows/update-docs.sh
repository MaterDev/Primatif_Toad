#!/bin/bash
set -e

echo "🐸 Updating documentation..."

just docs

toad skill sync

toad manifest

echo "✅ Documentation updated"
