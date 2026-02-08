# Specification: v1.1.0 Blockers (Architecture Hardening)

## Overview
This track focuses on resolving the critical architectural blockers identified in the v1.1.0 evolution strategy. These changes are prerequisites for decoupling the CLI from the repository and enabling the MCP server.

## Sources
- **Strategy:** `docs/releases/v1.1.0/evolution.md` (§ Blockers & Risks > 🔴 Critical Blockers)
- **Tasks:** `docs/releases/v1.1.0/tasks.md` (§ Pre-Flight: Blockers)

## Requirements
1. **No-Print Violation:** Commands must return structured result types; `main.rs` handles formatting.
2. **Terminal Decoupling:** Progress bars (Indicatif) must be abstracted behind a trait.
3. **Typed Errors:** Replace string errors with a formal `ToadError` enum.
4. **Interactive Prompts:** All stdin prompts must be bypassable via `--yes` and moved out of library crates.
