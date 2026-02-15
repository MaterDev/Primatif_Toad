# Implementation Plan: v1.1.0 Intelligence Regressions

## Tasks

### VCS & Git (toad-git)

- [x] **IR-1: Restore Preflight Checks**
  - Implement unpushed commit detection.
  - Implement SHA alignment detection.
- [x] **IR-2: Correct Status Mapping**
  - Map merge conflicts ('U') to `Dirty` status.
  - Ensure untracked files do not mask staged/modified changes.
- [x] **IR-3: Correct API Semantics**
  - Restore `has_unmerged_changes` to check upstream divergence.

### Operations (toad-ops)

- [x] **IR-4: Fix Timeout Regression**
  - Re-integrate `wait-timeout` crate.
  - Remove manual busy-loop in `run_in_dir`.
- [x] **IR-5: Restore Skill Distribution**
  - Restore support for all 10+ AI vendors.
  - Restore custom path mapping (`vendor:path`) support.
- [x] **IR-6: Robust Migration Scoring**
  - Add underflow protection to `compare_projects` score.

### Discovery & Manifest (toad-discovery, toad-manifest)

- [x] **IR-7: Accurate DNA Detection**
  - Fix path building for submodule DNA derived from `projects/` dir.
- [x] **IR-8: Fix Hub/Pond Detection**
  - Ensure correct classification when working in a Hub root with a `projects/`
    folder.
- [x] **IR-9: Eliminate Duplicate Scan Results**
  - Fix double-registration of submodules in `scan_all_projects`.
- [x] **IR-10: Deep-Dive CONTEXT.md**
  - Replace stub with spec-compliant project briefing generator.

## Verification

- `just test -p toad-git -p toad-ops -p toad-discovery -p toad-manifest`
- Manual verification of `toad ggit sync` safety checks.
- Manual verification of `toad skill sync` distribution.
