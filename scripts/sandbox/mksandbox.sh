#!/bin/bash
# Repeatable Sandbox Generator for Toad Testing
# Main Entry Point

# Source modular parts
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/parts/project_gen.sh"
source "$SCRIPT_DIR/parts/archive_gen.sh"

# Default values
COUNT=10
OUT_DIR="test_sandbox"
STACKS="rust,node,go,generic"
DEPTH=1
DO_TAR=false

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -c|--count) COUNT="$2"; shift ;;
        -o|--output) OUT_DIR="$2"; shift ;;
        -s|--stacks) STACKS="$2"; shift ;;
        -d|--depth) DEPTH="$2"; shift ;;
        -t|--tar) DO_TAR=true ;;
        -h|--help)
            echo "Usage: ./mksandbox.sh [options]"
            echo "Options:"
            echo "  -c, --count N      Total number of projects (default: 10)"
            echo "  -o, --output DIR   Output directory (default: test_sandbox)"
            echo "  -s, --stacks LIST  Comma-separated tech stacks (rust,node,go,generic)"
            echo "  -d, --depth N      Nesting depth of project files (default: 1)"
            echo "  -t, --tar          Archive the output to a .tar.gz"
            exit 0
            ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo "🐸 Generating sandbox with $COUNT projects in '$OUT_DIR'..."
echo "🛠️ Configuration: Stacks=[$STACKS], Depth=$DEPTH"

mkdir -p "$OUT_DIR/projects"

# Split stacks into array
IFS=',' read -ra STACK_ARRAY <<< "$STACKS"
NUM_STACKS=${#STACK_ARRAY[@]}

for i in $(seq 1 $COUNT); do
    PROJ_NAME="mock-proj-$i"
    PROJ_DIR="$OUT_DIR/projects/$PROJ_NAME"
    
    # Pick stack in round-robin fashion
    STACK_INDEX=$(( (i-1) % NUM_STACKS ))
    CURRENT_STACK=${STACK_ARRAY[$STACK_INDEX]}
    
    generate_project "$PROJ_NAME" "$PROJ_DIR" "$CURRENT_STACK" "$DEPTH"
done

if [[ "$DO_TAR" == true ]]; then
    archive_sandbox "$OUT_DIR" "toad_sandbox_${COUNT}.tar.gz"
fi

echo "✅ Sandbox generation complete."