# Justfile for Primatif_Toad
#
# NOTE: 
# 'just' commands are for DEVELOPING the Toad system itself.
# 'toad' commands are for USING the installed tool to manage projects.

# Run the local version of the CLI (Development)
cli *args:
    cargo run -p toad -- {{args}}

# --- Bootstrap & Environment ---

# Fully initialize the development environment (Submodules + Tools + Hooks + Git Config)
setup: init setup-tools setup-hooks setup-git-config build
    @echo "\n🐸 Welcome to the Toad Pond! Your development environment is ready."

# Initialize and update all submodules
init:
    git submodule update --init --recursive
    @echo "✅ Submodules initialized."

# Configure specialized Git merge strategies
setup-git-config:
    @git config merge.ours.driver true
    @echo "✅ Git merge strategies configured."

# Install the Toad CLI to your system (Production)
install:
    ./scripts/install_toad.sh

# --- Quality Assurance (QA) ---

# Run full QA suite (Sync -> Docs -> Skills -> Check Licenses -> Format -> Lint -> Test -> Build)
qa: sync-version docs sync-skills check-licenses fmt lint test build
    @echo "\n✅ QA Complete: Codebase is clean, tested, and builds."

# Sync README version with Cargo.toml
sync-version:
    ./scripts/sync_version.sh

# Generate CLI documentation
docs:
    cargo run -p toad -- docs

# Synchronize AI agent skills
sync-skills:
    cargo run -p toad -- skill sync

# Verify MIT/BUSL-1.1 license boundaries
check-licenses:
    ./scripts/check_license_boundary.sh

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
    npm run fix:md

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
    cargo clippy --workspace --all-targets -- -D warnings
    npm run lint:md

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