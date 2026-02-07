# Spec: Production Hardening & Audit Trail

## Overview

Prepare Toad for the v1.0.0 milestone by addressing critical logic gaps,
implementing ecosystem-wide accountability through auditing, and establishing
automation to prevent documentation drift.

## Requirements

### 1. Discovery Lockdown & Workspace State

- **Strict Mode:** The CLI must halt with a clear error message if a valid
  workspace cannot be discovered, unless the subcommand is in the "Exempt List"
  (`home`, `version`, `list`, `help`).
- **Deterministic Discovery:** Upward recursive searching must canonicalize
  every step to resolve symlink loops or mount point misalignments on macOS.

### 2. Operation Auditing (`~/.toad/ops.log`)

- **Persistence:** Every execution of `toad do` must be logged to a central
  audit trail.
- **Format (JSON-L):** Log entries should use JSON Lines for easy machine
  parsing while remaining human-readable.
- **Metadata:**
  - `timestamp`: ISO8601.
  - `command`: The full shell string executed.
  - `targets`: List of project names affected.
  - `summary`: Success/Fail/Skip counts.
  - `user`: System username.

### 3. Panic-Free Core

- **Eliminate `expect`/`unwrap`:** Core configuration and discovery paths must
  return `anyhow::Result`.
- **Global Config Safety:** Handle missing `$HOME` gracefully by returning a
  descriptive error instead of crashing.

### 4. Automated Documentation Integrity

- **Pre-Push Hook:** Implement a git hook that:
  1. Regenerates `docs/CLI.md` using the current binary.
  2. Compares the result with the version currently on disk.
  3. Blocks the push if a mismatch is detected, forcing the user to commit
     updated documentation.

### 5. Resilient Cleanup & Harvest logic

- **Signal Traps:** `run_ephemeral.sh` must catch `SIGINT`, `SIGTERM`, and
  `SIGHUP` to ensure `/tmp` artifacts are purged if the session is closed.
- **Atomic Registry Ops:** Bulk tagging (`harvest`) must verify the integrity of
  the `tags.json` file after the operation.

## Design

- **Audit Engine:** A new `toad_ops::audit` module.
- **Hook Strategy:** `scripts/git-hooks/pre-push` script linked via `Justfile`.
- **Discovery Strategy:** Refactor `Workspace::discover` to use a `loop` with
  path canonicalization.
