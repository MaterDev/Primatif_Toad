# Implementation Plan: Submodule Awareness (v1.0.2 Phase 4)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 4. This plan is
> an execution guide. The release docs are authoritative.

## Phase 1: Models & Discovery

- [ ] **P4-1: `SubmoduleDetail` Model**
  - Add to `toad-core` (MIT).
- [ ] **P4-2: Submodule Discovery**
  - Implement `.gitmodules` parsing in `discovery` (BUSL-1.1).

## Phase 2: Git Analysis

- [ ] **P4-3: Submodule VCS Status**
  - Implement init-check and commit-alignment in `toad-git` (BUSL-1.1).

## Phase 3: CLI & Dogfooding

- [ ] **P4-4: CLI Integration**
  - Update `toad status` and `toad reveal` display.
- [ ] **P4-5: Self-Management**
  - Verify Toad correctly reports status of its own crate submodules.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — §
  Submodule-Aware Ecosystem Management
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 4
