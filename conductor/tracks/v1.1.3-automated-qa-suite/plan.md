# Plan: Automated QA Suite (v1.1.3-automated-qa-suite)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Sandbox Automation ✅ COMPLETE

- [x] Create `scripts/tests/integration_qa.sh` to automate sandbox creation,
      usage, and cleanup.
- [x] **Headless Execution**: CI mode detection via `CI` environment variable
      suppresses colors and uses plain text output.
- [x] Implement assertions for CLI output strings.

**Completed:**
- Created comprehensive integration test script with 10+ test suites
- Automated sandbox lifecycle (create, test, cleanup)
- CI-friendly output formatting
- Test assertion helpers (assert_success, assert_output_contains, assert_file_exists)
- Tests cover: version, help, sync, reveal, status, search, analytics, manifest, doctor

### Phase 2: MCP Validation

- [ ] Create a test client for `toad-mcp` to verify tool accessibility.
- [ ] Validate JSON responses against schemas (to be defined).

### Phase 3: CI Integration ✅ COMPLETE

- [x] Add `test-integration` to `Justfile`.
- [x] Include integration tests in `just qa`.

**Completed:**
- Added `just test-integration` command
- Integrated into `just qa` workflow
- Integration tests now run as part of full QA suite

---

## Acceptance Criteria

- `just test-integration` runs a full sandbox lifecycle and passes.
- MCP tools are verified to return valid, schema-compliant JSON.
