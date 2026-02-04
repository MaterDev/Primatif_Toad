# Primatif_Toad

**Primatif_Toad** is a Mac-agnostic **Toad Control** for managing local development environments.

## Quick Start
1. **Install:** `just install`
2. **Use:** `toad create my-project`

---

## 🛠️ Tooling Roles

### `toad` (The Installed Tool)
Use this command to **manage your projects**. It is the installed binary.
- `toad create <name>`: Scaffold a new project.
- `toad reveal <query>`: Find existing projects.
- `toad list`: See all commands.

### `just` (The Developer Runner)
Use this command to **develop Primatif_Toad itself**.
- `just install`: Compiles and installs `toad` to your system.
- `just test`: Runs the Rust test suite.
- `just cli`: Runs the *local* (uninstalled) version of the CLI code.

---

## Architecture
- **Workspace:** Rust-based monorepo.
- **`bin/toad`**: Unified CLI interface.
- **`crates/`**: Modular capabilities (`scaffold`, `discovery`).
- **`projects/`**: Directory for managed projects (ignored by Git).

## Features

### 1. Project Scaffolding
Standardized project creation within the `projects/` directory.

**Requirements:**
- `just`
- `git`
- `vscode` / `windsurf` (Optional)

**Usage:**
```bash
toad create <project-name>
```

### 2. Project Discovery (Reveal)
Search for projects by name.

**Usage:**
```bash
toad reveal cli
```

## Development

```bash
# Run tests
just test

# Run the local code without installing
just cli list
```