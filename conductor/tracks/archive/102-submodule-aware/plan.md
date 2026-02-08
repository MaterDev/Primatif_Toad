# Implementation Plan: Submodule Awareness (v1.0.2 Phase 4)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 4. This plan is
> an execution guide. The release docs are authoritative.

## Phase 1: Models & Discovery

- [ ] **P4-1: `SubmoduleDetail` Model**
  - Add to `toad-core` (MIT).
- [ ] **P4-2: Submodule & Orphan Discovery**
  - Implement `.gitmodules` parsing in `discovery` (BUSL-1.1).
  - Implement "Orphan" detection (directories with `.git` but not in
    `.gitmodules`).
  - Update `scan_projects` to include the Hub root itself if it contains
    projects.

## Phase 2: Git Analysis & Shadows

- [ ] **P4-3: Submodule VCS Status**
  - Implement init-check and commit-alignment in `toad-git` (BUSL-1.1).
- [ ] **P4-3b: Recursive Shadow Generation**
  - Update `toad-manifest` to generate shadows for submodules and orphans.
  - Ensure `MANIFEST.md` reflects the project hierarchy (Hub > Submodule).

## Phase 3: CLI & Dogfooding

- [ ] **P4-4: CLI Integration**
  - Update `toad status` and `toad reveal` display to show hierarchy.
- [ ] **P4-5: Self-Management**
  - Verify Toad correctly reports status and generates shadows for its own crate
    submodules.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — §
  Submodule-Aware Ecosystem Management
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 4
