#!/bin/bash
# High-Fidelity Sandbox Project Generator
# Part of the Toad Control Plane Test Suite

generate_project() {
    local name=$1
    local target_dir=$2
    local stack=$3
    local depth=$4

    mkdir -p "$target_dir"
    
    # Create nested structure if depth > 1
    if (( depth > 1 )); then
        local sub="subdir"
        for (( d=2; d<=depth; d++ )); do
            mkdir -p "$target_dir/$sub"
            target_dir="$target_dir/$sub"
            sub="sub-$d"
        done
    fi

    # Standard boilerplate
    echo "# $name" > "$target_dir/README.md"
    echo "This is a mock project for $stack testing." >> "$target_dir/README.md"

    case $stack in
        "rust")
            echo "[package]" > "$target_dir/Cargo.toml"
            echo "name = \"$name\"" >> "$target_dir/Cargo.toml"
            echo "version = \"0.1.0\"" >> "$target_dir/Cargo.toml"
            ;;
        "node")
            echo "{\"name\": \"$name\", \"version\": \"1.0.0\"}" > "$target_dir/package.json"
            ;;
        "go")
            echo "module $name" > "$target_dir/go.mod"
            ;;
        *)
            # Generic
            touch "$target_dir/.generic"
            ;;
    esac

    # Occasionally add git
    if (( RANDOM % 5 == 0 )); then
        git init -q "$target_dir"
    fi
}
