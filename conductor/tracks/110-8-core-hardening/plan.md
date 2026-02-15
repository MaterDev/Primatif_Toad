# Implementation Plan: v1.1.0 Core Hardening

## Tasks

- [x] **CH-1: Environment Variable Alignment**
  - Align `TOAD_HOME` and `TOAD_CONFIG_DIR` across `Workspace::discover` and `GlobalConfig::config_dir`.
- [x] **CH-2: Fix TOAD_ROOT Priority**
  - Ensure `TOAD_ROOT` env override takes precedence even when a global config exists.
- [x] **CH-3: Robust Migration Logic**
  - Replace `fs::rename` with a cross-filesystem safe implementation in `migrate_legacy_artifacts`.
- [x] **CH-4: Token Utility Refinement**
  - Document `estimate_tokens` approximation.
  - Fix unterminated string literal in `truncate_by_tokens`.
- [x] **CH-5: Safe Path Discovery**
  - Prevent `fs::canonicalize` from crashing during discovery when target paths do not yet exist.

## Verification

- `just test -p toad-core`
- Manual verification of `TOAD_ROOT` override.
- Manual verification of cross-filesystem migration (if feasible in dev env).
