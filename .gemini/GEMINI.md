# Gemini Context: Primatif_Toad

## Identity & Purpose
- **Role:** This is **Primatif_Toad** (CLI: `toad`), the **Toad Control** system for local development.
- **Goal:** Provide a portable, Mac-agnostic control plane to manage, audit, and manipulate software projects in the `projects/` directory.
- **Protocol:** This project uses the **Conductor** extension for solo-dev orchestration.

---

## 🧠 System Brain (Conductor)
At the start of every session, you MUST read the **Conductor Index** to understand the current state, active tracks, and next steps.

1.  **Index:** `conductor/index.md` (Root of all knowledge)
2.  **Vision:** `conductor/product.md` (What we are building)
3.  **Architecture:** `conductor/tech-stack.md` (How we build it)
4.  **Workflow:** `conductor/workflow.md` (How we work & QA gates)
5.  **Tracks:** `conductor/tracks.md` (What's in progress)

---

## 🛠️ Operational Protocol (Universal File Resolution)
You must adhere to the **Universal File Resolution Protocol** (defined in `conductor/USER_GUIDE.md`) to manage tracks and state.

- **Solo-Dev Flow:** You are the **AI Project Manager**. Your job is to track atomic progress in `plan.md` files so the user doesn't have to manage the complexity.
- **Modular Codebase:** Strictly follow the crate-based architecture defined in `tech-stack.md`.
- **Quality Gates:** Never consider a task finished until `just fmt`, `just lint`, and `just test` pass.
- **Commit History:** Follow the structured, high-context format (Motivation, Implementation, Impact, Context) to ensure the Git history serves as a readable long-term memory for future AI agents.

---

## ⚖️ Licensing Architecture (Open Core)
Toad follows an **Open Core** model. Every code change MUST land in the correct license boundary.

- **MIT Layer (Open Contracts):** `toad-core`, `scaffold`, `bin/toad`.
  - *Data models, traits, interfaces, and CLI glue.*
- **BUSL-1.1 Layer (Intelligence):** `discovery`, `toad-git`, `toad-manifest`, `toad-ops`.
  - *Scanning engine, VCS intelligence, context generation, and operational logic.*
- **The Hard Gate:** MIT crates MUST NEVER depend on BUSL-1.1 crates (except the binary). Violation triggers immediate CI/Hook failure.
- **Decision Framework:** New capability? If it's a data model → MIT. If it's analysis/intelligence → BUSL-1.1.

---

## 🌊 Multi-Repo & Submodule Conventions
Toad is a distributed ecosystem of Git submodules managed by a central Hub.

- **Git Orchestration:** Always use `toad ggit` for multi-repo operations (status, branch, commit, push, pull). Avoid raw `git` commands in the Hub root.
- **SOTW (State of the World):** Never run `toad ggit sync --force` unless explicitly authorized. Always run `toad ggit preflight` before syncing to ensure all submodules are pushed and clean.
- **Naming:** Follow the `{type}/{scope}` convention for all cross-repo branches (e.g., `feat/new-scanner`).

---

## 🗺️ Project Contexts & Navigation
Toad supports multiple workspace roots via **Named Project Contexts**.

- **Switching:** Use `toad project switch <name>` to change targets. `toad home` is legacy.
- **Active Awareness:** Always check `toad project current` before scanning or creating files.
- **AI Navigator:** For complex architecture links, always refer to `CROSS_REPO_MAP.md` at the root. This is the source of truth for dependency graphs and type flow.

## System Structure
- `bin/toad`: **The Interface.** Main CLI application (`toad`). (MIT)
- `crates/`: **The Capabilities.** Git submodules with individual licenses.
- `projects/`: **The Target.** Managed repositories (ignored by Git).
- `conductor/`: **The Orchestrator.** Project state, tracks, and orchestration rules.
- `shadows/`: **The Context.** AI-specific metadata and context maps (ignored by Git).
- `CROSS_REPO_MAP.md`: **The Map.** Declared architectural links and type flow.