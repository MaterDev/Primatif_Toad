# Specification: Project Contexts (v1.0.2 Phase 4b)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § Project Contexts
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 4b

## Overview

Introduce "Named Project Contexts" to allow Toad to manage multiple workspace
roots and switch between them explicitly. This replaces the single
`home_pointer` model.

## Requirements

- Support multiple registered workspaces with names and descriptions.
- `toad project` subcommand for context management (register, switch, current,
  list, update, delete, info).
- Backward compatibility for existing `home_pointer` users (auto-migrate to
  `default` context).
- Switchable context for all Toad commands (status, do, ggit, create).
- Per-context storage under `~/.toad/contexts/<name>/` for `registry.json` and
  `shadows/` (MANIFEST.md, tags.json).
- Shadows no longer live at the workspace root — all per-context artifacts are
  centralized in `~/.toad/`.
- `toad home <path>` updated to register and switch contexts (backward compat
  shortcut).
- Developer setup script (`scripts/dev_setup.sh`) for fresh clone + submodule
  init.
- History cleanup script (`scripts/history_cleanup.sh`) for post-split one-time
  cleanup.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Project
  Contexts, § Storage Reorganization
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 4b
