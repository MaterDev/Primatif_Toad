# Track-007: Toad Clean (Disk Hygiene)

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Implement a `clean` command to reclaim disk space by removing build artifacts
from managed projects, using the dynamic artifact metadata provided by the
Strategy Engine.

## Deliverables

### Phase 1: Core Cleaning Logic (`toad-ops`)

- [x] **Task: Implement Artifact Identification & Deletion**

  - [x] Write Tests: Verify `clean_project` correctly identifies targets and
        respects `dry_run`.

  - [x] Implement: Create `crates/toad-ops/src/clean.rs` with cleaning logic.

- [x] **Task: Space Calculation**

  - [x] Implement: Return detailed stats (bytes reclaimed) from the cleaning
        operation.

### Phase 2: CLI Integration (`bin/toad`)

- [x] **Task: Add `toad clean` command**

  - [x] Implement: Add `Clean` variant to `Commands` enum with
        query/tag/dry-run/yes flags.

- [x] **Task: Pre-flight Summary**

  - [x] Implement: Display a pond-themed summary of targets and potential
        savings before proceeding.

- [x] **Task: Batch Execution**

  - [x] Implement: Parallel batch cleaning with progress bar.

### Phase 3: Advanced Filtering

- [x] **Task: Activity-Based Targets**

  - [x] Implement: Support for `--cold` and `--archive` flags to target stagnant
        projects.

### Phase 4: Safety & Verification

- [x] **Task: Protection Layer**

  - [x] Implement: Hardcoded blacklist (e.g., `.git`, `src`) to prevent
        accidental deletion of critical files.

- [x] **Task: Documentation**

  - [x] Implement: Update `README.md` and `USER_GUIDE.md` with cleaning
        instructions.

## Verification

- [x] Run `toad clean --dry-run` on a known project and verify correct artifacts
      are listed.

- [x] Execute a real clean and verify disk space is reclaimed.

- [x] Full `just qa` pass.
