# Gemini Context: Primatif_Toad

## Identity & Purpose
- **Role:** This is **Primatif_Toad** (CLI: `toad`), the **Toad Control** system for local development.
- **Goal:** Provide a portable, Mac-agnostic control plane to manage, audit, and manipulate software projects in the `projects/` directory.
- **Strategy:** "Platform Architecture." A centralized Rust workspace (`bin` + `crates`) manages the environment.

## Architecture & Design Principles
1.  **Platform Pattern:**
    - `bin/toad`: The single entry point (CLI) for the user.
    - `crates/`: Reusable, testable logic modules (e.g., `scaffold`, `discovery`).
    - `projects/`: The managed user space (ignored by Git).
2.  **Environment Agnostic (Mac):** Code must run on *any* macOS system.
    - **No Hardcoded Paths:** Use relative paths or dynamic resolution.
    - **Portability:** System functions if moved or cloned.
3.  **Atomic Scripts:** `scripts/` is reserved for simple shell glue only.
4.  **Self-Documentation:**
    - The CLI must always include a `list` command.
    - `just install` delegates to `scripts/install_toad.sh` to verify installation and print the welcome banner.
5.  **Aesthetic Standard (Atari Style):**
    - CLI output should be styled with a retro "Pixel Art/Atari" aesthetic.
    - **Toad Green:** The primary brand color is standard terminal Green.
    - **ASCII Art:** Use blocky, pixel-like ASCII art for banners.

## System Structure
- `bin/toad`: **The Interface.** Main CLI application (`toad`).
- `crates/`: **The Capabilities.**
    - `scaffold`: Project generation logic.
    - `discovery`: Project finding and scanning logic.
- `projects/`: **The Target.** Managed repositories.
- `docs/`: **The Manual.** Documentation and Backlog.
- `.gemini/`: **The Brain.** Context and settings.

## Conventions
- **No Conflict:** Do not track sub-repositories in `projects/`.
- **Safety:** Always verify paths before moving or deleting.
- **Discovery:** Do not assume a fixed list of projects; always scan `projects/` dynamically.
