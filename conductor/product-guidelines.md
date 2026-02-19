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

## 3. Context Strategy: The Oracle Pattern

Documentation and generated metadata are the "Long-term Memory" for AI agents.

- **Global Portability:** Generated context MUST live in `~/.toad/` (Toad Home),
  never polluting the user's project directories.
- **Progressive Disclosure:** Context should be tiered (`llms.txt` →
  `SYSTEM_PROMPT.md` → `CONTEXT.md`) to minimize token usage while maximizing
  accuracy.
- **Track-Based History:** Major features are documented as "Tracks" with
  specific implementation plans that act as a live state machine.
- **Context Integrity:** Employs multi-level hash fingerprinting and auto-sync
  triggers to ensure the AI's view is always synchronized with the filesystem.

## 4. Engineering Standards

- **Concise:** Favor high information density over conversational filler.
- **Small File Principle:** NO file should ever exceed **700 lines of code**.
  Monolithic files are considered technical debt and MUST be refactored into
  logical modules or specialized crates once this limit is approached.
- **Action-Oriented:** Documentation and CLI output should focus on "What's
  next?" and "How do I fix this?".

## 5. Licensing & Architecture Boundaries

Toad follows a strict "Open Core" boundary to protect its intelligence while
enabling an open-source ecosystem. All BUSL-1.1 crates convert to MIT after 8
years (Change Date: 2034-02-07).

- **MIT (Open):** `toad-core`, `toad-scaffold`, `bin/toad`.
- **BUSL-1.1 (Source-Available):** `toad-discovery`, `toad-git`,
  `toad-manifest`, `toad-ops`, `toad-mcp`.
- **Dependency Rule:** MIT crates must never depend on BUSL-1.1 crates. Binary
  crates (`bin/toad`, `bin/toad-mcp`) may depend on everything.
- **Git Monopoly:** No crate outside of `toad-git` may execute `git` commands or
  reason about Git internals.
- **Logic Extraction:** Every command MUST extract its business logic into a
  library crate and return structured data. The CLI binary is purely for
  formatting.

## 6. Project Contexts & Global Path Separation

Toad supports multiple workspace roots via **Named Project Contexts**.

- **Toad Home (`~/.toad/`):** The persistent database and context oracle.
- **Projects Directory:** The user's managed repositories (read-only by
  default).
- **Active Context:** All commands operate against the active context resolved
  via `toad project current`.
- **AI Awareness:** Always verify the active context path before performing
  scans or creating files. Use `CROSS_REPO_MAP.md` to understand the
  architectural links within the active context.
