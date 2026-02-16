# Plan: `toad doctor` Command (111-doctor-command)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 30 minutes
- **Target:** v1.1.0 (optional, high DX value)
- **Priority:** P1 (High impact, moderate effort)

---

## Tasks

### Phase 1: Add Command to CLI

- [ ] Add `Doctor` variant to `Commands` enum in `bin/toad/src/cli.rs`
- [ ] Add doc comment: "Run health checks and diagnose issues"

### Phase 2: Implement Handler

- [ ] Create `bin/toad/src/commands/doctor.rs`
- [ ] Implement check functions:
  - [ ] `check_binary_version`
  - [ ] `check_mcp_binary`
  - [ ] `check_workspace_discovery`
  - [ ] `check_fingerprint`
  - [ ] `check_registry`
  - [ ] `check_git_remotes`
  - [ ] `check_submodule_status`
  - [ ] `check_mcp_initialization`
  - [ ] `check_manifest`
  - [ ] `check_atlas`
- [ ] Implement `handle` function with categorized output
- [ ] Add summary section with actionable recommendations

### Phase 3: Wire Up Command

- [ ] Add `pub mod doctor;` to `bin/toad/src/commands/mod.rs`
- [ ] Add `Commands::Doctor => commands::doctor::handle()?;` to `main.rs`

### Phase 4: Testing

- [ ] Build: `cargo build -p toad`
- [ ] Test on healthy workspace
- [ ] Test on workspace with missing registry
- [ ] Test on workspace with stale fingerprint
- [ ] Test with uninitialized submodules

### Phase 5: Documentation

- [ ] Run `toad docs` to regenerate CLI reference
- [ ] Verify `doctor` command appears in help text

---

## Acceptance Criteria

- `toad doctor` runs all checks and reports status
- Output is colored and easy to read
- Recommendations are actionable
- Command is documented in CLI guide
