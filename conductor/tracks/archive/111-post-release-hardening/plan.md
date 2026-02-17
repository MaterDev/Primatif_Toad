# Plan: Post-Release Hardening (111-post-release-hardening)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 1.5 hours
- **Target:** v1.1.1 (stability)
- **Priority:** P0 (Critical CI fix + Architecture health)

---

## Tasks

### Phase 1: Critical Fixes

- [ ] Fix CI typo in `.github/workflows/ci.yml`
- [ ] Fix redundant sync in `bin/toad/src/main.rs`
- [ ] Fix safe path handling in `bin/toad/src/commands/doctor.rs`

### Phase 2: MCP Refactoring

- [ ] Create sub-modules in `bin/toad-mcp/src/tools/`
  - [ ] `discovery.rs`
  - [ ] `context.rs`
  - [ ] `management.rs`
  - [ ] `analysis.rs`
- [ ] Refactor `bin/toad-mcp/src/server.rs` to delegate to sub-modules
- [ ] Implement `StatsParams::all` logic or remove dead field

### Phase 3: Diagnostic Improvements

- [ ] Update `toad-ops::doctor` to include detailed registry errors
- [ ] Update `toad doctor` UI to display registry errors

### Phase 4: Verification

- [ ] Build all modules: `just build`
- [ ] Run QA suite: `just qa`
- [ ] Manual test: `toad doctor`
- [ ] Manual test: Verify MCP tool listing

---

## Acceptance Criteria

- CI parses and runs correctly.
- `bin/toad-mcp/src/server.rs` is under 700 lines.
- `toad doctor` does not trigger redundant sync.
- All `.unwrap()` calls in CLI handlers are replaced with safe handling.
- `just qa` passes.
