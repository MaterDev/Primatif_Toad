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

# Run full QA suite (Sync -> Format -> Lint -> Test -> Build)
qa: sync-version fmt lint test build
    @echo "\n✅ QA Complete: Codebase is clean, tested, and builds."

# Sync README version with Cargo.toml
sync-version:
    ./scripts/sync_version.sh

# Setup Git hooks
setup-hooks:
    @mkdir -p .git/hooks
    @ln -sf ../../scripts/git-hooks/pre-commit .git/hooks/pre-commit
    @ln -sf ../../scripts/git-hooks/pre-push .git/hooks/pre-push
    @echo "✅ Git hooks installed."

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

# Run code coverage (requires cargo-tarpaulin)
coverage:
    cargo tarpaulin --workspace --ignore-tests --exclude-files "projects/*"

# Install development tools (dprint, tarpaulin)
setup-tools:
    cargo install dprint
    cargo install cargo-tarpaulin

# Build the system
build:
    cargo build

# Dev Shortcut: Create a project using the local dev build
dev-create name:
    just cli create {{name}}
