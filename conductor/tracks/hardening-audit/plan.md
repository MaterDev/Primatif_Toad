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

- [ ] **Discovery Lockdown:**
  - [ ] Refactor `main.rs` to strictly enforce `Workspace::discover()` results
        for non-bootstrap commands.
  - [ ] Update `Workspace::discover()` to use iterative canonicalization for
        upward searching.
- [ ] **Panic Removal:**
  - [ ] Audit `toad-core` for all `expect()` and `unwrap()` calls.
  - [ ] Refactor `GlobalConfig` directory resolution to return
        `Result<PathBuf>`.
- [ ] **Audit Trail implementation:**
  - [ ] Create `crates/toad-ops/src/audit.rs`.
  - [ ] Implement JSON-L logging for `toad do` (timestamp, command,
        target_count, result_summary).
  - [ ] Ensure `~/.toad/ops.log` is created if missing.

### Phase 2: Automation & Hooks (High Priority)

- [ ] **Documentation Hook:**
  - [ ] Create `scripts/git-hooks/pre-push` with doc-diff logic.
  - [ ] Update `Justfile` with `setup-hooks` to include pre-push.
  - [ ] Update `scripts/install_toad.sh` to ensure all hooks are active.
- [ ] **Harvest Reliability:**
  - [ ] Add error checking to the `registry.save()` call in
        `toad tag --harvest`.
  - [ ] Ensure final output message reflects the actual success of the write
        operation.

### Phase 3: Logic & Resilience (Medium Priority)

- [ ] **Signal Resilience:**
  - [ ] Update `scripts/sandbox/run_ephemeral.sh` with
        `trap ... EXIT SIGINT SIGTERM SIGHUP`.
- [ ] **Taxonomy Hardening:**
  - [ ] Add documentation to `scan_all_projects` regarding the thread-safe
        immutable snapshot of the `TagRegistry`.

### Phase 4: Release v1.0.0 (Final)

- [ ] **Hygiene:**
  - [ ] Retroactively fix archived track checkboxes in `toad-tag/plan.md`.
- [ ] **Final RC Audit:**
  - [ ] Verify `docs/CLI.md` against v0.7.0 binary help.
  - [ ] Run full `just qa` one last time.
- [ ] **Major Bump:**
  - [ ] Release version **1.0.0**.
