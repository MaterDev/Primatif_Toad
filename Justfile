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

# --- Quality Assurance (QA) ---

# Run full QA suite (Format -> Lint -> Test -> Build)
qa: fmt lint test build
    @echo "\n✅ QA Complete: Codebase is clean, tested, and builds."

# Run all tests
test:
    cargo test --workspace

# Check everything (CI Gate)
check: lint test
    cargo fmt --all -- --check
    dprint check

# Auto-fix everything possible
fix:
    cargo clippy --workspace --fix --allow-dirty --allow-staged
    just fmt
    @command -v markdownlint > /dev/null || (echo "❌ ERROR: markdownlint-cli is NOT installed. Install with 'npm install -g markdownlint-cli'" && exit 1)
    markdownlint "**/*.md" --fix

# Format code and docs
fmt: fmt-rust fmt-misc

# Format Rust code
fmt-rust:
    cargo fmt --all

# Format Markdown, TOML, JSON (requires dprint)
fmt-misc:
    dprint fmt

# Lint Rust code (Clippy) and Markdown
lint:
    cargo clippy --workspace -- -D warnings
    @command -v markdownlint > /dev/null || (echo "❌ ERROR: markdownlint-cli is NOT installed. Install with 'npm install -g markdownlint-cli'" && exit 1)
    markdownlint "**/*.md"

# Install development tools (dprint)
setup-tools:
    cargo install dprint

# Build the system
build:
    cargo build

# Dev Shortcut: Create a project using the local dev build
dev-create name:
    just cli create {{name}}
