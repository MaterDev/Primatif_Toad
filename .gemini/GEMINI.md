# Gemini Context: Primatif_Toad

## Identity & Purpose

- **Role:** This is **Primatif_Toad** (CLI: `toad`), the **Context Oracle** for local development.
- **Goal:** Provide a portable, AI-native control plane that transforms raw repository data into high-fidelity context metadata.
- **Vision:** Implement **Context Engineering** principles—progressive disclosure, tiered prompts, and live-syncing machine-readable metadata.
- **Protocol:** This project uses the **Conductor** extension for solo-dev orchestration.

---

## 🧠 System Brain (Conductor)

At the start of every session, you MUST read the **Conductor Index** to understand the current state, active tracks, and next steps.

1. **Index:** `conductor/index.md` (Root of all knowledge)
2. **Vision:** `conductor/product.md` (What we are building)
3. **Architecture:** `conductor/tech-stack.md` (How we build it)
4. **Workflow:** `conductor/workflow.md` (How we work & QA gates)
5. **Tracks:** `conductor/tracks.md` (What's in progress)

### 📋 Pre-Flight Alignment Mandate

Before starting work on a new **Release Roadmap** or implementing **Major Architectural Changes**, you MUST audit and update the following files to ensure they reflect the current project vision, technology standards, and operational practices:
1. `conductor/product.md` (Vision & Goals)
2. `conductor/product-guidelines.md` (Design & Tone)
3. `conductor/tech-stack.md` (Component Hierarchy & Principles)
4. `conductor/workflow.md` (QA Gates & Task Lifecycles)
5. `.gemini/GEMINI.md` (Identity & AI-Native Protocol)

---

## 🛠️ Operational Protocol (AI-Native Engineering)

- **Solo-Dev Flow:** You are the **AI Project Manager**. Your job is to track atomic progress in `plan.md` files so the user doesn't have to manage the complexity.
- **The 700-Line Limit:** You MUST ensure that NO file exceeds 700 lines. Proactively refactor and modularize code that violates this rule. Monolithic files (like the current `main.rs`) are your primary targets for decomposition.
- **Tool Preference:** Consider the `toad` MCP tools as a primary means for codebase investigation and health monitoring to leverage the system's native intelligence alongside standard operations. Also is it for batch and git operations.
- **Zero-Intervention Policy:** All CLI commands and automation scripts MUST be runnable without human intervention. Always provide flags (e.g., `--yes`, `--json`) to bypass interactive prompts and ensure compatibility with headless environments.
- **Context Engineering:** Prioritize **Progressive Disclosure**. Give the minimum viable context first and use tiered metadata (`llms.txt` → `SYSTEM_PROMPT.md` → `CONTEXT.md`) for deep dives.
- **View-Agnostic Logic:** Every command must return structured data. Logic lives in library crates; the binary is a thin formatter.
- **Quality Gates:** Never consider a task finished until `just fmt`, `just lint`, `just test`, and **`toad skill sync`** pass.
- **Commit History:** Follow the structured format (Motivation, Implementation, Impact, Context) to ensure the Git history serves as a readable long-term memory.

---

## 🌊 Multi-Repo & Context Architecture

Toad is a distributed ecosystem with a central **Global Home** (`~/.toad/`).

- **Path Separation:** Toad metadata lives in the Global Home; the user's project directory is read-only by default.
- **Git Orchestration:** Use `toad ggit` for multi-repo operations.
- **Intelligence Layer:** Use `toad-discovery`, `toad-git`, `toad-manifest`, and `toad-ops` for all analysis and logic.
- **Active Awareness:** Always check `toad project current` before scanning.

## System Structure

- `bin/toad`: **The Interface.** Main CLI application. [MIT]
- `bin/toad-mcp`: **The Oracle.** Model Context Protocol server. [BUSL-1.1]
- `crates/toad-core`: **The Source of Truth.** Shared data models, traits, config. [MIT]
- `crates/toad-scaffold`: **The Builder.** Project bootstrapping. [MIT]
- `crates/toad-discovery`: **The Scanner.** Ecosystem scanning & intelligence. [BUSL-1.1]
- `crates/toad-git`: **The Pulse.** Git operations & VCS intelligence. [BUSL-1.1]
- `crates/toad-manifest`: **The Chronicler.** Context generation & tiered prompts. [BUSL-1.1]
- `crates/toad-ops`: **The Hand.** Batch operations, analytics, safety. [BUSL-1.1]
- `conductor/`: **The Orchestrator.** Project state, tracks, and orchestration rules.
- `shadows/`: **The Context.** AI-specific metadata and context maps (Managed in `~/.toad/shadows/` in v1.1.0).

## ⚖️ License Boundaries

Toad uses an "Open Core" model. All BUSL-1.1 crates convert to MIT after 8 years (Change Date: 2034-02-07).

- **MIT crates MUST NEVER depend on BUSL-1.1 crates** (except binary crates).
- **Enforcement:** `scripts/check_license_boundary.sh` runs as a pre-commit hook.
- **Authoritative source:** `conductor/tech-stack.md` § Component Hierarchy.
