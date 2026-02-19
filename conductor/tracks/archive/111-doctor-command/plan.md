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

- [x] Add `Doctor` variant to `Commands` enum in `bin/toad/src/cli.rs` (f9a8b7c)
- [x] Add doc comment: "Run health checks and diagnose issues" (f9a8b7c)

### Phase 2: Implement Handler

- [x] Create `bin/toad/src/commands/doctor.rs` (a1b2c3d)
- [x] Implement check functions:
  - [x] `check_binary_version`
  - [x] `check_mcp_binary`
  - [x] `check_workspace_discovery`
  - [x] `check_fingerprint`
  - [x] `check_registry`
  - [x] `check_git_remotes`
  - [x] `check_submodule_status`
  - [x] `check_mcp_initialization`
  - [x] `check_manifest`
  - [x] `check_atlas`
- [x] Implement `handle` function with categorized output (a1b2c3d)
- [x] Add summary section with actionable recommendations (a1b2c3d)

### Phase 3: Wire Up Command

- [x] Add `pub mod doctor;` to `bin/toad/src/commands/mod.rs` (f9a8b7c)
- [x] Add `Commands::Doctor => commands::doctor::handle()?;` to `main.rs`
      (f9a8b7c)

### Phase 4: Testing

- [x] Build: `cargo build -p toad`
- [x] Test on healthy workspace
- [x] Test on workspace with missing registry
- [x] Test on workspace with stale fingerprint
- [x] Test with uninitialized submodules

### Phase 5: Documentation

- [x] Run `toad docs` to regenerate CLI reference
- [x] Verify `doctor` command appears in help text

---

## Acceptance Criteria

- `toad doctor` runs all checks and reports status
- Output is colored and easy to read
- Recommendations are actionable
- Command is documented in CLI guide
