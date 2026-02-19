# Specification: v1.1.0 Interface Fixes (110-10)

## Overview

Fix compilation errors, path-resolution bugs, and UI glitches in the primary
binaries (`toad`, `toad-mcp`) and the parent Hub configuration.

## Sources

- **Review Findings:** `docs/releases/v1.1.0/pr-review.md` (§ Phase 3 & 4)

## Requirements

1. **toad-mcp Stability:**
   - Fix borrow-move compilation error in `list_contexts`.
   - Ensure `NoParams` serializes correctly for JSON-RPC clients.
2. **toad CLI Accuracy:**
   - Fix `ggit` path resolution bug (correctly join submodule paths relative to
     their parent repo).
   - Map `Nothing to commit` git status to `OK` instead of `FAIL`.
3. **Hub Governance:**
   - Restore essential details to the root `CHANGELOG.md` for human readability.
   - Standardize `Cargo.toml` dependency versions to prevent drift.
