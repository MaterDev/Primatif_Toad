# Plan: Automated QA Suite (112-automated-qa-suite)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Sandbox Automation

- [ ] Create `scripts/tests/integration_qa.sh` to automate sandbox creation,
      usage, and cleanup.
- [ ] Implement assertions for CLI output strings.

### Phase 2: MCP Validation

- [ ] Create a test client for `toad-mcp` to verify tool accessibility.
- [ ] Validate JSON responses against schemas (to be defined).

### Phase 3: CI Integration

- [ ] Add `test-integration` to `Justfile`.
- [ ] Include integration tests in `just qa`.

---

## Acceptance Criteria

- `just test-integration` runs a full sandbox lifecycle and passes.
- MCP tools are verified to return valid, schema-compliant JSON.
