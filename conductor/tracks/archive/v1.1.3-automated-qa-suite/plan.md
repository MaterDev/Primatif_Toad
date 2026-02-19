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

### Phase 2: MCP Validation ✅ COMPLETE

- [x] Create a test client for `toad-mcp` to verify tool accessibility.
- [x] Validate JSON responses against schemas (to be defined).

**Completed:**
- Created `scripts/tests/mcp_validation.sh` - comprehensive MCP test suite
- Automated MCP server initialization and tool discovery testing
- Validates JSON-RPC 2.0 protocol compliance
- Tests 8+ core MCP tools (list_projects, get_manifest, search_projects, etc.)
- Schema validation for tool responses and error handling
- Created `scripts/tests/mcp_schemas.json` with JSON Schema definitions
- Added `just test-mcp` command for easy execution
- CI-friendly output with color support detection

### Phase 3: CI Integration ✅ COMPLETE

- [x] Add `test-integration` to `Justfile`.
- [x] Include integration tests in `just qa`.

**Completed:**
- Added `just test-integration` command
- Integrated into `just qa` workflow
- Integration tests now run as part of full QA suite

---

## Acceptance Criteria

- ✅ `just test-integration` runs a full sandbox lifecycle and passes.
- ✅ `just test-mcp` validates MCP tools return valid, schema-compliant JSON.
- ✅ Both test suites integrated into CI workflow.

## Notes

Tests are invoked via `bash` explicitly in the Justfile, so no execute permissions are required.
