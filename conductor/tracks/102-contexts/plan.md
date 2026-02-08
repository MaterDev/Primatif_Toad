# Implementation Plan: Project Contexts (v1.0.2 Phase 4b)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 4b. This plan is
> an execution guide. The release docs are authoritative.

## Phase 1: Data Models & Storage

- [~] **P4b-1: GlobalConfig Extension**
  - Add `ProjectContext` struct, `active_context`, `project_contexts` HashMap.
  - Implement auto-migration from old `home_pointer`-only config to `default`
    context.
- [ ] **P4b-2: Storage Reorganization — Per-Context Artifacts**
  - Add `context_dir()` and `context_shadows_dir()` to `GlobalConfig`.
  - Move `ProjectRegistry` to `~/.toad/contexts/<name>/registry.json`.
  - Move `shadows/` from workspace root to `~/.toad/contexts/<name>/shadows/`.
  - Update `Workspace::with_root()` so `shadows_dir` resolves to per-context
    path.
  - Backward compat: migrate old `~/.toad/registry.json` and `<root>/shadows/`
    to `~/.toad/contexts/default/`.

## Phase 2: Workspace Resolution

- [ ] **P4b-3: Workspace Resolution Update**
  - Update `Workspace::discover()` to resolve via active context.
  - Priority: `TOAD_ROOT` env > `.toad-root` upward search > `active_context` >
    `home_pointer`.

## Phase 3: CLI Integration

- [ ] **P4b-4: Installation Flow — `toad home` Update**
  - `toad home <path>` registers and switches context (backward compat
    shortcut).
  - `toad home` (no args) shows active context's root.
- [ ] **P4b-5: `toad project` Command**
  - Implement `register`, `switch`, `current`, `list`, `update`, `delete`,
    `info`.
  - `register` creates `~/.toad/contexts/<name>/shadows/`.
  - `delete` removes context directory; MUST block or switch to 'default' if
    deleting the active context.

## Phase 4: Scripts

- [ ] **P4b-6: Developer Setup Script**
  - Create `scripts/dev_setup.sh` for fresh clone + submodule init.
- [ ] **P4b-7: History Cleanup Script**
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
