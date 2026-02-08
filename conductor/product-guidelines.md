# Product Guidelines: Primatif Toad

## 1. Visual Identity & Aesthetic

Primatif Toad follows a **Retro-Atari / Cybernetic** aesthetic. The goal is to
provide a "terminal arcade" feel that makes development feel tactile and
high-stakes.

- **Primary Color:** "Toad Green" (Standard ANSI Green).
- **Art Style:** High-block ASCII art for banners and major status changes.
- **Tone:** Technical, direct, and slightly futuristic.

## 2. CLI Design Standards

To maintain the "Solo-Dev Smooth Flow," the CLI must be its own best manual.

- **Self-Documentation:** Every subcommand must implement a robust `--help`
  flag.
- **Discovery:** The `toad list` command is the central directory for all
  capabilities.
- **Feedback:** Success and error states should be visually distinct (e.g.,
  using block characters or specific ASCII patterns).

## 3. Documentation & Context Strategy

Documentation is not just for humans; it is the "Long-term Memory" for AI
agents.

- **Single Source of Truth:** All project standards, workflows, and technical
  specs live in the `conductor/` directory.
- **Track-Based History:** Major features are documented as "Tracks" with
  specific implementation plans that act as a live state machine.
- **Context Integrity:** Employs multi-level mtime fingerprinting to ensure the
  AI's view of the `projects/` directory is always synchronized with the actual
  filesystem.

## 4. Engineering Tone

- **Concise:** Favor high information density over conversational filler.
- **Action-Oriented:** Documentation and CLI output should focus on "What's
  next?" and "How do I fix this?".

## 5. Licensing & Architecture Boundaries

Toad follows a strict **Open Core** boundary to protect its intelligence while
enabling an open-source ecosystem.

- **Dependency Rule:** MIT crates (`toad-core`, `scaffold`) must never depend on
  BUSL-1.1 crates (`discovery`, `toad-git`, `toad-manifest`, `toad-ops`).
- **Git Monopoly:** No crate outside of `toad-git` may execute `git` commands or
  reason about Git internals.
- **SDK Stability:** `toad-core` is the platform SDK. New types added here must
  be treated as stable API contracts for third-party plugin developers.
- **Decision Framework:**
  - Data Model / Trait / Contract? → **`toad-core` (MIT)**.
  - Intelligence / Analysis / Logic? → **BUSL-1.1 crate**.
  - CLI Glue / Formatting? → **`bin/toad` (MIT)**.

## 6. Project Contexts & Workspace Switching

Toad supports multiple workspace roots via **Named Project Contexts**.

- **Active Context:** All commands operate against the active context resolved
  via `toad project current`.
- **Legacy Migration:** `toad home` is deprecated. Use `toad project switch` for
  all context changes.
- **AI Awareness:** Always verify the active context path before performing
  scans or creating files. Use `CROSS_REPO_MAP.md` to understand the
  architectural links within the active context.
