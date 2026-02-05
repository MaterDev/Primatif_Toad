# Track-007: Toad Clean (Disk Hygiene)

## Status

- **Priority:** Low
- **Owner:** Pending
- **Status:** Pending

## Objective

Implement a `clean` command to reclaim disk space by removing build artifacts
from managed projects.

## Deliverables

- [ ] Logic to identify and calculate size of build artifacts.
- [ ] Integration into `bin/toad` CLI (`toad clean [--dry-run] [--query]`).
- [ ] Support for cleaning by activity tier (e.g., `--cold`).
- [ ] Safety verification to prevent accidental source code deletion.
