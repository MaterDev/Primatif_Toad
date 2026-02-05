# Spec: Bulk Operation Safety & Sandbox Integration

## Overview

`toad do` is a powerful tool that can cause irreversible damage if misused. This
track implements a safety-first architecture to protect the developer from
destructive accidents and integrates a repeatable sandbox environment for
verifying high-risk operations.

## Requirements

1. **Ghost Mode (`--dry-run`):** Mandatory flag support to simulate execution
   and show exact paths.
2. **Destructive Pattern Matching:** Detect "Danger Words" in command strings.
3. **High-Bar Confirmation:** Require specific user input (e.g., typing
   `PROCEED` or the target count) when destructive patterns are found.
4. **Resilience Controls:**
   - `--fail-fast`: Stop the whole batch if one project fails.
   - **Timeouts:** Kill any subprocess that hangs for more than 30s.
5. **Audit Trail:** Persistent log of all `do` operations for historical
   debugging.
6. **Enhanced Sandbox Utility:**
   - Modularized and configurable sandbox generation.
   - Isolated directory (`scripts/sandbox/`) with its own documentation.
   - Arguments for tech stack mix and directory structure complexity.

## Design

1. **Safety Engine:** Create a `toad_ops::safety` module.
2. **Danger Patterns:** Static array of regexes: `rm -rf`, `git reset --hard`,
   `force`, `prune`, `drop`, `truncate`.
3. **Audit Logger:** Simple append-only text file at `~/.toad/ops.log`.
4. **Timeout Wrapper:** Use `std::process::Child::wait_timeout` or similar logic
   to enforce limits.
5. **Sandbox Refactor:** Move `mksandbox.sh` to a dedicated sub-package
   structure in `scripts/sandbox/`.
