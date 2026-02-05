#!/bin/bash
# Repeatable Sandbox Generator for Toad Testing
# Usage: ./scripts/mksandbox.sh [num_projects] [output_dir] [--tar]

COUNT=${1:-100}
OUT_DIR=${2:-"test_sandbox"}
TAR_FILE="toad_sandbox_${COUNT}.tar.gz"

echo "🐸 Generating sandbox with $COUNT projects in '$OUT_DIR'..."

mkdir -p "$OUT_DIR/projects"

for i in $(seq 1 $COUNT); do
    PROJ_NAME="mock-proj-$i"
    PROJ_DIR="$OUT_DIR/projects/$PROJ_NAME"
    mkdir -p "$PROJ_DIR"
    
    # Standard boilerplate
    echo "# $PROJ_NAME" > "$PROJ_DIR/README.md"
    
    # Randomly assign tech stack metadata
    if (( i % 3 == 0 )); then
        touch "$PROJ_DIR/Cargo.toml"
    elif (( i % 3 == 1 )); then
        touch "$PROJ_DIR/package.json"
    else
        touch "$PROJ_DIR/go.mod"
    fi
    
    # Initialize Git occasionally
    if (( i % 5 == 0 )); then
        git init -q "$PROJ_DIR"
    fi
done

if [[ "$3" == "--tar" ]]; then
    echo "🌊 Archiving to $TAR_FILE..."
    tar -czf "$TAR_FILE" "$OUT_DIR"
    echo "✅ Archive ready: $TAR_FILE"
fi

echo "✅ Sandbox generation complete."
