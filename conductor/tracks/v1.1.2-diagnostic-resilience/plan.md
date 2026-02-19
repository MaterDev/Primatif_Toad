# Plan: Diagnostic Resilience (v1.1.2-diagnostic-resilience)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Core Detection ✅ COMPLETE

- [x] Update strategy matching logic to differentiate between "No Match" and
      "Parse Failure".
- [x] Store parsing errors in the `ProjectDetail` or a transient diagnostic
      cache.

**Completed:**

- Created diagnostic type system in `toad-core`:
  - `ParseDiagnostic` - tracks individual parsing errors
  - `DiagnosticSeverity` - Error/Warning/Info classification
  - `DiagnosticReport` - collection with helper methods
- Implemented `detect_metadata_issues()` in `toad-discovery/src/scanner.rs`
- Added TOML parsing validation for `Cargo.toml`
- Added JSON parsing validation for `package.json`
- Added `toml = "0.8"` dependency to `toad-discovery`

**Files Created:**

- `crates/toad-core/src/models/diagnostics.rs`

**Files Modified:**

- `crates/toad-core/src/models/mod.rs`
- `crates/toad-core/src/lib.rs`
- `crates/toad-discovery/src/scanner.rs`
- `crates/toad-discovery/src/lib.rs`
- `crates/toad-discovery/Cargo.toml`

### Phase 2: User Reporting 🚧 IN PROGRESS

- [x] Add a "Metadata Health" section to `toad doctor`.
- [ ] **Update `toad status` to show a ⚠️ symbol next to projects with malformed
      metadata.**
- [ ] **Create test projects with malformed metadata for verification**

**Completed:**

- Enhanced `toad doctor` with diagnostic reporting section
- Added `diagnostics` field to `HealthReport` in `toad-ops`
- Diagnostic collection happens in `bin/toad/src/commands/doctor.rs`
- Shows error/warning icons and detailed error messages

**Files Modified:**

- `bin/toad/src/commands/doctor.rs`
- `crates/toad-ops/src/doctor.rs`

**Remaining:**

- Modify `toad status` output to display ⚠️ for projects with diagnostics
- Create test directory with intentionally malformed files
- Verify detection works end-to-end

### Phase 3: Documentation

- [ ] Update USER_GUIDE.md with diagnostic features
- [ ] Document how to interpret diagnostic messages

---

## Acceptance Criteria

- [x] Intentionally breaking a `Cargo.toml` results in a clear warning in
      `toad doctor`.
- [ ] `toad status` reports the project as malformed rather than silently
      defaulting to "Generic".
- [ ] USER_GUIDE.md documents diagnostic features.
