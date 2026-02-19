# Plan: Dogfooding Improvements (v1.1.1-dogfooding)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 40 minutes
- **Target:** v1.1.0 (improves development workflow)
- **Priority:** P2 (High value, not blocking)

---

## Tasks

### Phase 1: Apply Tags

- [x] Tag core crates: `toad tag toad-core --tag core --yes`
- [x] Tag intelligence crates: `toad-discovery`, `toad-manifest`
- [x] Tag orchestration crates: `toad-git`, `toad-ops`
- [x] Tag interface crates: `Primatif_Toad`, `bin/toad-mcp`
- [x] Tag utility crates: `toad-scaffold`
- [x] Verify tags: `toad status --tag core`

### Phase 2: Register Workflows

- [x] Create `scripts/workflows/` directory
- [x] Create `qa.sh` workflow
- [x] Create `release-check.sh` workflow
- [x] Create `update-docs.sh` workflow
- [x] Make scripts executable: `chmod +x scripts/workflows/*.sh`
- [x] Register workflows: `toad cw register <name> <path>`
- [x] Test workflows: `toad cw run qa`

### Phase 3: Add Git Hook

- [x] Create `scripts/git-hooks/post-commit`
- [x] Add auto-refresh logic
- [x] Update `Justfile` to install post-commit hook
- [x] Run `just setup-hooks` to install
- [x] Test: make a commit and verify manifest refreshes

### Phase 4: MCP Configuration

- [x] Document MCP setup in README or `.windsurf/`
- [x] Test MCP server with Toad workspace
- [x] Verify tools work for querying Toad's ecosystem

### Phase 5: Documentation

- [x] Add dogfooding section to README
- [x] Document workflow commands
- [x] Document tag taxonomy
- [x] Add examples of using Toad to develop Toad

---

## Acceptance Criteria

- All submodules tagged with appropriate taxonomy
- Common workflows registered and working
- Git hook auto-refreshes manifest after commits
- MCP server configured for development
- Documentation includes dogfooding examples
