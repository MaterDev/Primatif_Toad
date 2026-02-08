# Specification: Global Install Architecture (110-1)

## Overview
Decouple Toad's context from the source repository. Enable Toad to function as a global install with all metadata stored in `~/.toad/`.

## Sources
- **Strategy:** `docs/releases/v1.1.0/evolution.md` (§ Phase 0)
- **Tasks:** `docs/releases/v1.1.0/tasks.md` (§ Phase 0)

## Requirements
1. `Workspace` path separation (`toad_home` vs `projects_dir`).
2. Move `shadows/` and `tags.json` to `~/.toad/`.
3. Simplify `Workspace::discover()` (env var -> config.json).
4. Auto-migration for legacy `.toad-root` workspaces.
