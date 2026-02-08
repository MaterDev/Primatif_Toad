# Implementation Plan: Project Contexts (v1.0.2 Phase 4b)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 4b. This plan is
> an execution guide. The release docs are authoritative.

## Phase 1: Data Models & Storage

- [x] **P4b-1: GlobalConfig Extension** [06b22e6]
  - Add `ProjectContext` struct, `active_context`, `project_contexts` HashMap.
  - Implement auto-migration from old `home_pointer`-only config to `default`
    context.
- [x] **P4b-2: Storage Reorganization — Per-Context Artifacts** [06b22e6]
  - Add `context_dir()` and `context_shadows_dir()` to `GlobalConfig`.
  - Move `ProjectRegistry` to `~/.toad/contexts/<name>/registry.json`.
  - Move `shadows/` from workspace root to `~/.toad/contexts/<name>/shadows/`.
  - Update `Workspace::with_root()` so `shadows_dir` resolves to per-context
    path.
  - Backward compat: migrate old `~/.toad/registry.json` and `<root>/shadows/`
    to `~/.toad/contexts/default/`.

## Phase 2: Workspace Resolution

- [x] **P4b-3: Workspace Resolution Update** [06b22e6]
  - Update `Workspace::discover()` to resolve via active context.
  - Priority: `TOAD_ROOT` env > `.toad-root` upward search > `active_context` >
    `home_pointer`.

## Phase 3: CLI Integration

- [x] **P4b-4: Installation Flow — `toad home` Update** [06b22e6]
  - `toad home <path>` registers and switches context (backward compat
    shortcut).
  - `toad home` (no args) shows active context's root.
- [x] **P4b-5: `toad project` Command** [06b22e6]
  - Implement `register`, `switch`, `current`, `list`, `update`, `delete`,
    `info`.
  - `register` creates `~/.toad/contexts/<name>/shadows/`.
  - `delete` removes context directory; MUST block or switch to 'default' if
    deleting the active context.

## Phase 4: Scripts

- [x] **P4b-6: Developer Setup Script** [06b22e6]
  - Create `scripts/dev_setup.sh` for fresh clone + submodule init.
- [x] **P4b-7: History Cleanup Script** [06b22e6]
  - Create `scripts/history_cleanup.sh` (one-time post-split,
    `git filter-repo`).

## Phase 5: Verification

- [ ] **P4b-8: Integration Tests**
  - Full lifecycle: register → switch → current → list → delete.
  - Backward compat: old config migrates, old registry migrates, old shadows
    migrate.
  - Context switching changes which registry and shadows are loaded.
  - `TOAD_ROOT` env var still overrides active context.
  - `toad home <path>` registers and switches correctly.
  - Dogfood with `toad-dev` and `my-code` contexts.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Project
  Contexts, § Storage Reorganization
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 4b
