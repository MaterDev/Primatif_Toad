# Specification: Post-Release Hardening (111-post-release-hardening)

## Overview

Address technical debt and critical issues identified during the review of Phase
3 completion. This includes fixing CI pipelines, refactoring the overgrown MCP
server, and refining diagnostic logic.

## Goals

1. **Fix CI/CD** — Restore the broken GitHub Actions workflow.
2. **Refactor MCP Server** — Decompose `server.rs` to meet the 700-line limit.
3. **Refine Doctor Command** — Fix safety issues and redundant logic in
   `toad doctor`.
4. **Improve Error Reporting** — Surface more detail in health checks.

---

## Issues to Address

### 1. Hub: Broken CI Keyword (CRITICAL)

- **File:** `.github/workflows/ci.yml`
- **Issue:** `name:` keyword replaced by `rename:`.
- **Fix:** Restore `name: CI`.

### 2. toad-mcp: 700-Line Limit Violation (HIGH)

- **File:** `bin/toad-mcp/src/server.rs`
- **Issue:** File is ~1100 lines.
- **Fix:** Refactor into sub-modules:
  - `src/tools/discovery.rs`
  - `src/tools/context.rs`
  - `src/tools/management.rs`
  - `src/tools/analysis.rs`

### 3. toad-mcp: Unused Stats Parameter (MEDIUM)

- **File:** `bin/toad-mcp/src/server.rs`
- **Issue:** `StatsParams::all` is suppressed with `#[allow(dead_code)]`.
- **Fix:** Use the parameter in `get_disk_stats` or remove it.

### 4. toad: Doctor Command Safety (MEDIUM)

- **File:** `bin/toad/src/commands/doctor.rs`
- **Issue:** Potential panic from `.unwrap()` on `workspace_path`.
- **Fix:** Use safe option handling.

### 5. toad: Redundant Sync (LOW)

- **File:** `bin/toad/src/main.rs`
- **Issue:** `toad doctor` triggers opportunistic sync before running its own
  diagnostics.
- **Fix:** Skip opportunistic sync when running `doctor`.

### 6. toad-ops: Opaque Registry Errors (LOW)

- **File:** `crates/toad-ops/src/doctor.rs`
- **Issue:** Registry load errors are discarded.
- **Fix:** Surface specific registry error messages in `HealthReport`.

---

## Architecture Decisions

### AD-1: MCP Sub-Module Pattern

Use a categorized module structure for tools to keep the main service file
clean.

### AD-2: Explicit Sync Control

The `doctor` command should be added to the `is_bootstrap` or similar exclusion
list in `main.rs` to prevent automatic context mutation during diagnostics.
