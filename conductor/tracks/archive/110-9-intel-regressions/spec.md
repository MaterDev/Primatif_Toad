# Specification: v1.1.0 Intelligence Regressions (110-9)

## Overview

Restore critical logic and fix regressions in the intelligence layer
(`toad-git`, `toad-ops`, `toad-discovery`, `toad-manifest`) identified during
the v1.1.0 PR review.

## Sources

- **Review Findings:** `docs/releases/v1.1.0/pr-review.md` (§ Phase 2:
  Intelligence Layer)

## Requirements

1. **VCS Integrity:**
   - Restore `preflight_check` logic (alignment and unpushed commit detection).
   - Fix `VcsStatus` mapping for merge conflicts.
   - Restore proper dirty-tree detection in `check_status`.
2. **Operational Safety:**
   - Replace manual busy-loop timeout with the `wait-timeout` crate in
     `run_in_dir`.
   - Restore full vendor support and custom mapping in `distribute_skills`.
   - Fix score underflow in `compare_projects`.
3. **Discovery Accuracy:**
   - Fix submodule path resolution in DNA detection.
   - Eliminate duplicate submodule registration in `scan_all_projects`.
   - Fix Hub/Pond classification logic under `projects/` subdirectories.
4. **Manifest Depth:**
   - Replace `CONTEXT.md` stub with a real project deep-dive generator.
