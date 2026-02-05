#!/bin/bash
# Sandbox Archival Logic

archive_sandbox() {
    local source_dir=$1
    local tar_name=$2

    echo "🌊 Archiving to $tar_name..."
    tar -czf "$tar_name" "$source_dir"
    echo "✅ Archive ready: $tar_name"
}
