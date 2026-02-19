# Specification: v1.1.0 Core Hardening (110-8)

## Overview

Address foundational issues and inconsistencies identified in `toad-core` and
general workspace management during the v1.1.0 PR review.

## Sources

- **Review Findings:** `docs/releases/v1.1.0/pr-review.md` (§ Phase 1:
  Foundation)

## Requirements

1. **Environment Inconsistency:** Align `TOAD_HOME` and `TOAD_CONFIG_DIR` usage
   across the codebase to ensure deterministic configuration discovery.
2. **Discovery Tier Fixes:** Ensure `TOAD_ROOT` override correctly handles
   existing global configs and `projects_dir` resolution.
3. **Migration Robustness:** Update legacy artifact migration to handle
   cross-filesystem moves gracefully (replace `fs::rename` with copy-then-delete
   logic).
4. **Token Heuristics:** Document `estimate_tokens` as an approximation and fix
   compilation errors in `truncate_by_tokens`.
5. **Path Normalization:** Fix `fs::canonicalize` failures on non-existent
   directories during initialization.
