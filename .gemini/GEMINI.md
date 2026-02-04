# Gemini Context: Primatif_Toad

## Identity & Purpose
- **Role:** This is **Primatif_Toad**, the Control Plane for the local development environment.
- **Goal:** Provide a portable, Mac-agnostic system to manage, audit, and manipulate any collection of software projects located in the `projects/` directory.
- **Strategy:** "Platform Architecture." A centralized Rust workspace manages the environment using composable libraries ("Crates") and a unified CLI ("Toad").

## Architecture & Design Principles
1.  **Platform Pattern:**
    - `bin/toad`: The single entry point (CLI) for the user.
    - `crates/`: Reusable, testable logic modules (e.g., `scaffold`, `git-ops`).
    - `projects/`: The managed user space (ignored by Git).
2.  **Environment Agnostic (Mac):** Code must run on *any* macOS system.
    - **No Hardcoded Paths:** Use relative paths or dynamic resolution.
    - **Portability:** System functions if moved or cloned.
4.  **Self-Documentation:**
    - The CLI must always include a `list` command (or equivalent) that explains all available capabilities to the user directly in the terminal.

## System Structure
- `bin/toad`: **The Interface.** Main CLI application (`toad`).
- `crates/`: **The Capabilities.**
    - `scaffold`: Project generation logic.
- `projects/`: **The Target.** Managed repositories.
- `docs/`: **The Manual.** Documentation.
- `.gemini/`: **The Brain.** Context and settings.

## Conventions
- **No Conflict:** Do not track sub-repositories in `projects/`.
- **Safety:** Always verify paths before moving or deleting.
- **Discovery:** Do not assume a fixed list of projects; always scan `projects/` dynamically.