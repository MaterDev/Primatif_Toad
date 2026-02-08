# Specification: Agent Interface & Meta-Ops (110-4)

## Overview
Bridge the gap between generated context and AI agent consumption via tiered prompt structures.

## Sources
- **Strategy:** `docs/releases/v1.1.0/evolution.md` (§ Phase 3)
- **Tasks:** `docs/releases/v1.1.0/tasks.md` (§ Phase 3)

## Requirements
1. Standardized `AGENTS.md` generation for every project.
2. Tiered prompt files: `llms.txt`, `SYSTEM_PROMPT.md`, and per-project `CONTEXT.md`.
3. Context-aware pre-flights for `toad do`.
4. `toad init-context` orchestration command.
