<div align="center">

[![Toad Project Lifecycle](https://github.com/Primatif/Primatif_Toad/raw/main/assets/video.gif)](https://github.com/Primatif/Primatif_Toad/raw/main/assets/video.mp4)

### Toad Control CLI

The modular meta-engineering platform.

[![Version: v1.1.0](https://img.shields.io/badge/version-v1.1.0-green.svg)](Cargo.toml)
[![CI](https://github.com/Primatif/Primatif_Toad/actions/workflows/ci.yml/badge.svg)](https://github.com/Primatif/Primatif_Toad/actions/workflows/ci.yml)
[![Coverage: >80%](https://img.shields.io/badge/coverage-%3E80%25-brightgreen.svg)](Justfile)

---
</div>

Toad is a High-Performance Local-Ops Platform designed to manage a vast ecosystem of independent projects. It provides a multi-threaded administrative layer for orchestrating workspaces, performing data-driven analytics, and enforcing safety guardrails across dozens of repositories.

With the **v1.1.0 "Deep Croak"** release, Toad transforms into a portable, AI-native **Context Oracle**, providing AI agents with zero-latency vision across complex multi-repo environments via the Model Context Protocol (MCP) and deep structural DNA mapping.

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

# 4. Initialize AI Context
toad init-context
```

## What's New in v1.1.0 "Deep Croak"

- **Model Context Protocol (MCP):** A full-featured MCP server (`toad-mcp`) that exposes Toad's intelligence directly to AI agents in Cursor, Windsurf, and other compatible IDEs.
- **Pattern Intel & DNA Mapping:** Deep structural analysis that automatically identifies component roles (e.g., Data Layer, API Surface) and capabilities (e.g., Dockerized, Async).
- **Context Engineering:** Tiered metadata architecture (`llms.txt` → `SYSTEM_PROMPT.md` → `MANIFEST.md`) for progressive disclosure of architectural context.
- **Project Briefings:** Generates high-fidelity `CONTEXT.md` files with entry points, lifecycle data, and operational intelligence for every project.
- **Ecosystem Indexing:** Centralized `ATLAS.json` for rapid cross-project pattern matching and architectural synthesis.
- **Hardened Multi-Repo Ops:** Improved `ggit` with unpushed commit detection, SHA drift safety, and cross-platform migration support.

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

### AI Skills & Context Engineering

- **`toad manifest`** — Generate architectural manifests and ecosystem DNA
  indices.
- **`toad context`** — High-density task briefing generator for AI agents.
- **`toad skill sync`** — Distribute context metadata to 10+ AI vendors
  (Cursor, Windsurf, Copilot, Copilot, etc.).
- **`toad-mcp`** — Model Context Protocol server for real-time AI context
  injection.

### Orchestration (Bulk Ops)

- **`toad do "<command>"`** — Execute shell commands across matching projects in
  parallel.
- **`toad cw run <name>`** — Execute a registered custom workflow or automation
  script.

### Project Contexts

- **`toad project register <name> <path>`** — Register a new workspace root.
- **`toad project switch <name>`** — Switch the active context.
- **`toad project list`** — List all registered contexts.
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

Toad is built as a modular Rust workspace. Each component is its own git
repository, managed as a submodule:

```text
Primatif_Toad/                          (Hub)
├── bin/
│   ├── toad/                           Primary CLI
│   └── toad-mcp/                       MCP Context Server
├── crates/
│   ├── toad-core/                      Shared models & config
│   ├── toad-scaffold/                  Project bootstrapping
│   ├── toad-discovery/                 DNA & Ecosystem scanning
│   ├── toad-git/                       VCS Orchestration
│   ├── toad-manifest/                  Tiered metadata generation
│   └── toad-ops/                       Batch operations & safety
└── docs/                               Release & Architecture guides
```

### Dependency Graph

```text
toad / toad-mcp
├── toad-discovery  ← depends on: toad-core, toad-git, toad-ops
├── toad-manifest   ← depends on: toad-core
├── toad-ops        ← depends on: toad-core
├── toad-git        ← depends on: toad-core
└── toad-core       ← foundation
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

Primatif Toad is released under the **MIT License**. See the [LICENSE](LICENSE)
file for the full text.

Independent components (crates) managed by this Hub are subject to their own
respective licenses, which can be found within their individual submodule
directories.

---

<!-- Project Banner -->

<div align="center">

![Toad Banner](assets/cover.jpeg)

</div>
