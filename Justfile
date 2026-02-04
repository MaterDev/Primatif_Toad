# Justfile for Primatif_Toad

# Run the Toad CLI
cli *args:
    cargo run -p toad -- {{args}}

# Create a new project
create name:
    just cli create {{name}}

# Build the system
build:
    cargo build

# Run tests
test:
    cargo test --workspace

# Install the Toad CLI to ~/.cargo/bin
install:
    cargo install --path bin/toad

# List available Toad commands
list:
    just cli list