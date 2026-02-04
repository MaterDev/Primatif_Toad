# Primatif_Toad

**Primatif_Toad** is a **DevOps Overlay and Developer CLI (Toad)** designed to
manage a modular ecosystem of independent projects. It acts as a clean
administrative layer for orchestrating workspaces, discovering patterns, and
unified project management.

## Core Roles

- **Workspace Orchestration:** Uses a "whitelist-only" strategy to manage the
  `projects/` directory as an external collection of independent repositories.
- **Discovery & Scaffolding:** Automates project pattern detection and component
  generation via specialized Rust crates (`discovery`, `scaffold`).
- **Unified Tooling:** Centralizes quality gates and management scripts for a
  diverse set of local codebases without impacting their individual Git
  histories.

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
- `just cli`: Runs the _local_ (uninstalled) version of the CLI code.

---

## Architecture

- **Workspace:** Rust-based monorepo.
- **`bin/toad`**: Unified CLI interface (The Orchestrator).
- **`crates/`**: Modular capabilities:
  - `toad-core`: Shared data models and workspace context.
  - `toad-discovery`: Project scanning and tech stack detection strategies.
  - `toad-manifest`: Markdown reporting and context shadow generation.
  - `scaffold`: Project generation logic.
- **`projects/`**: Directory for managed projects (ignored by Git).
- **`shadows/`**: Directory for AI context metadata (ignored by Git).

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
