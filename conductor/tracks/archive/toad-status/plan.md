# Track-001: Toad Status (The Health Check)

## Status
- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Completed

## Objective
Implement a `status` command for the Toad CLI that scans all projects in the `projects/` directory and reports their Git health (uncommitted changes, untracked files).

## Deliverables
- [x] Logic in `crates/toad-git` to check git status. [commit: f1b4067]
- [x] Integration into `bin/toad` CLI (`toad status`). [commit: f1b4067]
- [x] Support for filtering (`--query`). [commit: f1b4067]
- [x] Retro ASCII output for status report. [commit: f1b4067]
- [x] Unit tests for status detection. [commit: f1b4067]