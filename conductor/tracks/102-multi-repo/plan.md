# Implementation Plan: Multi-Repo Split (v1.0.2 Phase 1 & 2)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 1, Phase 2, Phase 3.
> This plan is an execution guide. The release docs are authoritative.

## Phase 1: Repo Creation
- [ ] **P1-1 & P1-2: Create GitHub Repos**
  - Create 6 new public repos under `Primatif/`.
- [ ] **P1-3: Configure Repo Settings**
  - Set default branch to `main` for all new repos.
  - Add repo descriptions and topic tags (`toad`, `primatif`, `rust`, license).

## Phase 2: History Extraction
- [ ] **P2-1: Install Prerequisites**
  - Install `git-filter-repo`, verify with `--version`.
- [ ] **P2-2: Extract Crate Histories**
  - Run `git-filter-repo --subdirectory-filter` for each crate.
  - Push filtered histories to new remotes.
- [ ] **P2-3: Verify Extracted Repos**
  - For each repo: `git log` shows correct filtered history.
  - `Cargo.toml`, `LICENSE`, `src/` all present at root.

## Phase 3: Submodule Conversion
- [ ] **P2-4: Convert to Submodules**
  - `git rm -rf crates/<name>`.
  - `git submodule add <url> crates/<name>`.
  - **Version pinning:** Verify each submodule entry references the SHA from
    the newly pushed `main` branch. `git submodule status` must show all
    submodules pointing to valid, reachable remote commits.
- [ ] **P2-5: Verify Workspace**
  - `cargo build` and `cargo test` pass in submodule layout.
  - All `path = "../toad-core"` references still resolve correctly.
- [ ] **P2-6: Merge & Tag**
  - Merge `feat/multi-repo-split` into `main`.
  - Tag: `git tag v1.0.2`.
  - Push with submodules: `git push --recurse-submodules=on-demand`.

## Phase 4: Documentation & Cleanup
- [ ] **P3-1 & P3-2: Documentation**
  - Update README and Contributing guide with submodule instructions.
- [ ] **P4b-7: History Cleanup** _(post-split, one-time)_
  - Run `scripts/history_cleanup.sh` to remove old source from Hub history.

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Multi-Repo Architecture, § History Preservation
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 1, Phase 2, Phase 3
