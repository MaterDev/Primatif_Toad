<div align="center">

[![Toad Project Lifecycle](https://github.com/Primatif/Primatif_Toad/raw/main/assets/video.gif)](https://github.com/Primatif/Primatif_Toad/raw/main/assets/video.mp4)

### Toad Control CLI

The modular meta-engineering platform.

[![Version: v1.1.0](https://img.shields.io/badge/version-v1.1.0-green.svg)](Cargo.toml)
[![Coverage: >80%](https://img.shields.io/badge/coverage-%3E80%25-brightgreen.svg)](Justfile)

---
</div>

Toad is a High-Performance Local-Ops Platform designed to manage a vast ecosystem of independent projects. It provides a multi-threaded administrative layer for orchestrating workspaces, performing data-driven analytics, and enforcing safety guardrails across dozens of repositories.

> [!CAUTION]
> **Toad is a powerful meta-engineering tool.** Commands execute across multiple independent repositories simultaneously. Misuse can lead to significant data loss. Always verify your targets and commands before execution.

## What is Local-Ops?

Local-Ops is the discipline of managing local development environments at scale.

By centralizing observability and orchestration, Local-Ops solves the critical problems of maintenance drift and resource bloat. It provides a "bird's-eye view" through visual health analytics and parallel command dispatching, allowing a single developer to perform ecosystem-wide tasks—like security patching, artifact cleaning, or AI context synchronization—in seconds rather than hours.

Ultimately, Local-Ops bridges the gap between raw code and developer productivity, ensuring that a massive portfolio of work remains a live asset rather than an unmanageable burden of technical debt.

## 🐸 Quick Start

```bash
# 1. Clone the Hub
git clone https://github.com/Primatif/Primatif_Toad.git
cd Primatif_Toad

# 2. Fully automate your setup (Initializes submodules, installs tools, and builds)
just setup

# 3. Anchor your system
toad home .

# 4. Analyze your ecosystem
toad status
```

## What's New in v1.0.2

- **Modular Architecture:** Codebase organized into specialized internal crates
  across separate git repositories, managed as submodules.
- **Multi-Repo Git Orchestration (`toad ggit`):** First-class git operations
  across all repos — status, commit, push, pull, sync, branch listing, and
  submodule alignment.
- **Named Project Contexts (`toad project`):** Register multiple workspace roots
  and switch between them instantly. All commands resolve against the active
  context.
- **AI Skill Distribution (`toad skill sync`):** Generate and distribute
  architectural blueprints, CLI references, and manifests to any AI vendor
  (Windsurf, Gemini, Cursor, Claude, Copilot, and more).
- **Custom Workflows (`toad cw`):** Register and execute custom scripts as
  first-class Toad commands.
- **Submodule Awareness:** Automatic discovery and status reporting for git
  submodules within any managed project.

See the [CHANGELOG](CHANGELOG.md) for the full release history.

## Core Commands

### Analytics & Health

- **`toad status`** — Git health and activity tiers across the ecosystem (with
  submodule status).
- **`toad stats`** — Visual disk usage heatmap and artifact bloat analysis.
- **`toad clean`** — Reclaim disk space by removing detected build artifacts.
- **`toad reveal <query>`** — Search for projects by name or `#tag`.

### Multi-Repo Git Orchestration

- **`toad ggit status`** — Consolidated git status across all repositories.
- **`toad ggit commit -m "msg"`** — Commit changes across repositories.
- **`toad ggit commit -m "msg" --cascade`** — Commit submodules first, then
  cascade to the Hub root.
- **`toad ggit push`** — Push all repositories to their remotes.
- **`toad ggit pull`** — Pull latest changes across all repositories.
- **`toad ggit sync`** — Synchronize submodule refs with pre-flight safety
  checks.
- **`toad ggit branches`** — List all branches across repositories.
- **`toad ggit align`** — Force-align submodules to Hub root expectations.

### Orchestration (Bulk Ops)

> [!WARNING] **High-Risk Operations:** Batch execution via `toad do` is
> potentially destructive. We strongly recommend using the `--dry-run` flag to
> preview changes before running them for real.

- **`toad do "<command>"`** — Execute shell commands across matching projects in
  parallel.
- **`toad do "git pull" --tag stable`** — Batch update only your stable tools.
- **`toad do "rm -rf target" --dry-run`** — Safely preview destructive
  maintenance.

### Project Contexts

- **`toad project register <name> <path>`** — Register a new workspace root.
- **`toad project switch <name>`** — Switch the active context.
- **`toad project current`** — Show the active context.
- **`toad project list`** — List all registered contexts.

### AI Skills & Context

- **`toad skill sync`** — Generate and distribute architectural blueprints, CLI
  references, and manifests to registered AI vendors.
- **`toad skill list`** — List distributed skills and registered vendors.
- **`toad-mcp`** — Model Context Protocol server for live AI context
  orchestration.

### Custom Workflows

- **`toad cw register <name> <script>`** — Register a custom workflow script.
- **`toad cw run <name>`** — Execute a registered workflow.
- **`toad cw list`** — List all registered workflows.

### Taxonomy & Stack Support

- **`toad strategy list`** — List all installed stack support plugins.
- **`toad strategy add`** — Interactively create a new stack support plugin.
- **`toad tag <project> <tag>`** — Assign custom metadata.
---

## 🌿 Stack Support Plugins

Toad uses a data-driven "Strategy Engine" to identify projects. You can extend
it easily:

```bash
# Add support for Elixir projects
toad strategy add Elixir --match "mix.exs" --clean "deps,_build" --tag "#elixir"
```

This creates a TOML manifest in `~/.toad/strategies/custom/elixir.toml`. Toad
will now identify Elixir projects, auto-tag them, and know exactly which folders
to clean.

See the [Stack Support Plugins Guide](docs/guides/PLUGINS.md) for more details.

### Workspace Anchor

> [!IMPORTANT] **Context Steering:** The `toad home` command updates the global
> system pointer. All subsequent CLI calls will target the projects and metadata
> in the newly anchored directory.

- `toad home .`: Anchor the current directory as your system-wide Toad home.
- `toad home [path]`: View or set the global workspace pointer manually.

---

## 🪵 Architecture

Toad is built as a modular Rust workspace. Each crate is its own git repository,
managed as a submodule:

```text
Primatif_Toad/                          (Hub)
├── bin/toad/                           CLI binary
├── crates/
│   ├── toad-core/                      Data models & config
│   ├── toad-scaffold/                  Project scaffolding
│   ├── toad-discovery/                 Ecosystem scanning
│   ├── toad-git/                       Git orchestration
│   ├── toad-manifest/                  Context generation
│   └── toad-ops/                       Batch ops & safety
├── docs/
└── scripts/
```

### Dependency Graph

```text
bin/toad
├── toad-core       ← shared data models
├── toad-scaffold   ← project creation
├── toad-discovery  ← depends on: toad-core, toad-git
├── toad-git        ← depends on: toad-core
├── toad-manifest   ← depends on: toad-core
└── toad-ops        ← depends on: toad-core
```

The system is designed for high separation of concerns. MIT types flow downward;
internal logic flows upward into the primary binary.

---

## Development

### Prerequisites

- **Rust & Cargo:** (Latest stable)
- **Just:** [just command runner](https://github.com/casey/just)
- **Dprint:** `cargo install dprint` (Formatted code and docs)
- **Tarpaulin:** `cargo install cargo-tarpaulin` (Coverage reporting)

### Clone & Build

```bash
git clone --recurse-submodules https://github.com/Primatif/Primatif_Toad.git
cd Primatif_Toad
cargo build
```

If you already cloned without `--recurse-submodules`:

```bash
git submodule update --init --recursive
```

### Quality Gates

We maintain a strict **>80% code coverage** mandate. Before every commit, run:

```bash
just qa
```

This executes version sync, docs generation, license boundary checks,
formatting, clippy, unit tests, and a full build.

---

## 🛠️ Contributor Guide: Multi-Repo Workflow

Toad is a distributed ecosystem. To contribute, you must understand how we
manage submodules.

### 1. The Hub & The Crates

- **The Hub (`Primatif_Toad`):** This repository. It manages the CLI binary and
  links all submodules.
- **The Crates (`crates/*`):** Specialized repositories. Each has its own
  lifecycle and license.

### 2. Managing Changes

**NEVER** use raw `git commit` in the root directory if you've changed code in
`crates/`. Instead, use Toad's own orchestration:

```bash
# 1. See what's dirty across the whole ecosystem
toad ggit status

# 2. Commit across submodules AND the Hub in one command
toad ggit commit -m "feat: your change" --cascade

# 3. Push everything safely
toad ggit push
```

The `--cascade` flag is critical: it commits your changes in the submodules
first, then automatically updates the Hub's pointers to those new commits.

### 3. Sticky Merge Strategy

We use an **intentional drift** strategy for branch tracking. The `dev` branch
tracks `dev` submodules, and the `main` branch tracks `main` submodules.

To prevent merge conflicts or accidental overwrites of these settings, we use a
**"Sticky Merge"** strategy:

- **`.gitattributes`**: Configured to use the `ours` merge driver for
  `.gitmodules`.
- **Automation**: Running `just setup` automatically configures your local git
  to respect this rule.

When you merge `dev` into `main`, Git will automatically preserve the production
branch tracking from the target branch.

---

## 📜 License

Primatif Toad uses an **Open Core** licensing model:

| Component | License |
| :--- | :--- |
| `bin/toad` (CLI) | MIT |
| `crates/toad-core` | MIT |
| `crates/toad-scaffold` | MIT |
| `crates/toad-discovery` | BUSL-1.1 |
| `crates/toad-git` | BUSL-1.1 |
| `crates/toad-manifest` | BUSL-1.1 |
| `crates/toad-ops` | BUSL-1.1 |
| `bin/toad-mcp` (MCP server) | BUSL-1.1 |

All BUSL-1.1 components convert to **MIT** on **2034-02-07** (8-year
conversion). Each crate's repository contains its own LICENSE file with full
terms.

---

<!-- Project Banner -->

<div align="center">

![Toad Banner](assets/cover.jpeg)

</div>
