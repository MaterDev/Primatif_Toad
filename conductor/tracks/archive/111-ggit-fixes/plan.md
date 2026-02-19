# Plan: ggit Submodule Path Fixes (111-ggit-fixes)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 10 minutes
- **Target:** v1.1.0 (critical bug fix)
- **Priority:** P0 (Blocks multi-context workflows)

---

## Tasks

### Phase 1: Code Fixes

- [x] Review `bin/toad/src/commands/ggit.rs` lines 119-130 (`checkout`)
  - Verify `p.path.join(&sub.path)` is used
  - ✅ Already correct
- [x] Fix `bin/toad/src/commands/ggit.rs` line 179 (`sync` preflight)
  - Change `Some(&sub.path)` to `Some(&sub_path)`
- [x] Review `bin/toad/src/commands/ggit.rs` lines 272-282 (`branches`)
  - Verify `p.path.join(&sub.path)` is used
  - ✅ Already correct

### Phase 2: Testing

- [x] Build: `cargo build -p toad`
- [x] Test `ggit checkout` on hub project with submodules
- [x] Test `ggit sync` on hub project with submodules
- [x] Test `ggit branches` on hub project with submodules
- [x] Register external project and test all three commands

---

## Acceptance Criteria

- `ggit sync` preflight passes correct submodule path
- All ggit commands work for projects outside hub root
- No regressions in hub project workflows
