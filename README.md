<div align="center">

[![Toad Project Lifecycle](https://github.com/Primatif/Primatif_Toad/raw/main/assets/video.gif)](https://github.com/Primatif/Primatif_Toad/raw/main/assets/video.mp4)

### Toad Control CLI

_Tactical Operations And Discovery_

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version: v0.7.0](https://img.shields.io/badge/version-v0.7.0-green.svg)](Cargo.toml)
[![Coverage: >80%](https://img.shields.io/badge/coverage-%3E80%25-brightgreen.svg)](Justfile)

---
</div>

**🐸 `Toad`** is a **High-Performance Local-Ops Platform** designed to
manage a vast ecosystem of independent projects. It provides a multi-threaded
administrative layer for orchestrating workspaces, performing data-driven
analytics, and enforcing safety guardrails across dozens of repositories.

**Local-Ops** is the engineering discipline of managing the "local cloud"—the
dense ecosystem of independent repositories, microservices, and experiments
living on a developer’s workstation. Toad automates this daily environment,
treating dozens of scattered folders as a single, orchestrated infrastructure
and transforming a fragmented filesystem into a high-fidelity control plane.

By centralizing observability and orchestration, Local-Ops solves the critical
problems of maintenance drift and resource bloat. It provides a "bird's-eye
view" through visual health analytics and parallel command dispatching, allowing
a single developer to perform ecosystem-wide tasks—like security patching,
artifact cleaning, or AI context synchronization—in seconds rather than hours.
Ultimately, Local-Ops bridges the gap between raw code and developer
productivity, ensuring that a massive portfolio of work remains a live asset
rather than an unmanageable burden of technical debt.

> [!CAUTION]
> **Toad is a powerful meta-engineering tool.** Commands execute across multiple
> independent repositories simultaneously. Misuse can lead to significant data
> loss. Always verify your targets and commands before execution.

## 🪷 The v0.7.0 Bloom

- **🦗 Multi-Core Parallelism:** Leverages `rayon` for sub-second scanning and
  concurrent bulk command execution across 100+ projects.
- **🛡️ Safety Guardrails:** Built-in "Danger Pattern" detection (`rm -rf`,
  `reset --hard`) with forced `PROCEED` confirmations and `--dry-run` modes.
- **🟩 Visual Analytics:** High-fidelity disk usage auditing with the **Atari
  Heatmap** and a "Bloat Index" (Source vs. Artifact ratio).
- **🌿 Taxonomy (Tagging):** Ubiquitous filtering across all commands using
  procedural hashtags (`#rust`, `#node`) and custom persistent tags.
- **🍄 Global Anchor:** System-wide CLI access via `toad home`, allowing management
  from any directory on your Mac.

## Quick Start

1. **Install:** `just install`
2. **Anchor:** `toad home .` (Set your current directory as the system default)
3. **Analyze:** `toad stats` (See your ecosystem's health heatmap)
4. **Tag:** `toad tag --harvest` (Automatically categorize your projects)

---

## 🐸 Core Commands

### 🪷 Analytics & Health
- `toad status`: Check Git health and activity tiers across the ecosystem.
- `toad stats`: View the visual disk usage heatmap and artifact bloat analysis.
- `toad reveal <query>`: Search for projects by name or `#tag`.

### 🪵 Orchestration (Bulk Ops)

> [!WARNING]
> **High-Risk Operations:** Batch execution via `toad do` is potentially
> destructive. We strongly recommend using the `--dry-run` flag to preview
> changes before running them for real.

- `toad do "<command>"`: Execute shell commands across matching projects in parallel.
- `toad do "git pull" --tag stable`: Batch update only your stable tools.
- `toad do "rm -rf target" --dry-run`: Safely preview destructive maintenance.

### 🌿 Taxonomy & Context
- `toad tag <project> <tag>`: Assign custom metadata.
- `toad tag --query "ui" #frontend`: Bulk tag projects by name.
- `toad manifest`: Synchronize high-fidelity AI context ("Shadows").

### 🍄 Workspace Anchor

> [!IMPORTANT]
> **Context Steering:** The `toad home` command updates the global system
> pointer. All subsequent CLI calls will target the projects and metadata in the
> newly anchored directory.

- `toad home .`: Anchor the current directory as your system-wide Toad home.
- `toad home [path]`: View or set the global workspace pointer manually.

---

## 🧱 Architecture

Toad is built as a modular Rust workspace, ensuring logic is decoupled and
reusable:

- **`toad` (The Orchestrator):** The primary CLI binary.
- **`toad-core`:** Shared data models, global configuration, and fingerprinting.
- **`discovery`:** Strategy-based tech stack detection and parallel scanning.
- **`toad-ops`:** Shell execution engine with safety filters and timeouts.
- **`scaffold`:** Boilerplate generation logic.

---

## 🌊 Development

### Prerequisites

- **Rust & Cargo:** (Latest stable)
- **Just:** [just command runner](https://github.com/casey/just)
- **Dprint:** `cargo install dprint` (Formatted code and docs)
- **Tarpaulin:** `cargo install cargo-tarpaulin` (Coverage reporting)

### Quality Gates

We maintain a strict **>80% code coverage** mandate. Before every commit, run:

```bash
just qa
```

This executes formatting, clippy, unit tests, and coverage verification.

---

<!-- Project Banner -->

<div align="center">

![Toad Banner](assets/cover.jpeg)

<!-- Hop safely, little toads! 🐸 -->

</div>