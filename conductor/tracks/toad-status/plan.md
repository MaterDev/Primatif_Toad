# Track-001: Toad Status (The Health Check)

## Status

- **Priority:** High
- **Owner:** Pending
- **Status:** Pending

## Objective

Implement a `status` command for the Toad CLI that scans all projects in the
`projects/` directory and reports their Git health (uncommitted changes,
untracked files).

## Deliverables

- [x] Logic in `crates/toad-discovery` (or a new `crates/git-ops`) to check git
      status. [commit: 7ea6794]
- [~] Integration into `bin/toad` CLI (`toad status`).
- [ ] Support for filtering (`--query`).
- [ ] Retro ASCII output for status report.
- [ ] Unit tests for status detection.
