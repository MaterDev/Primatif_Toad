# Justfile for Primatif_Toad
#
# NOTE: 
# 'just' commands are for DEVELOPING the Toad system itself.
# 'toad' commands are for USING the installed tool to manage projects.

# Run the local version of the CLI (Development)
cli *args:
    cargo run -p toad -- {{args}}

# Install the Toad CLI to your system (Production)
install:
    ./scripts/install_toad.sh

# Run tests
test:
    cargo test --workspace

# Build the system
build:
    cargo build

# Dev Shortcut: Create a project using the local dev build
dev-create name:
    just cli create {{name}}