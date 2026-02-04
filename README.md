# Primatif_Toad

**Primatif_Toad** is a Mac-agnostic **Control Plane** for managing local development environments.

## Architecture
- **Workspace:** Rust-based monorepo.
- **`bin/toad`**: Unified CLI interface (Command: `toad`).
- **`crates/`**: Modular, testable capabilities.
- **`projects/`**: Directory for managed projects (ignored by Git).
- **`Justfile`**: Task runner for developer operations.

## Features

### 1. Project Scaffolding
Standardized project creation within the `projects/` directory.

**Requirements:**
- `just`
- `git`
- `vscode` (`code` command in PATH) - Optional
- `windsurf` (`windsurf` command in PATH) - Optional

**Usage:**
```bash
# Create a new project
just create <project-name>

# Simulate creation (Dry Run)
toad create <project-name> --dry-run
```
This will:
- Check for existing directory.
- Create structure (`docs/`, `README.md`, `.gitignore`).
- Initialize Git.
- Offer to open the project in VS Code or Windsurf.

### 2. Project Discovery (Reveal)
Search for projects by name.

**Usage:**
```bash
# Find projects matching "cli"
toad reveal cli
```

## Development

```bash
# Build the system
just build

# Run tests
just test

# Install the CLI locally
just install
```
