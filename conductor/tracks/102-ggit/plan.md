# Implementation Plan: Group-Git Orchestration (v1.0.2 Phase 5)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 5. This plan is
> an execution guide. The release docs are authoritative.

## Phase 0: Target Unification

- [ ] **P5-0a: TargetSource & Logical Targets**
  - Add `TargetSource` enum (HubRoot, Submodule, PondProject, Orphan) to
    `toad-core`.
  - Update `ProjectDetail` to include its source.
- [ ] **P5-0b: Registry-First Orchestration**
  - Refactor `toad-ops` (the engine for `do` and `ggit`) to consume the
    `ProjectRegistry` instead of scanning disk via `fs::read_dir`.
  - Update `toad do` to use the cached list of managed targets.

## Phase 1: Data Models

- [ ] **P5-1: Git Data Models**
  - Add `RepoStatus`, `BranchInfo`, `CommitInfo`, `GitOpResult`,
    `PreflightResult` to `toad-core` (MIT).
- [ ] **P5-5: Branch Tracking Models**
  - Add `BranchGroup`, `BranchPresence`, `PrStatus` to `toad-core` (MIT).

## Phase 2: Orchestration Logic

- [ ] **P5-2: Core Git Operations** (`toad-git`, BUSL-1.1)
  - Implement `branch.rs`, `commit.rs` modules.
- [ ] **P5-3: Remote Operations** (`toad-git`, BUSL-1.1)
  - Implement `remote.rs` (push, pull across repos).
- [ ] **P5-4: Sync & Pre-flight** (`toad-git`, BUSL-1.1)
  - Implement `sync.rs` with ghost commit prevention.
  - Pre-flight checks: dirty state, unpushed commits.
- [ ] **P5-6: Branch Orchestration** (`toad-git`, BUSL-1.1)
  - Implement `branches.rs` (group listing, alignment).
  - Implement `merge_status.rs` and `align.rs`.

## Phase 3: Output Consolidation

- [ ] **P5-7: `run_git` Helper & Summary Layer** (`toad-git`, BUSL-1.1)
  - Central `run_git()` function — all git operations go through it.
  - Capture stdout/stderr per repo.
  - Default: one-line-per-repo consolidated summaries.
  - `--verbose`: stream full raw git output with per-repo headers.

## Phase 4: CLI Surface

- [ ] **P5-8: `toad ggit` Command** (`bin/toad`, MIT)
  - Full CLI surface: status, branch, checkout, add, commit, push, pull, sync,
    log, diff, branches, merge-status, align, preflight.
  - `--project <name>` targeting and `--verbose` flag.

## Phase 5: Dogfooding

- [ ] **P5-9: Feature Branch Workflow**
  - Verify end-to-end lifecycle on Toad's own crate submodules.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Multi-Repo Git
  Orchestration
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 5
