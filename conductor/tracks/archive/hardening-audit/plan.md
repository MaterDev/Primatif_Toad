# Track-013: Production Hardening & Audit Trail

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Finalize the v1.0.0 "Release Candidate" by hardening core discovery logic,
implementing a persistent audit trail, and automating documentation integrity.

## Deliverables

### Phase 1: System Integrity (High Priority)

- [x] **Discovery Lockdown:**
  - [x] Refactor `main.rs` to strictly enforce `Workspace::discover()` results
        for non-bootstrap commands.
  - [x] Update `Workspace::discover()` to use iterative canonicalization for
        upward searching.
- [x] **Panic Removal:**
  - [x] Audit `toad-core` for all `expect()` and `unwrap()` calls.
  - [x] Refactor `GlobalConfig` directory resolution to return
        `Result<PathBuf>`.
- [x] **Audit Trail implementation:**
  - [x] Create `crates/toad-ops/src/audit.rs`.
  - [x] Implement JSON-L logging for `toad do` (timestamp, command,
        target_count, result_summary).
  - [x] Ensure `~/.toad/ops.log` is created if missing.

### Phase 2: Automation & Hooks (High Priority)

- [x] **Documentation Hook:**
  - [x] Create `scripts/git-hooks/pre-push` with doc-diff logic.
  - [x] Update `Justfile` with `setup-hooks` to include pre-push.
  - [x] Update `scripts/install_toad.sh` to ensure all hooks are active.
- [x] **Harvest Reliability:**
  - [x] Add error checking to the `registry.save()` call in
        `toad tag --harvest`.
  - [x] Ensure final output message reflects the actual success of the write
        operation.

### Phase 3: Logic & Resilience (Medium Priority)

- [x] **Signal Resilience:**
  - [x] Update `scripts/sandbox/run_ephemeral.sh` with
        `trap ... EXIT SIGINT SIGTERM SIGHUP`.
- [x] **Taxonomy Hardening:**
  - [x] Add documentation to `scan_all_projects` regarding the thread-safe
        immutable snapshot of the `TagRegistry`.

### Phase 4: Release v1.0.0 (Final)

- [x] **Hygiene:**

  - [x] Retroactively fix archived track checkboxes in `toad-tag/plan.md`.

- [x] **Final RC Audit:**

  - [x] Verify `docs/CLI.md` against v1.0.0 binary help.

  - [x] Run full `just qa` one last time.

- [x] **Major Bump:**

  - [x] Release version **1.0.0**.
